# P0 Temps 5 - Analyse securite

## Statut

- Etat : Termine
- Phase : P0 Temps 5
- Responsable principal : Victor
- Debut : 2026-03-06T13:59:19Z
- Fin : 2026-03-06T14:05:06Z

## TL;DR

Score actuel recalcule: 82/100. Cible: 96/100. Niveau DURCI requis. CVE-2025-6965 (SQLite) a verifier d'urgence. Surfaces critiques: WebDAV paths + CalDAV/CardDAV XML parsing. 16 recommandations (4 critiques, 6 hautes, 6 moyennes). Fondations crypto excellentes.

## Niveau de securite : DURCI (2/3)

## Score securite
- Actuel : 82/100
- Cible : 96/100 (avec R1-R10)

## Recommandations critiques (bloquantes >95)

| # | Recommandation | Impact |
|---|---------------|--------|
| R1 | Verifier rusqlite pour CVE-2025-6965 (SQLite <3.50.2) | Securite critique |
| R2 | Validation WebDAV paths (whitelist, rejet .., normalisation unicode) | Previent path traversal |
| R3 | Defenses XML CalDAV/CardDAV (desactiver DTD, limite taille/profondeur) | Previent XXE |
| R4 | Zeroize sur KeyManager.master_key | Nettoyage memoire |

## Recommandations hautes
- R5: Chiffrement DB (SQLCipher)
- R6: Logger evenements securite manquants
- R7: Supprimer 'unsafe-inline' du CSP
- R8: CORS explicite API interne
- R9: Limite max sessions web
- R10: Verifier permissions key.pem

## Surfaces d'attaque nouvelles (fork OxiCloud)
1. WebDAV paths (path traversal)
2. CalDAV/CardDAV XML (XXE, XML bomb)
3. Code OxiCloud non audite (revue ligne par ligne requise)

## Points forts existants
- unsafe_code = "forbid" sur les deux crates
- Comparaisons constant-time (subtle)
- Separation API/Web, SandboxedStore
- Nonces OsRng, Debug masque secrets
- Rate limiting par IP, journal RGPD

## Artefact source

Voir [P0-T5-securite.md](../../../briefs/2026-03-06-miyucloud-oxicloud-refonte-P0-T5-securite.md)
