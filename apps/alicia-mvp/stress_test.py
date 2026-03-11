"""
Stress test concurrentiel pour le pipeline Alicia MVP.

Teste chaque service individuellement puis le pipeline complet
sous charge concurrente (N requêtes simultanées).

Usage :
  python stress_test.py
  python stress_test.py --concurrency 10
  python stress_test.py --only llm
"""

import argparse
import asyncio
import time
import json
import statistics
import aiohttp

# --- Configuration -----------------------------------------------------------

LLM_URL = "http://10.212.61.220:1234"
LLM_KEY = "sk-lm-mG6IJnuL:Oe5PnXC4pQBIReJkeOou"
LLM_MODEL = "qwen/qwen3.5-9b"
STT_URL = "http://127.0.0.1:8100"
TTS_URL = "http://127.0.0.1:8101"
SEARCH_URL = "http://127.0.0.1:8103"

# Phrases de test variees pour simuler des utilisateurs differents
TEST_PHRASES = [
    "allume la lumiere du salon",
    "eteins la lampe de la chambre",
    "bonne nuit",
    "quel temps fait-il demain",
    "ouvre les volets du salon",
    "regle le chauffage a 22 degres",
    "ferme les volets de la chambre",
    "lance chrome",
    "quelle heure est-il",
    "mets la luminosite a 50 pourcent",
    "active le mode cinema",
    "quel est ton nom",
    "allume toutes les lumieres",
    "je pars",
    "ouvre spotify",
]

NLU_SYSTEM_PROMPT = """Tu es le module NLU d'Alicia, une assistante domotique. Tu recois une commande vocale transcrite et tu dois extraire l'intention au format JSON strict.

Pieces disponibles : salon, chambre-parentale, chambre-theresa, chambre-eleanore
Types de devices : light, shutter, thermostat, outlet, lock, sensor
Actions possibles : on, off, open, close, lock, unlock, set_brightness, set_temperature, set_position

Reponds UNIQUEMENT avec un JSON valide, sans texte avant/apres. Format :
{"intent": "<type>", "slots": {<slots>}}

Types d'intent :
- control_device : slots = device_type, room_id (optionnel), action, value (optionnel)
- activate_routine : slots = routine_name
- launch_app : slots = app_name
- query_state : slots = target, property (optionnel)
- unknown : slots = {}"""


# --- Helpers -----------------------------------------------------------------

def fmt_ms(seconds: float) -> str:
    return f"{seconds * 1000:.0f}ms"


def print_stats(label: str, times: list[float], errors: int):
    if not times:
        print(f"  {label}: AUCUNE reponse reussie ({errors} erreurs)")
        return

    avg = statistics.mean(times)
    med = statistics.median(times)
    p95 = sorted(times)[int(len(times) * 0.95)] if len(times) >= 2 else times[-1]
    mn, mx = min(times), max(times)

    print(f"  {label}:")
    print(f"    Reussites: {len(times)}/{len(times) + errors}")
    print(f"    Avg: {fmt_ms(avg)}  Med: {fmt_ms(med)}  P95: {fmt_ms(p95)}")
    print(f"    Min: {fmt_ms(mn)}  Max: {fmt_ms(mx)}")
    if errors:
        print(f"    Erreurs: {errors}")


# --- Test LLM ----------------------------------------------------------------

async def test_llm_single(session: aiohttp.ClientSession, phrase: str, idx: int):
    body = {
        "model": LLM_MODEL,
        "messages": [
            {"role": "system", "content": NLU_SYSTEM_PROMPT},
            {"role": "user", "content": phrase},
        ],
        "temperature": 0.1,
        "max_tokens": 200,
    }
    headers = {"Authorization": f"Bearer {LLM_KEY}"}

    t0 = time.time()
    try:
        async with session.post(
            f"{LLM_URL}/v1/chat/completions",
            json=body,
            headers=headers,
            timeout=aiohttp.ClientTimeout(total=30),
        ) as resp:
            if resp.status != 200:
                text = await resp.text()
                return None, f"HTTP {resp.status}: {text[:100]}"
            data = await resp.json()
            elapsed = time.time() - t0
            content = data["choices"][0]["message"]["content"]
            tokens = data.get("usage", {})
            print(f"    [{idx}] {fmt_ms(elapsed)} — {phrase[:30]}... → {content[:60]}...")
            return elapsed, None
    except Exception as e:
        return None, str(e)


