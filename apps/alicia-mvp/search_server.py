"""
Serveur de recherche web pour Alicia (via DuckDuckGo).

Endpoints :
  POST /api/search  -> { "query": "..." } -> { results: [...] }
  GET  /api/health  -> { status: "ok" }

Installation :
  pip install ddgs flask

Lancement :
  python search_server.py
  python search_server.py --port 8103
"""

import argparse
import time
from flask import Flask, request, jsonify

app = Flask(__name__)


@app.route("/api/health", methods=["GET"])
def health():
    return jsonify({"status": "ok", "service": "alicia-web-search"})


@app.route("/api/search", methods=["POST"])
def search():
    data = request.get_json(force=True)
    query = data.get("query", "").strip()
    max_results = data.get("max_results", 5)

    if not query:
        return jsonify({"error": "query vide"}), 400

    print(f"  [Search] '{query}' (max={max_results})", flush=True)

    t0 = time.time()
    try:
        from ddgs import DDGS
        with DDGS() as ddgs:
            raw = list(ddgs.text(query, max_results=max_results))

        results = []
        for r in raw:
            results.append({
                "title": r.get("title", ""),
                "url": r.get("href", ""),
                "snippet": r.get("body", ""),
            })

        duration_ms = int((time.time() - t0) * 1000)
        print(f"  [Search] {len(results)} resultats en {duration_ms}ms", flush=True)

        return jsonify({
            "query": query,
            "results": results,
            "count": len(results),
            "duration_ms": duration_ms,
        })

    except Exception as e:
        print(f"  [Search] Erreur : {e}", flush=True)
        return jsonify({"error": str(e)}), 500


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Serveur recherche web Alicia")
    parser.add_argument("--port", type=int, default=8103, help="Port HTTP")
    args = parser.parse_args()

    print(f"Serveur Web Search en ecoute sur http://127.0.0.1:{args.port}")
    app.run(host="127.0.0.1", port=args.port, debug=False)
