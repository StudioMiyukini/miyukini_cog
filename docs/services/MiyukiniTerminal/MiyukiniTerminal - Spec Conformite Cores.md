# MiyukiniTerminal — Spécification Conformité Cores

## Contexte

Ce document décrit le **mapping des Cores** Miyukini sur le Terminal : StrongFather (autorisation actions), KindMother (persistance), MasterButler (capacités), BorderGuard (frontières), WorrySentinel (niveaux sécurité), TAMR (conflits). Contrats respectés.

**Références :**

- [Document Fondateur](./MiyukiniTerminal%20-%20Document%20Fondateur.md)
- [Architecture Miyukini](.cursor/skills/miyukini-architecture/SKILL.md)

---

## Portée / Scope

- Mapping Cores sur Terminal
- Contrats respectés
- Délégation au parent

---

## 1. StrongFather

| Rôle | Application Terminal |
|------|----------------------|
| Décision stratégique | Les actions (dépense, événement) sont **déléguées** au parent. StrongFather du parent autorise ou refuse. |
| Terminal | Ne décide pas ; transmet l'intention. |

---

## 2. KindMother

| Rôle | Application Terminal |
|------|----------------------|
| Persistance | Cache local, queue, identity via SQLite/rusqlite ou KindMother. |
| Intégrité | Vérifier checksums si applicable ; pas de corruption silencieuse. |

---

## 3. MasterButler

| Rôle | Application Terminal |
|------|----------------------|
| Capacités | Le parent expose les capacités (jaykonta.expense, jaykoa.event). Terminal consomme selon la liste fournie. |
| Terminal | Ne possède pas de registry complet ; hérite du parent. |

---

## 4. BorderGuard

| Rôle | Application Terminal |
|------|----------------------|
| Frontières | parent_cog_id définit la frontière. Terminal reste dans le périmètre du parent. |
| Confiance | Vérifier identité parent au Relay. |

---

## 5. WorrySentinel

| Rôle | Application Terminal |
|------|----------------------|
| Sécurité | Niveaux de sécurité (stockage chiffré, TLS). |
| environment_health | Rapport simplifié (storage_integrity, config_valid). |

---

## 6. TAMR

| Rôle | Application Terminal |
|------|----------------------|
| Intervention humaine | En cas de conflit (queue vs parent) : proposer résolution manuelle (merge, choix). |
| Terminal | Déléguer la décision à l'utilisateur si conflit détecté. |

---

## 7. CaringNanny, EverBuddy

| Core | Application |
|------|-------------|
| CaringNanny | Observation d'état (connection_state, sync status). |
| EverBuddy | Cycle de vie : pas de migration directe ; évolution via parent. |

---

## 8. Récapitulatif

| Core | Terminal |
|------|----------|
| StrongFather | Délégation au parent |
| KindMother | Persistance locale |
| MasterButler | Capacités héritées |
| BorderGuard | parent_cog_id = frontière |
| WorrySentinel | Sécurité, health |
| TAMR | Conflits → utilisateur |
| CaringNanny | État observable |
| EverBuddy | Pas de migration directe |
