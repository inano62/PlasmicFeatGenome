# bio-service/app.py
from fastapi import FastAPI
from pydantic import BaseModel
from Bio.SeqUtils import gc_fraction
import os
import requests

app = FastAPI()

class AnalyzeRequest(BaseModel):
    seq: str

class AnalyzeResponse(BaseModel):
    gc_content: float
    llama_comment: str

@app.post("/analyze", response_model=AnalyzeResponse)
def analyze(req: AnalyzeRequest):
    # 1) GC 含量を計算
    gc = float(gc_fraction(req.seq))

    # 2) Llama サービス URL（デフォルト値つき）
    llama_url = os.getenv("LLAMA_URL", "http://llama-service:8000")

    try:
        resp = requests.post(
            f"{llama_url}/generate",   # ★ /llama → /generate に
            json={"prompt": req.seq},
            timeout=5,
        )
        resp.raise_for_status()
        data = resp.json()
        comment = data.get("output", str(data))
    except Exception as e:
        # 例外は握りつぶして文字列にして返す（ここで 500 にはしない）
        comment = f"llama error: {e}"

    return AnalyzeResponse(gc_content=gc, llama_comment=comment)
