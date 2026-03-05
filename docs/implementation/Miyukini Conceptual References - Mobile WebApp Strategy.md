# Miyukini Conceptual References â€” Mobile & WebApp Strategy

## 1. Contexte

Ce document dÃ©finit la **stratÃ©gie mobile et WebApp** de l'Ã©cosystÃ¨me Miyukini : comment les applications mobiles (Android/iOS) et les WebApps interagissent avec le systÃ¨me Miyukini tout en respectant les principes d'autonomie, de sÃ©curitÃ©, et de gouvernance.

**Principe fondamental (Ã  graver) :**

**"Le mobile n'est jamais une source de vÃ©ritÃ©. Il est un terminal intelligent, pas un nÅ“ud dÃ©cisionnel."**

## 2. PortÃ©e / Scope

Ce document dÃ©finit :
- L'architecture mobile cible (Android/iOS)
- L'optimisation de la passerelle Mobile â†” Serveur
- Les 3 niveaux de fonctionnement mobile (dÃ©gradÃ©)
- Le cache UX mobile (temporaire, pas persistance)
- La stratÃ©gie WebApp (filet de sÃ©curitÃ©)
- Le positionnement des cores (toujours cÃ´tÃ© serveur)

Ce document **ne couvre pas** :
- Les dÃ©tails d'implÃ©mentation technique (protocoles, APIs)
- Les spÃ©cifications UI/UX dÃ©taillÃ©es
- Les stratÃ©gies de synchronisation (voir KindMother)

---

## 3. Objectif Exact

Sur Android / iOS, Miyukini doit :

âœ… **Optimiser la passerelle app locale â†’ logique serveur**  
âš ï¸ **Fonctionner en mode dÃ©gradÃ© si la connexion est mauvaise ou absente**  
âŒ **Ne jamais exposer les cores**  
ðŸ” **Offrir une alternative WebApp sans casser la sÃ©curitÃ©**  
ðŸ§  **Rester cohÃ©rent avec StrongFather / BondingBrother / TAMR**

---

## 4. Principe Fondamental

### 4.1 Le Mobile n'est Jamais une Source de VÃ©ritÃ©

**Le mobile est :**
- âœ… Un terminal intelligent
- âœ… Un client d'interface utilisateur
- âœ… Un cache UX temporaire

**Le mobile n'est PAS :**
- âŒ Un nÅ“ud dÃ©cisionnel
- âŒ Une source de vÃ©ritÃ©
- âŒ Une autoritÃ© de persistance

### 4.2 RÃ¨gles Absolues

**Aucune dÃ©cision finale sur mobile :**
- Toutes les dÃ©cisions passent par StrongFather (serveur)
- Le mobile peut prÃ©parer, mais jamais valider

**Aucune persistance critique :**
- Le mobile ne persiste jamais de donnÃ©es critiques
- Toute persistance passe par KindMother (serveur)

**Aucune logique mÃ©tier forte :**
- Le mobile est une interface, pas une logique
- Toute logique mÃ©tier est cÃ´tÃ© serveur

---

## 5. Architecture Cible Mobile Miyukini

### 5.1 Vue d'Ensemble

