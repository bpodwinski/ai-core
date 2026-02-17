#!/usr/bin/env python3
import os, json, ssl, imaplib, argparse

def main():
    p = argparse.ArgumentParser()
    p.add_argument("--host", default=os.getenv("IMAP_HOST", "mail.infomaniak.com"))
    p.add_argument("--port", type=int, default=int(os.getenv("IMAP_PORT", "993")))
    p.add_argument("--user", default=os.getenv("IMAP_USER"))
    p.add_argument("--pass", dest="password", default=os.getenv("IMAP_PASS"))

    # 👇 dossier IMAP ciblé (priorité: --mailbox, sinon IMAP_MAILBOX, sinon INBOX)
    p.add_argument("--mailbox", default=os.getenv("IMAP_MAILBOX", "INBOX"))

    # Critères IMAP (ALL, UNSEEN, (SINCE "01-Feb-2026"), etc.)
    p.add_argument("--criteria", default=os.getenv("IMAP_CRITERIA", "ALL"))

    args = p.parse_args()

    if not args.user or not args.password:
        raise SystemExit("IMAP_USER / IMAP_PASS manquants.")

    ctx = ssl.create_default_context()
    imap = imaplib.IMAP4_SSL(args.host, args.port, ssl_context=ctx)
    imap.login(args.user, args.password)

    typ, _ = imap.select(args.mailbox, readonly=True)
    if typ != "OK":
        imap.logout()
        raise SystemExit(f"Cannot select mailbox: {args.mailbox}")

    typ, data = imap.uid("SEARCH", None, args.criteria)
    if typ != "OK":
        imap.logout()
        raise SystemExit(f"SEARCH failed for mailbox={args.mailbox}, criteria={args.criteria}")

    total = len(data[0].split()) if data and data[0] else 0
    imap.logout()

    print(json.dumps(
        {"total": total, "mailbox": args.mailbox, "criteria": args.criteria},
        ensure_ascii=False
    ))

if __name__ == "__main__":
    main()
