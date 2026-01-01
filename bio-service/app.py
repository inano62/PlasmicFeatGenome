from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from Bio import SeqIO
from Bio.SeqUtils import gc_fraction
from io import StringIO
import os, requests

app = FastAPI()

LLAMA_URL = os.getenv("LLAMA_URL", "http://llama-service:8000").rstrip("/")

class FastaReq(BaseModel):
    fasta: str
    prompt: str | None = None
    max_tokens: int = 256

@app.get("/healthz")
def healthz():
    return {"ok": True}

@app.post("/foxp2/analyze")
def foxp2_analyze(req: FastaReq):
    # 1) FASTA parse
    try:
        recs = list(SeqIO.parse(StringIO(req.fasta.strip()), "fasta"))
    except Exception as e:
        raise HTTPException(status_code=400, detail=f"FASTA parse error: {e}")

    if not recs:
        raise HTTPException(status_code=400, detail="No FASTA records")

    # 2) 単一（まずは1本だけ解析）
    r = recs[0]
    seq = str(r.seq).upper().replace(" ", "").replace("\n", "")
    length = len(seq)
    gc = gc_fraction(seq) * 100.0

    # 3) LLM用の“事実だけ”サマリ素材
    facts = (
        f"FASTA id={r.id}\n"
        f"length_nt={length}\n"
        f"gc_percent={gc:.2f}\n"
        f"sequence_head={seq[:60]}\n"
    )

    # 4) llamaに要約させる（任意）
    summary = None
    if req.prompt is not None and req.prompt.strip():
        llm_prompt = (
            "あなたはバイオ情報の補助AIです。推測や断定を避け、"
            "与えられた数値・事実のみを日本語で簡潔に説明してください。\n\n"
            f"[FACTS]\n{facts}\n\n"
            f"[USER_REQUEST]\n{req.prompt}\n"
        )
        try:
            resp = requests.post(
                f"{LLAMA_URL}/generate",
                json={"prompt": llm_prompt, "max_tokens": req.max_tokens},
                timeout=300,
            )
            if resp.status_code != 200:
                raise HTTPException(status_code=502, detail=f"llama error: {resp.text}")
            summary = resp.json()["choices"][0]["text"]
        except requests.RequestException as e:
            raise HTTPException(status_code=502, detail=f"llama request failed: {e}")

    return {
        "record_id": r.id,
        "length_nt": length,
        "gc_percent": round(gc, 2),
        "facts": facts,
        "summary": summary,
    }
