# P0 Temps 1 - Exploration et brainstorming

## Statut

- Etat : Termine
- Phase : P0 Temps 1
- Responsable principal : Maria
- Debut : 2026-03-06T13:43:48Z
- Fin : 2026-03-06T13:55:20Z

## TL;DR

Brainstorming structure en 5 sections (COMPRENDRE, CADRER, IMAGINER, EVALUER, DECIDER), 15 questions posees a l'utilisateur. Decisions cles : fork complet OxiCloud, completude > qualite > rapidite, vitrine technique, SQLite via adaptateur, auth Miyukini Connect.

## 5 sections du brainstorming

### Section 1 - COMPRENDRE
- Motivation : Maturite OxiCloud + Scalabilite + Securite + base solide
- Vision : Vitrine technique de l'ecosysteme Miyukini
- Raisons du fork : les 4 raisons (WebDAV/CalDAV/CardDAV natifs, dedup SHA-256, architecture Clean/DDD, licence MIT)

### Section 2 - CADRER
- Strategie : Repartir de zero sur les decisions v1
- Perimetre : Fork complet + integration Central + Miyukini Connect pour auth
- Qualite/securite d'abord, vitesse en second

### Section 3 - IMAGINER
- Framework : Migrer vers axum (decouverte T4 : OxiCloud deja sur axum)
- Base de donnees : SQLite via adaptateur (remplacer PostgreSQL)
- Reference : Nextcloud + OxiCloud tel quel

### Section 4 - EVALUER
- Objectif principal : Integration Central fluide
- Risques : tous les 4 identifies (SQLite adapter, conformite protocoles, volume code, securite)
- Complexite : Tres complexe (T5)

### Section 5 - DECIDER
- Approche : OxiCloud complet adapte
- Priorites : Completude > Qualite > Rapidite
- Decisions figees : fork OxiCloud + axum + Miyukini Connect
