use anyhow::{Context, Result};
use std::path::Path;
use walkdir::WalkDir;

pub fn strip_dir(dir: &Path) -> Result<()> {
    println!("Traitement MDX → Markdown dans : {}", dir.display());
    let mut processed = 0usize;
    let mut errors = 0usize;

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("mdx") {
            continue;
        }
        match strip_file(path) {
            Ok(()) => processed += 1,
            Err(e) => {
                eprintln!("  ✗ {}: {}", path.display(), e);
                errors += 1;
            }
        }
    }

    println!(
        "Terminé : {} fichiers convertis, {} erreurs.",
        processed, errors
    );
    Ok(())
}

fn strip_file(path: &Path) -> Result<()> {
    let source =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let normalized = source.replace("\r\n", "\n");
    let md = strip_mdx(&normalized);
    let out = path.with_extension("md");
    std::fs::write(&out, md).with_context(|| format!("writing {}", out.display()))?;
    Ok(())
}

/// Strip MDX-specific constructs from `src`, returning clean CommonMark.
///
/// Handles the subset present in our doc sources (Tailwind mdx):
///   - top-level `import`/`export` statements (until matching `;` at depth 0)
///   - top-level `{expression}` blocks (brace-balanced)
///   - JSX elements: `<Tag attrs...>…</Tag>` unwrapped to inner content;
///     self-closing `<Tag ... />` dropped.
pub fn strip_mdx(src: &str) -> String {
    let bytes = src.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(src.len());
    let mut i = 0;
    let mut at_line_start = true;

    while i < bytes.len() {
        let c = bytes[i];

        if at_line_start {
            let mut j = i;
            while j < bytes.len() && matches!(bytes[j], b' ' | b'\t') {
                j += 1;
            }
            if starts_with(bytes, j, b"import ") || starts_with(bytes, j, b"export ") {
                i = skip_esm_statement(bytes, j);
                while i < bytes.len() && bytes[i] == b'\n' {
                    i += 1;
                }
                at_line_start = true;
                continue;
            }
            if j < bytes.len() && bytes[j] == b'{' {
                if let Some(end) = skip_braced(bytes, j) {
                    i = end;
                    while i < bytes.len() && bytes[i] == b'\n' {
                        i += 1;
                    }
                    at_line_start = true;
                    continue;
                }
            }
        }

        if c == b'<' && is_jsx_tag_start(bytes, i) {
            let (kind, tag_end) = parse_jsx_tag(bytes, i);
            match kind {
                JsxKind::SelfClosing => {
                    i = tag_end;
                    // also consume a trailing newline if the tag was alone on its line
                    if at_line_start && i < bytes.len() && bytes[i] == b'\n' {
                        i += 1;
                    }
                    at_line_start = i > 0 && bytes[i.saturating_sub(1)] == b'\n';
                    continue;
                }
                JsxKind::Opening(name) => {
                    // Skip the opening tag (and trailing newline if it was alone).
                    let mut cursor = tag_end;
                    if at_line_start && cursor < bytes.len() && bytes[cursor] == b'\n' {
                        cursor += 1;
                    }
                    // Emit nothing for the tag itself — recursively keep content.
                    // We simply continue parsing; the matching closing tag </name>
                    // is detected by is_jsx_closing.
                    i = cursor;
                    at_line_start = i == 0 || bytes[i - 1] == b'\n';
                    // We don't need to track the stack because each closing tag
                    // handles itself below; we let the parse continue.
                    let _ = name;
                    continue;
                }
                JsxKind::Closing => {
                    let mut cursor = tag_end;
                    if at_line_start && cursor < bytes.len() && bytes[cursor] == b'\n' {
                        cursor += 1;
                    }
                    i = cursor;
                    at_line_start = i == 0 || bytes[i - 1] == b'\n';
                    continue;
                }
                JsxKind::NotJsx => {}
            }
        }

        out.push(c);
        at_line_start = c == b'\n';
        i += 1;
    }

    let as_str =
        String::from_utf8(out).expect("valid UTF-8: input was UTF-8 and skips are ASCII-bounded");
    let collapsed = collapse_blank_lines(&as_str);
    let trimmed = collapsed.trim_start_matches('\n').trim_end().to_string();
    format!("{}\n", trimmed)
}

fn starts_with(bytes: &[u8], i: usize, needle: &[u8]) -> bool {
    bytes.len() >= i + needle.len() && &bytes[i..i + needle.len()] == needle
}

