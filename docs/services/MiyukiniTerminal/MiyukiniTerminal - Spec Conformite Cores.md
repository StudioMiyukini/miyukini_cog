# MiyukiniTerminal â€” SpÃ©cification ConformitÃ© Cores

## Contexte

Ce document dÃ©crit le **mapping des Cores** Miyukini sur le Terminal : StrongFather (autorisation actions), KindMother (persistance), MasterButler (capacitÃ©s), BorderGuard (frontiÃ¨res), WorrySentinel (niveaux sÃ©curitÃ©), TAMR (conflits). Contrats respectÃ©s.

**RÃ©fÃ©rences :**

- [Document Fondateur](./MiyukiniTerminal%20-%20Document%20Fondateur.md)
- [Architecture Miyukini](_index.md)

---

## PortÃ©e / Scope

- Mapping Cores sur Terminal
- Contrats respectÃ©s
- DÃ©lÃ©gation au parent

---

## 1. StrongFather

| RÃ´le | Application Terminal |
|------|----------------------|
| DÃ©cision stratÃ©gique | Les actions (dÃ©pense, Ã©vÃ©nement) sont **dÃ©lÃ©guÃ©es** au parent. StrongFather du parent autorise ou refuse. |
| Terminal | Ne dÃ©cide pas ; transmet l'intention. |

---

## 2. KindMother

| RÃ´le | Application Terminal |
|------|----------------------|
| Persistance | Cache local, queue, identity via SQLite/rusqlite ou KindMother. |
| IntÃ©gritÃ© | VÃ©rifier checksums si applicable ; pas de corruption silencieuse. |

---

## 3. MasterButler

| RÃ´le | Application Terminal |
|------|----------------------|
| CapacitÃ©s | Le parent expose les capacitÃ©s (jaykonta.expense, jaykoa.event). Terminal consomme selon la liste fournie. |
| Terminal | Ne possÃ¨de pas de registry complet ; hÃ©rite du parent. |

---

## 4. BorderGuard

| RÃ´le | Application Terminal |
|------|----------------------|
| FrontiÃ¨res | parent_cog_id dÃ©finit la frontiÃ¨re. Terminal reste dans le pÃ©rimÃ¨tre du parent. |
| Confiance | VÃ©rifier identitÃ© parent au Relay. |

---

## 5. WorrySentinel

| RÃ´le | Application Terminal |
|------|----------------------|
| SÃ©curitÃ© | Niveaux de sÃ©curitÃ© (stockage chiffrÃ©, TLS). |
| environment_health | Rapport simplifiÃ© (storage_integrity, config_valid). |

---

## 6. TAMR

| RÃ´le | Application Terminal |
|------|----------------------|
| Intervention humaine | En cas de conflit (queue vs parent) : proposer rÃ©solution manuelle (merge, choix). |
| Terminal | DÃ©lÃ©guer la dÃ©cision Ã  l'utilisateur si conflit dÃ©tectÃ©. |

---

## 7. CaringNanny, EverBuddy

| Core | Application |
|------|-------------|
| CaringNanny | Observation d'Ã©tat (connection_state, sync status). |
| EverBuddy | Cycle de vie : pas de migration directe ; Ã©volution via parent. |

---

## 8. RÃ©capitulatif

| Core | Terminal |
|------|----------|
| StrongFather | DÃ©lÃ©gation au parent |
| KindMother | Persistance locale |
| MasterButler | CapacitÃ©s hÃ©ritÃ©es |
| BorderGuard | parent_cog_id = frontiÃ¨re |
| WorrySentinel | SÃ©curitÃ©, health |
| TAMR | Conflits â†’ utilisateur |
| CaringNanny | Ã‰tat observable |
| EverBuddy | Pas de migration directe |

