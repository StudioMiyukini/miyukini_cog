"""
Serveur de reconnaissance faciale pour Alicia.

Utilise OpenCV : detection (Haar/DNN) + reconnaissance (LBPH).
Les visages enregistres sont stockes dans faces_db/.

Endpoints :
  POST /api/face/identify  -> capture webcam + identification
  POST /api/face/register  -> capture webcam + enregistrement { "name": "Theresa" }
  GET  /api/face/list      -> liste des visages enregistres
  GET  /api/health         -> status

Installation :
  pip install opencv-python flask numpy

Lancement :
  python face_server.py
  python face_server.py --camera 0
"""

import argparse
import os
import time
import json
import pickle
from pathlib import Path

import cv2
import numpy as np
from flask import Flask, request, jsonify

app = Flask(__name__)
FACES_DB = Path(__file__).parent / "faces_db"
EMBEDDINGS_FILE = FACES_DB / "embeddings.pkl"
camera_index = 0

# Face detector (Haar cascade, rapide et fiable)
face_cascade = None
# LBPH recognizer pour la reconnaissance
recognizer = None
# Mapping label -> name
label_map = {}


def init_detector():
    global face_cascade
    face_cascade = cv2.CascadeClassifier(
        cv2.data.haarcascades + "haarcascade_frontalface_default.xml"
    )


def load_recognizer():
    """Charge ou re-entraine le recognizer LBPH depuis les photos."""
    global recognizer, label_map

    recognizer = cv2.face.LBPHFaceRecognizer_create()
    faces_data = []
    labels = []
    label_map = {}
    current_label = 0

    if not FACES_DB.exists():
        return

    for person_dir in sorted(FACES_DB.iterdir()):
        if not person_dir.is_dir():
            continue
        name = person_dir.name
        photos = list(person_dir.glob("*.jpg")) + list(person_dir.glob("*.png"))
        if not photos:
            continue

        label_map[current_label] = name

        for photo_path in photos:
            img = cv2.imread(str(photo_path), cv2.IMREAD_GRAYSCALE)
            if img is None:
                continue
            # Detecter le visage dans la photo
            detected = face_cascade.detectMultiScale(img, 1.1, 5, minSize=(80, 80))
            if len(detected) > 0:
                (x, y, w, h) = detected[0]
                face_roi = img[y:y+h, x:x+w]
                face_resized = cv2.resize(face_roi, (200, 200))
                faces_data.append(face_resized)
                labels.append(current_label)

        current_label += 1

    if faces_data:
        recognizer.train(faces_data, np.array(labels))
        print(f"  [Face] Recognizer entraine : {len(faces_data)} photos, {len(label_map)} personnes", flush=True)
    else:
        recognizer = None
        print("  [Face] Aucune donnee d'entrainement", flush=True)


def capture_frame(cam_idx: int = 0, warmup_frames: int = 8) -> np.ndarray | None:
    """Capture une frame depuis la webcam."""
    cap = cv2.VideoCapture(cam_idx, cv2.CAP_DSHOW)
    if not cap.isOpened():
        cap = cv2.VideoCapture(cam_idx)
    if not cap.isOpened():
        return None
    try:
        for _ in range(warmup_frames):
            cap.read()
        ret, frame = cap.read()
        return frame if ret else None
    finally:
        cap.release()


@app.route("/api/health", methods=["GET"])
def health():
    registered = []
    if FACES_DB.exists():
        registered = [d.name for d in FACES_DB.iterdir() if d.is_dir()]
    return jsonify({
        "status": "ok",
        "service": "alicia-face-recognition",
        "registered_faces": len(registered),
        "names": registered,
    })


