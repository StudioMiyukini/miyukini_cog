# BondingBrother â€” Core Interaction Contract

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif

---

## 1. Contexte

Ce document dÃ©finit le **modÃ¨le d'interaction** entre Bonding Brother et les autres cores de l'Ã©cosystÃ¨me Miyukini. Il spÃ©cifie comment Bonding Brother communique avec chaque core, les protocoles utilisÃ©s, et les garanties contractuelles de ces interactions.

**DÃ©pendances :**
- [Documentation Fondatrice](../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md)
- [Architecture & Flows](./BondingBrother%20-%20Architecture%20&%20Flows.md)
- [Miyukini Conceptual References - Connexion Inter-COG](..//..//..//miyukini-webway-system//reference//_index.md)

## 2. PortÃ©e / Scope

Ce document couvre :
- Le modÃ¨le d'interaction avec chaque core
- Les protocoles de communication
- Les garanties contractuelles
- Les flux d'information entre cores
- Les rÃ¨gles de prioritÃ© et de routage

Ce document **ne couvre pas** :
- Les dÃ©tails d'implÃ©mentation des adaptateurs (voir Architecture & Flows)
- Les contrats d'intÃ©gration spÃ©cifiques (voir `contracts/integration/`)
- Les rÃ¨gles mÃ©tier des cores individuels

---

## 3. Principe fondamental

> **Bonding Brother est le seul point de passage autorisÃ© entre les produits et les cores.**

Aucun produit, aucun opÃ©rateur, aucune interface ne peut accÃ©der directement aux cores. Toute interaction passe obligatoirement par Bonding Brother, qui traduit, filtre, journalise et transmet.

---

## 4. Cartographie des interactions

### 4.1 Diagramme de relations

```mermaid
graph TB
    subgraph Strate7[Strate 7 - Operateurs]
        OP[Operateurs/Produits]
    end

    subgraph Strate5[Strate 5 - Liaison]
        BB[BondingBrother]
    end

    subgraph Strate4[Strate 4 - Cores]
        SF[StrongFather]
        KM[KindMother]
        CN[CaringNanny]
        MB[MasterButler]
        BG[BorderGuard]
        WS[WorrySentinel]
        EB[EverBuddy]
        TAMR[TAMR]
    end

    OP -->|intentions| BB
    BB -->|demandes decisions| SF
    BB -->|demandes donnees| KM
    BB -->|expose metriques| CN
    BB -->|decouvre capacites| MB
    BB -->|contexte frontieres| BG
    BB -->|signale securite| WS
    BB -->|versions compatibilite| EB
    BB -->|escalade humain| TAMR
    
    SF -->|decisions| BB
    KM -->|donnees| BB
    CN -->|etats| BB
    MB -->|capacites| BB
    BG -->|regles| BB
    WS -->|alertes| BB
    EB -->|migrations| BB
    TAMR -->|interventions| BB
    
    BB -->|resultats| OP
```

### 4.2 Matrice des interactions

| Core | Direction | Type d'Ã©change | FrÃ©quence |
|------|-----------|----------------|-----------|
| **StrongFather** | BB â†’ SF | Demandes de dÃ©cision | Haute |
| **StrongFather** | SF â†’ BB | DÃ©cisions, mandats | Haute |
| **KindMother** | BB â†’ KM | Demandes donnÃ©es | Haute |
| **KindMother** | KM â†’ BB | DonnÃ©es, confirmations | Haute |
| **CaringNanny** | BB â†’ CN | MÃ©triques, Ã©tats | Moyenne |
| **CaringNanny** | CN â†’ BB | Alertes d'Ã©tat | Basse |
| **MasterButler** | BB â†’ MB | DÃ©couverte capacitÃ©s | Basse |
| **MasterButler** | MB â†’ BB | Catalogue capacitÃ©s | Basse |
| **BorderGuard** | BB â†’ BG | Contexte frontiÃ¨re | Moyenne |
| **BorderGuard** | BG â†’ BB | RÃ¨gles franchissement | Moyenne |
| **WorrySentinel** | BB â†’ WS | Signalements sÃ©curitÃ© | Basse |
| **WorrySentinel** | WS â†’ BB | Alertes, niveaux | Moyenne |
| **EverBuddy** | BB â†’ EB | VÃ©rification version | Basse |
| **EverBuddy** | EB â†’ BB | CompatibilitÃ©, migrations | Basse |
| **TAMR** | BB â†’ TAMR | Escalade humain | Rare |
| **TAMR** | TAMR â†’ BB | DÃ©cisions humaines | Rare |

---

## 5. Interaction avec StrongFather

### 5.1 RÃ´le de l'interaction

StrongFather est l'**autoritÃ© de dÃ©cision stratÃ©gique**. Bonding Brother lui transmet les demandes nÃ©cessitant une dÃ©cision d'autorisation, de validation, ou de politique.

### 5.2 Types de demandes

| Type | Description | Exemple |
|------|-------------|---------|
| **Demande d'autorisation** | VÃ©rifier si une action est permise | "L'opÃ©rateur X peut-il accÃ©der Ã  Y ?" |
| **Demande de mandat** | Obtenir un mandat de permission | "Mandat pour Ã©quipe d'opÃ©rateurs" |
| **Demande de validation** | Valider une intention complexe | "Cette intention respecte-t-elle les rÃ¨gles ?" |
| **Notification de contexte** | Informer d'un contexte de dÃ©cision | "Contexte de sÃ©curitÃ© pour dÃ©cision" |

### 5.3 Protocole

```
BB â†’ SF : IntentionDÃ©cision {
    intention_id: string,
    operateur_source: string,
    type_decision: enum,
    contexte: ContexteComplet,
    timestamp: LogicalClock
}

SF â†’ BB : DÃ©cisionRÃ©sultat {
    intention_id: string,
    decision: enum (AUTORISÃ‰ | REFUSÃ‰ | CONDITIONNEL),
    mandat?: Mandat,
    justification: string,
    validitÃ©: Duration
}
```

### 5.4 Garanties

| Garantie | Description |
|----------|-------------|
| **INTER-SF-1** | Toute dÃ©cision SF est transmise fidÃ¨lement sans modification |
| **INTER-SF-2** | Le contexte transmis est complet et non altÃ©rÃ© |
| **INTER-SF-3** | Les mandats sont stockÃ©s temporairement mais jamais interprÃ©tÃ©s |
| **INTER-SF-4** | Les refus sont journalisÃ©s avec justification |

---

## 6. Interaction avec KindMother

### 6.1 RÃ´le de l'interaction

KindMother est l'**autoritÃ© des donnÃ©es**. Bonding Brother lui transmet les demandes de lecture, d'Ã©criture, et de synchronisation de donnÃ©es.

### 6.2 Types de demandes

| Type | Description | Exemple |
|------|-------------|---------|
| **Lecture** | RÃ©cupÃ©rer des donnÃ©es | "Lire le contenu X" |
| **Ã‰criture** | Persister des donnÃ©es | "CrÃ©er le contenu Y" |
| **Synchronisation** | Synchroniser des Ã©tats | "Sync aprÃ¨s reconnexion" |
| **Validation** | VÃ©rifier cohÃ©rence donnÃ©es | "Ces donnÃ©es sont-elles valides ?" |

### 6.3 Protocole

```
BB â†’ KM : IntentionDonnÃ©es {
    intention_id: string,
    type_operation: enum (READ | WRITE | SYNC),
    cible: ResourceIdentifier,
    donnÃ©es?: Payload,
    contexte: ContexteComplet
}

KM â†’ BB : RÃ©sultatDonnÃ©es {
    intention_id: string,
    statut: enum (SUCCESS | FAILURE | PARTIAL),
    donnÃ©es?: Payload,
    erreur?: Erreur,
    metadata: MetadataRÃ©sultat
}
```

### 6.4 Garanties

| Garantie | Description |
|----------|-------------|
| **INTER-KM-1** | Les donnÃ©es retournÃ©es sont filtrÃ©es selon les permissions |
| **INTER-KM-2** | Les Ã©critures sont validÃ©es par SF avant transmission |
| **INTER-KM-3** | Les erreurs KM sont traduites en erreurs produit |
| **INTER-KM-4** | Les mÃ©tadonnÃ©es internes KM ne sont jamais exposÃ©es |

---

## 7. Interaction avec CaringNanny

### 7.1 RÃ´le de l'interaction

CaringNanny est l'**observateur d'Ã©tat**. Bonding Brother lui expose ses mÃ©triques et reÃ§oit les alertes d'Ã©tat systÃ¨me.

### 7.2 Types d'Ã©changes

| Type | Direction | Description |
|------|-----------|-------------|
| **MÃ©triques BB** | BB â†’ CN | Statistiques de fonctionnement |
| **Ã‰tat de santÃ©** | BB â†’ CN | SantÃ© des connexions aux autoritÃ©s |
| **Alertes systÃ¨me** | CN â†’ BB | Changements d'Ã©tat global |
| **DÃ©gradation** | CN â†’ BB | Notification de mode dÃ©gradÃ© |

### 7.3 Garanties

| Garantie | Description |
|----------|-------------|
| **INTER-CN-1** | Les mÃ©triques sont exposÃ©es en lecture seule |
| **INTER-CN-2** | Les alertes CN sont propagÃ©es aux produits concernÃ©s |
| **INTER-CN-3** | BB adapte son comportement selon l'Ã©tat CN |

---

## 8. Interaction avec BorderGuard

### 8.1 RÃ´le de l'interaction

BorderGuard **dÃ©finit les frontiÃ¨res et les rÃ¨gles de franchissement**. Bonding Brother l'interroge pour connaÃ®tre le contexte de frontiÃ¨re et les rÃ¨gles applicables.

### 8.2 Types d'Ã©changes

| Type | Direction | Description |
|------|-----------|-------------|
| **Contexte frontiÃ¨re** | BB â†’ BG | "Quelle est la frontiÃ¨re pour cette source ?" |
| **RÃ¨gles franchissement** | BG â†’ BB | RÃ¨gles Ã  appliquer pour le franchissement |
| **Classification confiance** | BG â†’ BB | Niveau de confiance de la source |

### 8.3 Garanties

| Garantie | Description |
|----------|-------------|
| **INTER-BG-1** | BB applique les rÃ¨gles BG sans les modifier |
| **INTER-BG-2** | BG ne filtre pas â€” il dÃ©finit les rÃ¨gles, BB les applique |
| **INTER-BG-3** | Le contexte de frontiÃ¨re accompagne toute demande aux autoritÃ©s |

---

## 9. Interaction avec WorrySentinel

### 9.1 RÃ´le de l'interaction

WorrySentinel **gouverne la sÃ©curitÃ©**. Bonding Brother lui signale les anomalies et reÃ§oit les alertes de sÃ©curitÃ©.

### 9.2 Types d'Ã©changes

| Type | Direction | Description |
|------|-----------|-------------|
| **Signalement** | BB â†’ WS | Anomalie dÃ©tectÃ©e (pattern suspect, etc.) |
| **Niveau sÃ©curitÃ©** | WS â†’ BB | Niveau de sÃ©curitÃ© actuel |
| **Alerte** | WS â†’ BB | Alerte de sÃ©curitÃ© active |
| **Mode dÃ©gradÃ©** | WS â†’ BB | Activation/dÃ©sactivation mode dÃ©gradÃ© |

### 9.3 Garanties

| Garantie | Description |
|----------|-------------|
| **INTER-WS-1** | BB signale tout pattern suspect Ã  WS |
| **INTER-WS-2** | BB adapte son filtrage selon le niveau WS |
| **INTER-WS-3** | Les alertes WS sont prioritaires |

---

## 10. Interaction avec MasterButler

### 10.1 RÃ´le de l'interaction

MasterButler est le **registre des capacitÃ©s**. Bonding Brother l'interroge pour dÃ©couvrir les capacitÃ©s disponibles.

### 10.2 Types d'Ã©changes

| Type | Direction | Description |
|------|-----------|-------------|
| **DÃ©couverte** | BB â†’ MB | "Quelles capacitÃ©s existent ?" |
| **Catalogue** | MB â†’ BB | Liste des capacitÃ©s et permissions |
| **RÃ©solution** | BB â†’ MB | "Cette capacitÃ© est-elle disponible ?" |

### 10.3 Garanties

| Garantie | Description |
|----------|-------------|
| **INTER-MB-1** | BB ne cache pas les capacitÃ©s MB |
| **INTER-MB-2** | La dÃ©couverte est toujours Ã  jour |

---

## 11. Interaction avec EverBuddy

### 11.1 RÃ´le de l'interaction

EverBuddy **gouverne le cycle de vie et l'Ã©volution**. Bonding Brother l'interroge pour vÃ©rifier la compatibilitÃ© des versions.

### 11.2 Types d'Ã©changes

| Type | Direction | Description |
|------|-----------|-------------|
| **VÃ©rification version** | BB â†’ EB | "Cette version est-elle compatible ?" |
| **Migration** | EB â†’ BB | Instructions de migration |
| **DÃ©prÃ©ciation** | EB â†’ BB | Notification de dÃ©prÃ©ciation |

### 11.3 Garanties

| Garantie | Description |
|----------|-------------|
| **INTER-EB-1** | BB respecte les rÃ¨gles de compatibilitÃ© EB |
| **INTER-EB-2** | Les migrations sont propagÃ©es aux produits |

---

## 12. Interaction avec TAMR

### 12.1 RÃ´le de l'interaction

TAMR **dÃ©finit les points d'intervention humaine**. Bonding Brother l'utilise pour escalader vers un humain quand nÃ©cessaire.

### 12.2 Types d'Ã©changes

| Type | Direction | Description |
|------|-----------|-------------|
| **Escalade** | BB â†’ TAMR | "Intervention humaine requise" |
| **DÃ©cision humaine** | TAMR â†’ BB | DÃ©cision aprÃ¨s intervention |
| **Timeout** | TAMR â†’ BB | Timeout d'intervention |

### 12.3 Garanties

| Garantie | Description |
|----------|-------------|
| **INTER-TAMR-1** | Les escalades sont traÃ§ables et justifiÃ©es |
| **INTER-TAMR-2** | Les dÃ©cisions humaines sont journalisÃ©es |
| **INTER-TAMR-3** | Un timeout dÃ©clenche un comportement par dÃ©faut |

---

## 13. RÃ¨gles de routage

### 13.1 Ordre de consultation

Pour une intention typique, l'ordre de consultation des cores est :

1. **BorderGuard** â€” Contexte de frontiÃ¨re
2. **WorrySentinel** â€” Niveau de sÃ©curitÃ©
3. **StrongFather** â€” DÃ©cision d'autorisation
4. **KindMother** â€” AccÃ¨s aux donnÃ©es
5. **CaringNanny** â€” Notification de l'Ã©tat

### 13.2 RÃ¨gles de prioritÃ©

| PrioritÃ© | Core | Raison |
|----------|------|--------|
| 1 | WorrySentinel | SÃ©curitÃ© prime sur tout |
| 2 | StrongFather | DÃ©cision avant action |
| 3 | BorderGuard | FrontiÃ¨res avant donnÃ©es |
| 4 | KindMother | DonnÃ©es aprÃ¨s autorisation |
| 5 | CaringNanny | Observation passive |
| 6 | MasterButler | DÃ©couverte Ã  la demande |
| 7 | EverBuddy | VÃ©rification rare |
| 8 | TAMR | Escalade exceptionnelle |

### 13.3 Court-circuits

| Condition | Court-circuit |
|-----------|---------------|
| WorrySentinel en alerte critique | Rejet immÃ©diat, pas de consultation SF/KM |
| BorderGuard refuse le franchissement | Rejet immÃ©diat, pas de consultation SF |
| StrongFather refuse | Rejet, pas de consultation KM |
| Mode offline | Buffer local, pas de consultation distante |

---

## 14. Gestion des erreurs inter-cores

### 14.1 Types d'erreurs

| Type | Description | Action |
|------|-------------|--------|
| **Timeout** | Core ne rÃ©pond pas | Retry puis dÃ©gradation |
| **Rejet** | Core refuse la demande | Propagation au produit |
| **IncohÃ©rence** | RÃ©ponse incohÃ©rente | Journalisation + escalade |
| **IndisponibilitÃ©** | Core indisponible | Mode offline ou dÃ©gradÃ© |

### 14.2 StratÃ©gie de retry

| Core | Retry max | DÃ©lai entre retry |
|------|-----------|-------------------|
| StrongFather | 3 | 100ms, 500ms, 2s |
| KindMother | 3 | 100ms, 500ms, 2s |
| Autres | 1 | 500ms |

---

## 15. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit les rÃ¨gles d'interaction entre Bonding Brother et les autres cores qui doivent Ãªtre respectÃ©es par toute implÃ©mentation.

---

## Navigation

- [Index BondingBrother](../_index.md)
- [Documentation Fondatrice](../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md)
- [Architecture & Flows](./BondingBrother%20-%20Architecture%20&%20Flows.md)
- [KindMother Integration Contract](../contracts/integration/BondingBrother%20-%20KindMother%20Integration%20Contract.md)
- [StrongFather Integration Contract](../contracts/integration/BondingBrother%20-%20StrongFather%20Integration%20Contract.md)

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :** Documentation Fondatrice v2.0, Architecture & Flows v2.0

