import os
import requests
from fastapi import FastAPI
from pydantic import BaseModel
from Bio.SeqUtils import gc_fraction  # 例として GC 含量を使う

app = FastAPI()

LLAMA_URL = os.getenv("LLAMA_URL", "http://llama-service:8000")

class GenomeRequest(BaseModel):
    seq: str

class GenomeResponse(BaseModel):
    gc_content: float
    llama_comment: str

@app.post("/analyze", response_model=GenomeResponse)
def analyze(req: GenomeRequest):
    # 1) まず Llama に投げる
    llama_resp = requests.post(
        f"{LLAMA_URL}/llama",
        json={"seq": req.seq},
        timeout=60,
    )
    llama_resp.raise_for_status()
    llama_comment = llama_resp.json()["comment"]

    # 2) BioPython で解析（ここはお好みで拡張）
    gc = gc_fraction(req.seq)  # 0.0〜1.0 の GC 含量

    return GenomeResponse(
        gc_content=gc,
        llama_comment=llama_comment,
    )
