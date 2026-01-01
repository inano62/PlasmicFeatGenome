import json, os, requests
from Bio import SeqIO

LLAMA_URL = os.getenv("LLAMA_URL", "http://llama-service:8000").rstrip("/")

def llm(prompt: str, max_tokens: int = 300) -> str:
    r = requests.post(f"{LLAMA_URL}/generate", json={"prompt": prompt, "max_tokens": max_tokens}, timeout=300)
    r.raise_for_status()
    return r.json()["choices"][0]["text"]

def seq_stats(seq: str):
    seq = seq.upper().replace("\n","").replace("\r","")
    length = len(seq)
    gc = (seq.count("G") + seq.count("C")) / length * 100 if length else 0
    return {"length": length, "gc_percent": round(gc, 2)}

def main():
    fasta_path = os.getenv("FASTA_PATH", "/data/fasta/FOXP2.fasta")

    recs = list(SeqIO.parse(fasta_path, "fasta"))
    if not recs:
        raise SystemExit(f"no records in fasta: {fasta_path}")

    # 先頭レコードだけでデモ（複数ある場合は拡張）
    r0 = recs[0]
    s = str(r0.seq)

    facts = {
        "gene": "FOXP2",
        "fasta_path": fasta_path,
        "record_id": r0.id,
        "record_description": r0.description,
        "sequence": seq_stats(s),
        "note": "FASTAはユーザーがローカルに用意した入力。解析はこの入力の範囲のみ。"
    }

    prompt = f"""あなたは生命科学の解説AIです。
次のJSONはBioPythonで計算した“事実”です。事実にない内容は捏造しないでください。

JSON:
{json.dumps(facts, ensure_ascii=False, indent=2)}

出力指示:
- まず、このFASTAから言えること（長さ・GCなど）を短く
- 次に、FOXP2が研究対象として何で知られるか（一般論）を慎重に
- 最後に、このFASTAだけでは分からないこと（変異の意味、疾患断定など）を明示
- 日本語で250〜450字
"""

    print(llm(prompt, max_tokens=350))

if __name__ == "__main__":
    main()
