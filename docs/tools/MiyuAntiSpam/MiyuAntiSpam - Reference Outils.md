# MiyuAntiSpam — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuAntiSpam** (`toolkit.security.antispam`). Chaque outil exécute une vérification ; **la décision de bloquer ou autoriser = StrongFather**.

**Référence :** [MiyuAntiSpam - Documentation Fondatrice](./MiyuAntiSpam%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|-----------------|------|
| `tool.antispam.captcha.generate` | Génère un défi CAPTCHA | 0–1 | Exécution seule ; pas de décision |
| `tool.antispam.captcha.verify` | Vérifie une réponse CAPTCHA | 0–1 | Exécution seule ; résultat fourni à StrongFather |
| `tool.antispam.flood.check` | Vérifie le flood (scope fourni) | 1 | Seuils = flux ou KindMother ; décision bloquer = StrongFather |
| `tool.antispam.rate_limit.check` | Vérifie la limite de tentatives (scope fourni) | 1 | Décision bloquer = StrongFather |

---

**Invariant :** Les Tools exécutent ; ils ne décident pas. Décision de bloquer = StrongFather.
