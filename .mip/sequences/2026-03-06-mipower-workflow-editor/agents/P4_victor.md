# Agent P4 — Victor (Cybersecurite) — MIPOWER

## Contexte sequence

Sequence : 2026-03-06-mipower-workflow-editor
Surfaces : Tauri IPC + FS access (.mip/) + SQLite + Script ps1 invocation + Markdown rendering

## Perimetre audit P4

1. Tauri capabilities : verifier que seules les permissions declarees dans `capabilities/default.json` sont accordees
2. Path traversal : toute commande Tauri recevant un chemin est canonicalisee et bornee
3. SQLite : verifier l'absence d'interpolation directe dans les requetes (parameterized obligatoire)
4. Script invocation : verifier que les parametres passes aux scripts ps1 sont valides et non injectables
5. Markdown XSS : verifier DOMPurify sur tous les rendus HTML inline
6. CSP WebView2 : verifier la politique dans tauri.conf.json

## Criteres de blocage

- Chemin non valide accepte par une commande Tauri -> BLOQUANT
- Query SQL avec interpolation directe -> BLOQUANT
- innerHTML sans sanitization -> BLOQUANT
- Commande Tauri non listee dans capabilities -> BLOQUANT

## Fichiers a charger

- `specs/2026-03-06-mipower-workflow-editor-spec.md` (sections 5, 4)
- `phases/p0/temps/temps-05-securite.md` (RPS complet)
- `.mip/memory/security-patterns.md`
