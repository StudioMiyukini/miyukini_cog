# MiyuNotify — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuNotify. Référence technique des capacités atomiques de notification. Décision d'envoi = StrongFather ; destinataire et contenu fournis dans le flux.

**Référence du kit :** [MiyuNotify - Documentation Fondatrice](./MiyuNotify%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.notify.email.send` | Envoyer email | Envoie un email (destinataire, sujet, corps, pièces jointes optionnelles fournis) ; autorisation = StrongFather | 1–3 |
| `tool.notify.push.send` | Envoyer push | Envoie une notification push (device/channel, payload fournis) ; autorisation = StrongFather | 1–3 |
| `tool.notify.inbox.write` | Écrire inbox | Écrit une entrée en boîte de réception in-app (destinataire, contenu, métadonnées fournis) ; WriteIntent KindMother | 1–2 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