async def test_llm(concurrency: int):
    print(f"\n{'='*60}")
    print(f"TEST LLM — {concurrency} requetes concurrentes")
    print(f"  Serveur: {LLM_URL}")
    print(f"  Modele: {LLM_MODEL}")
    print(f"{'='*60}")

    async with aiohttp.ClientSession() as session:
        # Warmup
        print("\n  Warmup...")
        await test_llm_single(session, "bonjour", 0)

        # Concurrent burst
        print(f"\n  Burst de {concurrency} requetes...")
        t0 = time.time()
        tasks = [
            test_llm_single(session, TEST_PHRASES[i % len(TEST_PHRASES)], i + 1)
            for i in range(concurrency)
        ]
        results = await asyncio.gather(*tasks)
        wall_time = time.time() - t0

        times = [r[0] for r in results if r[0] is not None]
        errors = sum(1 for r in results if r[0] is None)

        print(f"\n  Wall time: {fmt_ms(wall_time)}")
        print(f"  Throughput: {len(times) / wall_time:.1f} req/s")
        print_stats("Latences", times, errors)

        for r in results:
            if r[1]:
                print(f"    ERR: {r[1][:100]}")


# --- Test TTS ----------------------------------------------------------------

async def test_tts_single(session: aiohttp.ClientSession, text: str, idx: int):
    body = {
        "text": text,
        "voice": "fr_female_01_compact",
        "format": "wav",
        "language": "fr",
    }

    t0 = time.time()
    try:
        async with session.post(
            f"{TTS_URL}/api/tts/wav",
            json=body,
            timeout=aiohttp.ClientTimeout(total=15),
        ) as resp:
            if resp.status != 200:
                text_resp = await resp.text()
                return None, f"HTTP {resp.status}: {text_resp[:100]}"
            data = await resp.read()
            elapsed = time.time() - t0
            print(f"    [{idx}] {fmt_ms(elapsed)} — {len(data)} bytes audio")
            return elapsed, None
    except Exception as e:
        return None, str(e)


async def test_tts(concurrency: int):
    print(f"\n{'='*60}")
    print(f"TEST TTS — {concurrency} requetes concurrentes")
    print(f"  Serveur: {TTS_URL}")
    print(f"{'='*60}")

    responses = [
        "OK, j'allume la lumiere du salon.",
        "La chambre est maintenant eteinte.",
        "Bonne nuit ! Je ferme tout.",
        "Il fera 18 degres demain.",
        "Les volets sont ouverts.",
        "Le chauffage est regle a 22 degres.",
        "Volets fermes.",
        "Chrome est lance.",
    ]

    async with aiohttp.ClientSession() as session:
        print(f"\n  Burst de {concurrency} requetes...")
        t0 = time.time()
        tasks = [
            test_tts_single(session, responses[i % len(responses)], i + 1)
            for i in range(concurrency)
        ]
        results = await asyncio.gather(*tasks)
        wall_time = time.time() - t0

        times = [r[0] for r in results if r[0] is not None]
        errors = sum(1 for r in results if r[0] is None)

        print(f"\n  Wall time: {fmt_ms(wall_time)}")
        print_stats("Latences", times, errors)


# --- Test Search -------------------------------------------------------------

