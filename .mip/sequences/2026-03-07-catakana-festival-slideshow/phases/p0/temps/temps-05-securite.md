# P0 Temps 5 - Analyse securite

## Statut

- Etat : A completer
- Phase : P0 Temps 5
- Responsable principal : Victor

## TL;DR

[A completer -- alimente PASS-0/PASS-01 en P4 et le volet securite du GPI]

## Classification des donnees

- Niveau : [PUBLIC | PRIVE | CONFIDENTIEL]
- Surfaces exposees : [API HTTP / WebSocket / Fichiers / DB / autre]
- Conformite requise : [RGPD / autre / aucune]

## Surfaces d'attaque

| Surface | Risque | Niveau | Mitigation ciblee |
|---------|--------|--------|------------------|
| [surface] | [description risque] | [LOW/MED/HIGH/CRIT] | [mitigation] |

## CVE / dependances a surveiller

| Crate | Risque connu | Action |
|-------|-------------|--------|
| [crate] | [CVE-XXXX-XXXX ou RAS] | [Patcher / Monitorer / RAS] |

## Controles P4 applicables

> Cocher Oui/Non selon le perimetre technique de la sequence.

| Controle | Applicable | Priorite |
|----------|-----------|---------|
| PASS-0 : path traversal | [Oui/Non] | [CRIT/MED/LOW] |
| PASS-0 : XXE injection | [Oui/Non] | [CRIT/MED/LOW] |
| PASS-0 : auth bypass | [Oui/Non] | [CRIT/MED/LOW] |
| PASS-0 : SQL injection | [Oui/Non] | [CRIT/MED/LOW] |
| PASS-01 : CSP nonce per-request | [Oui/Non] | [CRIT/MED/LOW] |
| PASS-01 : HSTS + Secure headers | [Oui/Non] | [CRIT/MED/LOW] |
| PASS-01 : Rate limiting | [Oui/Non] | [CRIT/MED/LOW] |
| PASS-01 : HMAC + constant-time compare | [Oui/Non] | [CRIT/MED/LOW] |
| PASS-01 : IP hashed logs (RGPD) | [Oui/Non] | [CRIT/MED/LOW] |
| PASS-01 : cargo audit CVE | Oui | CRIT |
| PASS-01 : CSRF / replay tokens | [Oui/Non] | [CRIT/MED/LOW] |
| PASS-01 : Content-Type enforcement | [Oui/Non] | [CRIT/MED/LOW] |

## Score cible P4

- RAS securite : >= **90/100**
- Breche critique ou score < 60 → rebouclage MIP (P0 Temps 1)

