# MiyuNotify — Documentation Fondatrice

## 1. Contexte

**MiyuNotify** est le **kit d'outils (Toolkit)** de notification de l'écosystème Miyukini. Il intègre les outils d'envoi d'email, d'envoi de notification push, et d'écriture en boîte de réception in-app (inbox), sans logique métier — le destinataire, le contenu et les options sont fournis dans le flux gouverné ; la décision d'envoyer ou non relève de **StrongFather**.

L'autorité sur les données métier (destinataires, préférences, historique) appartient à **KindMother**. MiyuNotify expose des capacités d'exécution gouvernée (envoyer email, envoyer push, écrire inbox) ; les décisions (à qui envoyer, quand, contenu autorisé) relèvent de **StrongFather** et des Opérateurs.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuNotify, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'implémentation détaillée (SMTP, FCM, stockage inbox) ; la politique de contenu et les préférences utilisateur (StrongFather / Opérateurs).

---

## 3. Définition canonique

> **MiyuNotify est une composition officielle d'outils de notification (email, push, inbox), déclarée et gouvernée par l'environnement.**

- MiyuNotify **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuNotify **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (envoyer email, envoyer push, écrire inbox) ; destinataire, contenu et options fournis dans le flux ; décision d'envoi = StrongFather.

**Règle fondamentale :** Un Tool MiyuNotify **exécute** l'envoi ou l'écriture ; il **ne décide pas** si l'envoi doit avoir lieu — cela relève de StrongFather.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.notify.miyunotify` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `notify` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuNotify - Reference Outils](./MiyuNotify%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.notify.email.send` | Envoie un email (destinataire, sujet, corps fournis) ; autorisation = StrongFather |
| `tool.notify.push.send` | Envoie une notification push (device/channel, payload fournis) ; autorisation = StrongFather |
| `tool.notify.inbox.write` | Écrit une entrée en boîte de réception in-app (destinataire, contenu fournis) ; écriture = WriteIntent KindMother |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuNotify en contient trois.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : **décision d'envoi = StrongFather** ; pour inbox.write, écriture = WriteIntent KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **1 à 3** (données personnelles, envoi externe) |
| **États autorisés** | Tous sauf restriction WorrySentinel |
| **États interdits** | Selon politique WorrySentinel (ex. SECURITY_LOCKDOWN peut bloquer envoi externe) |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données : destinataires, préférences, historique inbox. L'outil `tool.notify.inbox.write` produit une **WriteIntent** vers KindMother. Les outils email.send et push.send exécutent un envoi externe (pas d'écriture directe en base métier par MiyuNotify ; les logs d'envoi relèvent de l'implémentation).

---

## 9. Alignement MIP

À l'implémentation : chaque Tool MiyuNotify est une unité logique pouvant devenir un **bloc MSCM** : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
