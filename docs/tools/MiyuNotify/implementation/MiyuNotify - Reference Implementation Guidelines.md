# MiyuNotify â€” Reference Implementation Guidelines

## Statut du document

**POST-FONDATION / NON NORMATIF / INFORMATIF**

Ce document est **informatif, non normatif, et non contractuel**. Il guide un dÃ©veloppeur pour implÃ©menter MiyuNotify conformÃ©ment aux contrats fondateurs, sans violer les invariants, interdictions et bornages.

**Objectif pÃ©dagogique :** Aider Ã  traduire les contrats MiyuNotify en logique d'implÃ©mentation (Tools email, push, inbox, gouvernance, WriteIntent pour inbox).

**Avertissement :** Ce document ne crÃ©e aucune nouvelle rÃ¨gle contractuelle et ne modifie aucun contrat existant. Les contrats fondateurs priment toujours sur ce guide.

---

## 1. Introduction

### 1.1 Objectif

Fournir des lignes directrices pour implÃ©menter le kit MiyuNotify (email.send, push.send, inbox.write) de maniÃ¨re conforme aux contrats : Documentation Fondatrice, Reference Outils, Tool Governance Compliance Contract.

### 1.2 Nature informative

Ce document est **purement informatif**. Il ne dÃ©finit pas de nouvelles rÃ¨gles, n'impose pas de technologies, et ne prescrit pas de solutions techniques. Il guide la comprÃ©hension et l'application des contrats.

### 1.3 Sources contractuelles

- **MiyuNotify - Documentation Fondatrice** : ToolkitId `toolkit.notify.miyunotify`, liste des Tools (email.send, push.send, inbox.write), gouvernance, relation KindMother (inbox).
- **MiyuNotify - Reference Outils** : DÃ©tail de chaque ToolId.
- **MiyuNotify - Tool Governance Compliance Contract** : ToolkitId, ToolIds, capabilities, obligations spÃ©cifiques.
- **Master Butler - Tool Governance Compliance Template** : Format ToolId, structure Toolkit.

---

## 2. Principes Ã  respecter absolument

### 2.1 Pas de dÃ©cision ALLOW/DENY (BOUND-1)

**DÃ©cision d'envoi = StrongFather.** MiyuNotify est invoquÃ© uniquement aprÃ¨s dÃ©cision de la gouvernance (destinataire, contenu, options fournis dans le flux ; la dÃ©cision d'envoyer ou non relÃ¨ve de StrongFather). L'implÃ©mentation ne doit pas rÃ©-Ã©valuer les permissions. En cas d'appel hors gouvernance, refuser l'exÃ©cution et signaler.

### 2.2 Pas de choix mÃ©tier (BOUND-2)

Les Tools exÃ©cutent sur les donnÃ©es fournies (destinataire, sujet, corps, device/channel, payload, contenu inbox). Aucune dÃ©cision sur Ã  qui envoyer, quand, ni contenu autorisÃ© â€” ressort de StrongFather / OpÃ©rateurs.

### 2.3 Pas d'accÃ¨s direct ; inbox.write = WriteIntent (BOUND-3)

- **tool.notify.inbox.write** : produit une **WriteIntent** vers KindMother (donnÃ©es destinataire, contenu fournis). Aucune Ã©criture directe en base par MiyuNotify.
- **tool.notify.email.send** et **tool.notify.push.send** : exÃ©cutent un envoi externe (SMTP, FCM, etc.) ; pas d'Ã©criture directe en base mÃ©tier par MiyuNotify. Les logs d'envoi relÃ¨vent de l'implÃ©mentation (traÃ§abilitÃ© technique sans dÃ©cision mÃ©tier).

### 2.4 Ã  2.6 (BOUND-4, BOUND-5, BOUND-6)

Pas de modification du contexte d'autorisation ; pas de connaissance de l'OpÃ©rateur appelant ; uniquement les ToolIds dÃ©clarÃ©s (email.send, push.send, inbox.write).

### 2.7 Niveau de sÃ©curitÃ© et Ã©tats

Niveau **1 Ã  3** (donnÃ©es personnelles, envoi externe). Ã‰tats autorisÃ©s : tous sauf restriction WorrySentinel. Ã‰tats interdits : selon politique WorrySentinel (ex. SECURITY_LOCKDOWN peut bloquer envoi externe). VÃ©rifier l'Ã©tat avant exÃ©cution.

