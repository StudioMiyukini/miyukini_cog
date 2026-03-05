# MiyuNotify â€” Documentation Fondatrice

## 1. Contexte

**MiyuNotify** est le **kit d'outils (Toolkit)** de notification de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils d'envoi d'email, d'envoi de notification push, et d'Ã©criture en boÃ®te de rÃ©ception in-app (inbox), sans logique mÃ©tier â€” le destinataire, le contenu et les options sont fournis dans le flux gouvernÃ© ; la dÃ©cision d'envoyer ou non relÃ¨ve de **StrongFather**.

L'autoritÃ© sur les donnÃ©es mÃ©tier (destinataires, prÃ©fÃ©rences, historique) appartient Ã  **KindMother**. MiyuNotify expose des capacitÃ©s d'exÃ©cution gouvernÃ©e (envoyer email, envoyer push, Ã©crire inbox) ; les dÃ©cisions (Ã  qui envoyer, quand, contenu autorisÃ©) relÃ¨vent de **StrongFather** et des OpÃ©rateurs.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuNotify, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e (SMTP, FCM, stockage inbox) ; la politique de contenu et les prÃ©fÃ©rences utilisateur (StrongFather / OpÃ©rateurs).

---

## 3. DÃ©finition canonique

> **MiyuNotify est une composition officielle d'outils de notification (email, push, inbox), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuNotify **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuNotify **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (envoyer email, envoyer push, Ã©crire inbox) ; destinataire, contenu et options fournis dans le flux ; dÃ©cision d'envoi = StrongFather.

**RÃ¨gle fondamentale :** Un Tool MiyuNotify **exÃ©cute** l'envoi ou l'Ã©criture ; il **ne dÃ©cide pas** si l'envoi doit avoir lieu â€” cela relÃ¨ve de StrongFather.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.notify.miyunotify` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `notify` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuNotify - Reference Outils](./MiyuNotify%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.notify.email.send` | Envoie un email (destinataire, sujet, corps fournis) ; autorisation = StrongFather |
| `tool.notify.push.send` | Envoie une notification push (device/channel, payload fournis) ; autorisation = StrongFather |
| `tool.notify.inbox.write` | Ã‰crit une entrÃ©e en boÃ®te de rÃ©ception in-app (destinataire, contenu fournis) ; Ã©criture = WriteIntent KindMother |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuNotify en contient trois.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : **dÃ©cision d'envoi = StrongFather** ; pour inbox.write, Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1 Ã  3** (donnÃ©es personnelles, envoi externe) |
| **Ã‰tats autorisÃ©s** | Tous sauf restriction WorrySentinel |
| **Ã‰tats interdits** | Selon politique WorrySentinel (ex. SECURITY_LOCKDOWN peut bloquer envoi externe) |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : destinataires, prÃ©fÃ©rences, historique inbox. L'outil `tool.notify.inbox.write` produit une **WriteIntent** vers KindMother. Les outils email.send et push.send exÃ©cutent un envoi externe (pas d'Ã©criture directe en base mÃ©tier par MiyuNotify ; les logs d'envoi relÃ¨vent de l'implÃ©mentation).

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuNotify - Tool Governance Compliance Contract](./contracts/governance/MiyuNotify%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

Ã€ l'implÃ©mentation : chaque Tool MiyuNotify est une unitÃ© logique pouvant devenir un **bloc MSCM** : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