```
[MOBILE DEVICE]
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ App native (Android/iOS)  â”‚
â”‚ â”œ UI / UX                 â”‚
â”‚ â”œ Cache UX local          â”‚
â”‚ â”œ Mode dÃ©gradÃ©            â”‚
â”‚ â”” Client BondingBrother   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–²â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
              â”‚
      (Optimized Gateway)
              â”‚
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Bonding Brother (Server)  â”‚
â”‚ â”œ Auth                    â”‚
â”‚ â”œ Session                 â”‚
â”‚ â”œ Adaptation mobile       â”‚
â”‚ â”” QoS / dÃ©gradation       â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–²â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
              â”‚
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â–¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ Miyukini Core Runtime     â”‚
â”‚ StrongFather / etc.       â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 5.2 Principe d'Isolation

**ðŸ‘‰ Le mobile ne parle jamais directement aux cores.**

Toute interaction passe par BondingBrother (serveur), qui :
- Adapte les requÃªtes mobiles
- GÃ¨re les sessions
- Applique la QoS et la dÃ©gradation
- ProtÃ¨ge les cores

---

## 6. Optimisation de la Passerelle Mobile â†” Serveur

### 6.1 Ce qu'on Optimise

- **Latence** : RÃ©duction du temps de rÃ©ponse
- **Nombre d'allers-retours** : AgrÃ©gation des requÃªtes
- **Taille des payloads** : Compression sÃ©mantique
- **TolÃ©rance rÃ©seau** : RÃ©silience aux pannes rÃ©seau

### 6.2 BondingBrother = Gateway Intelligent

BondingBrother joue 4 rÃ´les pour le mobile :

#### 1. AgrÃ©gation (Batch de RequÃªtes)
- Combine plusieurs requÃªtes en une seule
- RÃ©duit le nombre d'allers-retours
- Optimise la bande passante

#### 2. Compression SÃ©mantique
- Pas juste gzip (compression technique)
- Compression sÃ©mantique (adaptation du niveau de dÃ©tail)
- RÃ©ponses partielles autorisÃ©es

#### 3. RÃ©silience RÃ©seau
- Gestion des timeouts
- Retry intelligent
- DÃ©cisions DIFFÃ‰RÃ‰E si rÃ©seau instable

#### 4. Adaptation de Niveau de DÃ©tail
- GraphQL-like contractuel (mais contrÃ´lÃ©)
- Diffs d'Ã©tat, pas Ã©tats complets
- RÃ©ponses adaptÃ©es au contexte mobile

### 6.3 StratÃ©gies ConcrÃ¨tes

**GraphQL-like contractuel :**
- RequÃªtes structurÃ©es et contrÃ´lÃ©es
- Pas de requÃªtes arbitraires
- Contrats stricts dÃ©finis par Master Butler

**Diffs d'Ã©tat :**
- Envoi uniquement des changements
- Pas d'Ã©tats complets Ã  chaque requÃªte
- RÃ©duction de la bande passante

**RÃ©ponses partielles autorisÃ©es :**
- Le mobile peut accepter des rÃ©ponses incomplÃ¨tes
- Mode dÃ©gradÃ© explicite
- Indicateur de confiance visible

**Timeouts explicites :**
- Timeouts dÃ©finis et documentÃ©s
- DÃ©cision DIFFÃ‰RÃ‰E cÃ´tÃ© serveur si timeout
- Pas de blocage cÃ´tÃ© mobile

---

## 7. Mode DÃ©gradÃ© sur Mobile (ClÃ©)

### 7.1 Les 3 Niveaux de Fonctionnement Mobile

#### ðŸŸ¢ Niveau A â€” ConnectÃ© Normal

**Ã‰tat :** Connexion rÃ©seau stable et rapide.

**Comportement :**
- âœ… App native fonctionnelle
- âœ… Temps rÃ©el
- âœ… DonnÃ©es fraÃ®ches
- âœ… Interaction complÃ¨te

**âž¡ï¸ Ã‰tat nominal**

#### ðŸŸ¡ Niveau B â€” ConnectÃ© Instable / Lent

**Ã‰tat :** Connexion rÃ©seau instable ou lente.

**Comportement :**
- âœ… UI locale maintenue
- âš ï¸ DonnÃ©es potentiellement obsolÃ¨tes
- âš ï¸ Actions mises en attente
- âœ… Indicateur de confiance visible

**RÃ¨gles :**
- âž¡ï¸ DÃ©cisions jamais finales
- âž¡ï¸ TAMR informe l'humain
- âž¡ï¸ Actions marquÃ©es "non engagÃ©es"

#### ðŸ”´ Niveau C â€” Hors Ligne (DÃ©gradÃ© Maximal)

**Ã‰tat :** Pas de connexion rÃ©seau.

**Ce qui est autorisÃ© :**
- âœ… Navigation UI
- âœ… Consultation cache UX
- âœ… Actions prÃ©parÃ©es mais non envoyÃ©es
- âœ… Simulation locale (non dÃ©cisionnelle)

**Ce qui est interdit :**
- âŒ Validation finale
- âŒ ExÃ©cution
- âŒ Modification d'Ã©tat rÃ©el

**âž¡ï¸ Tout est marquÃ© "non engagÃ©"**

### 7.2 Transitions entre Niveaux

**A â†’ B :** DÃ©tection de latence Ã©levÃ©e ou instabilitÃ©  
**B â†’ A :** Retour Ã  la normale  
**B â†’ C :** Perte de connexion  
**C â†’ B :** Reconnexion instable  
**C â†’ A :** Reconnexion stable

**Gouvernance :**
- Caring Nanny dÃ©tecte l'Ã©tat rÃ©seau
- StrongFather dÃ©cide des restrictions
- BondingBrother adapte les rÃ©ponses
- TAMR informe l'utilisateur

---

## 8. Le Cache Mobile

### 8.1 Nature du Cache Mobile

**Important : le cache mobile**

Le cache mobile est :
- âŒ **Pas une persistance** : Ne remplace pas KindMother
- âŒ **Pas une vÃ©ritÃ©** : Ne remplace pas les cores
- âœ… **Un cache UX temporaire** : AmÃ©liore l'expÃ©rience utilisateur

### 8.2 CaractÃ©ristiques du Cache

**Le cache mobile est :**
- **Invalidable Ã  tout moment** : Le serveur peut invalider
- **SignÃ©** : VÃ©rification d'intÃ©gritÃ©
- **VersionnÃ©** : Gestion des versions
- **Jetable** : Peut Ãªtre supprimÃ© sans impact

### 8.3 RÃ¨gles d'Utilisation

**Le cache mobile peut contenir :**
- DonnÃ©es UI (affichage)
- PrÃ©fÃ©rences utilisateur (non critiques)
- Ã‰tat de navigation
- DonnÃ©es de formulaire (non validÃ©es)

**Le cache mobile ne peut JAMAIS contenir :**
- DonnÃ©es critiques
- DÃ©cisions validÃ©es
- Ã‰tat systÃ¨me
- ClÃ©s privÃ©es

---

## 9. WebApp = Client de Secours Universel

### 9.1 Positionnement

**WebApp = client de secours universel**

**CaractÃ©ristiques :**
- âœ… MÃªme protocole BondingBrother
- âœ… MÃªmes droits que mobile
- âš ï¸ Souvent moins performant
- âœ… Mais plus compatible

### 9.2 SÃ©curitÃ© WebApp

**RÃ¨gles absolues :**
- âŒ Jamais de clÃ© privÃ©e persistÃ©e
- âœ… Sessions courtes
- âœ… Permissions limitÃ©es
- âŒ Aucune capacitÃ© critique

**âž¡ï¸ WebApp = UI + contrÃ´le, rien de plus**

### 9.3 Comportement DÃ©gradÃ© WebApp

| Situation | Comportement |
|-----------|--------------|
| **Offline** | Quasi inutilisable |
| **RÃ©seau lent** | Lecture seule |
| **Session expirÃ©e** | Blocage doux |
| **DÃ©sync** | RafraÃ®chissement forcÃ© |

**ðŸ‘‰ Contrairement Ã  l'app native, le Web n'est pas autonome.**

### 9.4 Comparatif App Native vs WebApp

| Aspect | App native | WebApp |
|--------|------------|--------|
| **Performance** | â­â­â­â­ | â­â­ |
| **Mode hors ligne** | âœ… | âŒ |
| **SÃ©curitÃ©** | â­â­â­â­ | â­â­ |
| **UX** | â­â­â­â­ | â­â­ |
| **DÃ©pendance OS** | Oui | Non |
| **DÃ©pendance rÃ©seau** | Partielle | Forte |

### 9.5 RÃ¨gle StratÃ©gique Miyukini

**L'app native est le client principal.**  
**La WebApp est le filet de sÃ©curitÃ© universel.**

---

## 10. RÃ´le des Cores dans l'Architecture Mobile

### 10.1 BondingBrother

**RÃ´le :** Adaptation, session, QoS.

**ResponsabilitÃ©s :**
- Adaptation des requÃªtes mobiles
- Gestion des sessions
- Application de la QoS
- Gestion de la dÃ©gradation
- AgrÃ©gation et compression

### 10.2 StrongFather

**RÃ´le :** DÃ©cisions diffÃ©rÃ©es si instable.

**ResponsabilitÃ©s :**
- DÃ©cisions finales (jamais sur mobile)
- DÃ©cisions DIFFÃ‰RÃ‰E si rÃ©seau instable
- Ã‰valuation des intentions mobiles
- Application des politiques

### 10.3 Caring Nanny

**RÃ´le :** Ã‰tat global + confiance.

**ResponsabilitÃ©s :**
- Observation de l'Ã©tat rÃ©seau
- DÃ©tection des dÃ©gradations
- Calcul du niveau de confiance (T0-T4)
- Propagation de l'Ã©tat aux OpÃ©rateurs

### 10.4 TAMR

**RÃ´le :** Informe l'humain + autorise override.

**ResponsabilitÃ©s :**
- Information de l'utilisateur sur l'Ã©tat
- Autorisation d'override si nÃ©cessaire
- TraÃ§abilitÃ© des interventions
- Points d'intervention humaine

### 10.5 Border Guard

**RÃ´le :** ProtÃ¨ge contre injection mobile/web.

**ResponsabilitÃ©s :**
- Classification des sources (mobile, web)
- Niveaux de confiance des sources
- RÃ¨gles de franchissement
- Protection contre injection

### 10.6 Master Butler

**RÃ´le :** CapacitÃ©s exposÃ©es.

**ResponsabilitÃ©s :**
- Registre des capacitÃ©s disponibles
- Permissions pour mobile/web
- Contrats d'API
- Limitations par plateforme

---

## 11. Flux Typiques Mobile

### 11.1 Flux Action Utilisateur â†’ DÃ©cision

```
Mobile (Action utilisateur)
    â†“
