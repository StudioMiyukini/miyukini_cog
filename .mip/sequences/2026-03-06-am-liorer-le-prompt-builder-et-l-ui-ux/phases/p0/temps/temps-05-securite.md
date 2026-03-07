# P0 Temps 5 - Analyse securite

## Statut

- Etat : TERMINE
- Phase : P0 Temps 5
- Responsable principal : Victor

## TL;DR

Surface d'attaque limitee (app locale, pas d'auth, pas de DB pour le builder). Risques principaux : validation longueur inputs, whitelist agents cote serveur, Content-Type enforcement. Preview live = textarea readonly, pas de XSS. Score cible P4 >= 88/100. Aucun risque CRITIQUE nouveau.

## Classification des donnees

- Niveau : PRIVE (titres/descriptions de sequences — potentiellement sensibles si toggle active)
- Surfaces exposees : API HTTP locale (POST /api/prompt, POST /api/init-sequence), localStorage navigateur
- Conformite requise : Aucune (usage local uniquement)

## Surfaces d'attaque

| Surface | Risque | Niveau | Mitigation ciblee |
|---------|--------|--------|------------------|
| POST /api/prompt — champs libres (title, desc, constraints) | Contenu injecte dans le prompt genere (pas d'execution) | LOW | Validation longueur max cote Rust (title 200c, desc 2000c) |
| agents[] Vec<String> — valeurs libres | Agent name arbitraire injecte dans le prompt | LOW | Whitelist des 10 agents connus cote serveur |
| tags[] — valeurs saisies utilisateur | Tag trop long ou avec caracteres speciaux | LOW | Validation longueur max par tag (50c) |
| POST /api/init-sequence — slug | Path traversal deja couvert | CRIT | Validation existante [a-z0-9-] a maintenir — aucun nouveau champ slug |
| localStorage — sauvegarde config | Donnees lisibles par d'autres scripts | LOW | Acceptable (app locale, un seul origin) |
| Preview live HTML | XSS si innerHTML utilise | MED | Utiliser textarea readonly (textContent uniquement) — pas de rendu HTML |

## CVE / dependances a surveiller

| Crate | Risque connu | Action |
|-------|-------------|--------|
| rusqlite 0.32 | RAS | cargo audit en P4 |
| axum 0.8 | RAS | cargo audit en P4 |
| marked.js CDN v12 | Vecteurs XSS anciens corriges | Maintenir v12+ |
| DOMPurify CDN v3 | RAS | Maintenir v3+ |

## Controles P4 applicables

| Controle | Applicable | Priorite |
|----------|-----------|---------|
| PASS-0 : path traversal | Oui (slug init-sequence) | CRIT |
| PASS-0 : XXE injection | Non (pas de XML) | - |
| PASS-0 : auth bypass | Non (pas d'auth) | - |
| PASS-0 : SQL injection | Non (prompt_handler ne touche pas la DB) | - |
| PASS-01 : CSP nonce per-request | Non (mipower = HTTP local sans CSP actuel) | LOW |
| PASS-01 : HSTS + Secure headers | Non (HTTP local) | LOW |
| PASS-01 : Rate limiting | Non (usage local) | LOW |
| PASS-01 : HMAC + constant-time compare | Non | - |
| PASS-01 : IP hashed logs (RGPD) | Non (local) | - |
| PASS-01 : cargo audit CVE | Oui | CRIT |
| PASS-01 : CSRF / replay tokens | Non (local, pas d'auth) | - |
| PASS-01 : Content-Type enforcement | Oui (POST doit verifier application/json) | MED |
| Validation longueur max inputs | Oui (nouveau : title, desc, constraints, tags, agents) | MED |
| Whitelist agents Vec<String> | Oui (nouveau) | MED |

## Score cible P4

- RAS securite : >= **88/100** (app locale sans auth — controles auth/HSTS/CSRF non applicables)
- Breche critique ou score < 60 → rebouclage MIP (P0 Temps 1)