async def test_search_single(session: aiohttp.ClientSession, query: str, idx: int):
    t0 = time.time()
    try:
        async with session.post(
            f"{SEARCH_URL}/api/search",
            json={"query": query, "max_results": 3},
            timeout=aiohttp.ClientTimeout(total=15),
        ) as resp:
            if resp.status != 200:
                text = await resp.text()
                return None, f"HTTP {resp.status}: {text[:100]}"
            data = await resp.json()
            elapsed = time.time() - t0
            count = data.get("count", 0)
            print(f"    [{idx}] {fmt_ms(elapsed)} — '{query[:25]}' → {count} resultats")
            return elapsed, None
    except Exception as e:
        return None, str(e)


async def test_search(concurrency: int):
    print(f"\n{'='*60}")
    print(f"TEST SEARCH — {concurrency} requetes concurrentes")
    print(f"  Serveur: {SEARCH_URL}")
    print(f"{'='*60}")

    queries = [
        "meteo paris demain",
        "heure actuelle japon",
        "recette crepe facile",
        "cours bitcoin",
        "resultats ligue 1",
    ]

    async with aiohttp.ClientSession() as session:
        print(f"\n  Burst de {concurrency} requetes...")
        t0 = time.time()
        tasks = [
            test_search_single(session, queries[i % len(queries)], i + 1)
            for i in range(concurrency)
        ]
        results = await asyncio.gather(*tasks)
        wall_time = time.time() - t0

        times = [r[0] for r in results if r[0] is not None]
        errors = sum(1 for r in results if r[0] is None)

        print(f"\n  Wall time: {fmt_ms(wall_time)}")
        print_stats("Latences", times, errors)


# --- Test Pipeline complet ---------------------------------------------------

async def test_pipeline_single(
    session: aiohttp.ClientSession, phrase: str, idx: int
):
    """Simule le pipeline complet : NLU (LLM) -> Response (LLM) -> TTS."""
    timings = {}
    t_total = time.time()

    # Step 1: NLU
    t0 = time.time()
    nlu_body = {
        "model": LLM_MODEL,
        "messages": [
            {"role": "system", "content": NLU_SYSTEM_PROMPT},
            {"role": "user", "content": phrase},
        ],
        "temperature": 0.1,
        "max_tokens": 200,
    }
    headers = {"Authorization": f"Bearer {LLM_KEY}"}

    try:
        async with session.post(
            f"{LLM_URL}/v1/chat/completions",
            json=nlu_body,
            headers=headers,
            timeout=aiohttp.ClientTimeout(total=30),
        ) as resp:
            if resp.status != 200:
                return None, f"NLU HTTP {resp.status}"
            data = await resp.json()
            timings["nlu"] = time.time() - t0
            intent_raw = data["choices"][0]["message"]["content"]
    except Exception as e:
        return None, f"NLU: {e}"

    # Step 2: Response LLM
    t0 = time.time()
    resp_body = {
        "model": LLM_MODEL,
        "messages": [
            {
                "role": "system",
                "content": "Tu es Alicia, assistante domotique. Reponds en 1 phrase courte. Pas d'emoji.",
            },
            {
                "role": "user",
                "content": f'L\'utilisateur a dit : "{phrase}"\nIntent : {intent_raw}\nReponds.',
            },
        ],
        "temperature": 0.7,
        "max_tokens": 100,
    }

    try:
        async with session.post(
            f"{LLM_URL}/v1/chat/completions",
            json=resp_body,
            headers=headers,
            timeout=aiohttp.ClientTimeout(total=30),
        ) as resp:
            if resp.status != 200:
                return None, f"Response HTTP {resp.status}"
            data = await resp.json()
            timings["response"] = time.time() - t0
            response_text = data["choices"][0]["message"]["content"]
    except Exception as e:
        return None, f"Response: {e}"

    # Step 3: TTS
    t0 = time.time()
    tts_body = {
        "text": response_text[:100],
        "voice": "fr_female_01_compact",
        "format": "wav",
        "language": "fr",
    }

    try:
        async with session.post(
            f"{TTS_URL}/api/tts/wav",
            json=tts_body,
            timeout=aiohttp.ClientTimeout(total=15),
        ) as resp:
            if resp.status != 200:
                timings["tts"] = None
            else:
                await resp.read()
                timings["tts"] = time.time() - t0
    except Exception:
        timings["tts"] = None

    total = time.time() - t_total
    timings["total"] = total

    nlu_ms = fmt_ms(timings["nlu"])
    resp_ms = fmt_ms(timings["response"])
    tts_ms = fmt_ms(timings["tts"]) if timings.get("tts") else "ERR"
    print(
        f"    [{idx}] {fmt_ms(total)} total — NLU:{nlu_ms} Resp:{resp_ms} TTS:{tts_ms}"
        f" — '{phrase[:25]}...'"
    )
    return timings, None


