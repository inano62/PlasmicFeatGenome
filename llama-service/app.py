# llama-service/app.py
from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI()

class LlamaRequest(BaseModel):
    prompt: str

class LlamaResponse(BaseModel):
    output: str

@app.post("/generate", response_model=LlamaResponse)
async def generate(req: LlamaRequest):
    return LlamaResponse(output=f"[LLAMA ECHO] {req.prompt}")
