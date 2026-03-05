# MiyuPM â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter MiyuPM conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats MiyuPM en logique d'implÃ©mentation (Tools messagerie privÃ©e, gouvernance, WriteIntent KindMother).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit MiyuPM (envoi, list/get, dossiers, brouillons, conversations, export) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne dÃ©finit pas de nouvelles rÃ¨gles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la comprÃ©hension et l'application des contrats.

### 1.3 Sources contractuelles

- **MiyuPM - Documentation Fondatrice** : ToolkitId `toolkit.communication.pm`, liste des Tools (send, list, get, folder.*, draft.*, conversation.*, export), gouvernance, relation KindMother.
- **MiyuPM - Reference Outils** : DÃ©tail de chaque ToolId.
- **MiyuPM - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities, obligations spÃ©cifiques.
- **Master Butler - Tool Governance Compliance Template** : Format ToolId, structure Toolkit.

---

## 2. Principes Ã  respecter absolument

### 2.1 Pas de dÃ©cision ALLOW/DENY (BOUND-1)

**DÃ©cision d'envoi = StrongFather.** MiyuPM est invoquÃ© uniquement aprÃ¨s dÃ©cision de la gouvernance (envoi autorisÃ©, destinataires, quotas). L'implÃ©mentation ne doit pas rÃ©-Ã©valuer les permissions. En cas d'appel hors gouvernance, refuser l'exÃ©cution et signaler.

### 2.2 Pas de choix mÃ©tier (BOUND-2)

Les Tools exÃ©cutent sur les donnÃ©es fournies (destinataire, contenu, dossiers, filtres). Aucune dÃ©cision sur qui peut envoyer Ã  qui, ni sur les piÃ¨ces jointes (MiyuMedia) ou l'anti-spam (MiyuAntiSpam).

### 2.3 Toute Ã©criture = WriteIntent KindMother (BOUND-3)

Toute crÃ©ation, mise Ã  jour ou suppression (message, dossier, brouillon) = **WriteIntent** vers KindMother. Aucun accÃ¨s direct Ã  la base. `tool.pm.send` dÃ©clenche une WriteIntent pour le message ; `tool.pm.draft.*` et `tool.pm.folder.*` idem.

### 2.4 Ã  2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'OpÃ©rateur appelant (contexte anonymisÃ©) ; uniquement les ToolIds dÃ©clarÃ©s (send, list, get, folder.list|create|update, draft.create|update|list, conversation.list|get, export).

### 2.7 Niveau de sÃ©curitÃ© et Ã©tats

Niveau **2** (donnÃ©es personnelles, messagerie). Ã‰tats autorisÃ©s : `HEALTHY`, `DEGRADED`. Ã‰tats interdits : `SECURITY_LOCKDOWN`, `MAINTENANCE`. VÃ©rifier l'Ã©tat avant exÃ©cution.

### 2.8 Alignement MIP/MSCM

Domaine `communication`, layer Strate 6. Ã€ l'implÃ©mentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision ALLOW/DENY | DÃ©cision d'envoi = StrongFather ; exÃ©cution sur mandat uniquement |
| **BOUND-2** | Pas de choix mÃ©tier | Pas de dÃ©cision destinataires, quotas, piÃ¨ces jointes |
| **BOUND-3** | Pas d'accÃ¨s direct | Toute Ã©criture via WriteIntent KindMother |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Contexte anonymisÃ© |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Uniquement ToolIds dÃ©clarÃ©s |

---

## 4. Patterns recommandÃ©s

### 4.1 Structure des Tools

Chaque ToolId = unitÃ© atomique : entrÃ©e (contexte gouvernÃ©, paramÃ¨tres : destinataire, contenu, dossier_id, filtres, format export), sortie (rÃ©sultat ou erreur). Pas d'Ã©tat mÃ©tier partagÃ©. Format : `tool.pm.send`, `tool.pm.list`, `tool.pm.get`, `tool.pm.folder.*`, `tool.pm.draft.*`, `tool.pm.conversation.*`, `tool.pm.export`.

### 4.2 Interface avec KindMother

Messages, dossiers, brouillons, conversations : toute Ã©criture produit une **WriteIntent** vers KindMother. Les lectures (list, get) s'appuient sur des donnÃ©es fournies dans le flux ou sur un contrat d'intÃ©gration documentÃ©. MiyuPM n'accÃ¨de pas directement Ã  la base.

### 4.3 Export

`tool.pm.export` exÃ©cute l'export (format fourni) sur les donnÃ©es fournies ; exÃ©cution seule, pas d'Ã©criture mÃ©tier ; rÃ©sultat retournÃ© dans le flux.

### 4.4 Gestion des erreurs et traÃ§abilitÃ©

Erreurs remontÃ©es sans exposer de donnÃ©es personnelles. En cas de violation de bornage, refus et signal. Logger du Kernel pour traÃ§abilitÃ© (sans contenu des messages).

---

## 5. Alignement MIP / MSCM

- **Domaine** : `communication` (toolkit.communication.pm).
- **Layer** : Strate 6 dans layers.json.
- **Blocs** : Chaque Tool MiyuPM = unitÃ© logique avec `id`, `do`, `role`, `layer`. Balisage MSCM selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 6. Tests

Les tests relÃ¨vent des bonnes pratiques projet et du Tool Governance Compliance Contract. ScÃ©narios recommandÃ©s : envoi via WriteIntent, list/get avec donnÃ©es en flux, crÃ©ation dossier/brouillon, conversation.list|get, export sans Ã©criture mÃ©tier.

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuPM - Documentation Fondatrice | [MiyuPM - Documentation Fondatrice](../MiyuPM%20-%20Documentation%20Fondatrice.md) |
| MiyuPM - Reference Outils | [MiyuPM - Reference Outils](../MiyuPM%20-%20Reference%20Outils.md) |
| MiyuPM - Tool Governance Compliance Contract | [MiyuPM - Tool Governance Compliance Contract](../contracts/governance/MiyuPM%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//..//miyukini-webway-system//reference//_index.md) |
| MIP v1 | [MIP v1 MSCM Index Protocol](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document informatif, non normatif

