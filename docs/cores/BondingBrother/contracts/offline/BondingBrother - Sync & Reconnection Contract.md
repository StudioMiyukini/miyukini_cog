# BondingBrother - Sync & Reconnection Contract

## 1. Contexte

Ce document dÃ©finit le contrat de synchronisation et de reconnexion de Bonding Brother. Il spÃ©cifie comment Bonding Brother gÃ¨re la synchronisation des intentions journalisÃ©es aprÃ¨s une pÃ©riode de dÃ©connexion, comment il dÃ©tecte et gÃ¨re les reconnexions, et comment il garantit l'intÃ©gritÃ© et l'ordre des intentions lors de la synchronisation.

Ce document complÃ¨te la Section 8 de la [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) sur le rapport au temps et Ã  l'offline, et s'appuie sur les concepts d'offline et de journalisation pour dÃ©finir le processus de synchronisation.

Ce contrat implÃ©mente **LOI-2** (isolement comme Ã©tat normal) et **LOI-3** (Ã©tat local souverain) dÃ©finies dans les [Lois d'Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md). La synchronisation prÃ©serve la souverainetÃ© de l'Ã©tat local et ne remet jamais en question les dÃ©cisions prises en isolation.

## 2. PortÃ©e / Scope

Ce document couvre :
- La dÃ©tection de reconnexion
- Le processus de synchronisation des intentions buffÃ©es
- La prÃ©servation de l'ordre des intentions
- La gestion des conflits et des duplications
- La transmission des rÃ©sultats diffÃ©rÃ©s
- Les stratÃ©gies de retry en cas d'Ã©chec
- La gestion des timeouts lors de la synchronisation

Ce document **ne couvre pas** :
- Le mode offline lui-mÃªme (voir [Offline & Deferred Authority Contract](./BondingBrother%20-%20Offline%20&%20Deferred%20Authority%20Contract.md))
- La journalisation (voir [Journaling Contract](./BondingBrother%20-%20Journaling%20Contract.md))
- Les dÃ©tails d'implÃ©mentation du buffer offline
- Les protocoles rÃ©seau de reconnexion

---

## 3. Principe fondamental

**La synchronisation garantit que toutes les intentions exprimÃ©es en mode offline sont transmises aux autoritÃ©s dans l'ordre chronologique, sans perte ni duplication, et que leurs rÃ©sultats sont transmis aux produits.**

La synchronisation est transparente pour les produits : ils continuent d'exprimer leurs intentions normalement, sans savoir si Bonding Brother est en ligne ou hors ligne.

---

## 4. DÃ©tection de reconnexion

### 4.1 DÃ©finition

Une **reconnexion** est la restauration de la connectivitÃ© entre Bonding Brother et une autoritÃ© (Kind Mother ou Strong Father) aprÃ¨s une pÃ©riode de dÃ©connexion.

### 4.2 MÃ©canismes de dÃ©tection

**RÃ¨gle DET-01 : DÃ©tection active**

Bonding Brother dÃ©tecte la reconnexion en testant pÃ©riodiquement la connectivitÃ© vers les autoritÃ©s :
- Test de connectivitÃ© vers Kind Mother
- Test de connectivitÃ© vers Strong Father
- FrÃ©quence configurable (par dÃ©faut : toutes les 30 secondes)

**RÃ¨gle DET-02 : DÃ©tection passive**

Bonding Brother dÃ©tecte Ã©galement la reconnexion lors d'une tentative de transmission :
- Si une transmission rÃ©ussit aprÃ¨s un Ã©chec, la reconnexion est dÃ©tectÃ©e
- La dÃ©tection passive est immÃ©diate

**RÃ¨gle DET-03 : DÃ©tection par autoritÃ©**

Bonding Brother dÃ©tecte la reconnexion sÃ©parÃ©ment pour chaque autoritÃ© :
- Reconnexion Ã  Kind Mother â‰  Reconnexion Ã  Strong Father
- La synchronisation est dÃ©clenchÃ©e uniquement pour l'autoritÃ© reconnectÃ©e

### 4.3 Ã‰tats de connexion

| Ã‰tat | DÃ©finition | Action |
|------|-----------|--------|
| **CONNECTÃ‰** | ConnectivitÃ© vÃ©rifiÃ©e rÃ©cemment | Transmission immÃ©diate |
| **DÃ‰CONNECTÃ‰** | DerniÃ¨re tentative Ã©chouÃ©e | Buffer offline activÃ© |
| **EN_RECONNEXION** | DÃ©tection de reconnexion en cours | Synchronisation dÃ©clenchÃ©e |
| **INSTABLE** | Connexions intermittentes | Mode dÃ©gradÃ© avec retry |

---

## 5. Processus de synchronisation

### 5.1 DÃ©clenchement

**RÃ¨gle SYNC-01 : DÃ©clenchement automatique**

La synchronisation est dÃ©clenchÃ©e automatiquement dÃ¨s qu'une reconnexion est dÃ©tectÃ©e pour une autoritÃ© donnÃ©e.

**RÃ¨gle SYNC-02 : DÃ©clenchement manuel**

La synchronisation peut Ã©galement Ãªtre dÃ©clenchÃ©e manuellement (API d'administration) pour forcer une synchronisation.

**RÃ¨gle SYNC-03 : Une autoritÃ© Ã  la fois**

La synchronisation est effectuÃ©e sÃ©parÃ©ment pour chaque autoritÃ© :
- Synchronisation Kind Mother (intentions ciblant KM)
- Synchronisation Strong Father (intentions ciblant SF)

### 5.2 SÃ©quence de synchronisation

```
DÃ©tection de reconnexion
    â”‚
    â–¼
RÃ©cupÃ©ration des intentions buffÃ©es (ordre chronologique)
    â”‚
    â–¼
Pour chaque intention (sÃ©quentiel) :
    â”‚
    â”œâ”€ Transmission Ã  l'autoritÃ©
    â”œâ”€ RÃ©ception de la rÃ©ponse
    â”œâ”€ Journalisation de la rÃ©ponse
    â””â”€ Transmission du rÃ©sultat au produit
    â”‚
    â–¼
Nettoyage du buffer (intentions synchronisÃ©es)
    â”‚
    â–¼
Notification de fin de synchronisation
```

### 5.3 PrÃ©servation de l'ordre

**RÃ¨gle ORDRE-01 : Ordre chronologique strict**

Les intentions sont synchronisÃ©es dans l'ordre chronologique strict (FIFO) :
- Ordre basÃ© sur le timestamp de crÃ©ation de l'intention
- Aucun rÃ©ordonnancement n'est autorisÃ©

**RÃ¨gle ORDRE-02 : Traitement sÃ©quentiel**

Les intentions sont transmises sÃ©quentiellement (une Ã  la fois) pour prÃ©server l'ordre :
- Pas de transmission parallÃ¨le pour une mÃªme autoritÃ©
- La transmission suivante commence aprÃ¨s rÃ©ception de la rÃ©ponse

**RÃ¨gle ORDRE-03 : Blocage en cas d'Ã©chec**

Si une intention Ã©choue lors de la synchronisation, les intentions suivantes sont bloquÃ©es jusqu'Ã  rÃ©solution :
- Retry de l'intention en Ã©chec
- Ou passage en erreur dÃ©finitive
- Puis continuation avec les intentions suivantes

**Note sur l'autonomie :** Cette rÃ¨gle prÃ©serve l'ordre tout en respectant **LOI-3** : l'Ã©tat local reste souverain mÃªme si la synchronisation Ã©choue partiellement. Les intentions en Ã©chec ne remettent pas en question la validitÃ© des intentions suivantes.

### 5.4 Gestion des duplications

**RÃ¨gle DUP-01 : DÃ©tection de duplication**

Bonding Brother dÃ©tecte les duplications potentielles :
- VÃ©rification de l'ID d'intention avant transmission
- Comparaison avec les intentions dÃ©jÃ  synchronisÃ©es

**RÃ¨gle DUP-02 : PrÃ©vention de duplication**

Les intentions dÃ©jÃ  synchronisÃ©es ne sont pas retransmises :
- Marquage des intentions synchronisÃ©es
- Exclusion du buffer de synchronisation

**RÃ¨gle DUP-03 : Gestion des ID dupliquÃ©s**

Si une intention avec un ID dÃ©jÃ  utilisÃ© est dÃ©tectÃ©e :
- L'intention est rejetÃ©e avec code d'erreur `SYNC-001 : ID dupliquÃ©`
- Notification au produit
- Journalisation de l'erreur

---

## 6. Transmission des rÃ©sultats diffÃ©rÃ©s

### 6.1 RÃ©sultats diffÃ©rÃ©s

Un **rÃ©sultat diffÃ©rÃ©** est la rÃ©ponse d'une autoritÃ© Ã  une intention qui a Ã©tÃ© exprimÃ©e en mode offline et qui est maintenant Ã©valuÃ©e aprÃ¨s la reconnexion.

### 6.2 Transmission aux produits

**RÃ¨gle RES-01 : Transmission immÃ©diate**

Les rÃ©sultats diffÃ©rÃ©s sont transmis aux produits immÃ©diatement aprÃ¨s rÃ©ception de la rÃ©ponse de l'autoritÃ© :
- Pas d'attente de fin de synchronisation complÃ¨te
- Transmission au fur et Ã  mesure

**RÃ¨gle RES-02 : Format identique**

Les rÃ©sultats diffÃ©rÃ©s ont le mÃªme format que les rÃ©sultats en ligne :
- Aucune diffÃ©rence de structure
- Marqueur optionnel indiquant que c'est un rÃ©sultat diffÃ©rÃ©

**RÃ¨gle RES-03 : Produit indisponible**

Si le produit n'est pas disponible pour recevoir le rÃ©sultat :
- Le rÃ©sultat est journalisÃ©
- Retry de transmission selon stratÃ©gie configurable
- Notification d'Ã©chec si retry Ã©choue

### 6.3 Format de rÃ©sultat diffÃ©rÃ©

```typescript
interface RÃ©sultatDiffÃ©rÃ© {
    rÃ©sultat_id: RÃ©sultatId;
    intention_id: IntentionId;
    statut: "ACCEPTÃ‰" | "REFUSÃ‰" | "ERREUR";
    
    // RÃ©sultat normal
    rÃ©sultat?: RÃ©sultatNormal;
    
    // MÃ©tadonnÃ©es de diffÃ©rÃ©
    diffÃ©rÃ©: {
        crÃ©Ã©_le: Timestamp;           // Moment de crÃ©ation de l'intention
        synchronisÃ©_le: Timestamp;     // Moment de synchronisation
        dÃ©lai: DurÃ©e;                   // DÃ©lai entre crÃ©ation et synchronisation
    };
    
    timestamp: Timestamp;
}
```

---

## 7. StratÃ©gies de retry

### 7.1 Retry lors de la synchronisation

**RÃ¨gle RETRY-01 : Retry automatique**

En cas d'Ã©chec de transmission lors de la synchronisation, Bonding Brother retente automatiquement :
- Nombre maximum de tentatives configurable (par dÃ©faut : 3)
- Backoff exponentiel entre tentatives

**RÃ¨gle RETRY-02 : Types d'erreurs retentables**

Seules les erreurs transitoires sont retentÃ©es :
- Erreurs de transmission rÃ©seau
- Timeouts temporaires
- Erreurs d'autoritÃ© temporaires (indisponibilitÃ©)

**RÃ¨gle RETRY-03 : Erreurs non retentables**

Les erreurs dÃ©finitives ne sont pas retentÃ©es :
- Erreurs de validation (intention invalide)
- Refus explicite de l'autoritÃ©
- Erreurs de format

### 7.2 Backoff exponentiel

**RÃ¨gle BACKOFF-01 : DÃ©lai initial**

Le dÃ©lai initial entre tentatives est configurable (par dÃ©faut : 1 seconde).

**RÃ¨gle BACKOFF-02 : Multiplicateur**

Le dÃ©lai est multipliÃ© par un facteur Ã  chaque tentative (par dÃ©faut : 2x).

**RÃ¨gle BACKOFF-03 : DÃ©lai maximum**

Le dÃ©lai maximum est limitÃ© (par dÃ©faut : 60 secondes).

**Exemple :**
- Tentative 1 : ImmÃ©diate
- Tentative 2 : AprÃ¨s 1 seconde
- Tentative 3 : AprÃ¨s 2 secondes
- Tentative 4 : AprÃ¨s 4 secondes
- Tentative 5 : AprÃ¨s 8 secondes (max 60s)

### 7.3 Abandon aprÃ¨s Ã©chec

**RÃ¨gle ABANDON-01 : Nombre maximum de tentatives**

AprÃ¨s le nombre maximum de tentatives, l'intention est abandonnÃ©e :
- Passage en Ã©tat `ABANDONNÃ‰E`
- Notification au produit
- Journalisation de l'Ã©chec

**RÃ¨gle ABANDON-02 : Notification au produit**

Le produit est notifiÃ© de l'abandon avec :
- Code d'erreur `SYNC-002 : Synchronisation Ã©chouÃ©e`
- Nombre de tentatives effectuÃ©es
- DerniÃ¨re erreur rencontrÃ©e

---

## 8. Gestion des timeouts

### 8.1 Timeout de synchronisation

**RÃ¨gle TIMEOUT-01 : Timeout par intention**

Chaque intention a un timeout individuel lors de la synchronisation :
- Timeout configurable (par dÃ©faut : 30 secondes)
- Timeout global de synchronisation (par dÃ©faut : 1 heure)

**RÃ¨gle TIMEOUT-02 : Gestion du timeout**

Si une intention dÃ©passe son timeout :
- Retry selon stratÃ©gie de retry
- Si tous les retry Ã©chouent, passage en Ã©tat `TIMEOUT`

**RÃ¨gle TIMEOUT-03 : Timeout global**

Si la synchronisation complÃ¨te dÃ©passe le timeout global :
- Les intentions non synchronisÃ©es restent dans le buffer
- Nouvelle tentative de synchronisation dÃ©clenchÃ©e
- Notification d'incomplÃ©tude

---

## 9. Conflits et rÃ©solution

### 9.1 Conflits de synchronisation

Un **conflit de synchronisation** survient quand :
- Une intention exprimÃ©e en offline entre en conflit avec l'Ã©tat actuel de l'autoritÃ©
- L'autoritÃ© rejette l'intention pour cause de conflit
- Plusieurs intentions modifient la mÃªme ressource

### 9.2 RÃ©solution des conflits

**RÃ¨gle CONFLIT-01 : DÃ©lÃ©gation Ã  l'autoritÃ©**

La rÃ©solution des conflits est dÃ©lÃ©guÃ©e Ã  l'autoritÃ© :
- Bonding Brother transmet l'intention
- L'autoritÃ© dÃ©cide de la rÃ©solution
- Bonding Brother transmet la dÃ©cision au produit

**RÃ¨gle CONFLIT-02 : Pas de rÃ©solution locale**

Bonding Brother ne rÃ©sout jamais les conflits localement :
- Pas de logique de rÃ©solution dans BB
- Pas de modification de l'intention
- Pas de retry avec modification

**RÃ¨gle CONFLIT-03 : Notification au produit**

En cas de conflit dÃ©tectÃ© par l'autoritÃ© :
- Le produit est notifiÃ© avec code d'erreur `SYNC-003 : Conflit dÃ©tectÃ©`
- Le produit peut soumettre une nouvelle intention corrigÃ©e

---

## 10. MÃ©triques et observabilitÃ©

### 10.1 MÃ©triques de synchronisation

Bonding Brother expose les mÃ©triques suivantes :
- Nombre d'intentions en attente de synchronisation
- Nombre d'intentions synchronisÃ©es avec succÃ¨s
- Nombre d'intentions en Ã©chec de synchronisation
- DÃ©lai moyen de synchronisation
- DurÃ©e totale de synchronisation

### 10.2 Ã‰vÃ©nements de synchronisation

Les Ã©vÃ©nements suivants sont journalisÃ©s :
- `SYNC_STARTED` : DÃ©but de synchronisation
- `SYNC_INTENTION_SENT` : Intention transmise
- `SYNC_INTENTION_SUCCESS` : Intention synchronisÃ©e avec succÃ¨s
- `SYNC_INTENTION_FAILED` : Ã‰chec de synchronisation d'une intention
- `SYNC_COMPLETED` : Fin de synchronisation
- `SYNC_PARTIAL` : Synchronisation partielle (certaines intentions en Ã©chec)

---

## 11. Exemples

### 11.1 Synchronisation rÃ©ussie

**ScÃ©nario :** 5 intentions buffÃ©es, reconnexion dÃ©tectÃ©e, toutes synchronisÃ©es avec succÃ¨s.

**Ã‰vÃ©nements :**
1. Reconnexion dÃ©tectÃ©e Ã  Kind Mother
2. 5 intentions rÃ©cupÃ©rÃ©es (ordre chronologique)
3. Intention 1 transmise â†’ RÃ©ponse reÃ§ue â†’ RÃ©sultat envoyÃ© au produit
4. Intention 2 transmise â†’ RÃ©ponse reÃ§ue â†’ RÃ©sultat envoyÃ© au produit
5. ... (rÃ©pÃ©tÃ© pour les 5 intentions)
6. Synchronisation complÃ©tÃ©e

### 11.2 Synchronisation avec Ã©chec partiel

**ScÃ©nario :** 5 intentions buffÃ©es, 3 rÃ©ussies, 2 en Ã©chec aprÃ¨s retry.

**Ã‰vÃ©nements :**
1. Reconnexion dÃ©tectÃ©e
2. Intention 1 : SuccÃ¨s
3. Intention 2 : SuccÃ¨s
4. Intention 3 : Ã‰chec (retry 1) â†’ Ã‰chec (retry 2) â†’ Ã‰chec (retry 3) â†’ AbandonnÃ©e
5. Intention 4 : SuccÃ¨s
6. Intention 5 : Ã‰chec (retry 1) â†’ Ã‰chec (retry 2) â†’ Ã‰chec (retry 3) â†’ AbandonnÃ©e
7. Synchronisation partielle complÃ©tÃ©e (3/5)

---

## 12. Contraintes et limites

### 12.1 Taille du buffer

**RÃ¨gle LIM-01 : Taille maximale**

La taille maximale du buffer offline est configurable (par dÃ©faut : 10 000 intentions).

**RÃ¨gle LIM-02 : Buffer plein**

Si le buffer est plein :
- Les nouvelles intentions sont rejetÃ©es avec code `SYNC-004 : Buffer plein`
- Notification au produit
- Journalisation de l'erreur

### 12.2 DurÃ©e de rÃ©tention

**RÃ¨gle LIM-03 : Expiration**

Les intentions non synchronisÃ©es expirent aprÃ¨s une durÃ©e configurable (par dÃ©faut : 7 jours).

**RÃ¨gle LIM-04 : Nettoyage**

Les intentions expirÃ©es sont nettoyÃ©es automatiquement :
- Passage en Ã©tat `EXPIRÃ‰E`
- Notification au produit
- Suppression du buffer

---

## 13. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit le processus de synchronisation et de reconnexion que Bonding Brother doit respecter pour garantir l'intÃ©gritÃ© des intentions en mode offline.

Toute synchronisation doit suivre ce contrat. Toute dÃ©viation est considÃ©rÃ©e comme une violation.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :** 
- [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) v2.0 (Section 8)
- [Offline & Deferred Authority Contract](./BondingBrother%20-%20Offline%20&%20Deferred%20Authority%20Contract.md) v2.0
- [Journaling Contract](./BondingBrother%20-%20Journaling%20Contract.md) v2.0
- [Intent Model Contract](../intent/BondingBrother%20-%20Intent%20Model%20Contract.md) v2.0
- [Error & Rejection Model](../error/BondingBrother%20-%20Error%20&%20Rejection%20Model.md) v2.0

