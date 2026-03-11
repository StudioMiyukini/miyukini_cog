"""
Serveur TTS local utilisant pyttsx3 (Windows SAPI) ou edge-tts.

Expose l'API compatible avec le contrat MiyuTTS :
  POST /api/tts/wav  -> audio/wav binary
  GET  /api/tts/voices -> { voices: [...] }
  GET  /api/health   -> { status: "ok" }

Installation (option A - Windows SAPI, offline) :
  pip install pyttsx3 flask

Installation (option B - Edge TTS, meilleure qualite, besoin internet) :
  pip install edge-tts flask

Lancement :
  python tts_server.py                    # auto-detect backend
  python tts_server.py --backend sapi     # forcer Windows SAPI
  python tts_server.py --backend edge     # forcer Edge TTS
"""

import argparse
import io
import struct
import time
import wave
from flask import Flask, request, jsonify, Response

app = Flask(__name__)
backend = None


# ---------------------------------------------------------------------------
# Backend SAPI (pyttsx3, offline, Windows only)
# ---------------------------------------------------------------------------

class SapiBackend:
    def __init__(self):
        import pyttsx3
        self.engine = pyttsx3.init()
        # Essayer de mettre une voix francaise
        for voice in self.engine.getProperty("voices"):
            if "french" in voice.name.lower() or "fr" in voice.id.lower():
                self.engine.setProperty("voice", voice.id)
                break
        self.engine.setProperty("rate", 160)

    def synthesize(self, text: str, voice: str, sample_rate: int) -> bytes:
        """Genere un WAV via pyttsx3 (save to memory)."""
        import tempfile
        import os

        tmp = tempfile.NamedTemporaryFile(suffix=".wav", delete=False)
        tmp.close()
        try:
            self.engine.save_to_file(text, tmp.name)
            self.engine.runAndWait()
            with open(tmp.name, "rb") as f:
                return f.read()
        finally:
            os.unlink(tmp.name)

    def voices(self):
        return [
            {"id": "fr_sapi", "lang": "fr", "engine": "pyttsx3-sapi"},
            {"id": "en_sapi", "lang": "en", "engine": "pyttsx3-sapi"},
        ]


# ---------------------------------------------------------------------------
# Backend Edge TTS (internet requis, haute qualite)
# ---------------------------------------------------------------------------

class EdgeBackend:
    def synthesize(self, text: str, voice: str, sample_rate: int) -> bytes:
        import asyncio
        import edge_tts

        edge_voice = "fr-FR-DeniseNeural"
        if "en" in voice:
            edge_voice = "en-US-JennyNeural"

        async def _synth():
            communicate = edge_tts.Communicate(text, edge_voice)
            buf = io.BytesIO()
            async for chunk in communicate.stream():
                if chunk["type"] == "audio":
                    buf.write(chunk["data"])
            return buf.getvalue()

        return asyncio.run(_synth())

    def voices(self):
        return [
            {"id": "fr_female_01_compact", "lang": "fr", "engine": "edge-tts"},
            {"id": "en_female_01_compact", "lang": "en", "engine": "edge-tts"},
        ]


# ---------------------------------------------------------------------------
# Routes
# ---------------------------------------------------------------------------

@app.route("/api/health", methods=["GET"])
def health():
    return jsonify({"status": "ok", "service": "miyutts-local", "backend": backend.__class__.__name__})


@app.route("/api/tts/voices", methods=["GET"])
def voices():
    return jsonify({"voices": backend.voices()})


@app.route("/api/tts/wav", methods=["POST"])
def tts_wav():
    data = request.get_json(force=True)
    text = data.get("text", "").strip()
    voice = data.get("voice", "fr_female_01_compact")
    sample_rate = data.get("sample_rate", 22050)

    if not text:
        return jsonify({"error": "empty text"}), 400

    t0 = time.time()
    wav_bytes = backend.synthesize(text, voice, sample_rate)
    duration_ms = int((time.time() - t0) * 1000)

    mime = "audio/wav" if isinstance(backend, SapiBackend) else "audio/mpeg"
    return Response(
        wav_bytes,
        mimetype=mime,
        headers={
            "x-miyu-tts-engine": backend.__class__.__name__,
            "x-miyu-tts-duration-ms": str(duration_ms),
        },
    )


@app.route("/api/tts", methods=["POST"])
def tts_json():
    """Endpoint JSON avec audio en base64."""
    import base64

    data = request.get_json(force=True)
    text = data.get("text", "").strip()
    voice = data.get("voice", "fr_female_01_compact")
    sample_rate = data.get("sample_rate", 22050)

    if not text:
        return jsonify({"error": "empty text"}), 400

    t0 = time.time()
    wav_bytes = backend.synthesize(text, voice, sample_rate)
    duration_ms = int((time.time() - t0) * 1000)

    return jsonify({
        "audio_base64": base64.b64encode(wav_bytes).decode(),
        "engine": backend.__class__.__name__,
        "voice_used": voice,
        "duration_ms": duration_ms,
    })


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Serveur TTS local")
    parser.add_argument("--backend", default="auto", help="Backend TTS (sapi, edge, auto)")
    parser.add_argument("--port", type=int, default=8101, help="Port HTTP")
    args = parser.parse_args()

    if args.backend == "sapi":
        backend = SapiBackend()
    elif args.backend == "edge":
        backend = EdgeBackend()
    else:
        # Auto-detect
        try:
            import pyttsx3
            backend = SapiBackend()
            print("Backend: pyttsx3 (Windows SAPI)")
        except ImportError:
            try:
                import edge_tts
                backend = EdgeBackend()
                print("Backend: edge-tts")
            except ImportError:
                print("ERREUR: aucun backend TTS disponible.")
                print("Installe: pip install pyttsx3  (ou)  pip install edge-tts")
                exit(1)

    print(f"Serveur TTS en ecoute sur http://127.0.0.1:{args.port}")
    app.run(host="127.0.0.1", port=args.port, debug=False)
