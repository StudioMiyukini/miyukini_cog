# MWS — Architecture et subordination aux Cores

## Contexte

Ce document précise la **position architecturale** du Miyukini Webway System (MWS) par rapport à la **pyramide des strates** Miyukini et à la **strate Cores**. Le MWS est un **système complet** uniquement **subordonné aux Cores** ; il n'est pas une strate et il est consommé par toutes les strates.

**Référence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md).

## Portée / Scope

- Position du MWS par rapport aux strates 0–9.
- Subordination exclusive aux Cores : ce que cela implique en décision et en exécution.
- Cohérence avec les Lois d'Autonomie (LOI-2, LOI-6, LOI-7, LOI-8).
- Schéma d'architecture (orientation).

---

## 1. Le MWS n'est pas une strate

La pyramide Miyukini définit des **strates** (Kernel, Cores, Outils, Opérateurs, etc.). Chaque strate a un rang et un contenu précis.

| Constat | Description |
|---------|-------------|
| **MWS trop vaste** | Le MWS couvre présence, découverte, transport, conformité, catalogue, Lobbys, permis de circulation, relay, trackers — soit des capacités qui touchent le Kernel (réseau), les Cores (gouvernance), les Outils (MiyuWebway*), les Opérateurs (annonces, Lobbys). |
| **Pas d'étiquette de strate** | On ne dit pas « le MWS est la strate X » ; on dit « le MWS est un **système complet** dont la racine documentaire est `docs/miyukini-webway-system` et qui est **uniquement subordonné aux Cores** ». |
| **Transversal** | Le MWS est **consommé** par plusieurs strates ; il ne *s'assoit* pas sur une seule. |

---

## 2. Subordination exclusive aux Cores

**Subordination aux Cores** signifie :

| Aspect | Rôle des Cores | Rôle du MWS |
|--------|----------------|-------------|
| **Politique** | Les Cores (WorrySentinel, Border Guard, StrongFather, etc.) définissent ou valident les politiques de présence, conformité, attestation, sécurité réseau. | Le MWS met en œuvre les mécanismes (protocoles, relay, trackers, catalogue) conformément à ces politiques. |
| **Décision** | Les Cores décident qui est conforme, qui reçoit un Permis de circulation (via Origin/relays, accord relay), qui est en quarantaine ou blacklisté. | Origin et les relays **exécutent** les vérifications et délivrent les Permis de circulation selon les critères hérités des Cores. |
| **Gouvernance locale** | Chaque COG est gouverné par **ses** Cores. Le MWS ne remplace pas cette gouvernance. | Le MWS fournit le maillage et les chemins ; il ne gouverne pas les accès métier (accord d'hôte = COG Hébergeur). |

**Aucune autre strate ne commande le MWS.** Les Outils (ex. MiyuWebwayTracker) sont des **consommateurs** du MWS et sont gouvernés par les Cores ; ils n'ordonnent pas au MWS en tant que système.

---

## 3. Schéma d'orientation

```
                    +------------------+
                    |     CORES        |  ← Gouvernance, politique, conformité
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
 Kernel    Outils (6)   Opérateurs   Services    BondingBrother
 (réseau)  MiyuWebway*  (annonces)   (Lobbys)   (Strate 5)
```

---

## 4. Lois d'Autonomie

| Loi | Relation avec le MWS |
|-----|----------------------|
| **LOI-2** | Le système accepte l'isolement ; un COG peut refuser ou ne pas utiliser le réseau. Le MWS est optionnel. |
| **LOI-6** | L'autonomie n'empêche pas la fédération ; le MWS est le support de la fédération (présence, découverte, Lobbys). |
| **LOI-7** | Les Cores sont immuables ; les politiques Webway sont sous leur responsabilité. Le MWS n'évolue pas contre les Cores. |
| **LOI-8** | La migration est diplomatie entre environnements ; le MWS facilite la découverte et les chemins, pas la migration des données métier. |

---

## 5. Références croisées

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Consommation par les Strates](../strates/MWS%20-%20Consommation%20par%20les%20Strates.md)
- [Référence - Miyukini Webway Relay](../../reference/Miyukini%20Conceptual%20References%20-%20Miyukini%20Webway%20Relay.md) (architecture relay, Origin, trackers)

**Version :** 1.0