async def test_pipeline(concurrency: int):
    print(f"\n{'='*60}")
    print(f"TEST PIPELINE COMPLET — {concurrency} requetes concurrentes")
    print(f"  NLU(LLM) -> Response(LLM) -> TTS")
    print(f"{'='*60}")

    async with aiohttp.ClientSession() as session:
        print(f"\n  Burst de {concurrency} pipelines...")
        t0 = time.time()
        tasks = [
            test_pipeline_single(session, TEST_PHRASES[i % len(TEST_PHRASES)], i + 1)
            for i in range(concurrency)
        ]
        results = await asyncio.gather(*tasks)
        wall_time = time.time() - t0

        nlu_times = [r[0]["nlu"] for r in results if r[0]]
        resp_times = [r[0]["response"] for r in results if r[0]]
        tts_times = [r[0]["tts"] for r in results if r[0] and r[0].get("tts")]
        total_times = [r[0]["total"] for r in results if r[0]]
        errors = sum(1 for r in results if r[0] is None)

        print(f"\n  Wall time: {fmt_ms(wall_time)}")
        print(f"  Throughput: {len(total_times) / wall_time:.1f} pipelines/s")
        print_stats("NLU (LLM)", nlu_times, 0)
        print_stats("Response (LLM)", resp_times, 0)
        print_stats("TTS", tts_times, len(total_times) - len(tts_times))
        print_stats("Pipeline total", total_times, errors)


# --- Health checks -----------------------------------------------------------

async def check_health():
    print("\n--- Health checks ---")
    async with aiohttp.ClientSession() as session:
        services = [
            ("LLM", f"{LLM_URL}/v1/models", {"Authorization": f"Bearer {LLM_KEY}"}),
            ("STT", f"{STT_URL}/api/health", {}),
            ("TTS", f"{TTS_URL}/api/health", {}),
            ("Search", f"{SEARCH_URL}/api/health", {}),
        ]
        for name, url, headers in services:
            try:
                async with session.get(
                    url, headers=headers, timeout=aiohttp.ClientTimeout(total=5)
                ) as resp:
                    status = "OK" if resp.status == 200 else f"HTTP {resp.status}"
                    print(f"  {name:8s} : {status} ({url})")
            except Exception as e:
                print(f"  {name:8s} : ERREUR — {e}")


# --- Main --------------------------------------------------------------------

async def main():
    parser = argparse.ArgumentParser(description="Stress test Alicia MVP")
    parser.add_argument(
        "-c", "--concurrency", type=int, default=5, help="Nombre de requetes concurrentes"
    )
    parser.add_argument(
        "--only",
        choices=["llm", "tts", "search", "pipeline"],
        help="Tester un seul service",
    )
    args = parser.parse_args()

    print("=" * 60)
    print("  STRESS TEST — Alicia MVP Pipeline")
    print(f"  Concurrency: {args.concurrency}")
    print("=" * 60)

    await check_health()

    if args.only == "llm" or args.only is None:
        await test_llm(args.concurrency)
    if args.only == "tts" or args.only is None:
        await test_tts(args.concurrency)
    if args.only == "search" or args.only is None:
        await test_search(args.concurrency)
    if args.only == "pipeline" or args.only is None:
        await test_pipeline(args.concurrency)

    print(f"\n{'='*60}")
    print("  DONE")
    print(f"{'='*60}")


if __name__ == "__main__":
    asyncio.run(main())
