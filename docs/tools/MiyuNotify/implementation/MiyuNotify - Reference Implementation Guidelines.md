# MiyuNotify — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter MiyuNotify conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MiyuNotify en logique d'implémentation (Tools email, push, inbox, gouvernance, WriteIntent pour inbox).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuNotify (email.send, push.send, inbox.write) de manière conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne définit pas de nouvelles règles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la compréhension et l'application des contrats.

### 1.3 Sources contractuelles

- **MiyuNotify - Documentation Fondatrice** : ToolkitId `toolkit.notify.miyunotify`, liste des Tools (email.send, push.send, inbox.write), gouvernance, relation KindMother (inbox).
- **MiyuNotify - Reference Outils** : Détail de chaque ToolId.
- **MiyuNotify - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities, obligations spécifiques.
- **Master Butler - Tool Governance Compliance Template** : Format ToolId, structure Toolkit.

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision ALLOW/DENY (BOUND-1)

**Décision d'envoi = StrongFather.** MiyuNotify est invoqué uniquement après décision de la gouvernance (destinataire, contenu, options fournis dans le flux ; la décision d'envoyer ou non relève de StrongFather). L'implémentation ne doit pas ré-évaluer les permissions. En cas d'appel hors gouvernance, refuser l'exécution et signaler.

### 2.2 Pas de choix métier (BOUND-2)

Les Tools exécutent sur les données fournies (destinataire, sujet, corps, device/channel, payload, contenu inbox). Aucune décision sur à qui envoyer, quand, ni contenu autorisé — ressort de StrongFather / Opérateurs.

### 2.3 Pas d'accès direct ; inbox.write = WriteIntent (BOUND-3)

- **tool.notify.inbox.write** : produit une **WriteIntent** vers KindMother (données destinataire, contenu fournis). Aucune écriture directe en base par MiyuNotify.
- **tool.notify.email.send** et **tool.notify.push.send** : exécutent un envoi externe (SMTP, FCM, etc.) ; pas d'écriture directe en base métier par MiyuNotify. Les logs d'envoi relèvent de l'implémentation (traçabilité technique sans décision métier).

### 2.4 à 2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'Opérateur appelant ; uniquement les ToolIds déclarés (email.send, push.send, inbox.write).

### 2.7 Niveau de sécurité et états

Niveau **1 à 3** (données personnelles, envoi externe). États autorisés : tous sauf restriction WorrySentinel. États interdits : selon politique WorrySentinel (ex. SECURITY_LOCKDOWN peut bloquer envoi externe). Vérifier l'état avant exécution.

### 2.8 Alignement MIP/MSCM

Domaine `notify`, layer Strate 6. À l'implémentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Décision d'envoi = StrongFather ; exécution sur mandat uniquement |
| **BOUND-2** | Pas de choix métier | Pas de décision destinataire, contenu, politique |
| **BOUND-3** | Pas d'accès direct | inbox.write = WriteIntent KindMother ; email/push = envoi externe sans écriture métier directe |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement email.send, push.send, inbox.write |

---

## 4. Patterns recommandés

### 4.1 Structure des Tools

Chaque ToolId = unité atomique : entrée (contexte gouverné, paramètres : destinataire, sujet, corps, device/channel, payload, contenu inbox), sortie (succès/échec ou erreur). Pas d'état métier partagé. Format : `tool.notify.email.send`, `tool.notify.push.send`, `tool.notify.inbox.write`.

### 4.2 Interface avec KindMother

- **inbox.write** : données fournies dans le flux → **WriteIntent** vers KindMother (boîte de réception in-app). Aucun appel direct à la base depuis MiyuNotify.
- **email.send / push.send** : exécution technique d'envoi ; pas d'écriture métier ; logs d'envoi selon politique projet (sans décision métier).

### 4.3 Sécurité et envoi externe

Données personnelles et envoi externe : respecter le niveau de sécurité du kit (1 à 3) et les politiques WorrySentinel (blocage envoi en SECURITY_LOCKDOWN si applicable). Ne pas exposer de données sensibles dans les erreurs.

### 4.4 Gestion des erreurs et traçabilité

Erreurs techniques (destinataire invalide, service externe indisponible) remontées sans exposer de données personnelles. Logger du Kernel pour traçabilité (sans contenu des messages).

---

## 5. Alignement MIP / MSCM

- **Domaine** : `notify` (toolkit.notify.miyunotify).
- **Layer** : Strate 6 dans layers.json.
- **Blocs** : Chaque Tool MiyuNotify = unité logique avec `id`, `do`, `role`, `layer`. Balisage MSCM selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 6. Tests

Les tests relèvent des bonnes pratiques projet et du Tool Governance Compliance Contract. Scénarios recommandés : inbox.write via WriteIntent, email.send et push.send avec paramètres en flux (mocks pour SMTP/FCM), vérification des états (blocage envoi si SECURITY_LOCKDOWN selon politique).

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuNotify - Documentation Fondatrice | [MiyuNotify - Documentation Fondatrice](../MiyuNotify%20-%20Documentation%20Fondatrice.md) |
| MiyuNotify - Reference Outils | [MiyuNotify - Reference Outils](../MiyuNotify%20-%20Reference%20Outils.md) |
| MiyuNotify - Tool Governance Compliance Contract | [MiyuNotify - Tool Governance Compliance Contract](../contracts/governance/MiyuNotify%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| MIP v1 | [MIP v1 MSCM Index Protocol](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document informatif, non normatif
