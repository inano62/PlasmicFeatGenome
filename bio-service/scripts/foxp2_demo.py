import json, os, requests
from Bio.Seq import Seq

LLAMA_URL = os.getenv("LLAMA_URL", "http://llama-service:8000").rstrip("/")

def call_llm(prompt: str, max_tokens: int = 256) -> str:
    r = requests.post(f"{LLAMA_URL}/generate", json={"prompt": prompt, "max_tokens": max_tokens}, timeout=300)
    r.raise_for_status()
    return r.json()["choices"][0]["text"]

def main():
    # ここは“デモ用の超短い配列”（本物のFOXP2配列ではない）
    seq = Seq("ATGCGTACGTTAGCGTACGTTAGC")
    facts = {
        "gene": "FOXP2",
        "demo_seq_length": len(seq),
        "demo_seq_gc_percent": round(100 * (seq.count("G")+seq.count("C")) / len(seq), 2),
        "note": "This is a toy sequence for pipeline testing, not an actual FOXP2 sequence."
    }

    prompt = f"""あなたは遺伝学の解説者です。
以下はBioPythonで計算された“事実データ(JSON)”です。捏造せず、この範囲だけで説明してください。

事実データ:
{json.dumps(facts, ensure_ascii=False, indent=2)}

指示:
- 何が分かって、何が分からないかを分けて
- FOXP2について一般に知られる話題（言語・発話関連で研究対象）に触れてよいが、断定は避ける
- 200〜400字で日本語
"""

    out = call_llm(prompt, max_tokens=256)
    print(out)

if __name__ == "__main__":
    main()