BondingBrother (AgrÃ©gation, adaptation)
    â†“
StrongFather (DÃ©cision)
    â†“
KindMother (ExÃ©cution si acceptÃ©e)
    â†“
BondingBrother (RÃ©ponse adaptÃ©e)
    â†“
Mobile (Affichage rÃ©sultat)
```

### 11.2 Flux Mode DÃ©gradÃ©

```
Mobile (DÃ©tection rÃ©seau instable)
    â†“
Caring Nanny (Observation Ã©tat)
    â†“
StrongFather (DÃ©cision DIFFÃ‰RÃ‰E)
    â†“
TAMR (Information utilisateur)
    â†“
Mobile (Affichage indicateur, actions en attente)
```

### 11.3 Flux Hors Ligne

```
Mobile (Pas de connexion)
    â†“
Cache UX local (Navigation, prÃ©paration)
    â†“
Actions marquÃ©es "non engagÃ©es"
    â†“
Reconnexion
    â†“
BondingBrother (Envoi actions en attente)
    â†“
StrongFather (DÃ©cision)
    â†“
Mobile (RÃ©sultat)
```

---

## 12. SÃ©curitÃ© Mobile/WebApp

### 12.1 Principes de SÃ©curitÃ©

**Isolation stricte :**
- Aucun core exposÃ© directement
- Toute interaction via BondingBrother
- Sessions authentifiÃ©es
- Permissions limitÃ©es

**Protection contre injection :**
- Border Guard classifie les sources
- Validation stricte des requÃªtes
- Signatures cryptographiques
- VÃ©rification d'intÃ©gritÃ©

**Gestion des sessions :**
- Sessions courtes (WebApp)
- Tokens renouvelables
- RÃ©vocation possible
- TraÃ§abilitÃ© complÃ¨te

### 12.2 Protection des DonnÃ©es

**DonnÃ©es critiques :**
- Jamais stockÃ©es sur mobile
- Toujours cÃ´tÃ© serveur (KindMother)
- Chiffrement en transit
- VÃ©rification d'intÃ©gritÃ©

**Cache UX :**
- DonnÃ©es non critiques uniquement
- Invalidable Ã  tout moment
- SignÃ© et versionnÃ©
- Jetable sans impact

---

## 13. Conclusion

La stratÃ©gie mobile et WebApp de Miyukini garantit que :

- âœ… **Le mobile est optimisÃ©** : Passerelle intelligente, agrÃ©gation, compression
- âœ… **Le mode dÃ©gradÃ© fonctionne** : 3 niveaux, cache UX, actions non engagÃ©es
- âœ… **Les cores ne sont jamais exposÃ©s** : Isolation via BondingBrother
- âœ… **La WebApp est un filet de sÃ©curitÃ©** : CompatibilitÃ© universelle, sÃ©curitÃ© limitÃ©e
- âœ… **La cohÃ©rence est maintenue** : Tous les cores respectent leurs rÃ´les

**RÃ¨gle stratÃ©gique finale :**

**L'app native est le client principal.**  
**La WebApp est le filet de sÃ©curitÃ© universel.**  
**Les cores sont toujours cÃ´tÃ© serveur.**

---

**Date de crÃ©ation :** 2026-01-26  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence stratÃ©gique

**Documentation associÃ©e :**
- [BondingBrother - Documentation Fondatrice](..//cores//BondingBrother//foundation//BondingBrother%20-%20Documentation%20Fondatrice.md) : Gateway intelligent mobile
- [StrongFather - Documentation Fondatrice](..//cores//StrongFather//foundation//StrongFather%20-%20Documentation%20Fondatrice.md) : DÃ©cisions diffÃ©rÃ©es
- [Caring Nanny - Documentation Fondatrice](..//cores//CaringNanny//foundation//Caring%20Nanny%20-%20Documentation%20Fondatrice.md) : Ã‰tat rÃ©seau et dÃ©gradation
- [TAMR - Documentation Fondatrice](..//cores//TAMR//foundation//TAMR%20-%20Documentation%20Fondatrice.md) : Information utilisateur
- [Border Guard - Documentation Fondatrice](..//cores//BorderGuard//foundation//Border%20Guard%20-%20Documentation%20Fondatrice.md) : Protection injection mobile/web
- [Miyukini Conceptual References - Integrity & Degradation System](..//_index.md) : Niveaux de confiance (T0-T4)
- [Miyukini Conceptual References - External Signal & Trust Reinforcement Contract](..//_index.md) : Gestion rÃ©seau instable
- [Miyukini Conceptual References - Security Protocols](..//_index.md) : Protocoles sÃ©curitÃ© temps rÃ©el (RT-SEC) et asynchrone (AS-SEC)
- [Miyukini Conceptual References - Security Performance Impact](..//_index.md) : Impact performance mobile/offline
- [Miyukini Conceptual References - Security Levels](..//_index.md) : Niveaux de sÃ©curitÃ© selon plateforme (mobile â‰¥ 2, WebApp max 2)


