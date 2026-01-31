# MiyuPM — Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un développeur pour implémenter MiyuPM conformément aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pédagogique :** Aider à traduire les contrats MiyuPM en logique d'implémentation (Tools messagerie privée, gouvernance, WriteIntent KindMother).

**Avertissement :** Ce document ne crée aucune nouvelle règle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implémenter le kit MiyuPM (envoi, list/get, dossiers, brouillons, conversations, export) de manière conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne définit pas de nouvelles règles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la compréhension et l'application des contrats.

### 1.3 Sources contractuelles

- **MiyuPM - Documentation Fondatrice** : ToolkitId `toolkit.communication.pm`, liste des Tools (send, list, get, folder.*, draft.*, conversation.*, export), gouvernance, relation KindMother.
- **MiyuPM - Reference Outils** : Détail de chaque ToolId.
- **MiyuPM - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities, obligations spécifiques.
- **Master Butler - Tool Governance Compliance Template** : Format ToolId, structure Toolkit.

---

## 2. Principes à respecter absolument

### 2.1 Pas de décision ALLOW/DENY (BOUND-1)

**Décision d'envoi = StrongFather.** MiyuPM est invoqué uniquement après décision de la gouvernance (envoi autorisé, destinataires, quotas). L'implémentation ne doit pas ré-évaluer les permissions. En cas d'appel hors gouvernance, refuser l'exécution et signaler.

### 2.2 Pas de choix métier (BOUND-2)

Les Tools exécutent sur les données fournies (destinataire, contenu, dossiers, filtres). Aucune décision sur qui peut envoyer à qui, ni sur les pièces jointes (MiyuMedia) ou l'anti-spam (MiyuAntiSpam).

### 2.3 Toute écriture = WriteIntent KindMother (BOUND-3)

Toute création, mise à jour ou suppression (message, dossier, brouillon) = **WriteIntent** vers KindMother. Aucun accès direct à la base. `tool.pm.send` déclenche une WriteIntent pour le message ; `tool.pm.draft.*` et `tool.pm.folder.*` idem.

### 2.4 à 2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'Opérateur appelant (contexte anonymisé) ; uniquement les ToolIds déclarés (send, list, get, folder.list|create|update, draft.create|update|list, conversation.list|get, export).

### 2.7 Niveau de sécurité et états

Niveau **2** (données personnelles, messagerie). États autorisés : `HEALTHY`, `DEGRADED`. États interdits : `SECURITY_LOCKDOWN`, `MAINTENANCE`. Vérifier l'état avant exécution.

### 2.8 Alignement MIP/MSCM

Domaine `communication`, layer Strate 6. À l'implémentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | Implémentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de décision ALLOW/DENY | Décision d'envoi = StrongFather ; exécution sur mandat uniquement |
| **BOUND-2** | Pas de choix métier | Pas de décision destinataires, quotas, pièces jointes |
| **BOUND-3** | Pas d'accès direct | Toute écriture via WriteIntent KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'Opérateur appelant | Contexte anonymisé |
| **BOUND-6** | Pas de capacité nouvelle | Uniquement ToolIds déclarés |

---

## 4. Patterns recommandés

### 4.1 Structure des Tools

Chaque ToolId = unité atomique : entrée (contexte gouverné, paramètres : destinataire, contenu, dossier_id, filtres, format export), sortie (résultat ou erreur). Pas d'état métier partagé. Format : `tool.pm.send`, `tool.pm.list`, `tool.pm.get`, `tool.pm.folder.*`, `tool.pm.draft.*`, `tool.pm.conversation.*`, `tool.pm.export`.

### 4.2 Interface avec KindMother

Messages, dossiers, brouillons, conversations : toute écriture produit une **WriteIntent** vers KindMother. Les lectures (list, get) s'appuient sur des données fournies dans le flux ou sur un contrat d'intégration documenté. MiyuPM n'accède pas directement à la base.

### 4.3 Export

`tool.pm.export` exécute l'export (format fourni) sur les données fournies ; exécution seule, pas d'écriture métier ; résultat retourné dans le flux.

### 4.4 Gestion des erreurs et traçabilité

Erreurs remontées sans exposer de données personnelles. En cas de violation de bornage, refus et signal. Logger du Kernel pour traçabilité (sans contenu des messages).

---

## 5. Alignement MIP / MSCM

- **Domaine** : `communication` (toolkit.communication.pm).
- **Layer** : Strate 6 dans layers.json.
- **Blocs** : Chaque Tool MiyuPM = unité logique avec `id`, `do`, `role`, `layer`. Balisage MSCM selon le [Protocole MIP v1](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 6. Tests

Les tests relèvent des bonnes pratiques projet et du Tool Governance Compliance Contract. Scénarios recommandés : envoi via WriteIntent, list/get avec données en flux, création dossier/brouillon, conversation.list|get, export sans écriture métier.

---

## 7. Références croisées

| Document | Lien |
|----------|------|
| MiyuPM - Documentation Fondatrice | [MiyuPM - Documentation Fondatrice](../MiyuPM%20-%20Documentation%20Fondatrice.md) |
| MiyuPM - Reference Outils | [MiyuPM - Reference Outils](../MiyuPM%20-%20Reference%20Outils.md) |
| MiyuPM - Tool Governance Compliance Contract | [MiyuPM - Tool Governance Compliance Contract](../contracts/governance/MiyuPM%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| MIP v1 | [MIP v1 MSCM Index Protocol](../../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document informatif, non normatif
