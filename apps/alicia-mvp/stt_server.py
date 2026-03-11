"""
Serveur STT local utilisant faster-whisper avec GPU CUDA.

Expose l'API compatible avec le contrat MiyuSTT :
  POST /api/stt   -> { transcript, confidence, duration_ms, engine, model_used, language_detected }
  GET  /api/health -> { status: "ok" }

Installation :
  pip install faster-whisper flask numpy

Lancement :
  python stt_server.py                          # small, GPU auto
  python stt_server.py --model medium           # medium, plus precis
  python stt_server.py --model large-v3         # large, max qualite
  python stt_server.py --device cpu             # forcer CPU

RTX 4060m (8GB VRAM) : small = ~1.5GB VRAM, medium = ~3GB, large-v3 = ~6GB
"""

import argparse
import base64
import time
import struct
import json
import numpy as np
from flask import Flask, request, jsonify

app = Flask(__name__)
model = None
model_name = "small"


def load_model(name: str, device: str, compute_type: str):
    global model, model_name
    from faster_whisper import WhisperModel

    model_name = name
    print(f"Chargement du modele faster-whisper '{name}' sur {device} ({compute_type})...")
    t0 = time.time()
    model = WhisperModel(name, device=device, compute_type=compute_type)
    print(f"Modele charge en {time.time() - t0:.1f}s")


@app.route("/api/health", methods=["GET"])
def health():
    return jsonify({"status": "ok", "service": "miyustt-faster-whisper", "model": model_name})


@app.route("/api/stt", methods=["POST"])
def stt():
    data = request.get_json(force=True, silent=True)
    if data is None:
        raw_len = request.content_length or 0
        return jsonify({"error": f"failed to parse JSON body (content_length={raw_len})"}), 400

    sample_rate = data.get("sample_rate", 16000)
    language = data.get("language", "fr")
    keys = list(data.keys())
    print(f"  [STT] Keys: {keys}, content_length={request.content_length}", flush=True)

    # Support base64-encoded PCM f32 LE (preferred) or raw JSON array
    samples_b64 = data.get("samples_b64")
    if samples_b64:
        pcm_bytes = base64.b64decode(samples_b64)
        audio = np.frombuffer(pcm_bytes, dtype=np.float32)
        print(f"  [STT] Recu {len(audio)} samples via base64 ({len(pcm_bytes)} bytes)", flush=True)
    else:
        samples = data.get("samples", [])
        if not samples:
            return jsonify({"error": f"no samples provided (keys={keys})"}), 400
        audio = np.array(samples, dtype=np.float32)
        print(f"  [STT] Recu {len(audio)} samples via JSON array", flush=True)

    # Transcription
    t0 = time.time()
    lang_param = language if language != "auto" else None
    segments, info = model.transcribe(
        audio,
        language=lang_param,
        beam_size=5,
        vad_filter=True,
    )
    transcript = " ".join(seg.text.strip() for seg in segments)
    duration_ms = int((time.time() - t0) * 1000)

    return jsonify({
        "transcript": transcript,
        "confidence": round(getattr(info, "language_probability", 0.9), 3),
        "duration_ms": duration_ms,
        "engine": "faster-whisper",
        "model_used": model_name,
        "language_detected": getattr(info, "language", language),
    })


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Serveur STT faster-whisper local")
    parser.add_argument("--model", default="small", help="Modele whisper (tiny, base, small, medium, large-v3)")
    parser.add_argument("--device", default="cuda", help="Device (cuda, cpu)")
    parser.add_argument("--compute-type", default="float16", help="Type de calcul (float16, int8, float32)")
    parser.add_argument("--port", type=int, default=8100, help="Port HTTP")
    args = parser.parse_args()

    load_model(args.model, args.device, args.compute_type)
    print(f"Serveur STT en ecoute sur http://127.0.0.1:{args.port}")
    app.run(host="127.0.0.1", port=args.port, debug=False)
