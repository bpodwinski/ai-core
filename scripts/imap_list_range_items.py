#!/usr/bin/env python3
"""
imap_list_range_items.py

Retourne une LISTE JSON (tableau) de mails, du plus récent au plus ancien,
avec pagination offset/limit.

Ex:
- offset=0 limit=100 => 100 plus récents
- offset=100 limit=100 => 100 suivants
"""

import os, json, ssl, imaplib, argparse
from email import message_from_bytes
from email.header import decode_header

def dh(v: str) -> str:
    if not v:
        return ""
    parts = decode_header(v)
    out = []
    for b, enc in parts:
        if isinstance(b, bytes):
            out.append(b.decode(enc or "utf-8", errors="replace"))
        else:
            out.append(str(b))
    return "".join(out)

def get_text_plain(msg) -> str:
    if msg.is_multipart():
        for part in msg.walk():
            disp = (part.get("Content-Disposition") or "").lower()
            if "attachment" in disp:
                continue
            if part.get_content_type() == "text/plain":
                payload = part.get_payload(decode=True) or b""
                charset = part.get_content_charset() or "utf-8"
                return payload.decode(charset, errors="replace")
        return ""
    payload = msg.get_payload(decode=True) or b""
    charset = msg.get_content_charset() or "utf-8"
    return payload.decode(charset, errors="replace")

def main():
    p = argparse.ArgumentParser()
    p.add_argument("--host", default=os.getenv("IMAP_HOST", "mail.infomaniak.com"))
    p.add_argument("--port", type=int, default=int(os.getenv("IMAP_PORT", "993")))
    p.add_argument("--user", default=os.getenv("IMAP_USER"))
    p.add_argument("--pass", dest="password", default=os.getenv("IMAP_PASS"))
    p.add_argument("--mailbox", default=os.getenv("IMAP_MAILBOX", "INBOX"))
    p.add_argument("--offset", type=int, default=0)
    p.add_argument("--limit", type=int, default=100)
    p.add_argument("--criteria", default=os.getenv("IMAP_CRITERIA", "ALL"))
    p.add_argument("--body-max", type=int, default=int(os.getenv("IMAP_BODY_MAX", "0")),
                   help="0=pas de body, sinon tronque à N chars (ex 4000)")
    args = p.parse_args()

    if not args.user or not args.password:
        raise SystemExit("IMAP_USER / IMAP_PASS manquants.")

    ctx = ssl.create_default_context()
    imap = imaplib.IMAP4_SSL(args.host, args.port, ssl_context=ctx)
    imap.login(args.user, args.password)
    typ, _ = imap.select(args.mailbox, readonly=True)
    if typ != "OK":
        raise SystemExit(f"Cannot select mailbox {args.mailbox}")

    typ, data = imap.uid("SEARCH", None, args.criteria)
    if typ != "OK":
        raise SystemExit(f"SEARCH failed: {typ}")

    uids = data[0].split() if data and data[0] else []
    uids_recent = list(reversed(uids))
    page = uids_recent[args.offset: args.offset + args.limit]

    fetch_what = "(BODY.PEEK[HEADER])" if args.body_max == 0 else "(RFC822)"

    out = []
    for uid_b in page:
        uid = uid_b.decode()
        typ, msg_data = imap.uid("FETCH", uid, fetch_what)
        if typ != "OK" or not msg_data:
            continue

        raw = None
        for part in msg_data:
            if isinstance(part, tuple) and part[1]:
                raw = part[1]
                break
        if not raw:
            continue

        msg = message_from_bytes(raw)
        item = {
            "uid": uid,
            "mailbox": args.mailbox,
            "subject": dh(msg.get("Subject", "")),
            "from": dh(msg.get("From", "")),
            "date": msg.get("Date", ""),
            "message_id": msg.get("Message-ID", ""),
            "page_offset": args.offset,
            "page_limit": args.limit,
        }

        if args.body_max != 0:
            txt = (get_text_plain(msg) or "").strip()
            if args.body_max > 0 and len(txt) > args.body_max:
                txt = txt[: args.body_max] + "\n…(truncated)…"
            item["text"] = txt

        out.append(item)

    imap.logout()
    print(json.dumps(out, ensure_ascii=False))

if __name__ == "__main__":
    main()