/// Advance past an `import …` or `export …` statement, consuming string
/// literals and balanced braces/brackets/parens so semicolons inside them
/// don't terminate early. Stops at `;` at depth 0 or at a blank line at depth 0.
fn skip_esm_statement(bytes: &[u8], start: usize) -> usize {
    let mut i = start;
    let mut depth: i32 = 0;
    while i < bytes.len() {
        let c = bytes[i];
        match c {
            b'"' | b'\'' | b'`' => {
                i = skip_js_string(bytes, i);
                continue;
            }
            b'{' | b'[' | b'(' => depth += 1,
            b'}' | b']' | b')' => depth -= 1,
            b';' if depth == 0 => return i + 1,
            b'\n' if depth == 0 => {
                // Blank line (\n\n) terminates the statement even without ';'.
                if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                    return i + 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    i
}

fn skip_js_string(bytes: &[u8], start: usize) -> usize {
    let quote = bytes[start];
    let mut i = start + 1;
    while i < bytes.len() {
        let c = bytes[i];
        if c == b'\\' && i + 1 < bytes.len() {
            i += 2;
            continue;
        }
        if c == quote {
            return i + 1;
        }
        if quote == b'`' && c == b'$' && i + 1 < bytes.len() && bytes[i + 1] == b'{' {
            // Skip template interpolation ${...}
            if let Some(end) = skip_braced(bytes, i + 1) {
                i = end;
                continue;
            }
        }
        i += 1;
    }
    i
}

/// Given the byte index of `{`, return the index just after the matching `}`.
fn skip_braced(bytes: &[u8], start: usize) -> Option<usize> {
    debug_assert_eq!(bytes[start], b'{');
    let mut depth: i32 = 0;
    let mut i = start;
    while i < bytes.len() {
        let c = bytes[i];
        match c {
            b'"' | b'\'' | b'`' => {
                i = skip_js_string(bytes, i);
                continue;
            }
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i + 1);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

/// Returns true if bytes[i] starts a JSX tag: `<` followed by a capital letter,
/// or `</` followed by a capital letter. Standard HTML tags (lowercase) are
/// left alone (they stay as HTML in markdown).
fn is_jsx_tag_start(bytes: &[u8], i: usize) -> bool {
    if bytes[i] != b'<' {
        return false;
    }
    let mut j = i + 1;
    if j < bytes.len() && bytes[j] == b'/' {
        j += 1;
    }
    if j >= bytes.len() {
        return false;
    }
    let c = bytes[j];
    c.is_ascii_uppercase()
}

enum JsxKind {
    Opening(usize),
    Closing,
    SelfClosing,
    NotJsx,
}

/// Parse a JSX tag starting at `<`. Returns kind and index after `>`.
fn parse_jsx_tag(bytes: &[u8], start: usize) -> (JsxKind, usize) {
    let mut i = start + 1;
    let is_closing = i < bytes.len() && bytes[i] == b'/';
    if is_closing {
        i += 1;
    }

    let name_start = i;
    while i < bytes.len()
        && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'.')
    {
        i += 1;
    }
    let name_len = i - name_start;
    if name_len == 0 {
        return (JsxKind::NotJsx, start + 1);
    }

    // Scan until matching `>`, respecting strings and balanced braces.
    while i < bytes.len() {
        let c = bytes[i];
        match c {
            b'"' | b'\'' | b'`' => {
                i = skip_js_string(bytes, i);
                continue;
            }
            b'{' => {
                if let Some(end) = skip_braced(bytes, i) {
                    i = end;
                    continue;
                } else {
                    return (JsxKind::NotJsx, start + 1);
                }
            }
            b'/' if i + 1 < bytes.len() && bytes[i + 1] == b'>' => {
                return (JsxKind::SelfClosing, i + 2);
            }
            b'>' => {
                return (
                    if is_closing {
                        JsxKind::Closing
                    } else {
                        JsxKind::Opening(name_len)
                    },
                    i + 1,
                );
            }
            _ => {}
        }
        i += 1;
    }
    (JsxKind::NotJsx, start + 1)
}

fn collapse_blank_lines(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut newlines = 0usize;
    for c in s.chars() {
        if c == '\n' {
            newlines += 1;
            if newlines <= 2 {
                out.push(c);
            }
        } else {
            newlines = 0;
            out.push(c);
        }
    }
    out
}
