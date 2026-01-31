# MiyuAntiSpam — Documentation Fondatrice

## 1. Contexte

**MiyuAntiSpam** est le **kit d'outils (Toolkit)** anti-spam et contrôle d'accès (CAPTCHA, flood control, limite tentatives) de l'écosystème Miyukini. Il intègre les outils de génération/vérification CAPTCHA, de vérification flood et de limite de tentatives, alignés sur [Équivalents Moteur Forum](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Moteur%20Forum.md).

Les seuils et règles (flood, taux, limites) peuvent être fournis par le flux ou par **KindMother**. MiyuAntiSpam expose des capacités d'**exécution** (générer CAPTCHA, vérifier, compter) ; **la décision de bloquer ou autoriser** relève de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuAntiSpam, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'intégration reCAPTCHA (implémentation) ; la politique de seuils (StrongFather / données KindMother).

---

## 3. Définition canonique

> **MiyuAntiSpam est une composition officielle d'outils anti-spam (CAPTCHA, flood control, limite tentatives), déclarée et gouvernée par l'environnement.**

- MiyuAntiSpam **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuAntiSpam **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (générer, vérifier, compter) ; **décision de bloquer = StrongFather**.

**Règle fondamentale :** Les Tools **exécutent** (générer CAPTCHA, vérifier, vérifier flood, vérifier rate limit) ; ils **ne décident pas** si l'utilisateur doit être bloqué — cela relève de StrongFather.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.security.antispam` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `security` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuAntiSpam - Reference Outils](./MiyuAntiSpam%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.antispam.captcha.generate` | Génère un défi CAPTCHA (exécution seule) |
| `tool.antispam.captcha.verify` | Vérifie une réponse CAPTCHA (exécution seule) ; décision bloquer = StrongFather |
| `tool.antispam.flood.check` | Vérifie le flood (scope : post, pm, registration) ; seuils fournis ou KindMother ; décision bloquer = StrongFather |
| `tool.antispam.rate_limit.check` | Vérifie la limite de tentatives (scope : search, registration, etc.) ; décision bloquer = StrongFather |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuAntiSpam en contient quatre.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : **décision de bloquer ou autoriser = StrongFather** ; les Tools exécutent la vérification et renvoient un résultat ; StrongFather décide de l'action (autoriser, bloquer, CAPTCHA obligatoire).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **1 à 2** (sécurité périphérique) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | Selon politique WorrySentinel |

---

## 8. Relation avec KindMother

**KindMother** peut fournir les seuils (flood, rate limit) ou les règles ; les Tools **lisent** ces données pour exécuter la vérification. Pas d'écriture métier par MiyuAntiSpam (sauf compteurs si définis en données ; alors WriteIntent KindMother).

Les obligations de conformité détaillées sont dans [MiyuAntiSpam - Tool Governance Compliance Contract](./contracts/governance/MiyuAntiSpam%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuAntiSpam sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuAntiSpam devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Équivalents Moteur Forum | [Miyukini Conceptual References - Equivalents Moteur Forum](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Moteur%20Forum.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
