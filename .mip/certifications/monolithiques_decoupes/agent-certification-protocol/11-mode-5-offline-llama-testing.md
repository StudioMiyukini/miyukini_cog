<!-- source: .mip/certifications/AGENT-CERTIFICATION-PROTOCOL.md lines 352-393 -->

## Mode 5 : Offline Llama (Testing âš ï¸)

**Status** : Testing, limited parallelism

**Certification Date** : In progress (target 2026-06-05)

**Capabilities** :
- T1-T2 âœ… (works, slow)
- T3 âš ï¸ (possible but CPU-bottleneck)
- T4-T5 âŒ (not recommended)

**Hard constraints** :
- Context : 128k (depends LLM)
- Speed : 1 token/sec (CPU-bound)
- Web search : âŒ (fully offline)
- Parallelism : âš ï¸ (CPU contention)

**Testing in progress** :
- [ ] Verify T2 performance on M1 (target < 15 min)
- [ ] Verify offline docs sufficiency (no web)
- [ ] Benchmark CPU + memory usage
- [ ] Test disk caching (inference cache)

**Issues encountered** :
- #1 : CPU spikes cause context thrashing
- #2 : Semantic search too slow (local embedding)
- #3 : Context window variable per GGUF

**Workarounds being tested** :
- #1 : Limit parallel agents to 1-2 max
- #2 : Use grep instead of semantic search
- #3 : Standardize on llama3.1 contexts.bin

**Target use cases for Mode 5** :
- Offline-first production (no internet access)
- 24/7 background inference (long-running P3)
- Educational/open-source (zero API cost)

**Review schedule** : Bi-weekly (active testing)

---

