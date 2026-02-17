from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import subprocess
import json

app = FastAPI()


class ListReq(BaseModel):
    offset: int = 0
    limit: int = 100
    body_max: int = 0
    criteria: str = "ALL"
    mailbox: str = "INBOX"


class CountReq(BaseModel):
    criteria: str = "ALL"
    mailbox: str = "INBOX"


def run_json(cmd: list[str]):
    try:
        out = subprocess.check_output(cmd, text=True, stderr=subprocess.STDOUT)
        return json.loads(out)
    except subprocess.CalledProcessError as e:
        raise HTTPException(status_code=500, detail=e.output)
    except json.JSONDecodeError as e:
        raise HTTPException(status_code=500, detail=f"Invalid JSON from script: {e}")


@app.get("/count")
def count(mailbox: str = "INBOX", criteria: str = "ALL"):
    return run_json(["python", "/scripts/imap_count.py", "--mailbox", mailbox, "--criteria", criteria])


@app.post("/count")
def count_post(req: CountReq):
    return run_json(["python", "/scripts/imap_count.py", "--mailbox", req.mailbox, "--criteria", req.criteria])


@app.get("/list")
def list_get(offset: int = 0, limit: int = 100, body_max: int = 0, criteria: str = "ALL", mailbox: str = "INBOX"):
    cmd = [
        "python", "/scripts/imap_list_range_items.py",
        "--offset", str(offset),
        "--limit", str(limit),
        "--body-max", str(body_max),
        "--criteria", criteria,
        "--mailbox", mailbox,
    ]
    return run_json(cmd)


@app.post("/list")
def list_emails(req: ListReq):
    cmd = [
        "python", "/scripts/imap_list_range_items.py",
        "--offset", str(req.offset),
        "--limit", str(req.limit),
        "--body-max", str(req.body_max),
        "--criteria", req.criteria,
        "--mailbox", req.mailbox,
    ]
    return run_json(cmd)
