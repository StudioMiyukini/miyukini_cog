# MiyuAntiSpam â€” Documentation Fondatrice

## 1. Contexte

**MiyuAntiSpam** est le **kit d'outils (Toolkit)** anti-spam et contrÃ´le d'accÃ¨s (CAPTCHA, flood control, limite tentatives) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de gÃ©nÃ©ration/vÃ©rification CAPTCHA, de vÃ©rification flood et de limite de tentatives, alignÃ©s sur [Ã‰quivalents Moteur Forum](..//..//miyukini-webway-system//reference//_index.md).

Les seuils et rÃ¨gles (flood, taux, limites) peuvent Ãªtre fournis par le flux ou par **KindMother**. MiyuAntiSpam expose des capacitÃ©s d'**exÃ©cution** (gÃ©nÃ©rer CAPTCHA, vÃ©rifier, compter) ; **la dÃ©cision de bloquer ou autoriser** relÃ¨ve de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuAntiSpam, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'intÃ©gration reCAPTCHA (implÃ©mentation) ; la politique de seuils (StrongFather / donnÃ©es KindMother).

---

## 3. DÃ©finition canonique

> **MiyuAntiSpam est une composition officielle d'outils anti-spam (CAPTCHA, flood control, limite tentatives), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuAntiSpam **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuAntiSpam **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (gÃ©nÃ©rer, vÃ©rifier, compter) ; **dÃ©cision de bloquer = StrongFather**.

**RÃ¨gle fondamentale :** Les Tools **exÃ©cutent** (gÃ©nÃ©rer CAPTCHA, vÃ©rifier, vÃ©rifier flood, vÃ©rifier rate limit) ; ils **ne dÃ©cident pas** si l'utilisateur doit Ãªtre bloquÃ© â€” cela relÃ¨ve de StrongFather.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.security.antispam` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `security` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuAntiSpam - Reference Outils](./MiyuAntiSpam%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.antispam.captcha.generate` | GÃ©nÃ¨re un dÃ©fi CAPTCHA (exÃ©cution seule) |
| `tool.antispam.captcha.verify` | VÃ©rifie une rÃ©ponse CAPTCHA (exÃ©cution seule) ; dÃ©cision bloquer = StrongFather |
| `tool.antispam.flood.check` | VÃ©rifie le flood (scope : post, pm, registration) ; seuils fournis ou KindMother ; dÃ©cision bloquer = StrongFather |
| `tool.antispam.rate_limit.check` | VÃ©rifie la limite de tentatives (scope : search, registration, etc.) ; dÃ©cision bloquer = StrongFather |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuAntiSpam en contient quatre.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : **dÃ©cision de bloquer ou autoriser = StrongFather** ; les Tools exÃ©cutent la vÃ©rification et renvoient un rÃ©sultat ; StrongFather dÃ©cide de l'action (autoriser, bloquer, CAPTCHA obligatoire).

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1 Ã  2** (sÃ©curitÃ© pÃ©riphÃ©rique) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | Selon politique WorrySentinel |

---

## 8. Relation avec KindMother

**KindMother** peut fournir les seuils (flood, rate limit) ou les rÃ¨gles ; les Tools **lisent** ces donnÃ©es pour exÃ©cuter la vÃ©rification. Pas d'Ã©criture mÃ©tier par MiyuAntiSpam (sauf compteurs si dÃ©finis en donnÃ©es ; alors WriteIntent KindMother).

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuAntiSpam - Tool Governance Compliance Contract](./contracts/governance/MiyuAntiSpam%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuAntiSpam sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuAntiSpam devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents Moteur Forum | [Miyukini Conceptual References - Equivalents Moteur Forum](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


