#!/usr/bin/env node
/**
 * Splits daisyui-docs.md (llms.txt) into one .md file per component/section.
 * Output goes to /tmp/daisyui-split/
 *
 * Structure of llms.txt:
 *   ## Section title        → top-level sections (intro, install, rules…)
 *   ### component-name      → one file per component
 */

import { readFileSync, writeFileSync, mkdirSync } from "fs";
import { join } from "path";

const [, , inputFile, outputDir] = process.argv;
if (!inputFile || !outputDir) {
  console.error("Usage: node split-daisyui.mjs <input.md> <output-dir>");
  process.exit(1);
}

const raw = readFileSync(inputFile, "utf8");
const lines = raw.split("\n");

mkdirSync(outputDir, { recursive: true });

/**
 * Slugify a heading title to a safe filename.
 */
function toSlug(title) {
  return title
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
}

/**
 * Write a section to disk.
 */
function writeSection(slug, content) {
  if (!slug || !content.trim()) return;
  const file = join(outputDir, `${slug}.md`);
  writeFileSync(file, content.trimEnd() + "\n", "utf8");
  console.log(`  → ${slug}.md`);
}

// ── Parse ──────────────────────────────────────────────────────────────────

let currentSlug = null;
let currentLines = [];

// Collect the preamble (everything before the first ## heading) into "overview"
let inPreamble = true;

for (const line of lines) {
  const h3 = line.match(/^### (.+)/);
  const h2 = line.match(/^## (.+)/);

  if (h3) {
    // Save previous section
    writeSection(currentSlug, currentLines.join("\n"));
    currentSlug = toSlug(h3[1]);
    currentLines = [line];
    inPreamble = false;
  } else if (h2) {
    // Save previous section
    writeSection(currentSlug, currentLines.join("\n"));
    currentSlug = toSlug(h2[1]);
    currentLines = [line];
    inPreamble = false;
  } else {
    if (inPreamble) {
      // Still before the first heading — accumulate into "overview"
      if (!currentSlug) {
        currentSlug = "overview";
        currentLines = [];
      }
    }
    currentLines.push(line);
  }
}

// Flush last section
writeSection(currentSlug, currentLines.join("\n"));

console.log("Done.");
