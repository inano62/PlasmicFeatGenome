from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI()

class GenReq(BaseModel):
    prompt: str

@app.post("/generate")
def generate(req: GenReq):
    return {"output": f"[mock] received {len(req.prompt)} chars"}
