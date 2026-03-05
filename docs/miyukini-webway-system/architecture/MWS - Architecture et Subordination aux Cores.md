# MWS â€” Architecture et subordination aux Cores

## Contexte

Ce document prÃ©cise la **position architecturale** du Miyukini Webway System (MWS) par rapport Ã  la **pyramide des strates** Miyukini et Ã  la **strate Cores**. Le MWS est un **systÃ¨me complet** uniquement **subordonnÃ© aux Cores** ; il n'est pas une strate et il est consommÃ© par toutes les strates.

**RÃ©fÃ©rence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md).

## PortÃ©e / Scope

- Position du MWS par rapport aux strates 0â€“9.
- Subordination exclusive aux Cores : ce que cela implique en dÃ©cision et en exÃ©cution.
- CohÃ©rence avec les Lois d'Autonomie (LOI-2, LOI-6, LOI-7, LOI-8).
- SchÃ©ma d'architecture (orientation).

---

## 1. Le MWS n'est pas une strate

La pyramide Miyukini dÃ©finit des **strates** (Kernel, Cores, Outils, OpÃ©rateurs, etc.). Chaque strate a un rang et un contenu prÃ©cis.

| Constat | Description |
|---------|-------------|
| **MWS trop vaste** | Le MWS couvre prÃ©sence, dÃ©couverte, transport, conformitÃ©, catalogue, Lobbys, permis de circulation, relay, trackers â€” soit des capacitÃ©s qui touchent le Kernel (rÃ©seau), les Cores (gouvernance), les Outils (MiyuWebway*), les OpÃ©rateurs (annonces, Lobbys). |
| **Pas d'Ã©tiquette de strate** | On ne dit pas Â« le MWS est la strate X Â» ; on dit Â« le MWS est un **systÃ¨me complet** dont la racine documentaire est `docs/miyukini-webway-system` et qui est **uniquement subordonnÃ© aux Cores** Â». |
| **Transversal** | Le MWS est **consommÃ©** par plusieurs strates ; il ne *s'assoit* pas sur une seule. |

---

## 2. Subordination exclusive aux Cores

**Subordination aux Cores** signifie :

| Aspect | RÃ´le des Cores | RÃ´le du MWS |
|--------|----------------|-------------|
| **Politique** | Les Cores (WorrySentinel, Border Guard, StrongFather, etc.) dÃ©finissent ou valident les politiques de prÃ©sence, conformitÃ©, attestation, sÃ©curitÃ© rÃ©seau. | Le MWS met en Å“uvre les mÃ©canismes (protocoles, relay, trackers, catalogue) conformÃ©ment Ã  ces politiques. |
| **DÃ©cision** | Les Cores dÃ©cident qui est conforme, qui reÃ§oit un Permis de circulation (via Origin/relays, accord relay), qui est en quarantaine ou blacklistÃ©. | Origin et les relays **exÃ©cutent** les vÃ©rifications et dÃ©livrent les Permis de circulation selon les critÃ¨res hÃ©ritÃ©s des Cores. |
| **Gouvernance locale** | Chaque COG est gouvernÃ© par **ses** Cores. Le MWS ne remplace pas cette gouvernance. | Le MWS fournit le maillage et les chemins ; il ne gouverne pas les accÃ¨s mÃ©tier (accord d'hÃ´te = COG HÃ©bergeur). |

**Aucune autre strate ne commande le MWS.** Les Outils (ex. MiyuWebwayTracker) sont des **consommateurs** du MWS et sont gouvernÃ©s par les Cores ; ils n'ordonnent pas au MWS en tant que systÃ¨me.

---

## 3. SchÃ©ma d'orientation

```
                    +------------------+
                    |     CORES        |  â† Gouvernance, politique, conformitÃ©
                    | (strate 4)       |     Subordination unique du MWS
                    +--------+---------+
                             |
         +-------------------+-------------------+
         |                   |                   |
         v                   v                   v
  +-------------+   +----------------+   +------------------+
  |   Origin    |   |    Relays      |   |    Trackers      |
  | (relay+     |   | (duplication   |   | (catalogue,      |
  |  tracker)   |   |  d'Origin)     |   |  Lobbys, pools)   |
  +------+------+   +--------+--------+   +--------+---------+
         |                   |                   |
         +-------------------+-------------------+
                             |
                    Consommation par
                    toutes les strates
                             |
    +------------------------+------------------------+
    |            |            |            |           |
    v            v            v            v           v
 Kernel    Outils (6)   OpÃ©rateurs   Services    BondingBrother
 (rÃ©seau)  MiyuWebway*  (annonces)   (Lobbys)   (Strate 5)
```

---

## 4. Lois d'Autonomie

| Loi | Relation avec le MWS |
|-----|----------------------|
| **LOI-2** | Le systÃ¨me accepte l'isolement ; un COG peut refuser ou ne pas utiliser le rÃ©seau. Le MWS est optionnel. |
| **LOI-6** | L'autonomie n'empÃªche pas la fÃ©dÃ©ration ; le MWS est le support de la fÃ©dÃ©ration (prÃ©sence, dÃ©couverte, Lobbys). |
| **LOI-7** | Les Cores sont immuables ; les politiques Webway sont sous leur responsabilitÃ©. Le MWS n'Ã©volue pas contre les Cores. |
| **LOI-8** | La migration est diplomatie entre environnements ; le MWS facilite la dÃ©couverte et les chemins, pas la migration des donnÃ©es mÃ©tier. |

---

## 5. RÃ©fÃ©rences croisÃ©es

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Consommation par les Strates](../strates/MWS%20-%20Consommation%20par%20les%20Strates.md)
- [RÃ©fÃ©rence - Miyukini Webway Relay](..//reference//_index.md) (architecture relay, Origin, trackers)

**Version :** 1.0