### 2.8 Alignement MIP/MSCM

Domaine `notify`, layer Strate 6. Ã€ l'implÃ©mentation, baliser le code MSCM pour alimenter blocks.json, domains.json, layers.json selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 3. Interdictions (rappel contractuel)

| Code | Interdiction | ImplÃ©mentation |
|------|--------------|----------------|
| **BOUND-1** | Pas de dÃ©cision ALLOW/DENY | DÃ©cision d'envoi = StrongFather ; exÃ©cution sur mandat uniquement |
| **BOUND-2** | Pas de choix mÃ©tier | Pas de dÃ©cision destinataire, contenu, politique |
| **BOUND-3** | Pas d'accÃ¨s direct | inbox.write = WriteIntent KindMother ; email/push = envoi externe sans Ã©criture mÃ©tier directe |
| **BOUND-4** | Pas de modification du contexte d'autorisation | Lecture seule du contexte |
| **BOUND-5** | Pas de connaissance de l'OpÃ©rateur appelant | Contexte anonymisÃ© |
| **BOUND-6** | Pas de capacitÃ© nouvelle | Uniquement email.send, push.send, inbox.write |

---

## 4. Patterns recommandÃ©s

### 4.1 Structure des Tools

Chaque ToolId = unitÃ© atomique : entrÃ©e (contexte gouvernÃ©, paramÃ¨tres : destinataire, sujet, corps, device/channel, payload, contenu inbox), sortie (succÃ¨s/Ã©chec ou erreur). Pas d'Ã©tat mÃ©tier partagÃ©. Format : `tool.notify.email.send`, `tool.notify.push.send`, `tool.notify.inbox.write`.

### 4.2 Interface avec KindMother

- **inbox.write** : donnÃ©es fournies dans le flux â†’ **WriteIntent** vers KindMother (boÃ®te de rÃ©ception in-app). Aucun appel direct Ã  la base depuis MiyuNotify.
- **email.send / push.send** : exÃ©cution technique d'envoi ; pas d'Ã©criture mÃ©tier ; logs d'envoi selon politique projet (sans dÃ©cision mÃ©tier).

### 4.3 SÃ©curitÃ© et envoi externe

DonnÃ©es personnelles et envoi externe : respecter le niveau de sÃ©curitÃ© du kit (1 Ã  3) et les politiques WorrySentinel (blocage envoi en SECURITY_LOCKDOWN si applicable). Ne pas exposer de donnÃ©es sensibles dans les erreurs.

### 4.4 Gestion des erreurs et traÃ§abilitÃ©

Erreurs techniques (destinataire invalide, service externe indisponible) remontÃ©es sans exposer de donnÃ©es personnelles. Logger du Kernel pour traÃ§abilitÃ© (sans contenu des messages).

---

## 5. Alignement MIP / MSCM

- **Domaine** : `notify` (toolkit.notify.miyunotify).
- **Layer** : Strate 6 dans layers.json.
- **Blocs** : Chaque Tool MiyuNotify = unitÃ© logique avec `id`, `do`, `role`, `layer`. Balisage MSCM selon le [Protocole MIP v1](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 6. Tests

Les tests relÃ¨vent des bonnes pratiques projet et du Tool Governance Compliance Contract. ScÃ©narios recommandÃ©s : inbox.write via WriteIntent, email.send et push.send avec paramÃ¨tres en flux (mocks pour SMTP/FCM), vÃ©rification des Ã©tats (blocage envoi si SECURITY_LOCKDOWN selon politique).

---

## 7. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| MiyuNotify - Documentation Fondatrice | [MiyuNotify - Documentation Fondatrice](../MiyuNotify%20-%20Documentation%20Fondatrice.md) |
| MiyuNotify - Reference Outils | [MiyuNotify - Reference Outils](../MiyuNotify%20-%20Reference%20Outils.md) |
| MiyuNotify - Tool Governance Compliance Contract | [MiyuNotify - Tool Governance Compliance Contract](../contracts/governance/MiyuNotify%20-%20Tool%20Governance%20Compliance%20Contract.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//..//miyukini-webway-system//reference//_index.md) |
| MIP v1 | [MIP v1 MSCM Index Protocol](..//..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document informatif, non normatif

