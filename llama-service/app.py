import os
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

app = FastAPI()

@app.get("/healthz")
def healthz():
    return {"ok": True}

class Req(BaseModel):
    prompt: str
    max_tokens: int = 128

_llm = None

def get_llm():
    global _llm
    if _llm is not None:
        return _llm

    model_path = os.getenv("MODEL_PATH")
    if not model_path:
        raise HTTPException(503, "MODEL_PATH is not set")
    if not os.path.exists(model_path):
        raise HTTPException(503, f"model file not found: {model_path}")

    from llama_cpp import Llama
    _llm = Llama(model_path=model_path, n_ctx=2048)
    return _llm

@app.post("/generate")
def generate(req: Req):
    llm = get_llm()
    out = llm(req.prompt, max_tokens=req.max_tokens)
    return out