@app.route("/api/face/identify", methods=["POST"])
def identify():
    """Capture webcam + identification."""
    t0 = time.time()
    print("  [Face] Capture webcam...", flush=True)

    frame = capture_frame(camera_index)
    if frame is None:
        return jsonify({"error": "impossible d'ouvrir la camera"}), 500

    capture_ms = int((time.time() - t0) * 1000)
    print(f"  [Face] Frame capturee en {capture_ms}ms ({frame.shape})", flush=True)

    t1 = time.time()
    gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
    detected = face_cascade.detectMultiScale(gray, 1.1, 5, minSize=(80, 80))

    faces = []
    for (x, y, w, h) in detected:
        face_info = {"bbox": {"x": int(x), "y": int(y), "w": int(w), "h": int(h)}}

        if recognizer is not None:
            face_roi = gray[y:y+h, x:x+w]
            face_resized = cv2.resize(face_roi, (200, 200))
            label, confidence_raw = recognizer.predict(face_resized)
            # LBPH: confidence = distance (plus bas = mieux, 0=parfait, >80=inconnu)
            if confidence_raw < 80:
                face_info["name"] = label_map.get(label, "inconnu")
                face_info["confidence"] = round(max(0.0, (80 - confidence_raw) / 80), 3)
            else:
                face_info["name"] = "inconnu"
                face_info["confidence"] = 0.0
        else:
            face_info["name"] = "inconnu"
            face_info["confidence"] = 0.0

        faces.append(face_info)

    recog_ms = int((time.time() - t1) * 1000)
    total_ms = int((time.time() - t0) * 1000)

    print(f"  [Face] {len(faces)} visage(s) detecte(s) en {recog_ms}ms", flush=True)
    for f in faces:
        print(f"    -> {f['name']} (conf={f.get('confidence', 0):.2f})", flush=True)

    return jsonify({
        "faces": faces,
        "count": len(faces),
        "capture_ms": capture_ms,
        "recognition_ms": recog_ms,
        "total_ms": total_ms,
    })


@app.route("/api/face/register", methods=["POST"])
def register():
    """Capture webcam + enregistrement d'un visage."""
    data = request.get_json(force=True)
    name = data.get("name", "").strip()
    if not name:
        return jsonify({"error": "champ 'name' requis"}), 400

    print(f"  [Face] Enregistrement de '{name}'...", flush=True)

    frame = capture_frame(camera_index)
    if frame is None:
        return jsonify({"error": "impossible d'ouvrir la camera"}), 500

    gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
    detected = face_cascade.detectMultiScale(gray, 1.1, 5, minSize=(80, 80))
    if len(detected) == 0:
        return jsonify({"error": "aucun visage detecte"}), 400

    # Sauvegarder
    person_dir = FACES_DB / name
    person_dir.mkdir(parents=True, exist_ok=True)
    ts = int(time.time())
    img_path = person_dir / f"{name}_{ts}.jpg"
    cv2.imwrite(str(img_path), frame)

    count = len(list(person_dir.glob("*.jpg")))
    print(f"  [Face] '{name}' enregistre ({count} photo(s))", flush=True)

    # Re-entrainer le recognizer
    load_recognizer()

    return jsonify({
        "status": "ok",
        "name": name,
        "photos": count,
        "faces_detected": len(detected),
    })


@app.route("/api/face/list", methods=["GET"])
def list_faces():
    registered = {}
    if FACES_DB.exists():
        for d in FACES_DB.iterdir():
            if d.is_dir():
                registered[d.name] = len(list(d.glob("*.jpg")))
    return jsonify({"faces": registered, "total": len(registered)})


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Serveur reconnaissance faciale Alicia")
    parser.add_argument("--camera", type=int, default=0, help="Index camera (defaut: 0)")
    parser.add_argument("--port", type=int, default=8102, help="Port HTTP")
    args = parser.parse_args()

    camera_index = args.camera
    FACES_DB.mkdir(exist_ok=True)

    init_detector()
    load_recognizer()

    if label_map:
        print(f"Visages enregistres : {', '.join(label_map.values())}")
    else:
        print("Aucun visage enregistre. POST /api/face/register pour en ajouter.")

    print(f"Serveur Face Recognition en ecoute sur http://127.0.0.1:{args.port}")
    app.run(host="127.0.0.1", port=args.port, debug=False)
