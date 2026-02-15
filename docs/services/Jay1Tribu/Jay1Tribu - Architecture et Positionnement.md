# Jay1Tribu — Architecture et Positionnement

## Contexte

**Jay1Tribu** est un **Service Inter-COG (Type 3)** qui fournit la messagerie pair-à-pair (salons, tribus, amis, messages, fichiers, images) dans l'écosystème Miyukini COG. Ce document décrit son positionnement dans la Pyramide Miyukini, ses relations avec les Cores, le MWS et les autres Services, ainsi que les flux d'exécution.

## Portée / Scope

- **Applicable à :** Architecture, positionnement pyramidal, dépendances MWS et Cores, flux de gouvernance.
- **Audience :** Architectes, développeurs, équipes sécurité.
- **Statut :** Document normatif — référence architecturale du Service Jay1Tribu.

---

## 1. Positionnement dans la Pyramide Miyukini

| Strate | Élément | Rôle vis-à-vis de Jay1Tribu |
|--------|---------|-----------------------------|
| **7** | Opérateurs Jay1Tribu | Exécutent la messagerie, la gestion des tribus, des salons et des amis pour le compte de l'utilisateur. |
| **6** | Outils & Kits d'Outils | Chiffrement, transfert de fichiers, horodatage, persistance locale (via KindMother). |
| **5** | BondingBrother | Médiation entre les Opérateurs Jay1Tribu et les Cores. Traduit les intentions (envoi, création tribu, invitation) vers les autorités. |
| **4** | Cores | Gouvernent : StrongFather (autorisation), KindMother (persistance), Master Butler (permissions), WorrySentinel (sécurité), Border Guard (frontières Inter-COG). |
| **3** | Invariants & Contrats | Pas d'archives centrales, transit crypté, hébergement utilisateur, persistance via KindMother. |
| **K** | Kernel | Substrat technique : scheduling, I/O, identifiants. |

### 1.1 Flux d'exécution (envoi de message)

```
Utilisateur envoie un message
  → Opérateur Messagerie Jay1Tribu
    → BondingBrother
      → StrongFather : "Cet envoi est-il autorisé ?"
      → Border Guard : "Le destinataire est-il dans la frontière de confiance ?"
      → Master Butler : "L'utilisateur a-t-il la permission d'envoyer dans ce salon ?"
    → Chiffrement (Outil / WorrySentinel)
    → MWS : transport vers le(s) COG(s) destinataire(s)
  → COG destinataire : réception, déchiffrement, WriteIntent KindMother → archivage local
```

### 1.2 Flux d'exécution (création de tribu)

```
Utilisateur crée une tribu
  → Opérateur Tribu Jay1Tribu
    → BondingBrother
      → StrongFather : "La création de tribu est-elle autorisée ?"
      → Master Butler : "L'utilisateur peut-il être Chef de tribu ?"
    → KindMother : WriteIntent (création tribu, rôle Chef)
  → Tribu créée ; créateur = Chef de tribu
```

---

## 2. Type de Service et espaces

| Attribut | Valeur |
|----------|--------|
| **Type** | Service Inter-COG (Type 3) |
| **Espace Central** | Miyukini Central — gestion tribus, salons, amis, paramètres, UI |
| **Espace Inter-COG** | Protocoles d'échange (messages, fichiers, images) sur le MWS |
| **Surface externe** | Aucune surface web publique ; communication COG–COG uniquement via MWS |

**Règle :** Jay1Tribu déclare son type (Type 3) et prévoit les deux espaces (Central + Inter-COG).

---

## 3. Dépendance au MWS

Jay1Tribu **s'appuie sur le MWS** pour :

| Capacité MWS | Usage par Jay1Tribu |
|--------------|---------------------|
| **Présence** | Affichage en ligne / hors ligne des amis et des membres de tribu. |
| **Découverte** | Découverte des pairs (COGs) pour initier ou rejoindre des salons / tribus. |
| **Transport** | Acheminement des messages, métadonnées, fichiers et images entre COGs. |

Le MWS **ne stocke pas** le contenu des messages ; il assure le routage. Le cryptage est de la responsabilité de Jay1Tribu (ou des Outils qu'il utilise), en conformité avec WorrySentinel et Border Guard.

---

## 4. Opérateurs Jay1Tribu (vision)

Les Opérateurs sont les entités fonctionnelles gouvernées qui exécutent le service. Vision conceptuelle :

| Opérateur | Responsabilité |
|-----------|----------------|
| **Jay1TribuMessenger** | Envoi et réception de messages, fichiers, images ; gestion des salons (création, adhésion). |
| **Jay1TribuTribes** | Création et gestion des tribus, rôles, invitations, synchronisation à la reconnexion. |
| **Jay1TribuFriends** | Liste d'amis, présence (lecture MWS), initiation de discussions directes. |

Les noms et le découpage définitif seront fixés en phase d'implémentation. **Règle :** tout Opérateur passe par BondingBrother et les Cores pour toute décision et toute persistance.

---

## 5. Relations avec les autres Services

| Service | Relation |
|---------|----------|
| **Miyukini Central** | Central affiche Jay1Tribu dans la liste des services ; ouverture de l'interface ; pas de stockage des messages par Central. |
| **MiyukiniWatch** | Consomme des métadonnées de présence et d'interaction (ex. conversation ouverte/fermée, ami contacté) ; ne lit jamais le contenu des messages. |
| **Miou** | Peut proposer des bulles liées aux amis (ex. « Un ami est en ligne ») ; données fournies par Jay1Tribu (liste amis, présence) ou MiyukiniWatch (agrégats). Résolution des pseudos par Jay1Tribu, pas par MiyukiniWatch. |

---

## 6. Cores et gouvernance

| Core | Rôle vis-à-vis de Jay1Tribu |
|------|-----------------------------|
| **StrongFather** | Autorise ou refuse les actions (création tribu, envoi, invitation, attribution de rôles). Émet les Mandats de Permission pour les échanges Inter-COG. |
| **KindMother** | Autorité de persistance locale. Valide, refuse ou reporte les WriteIntent (archives, fichiers, liste d'amis, paramètres tribu). |
| **Master Butler** | Registre des capacités et permissions (créer salon, inviter, être Chef de tribu, envoyer des fichiers, etc.). |
| **WorrySentinel** | Niveaux de sécurité des contenus, règles de rétention, politique de chiffrement (transit et au repos). |
| **Border Guard** | Frontières Inter-COG : qui peut communiquer avec qui, règles de confiance entre COGs. |
| **Caring Nanny** | Observation de l'état du système ; peut restreindre les échanges en cas d'environnement dégradé. |
| **Ever Buddy** | Évolution du Service (versions, compatibilité, dépréciation). |
| **TAMR** | Points d'intervention humaine (modération, litiges, révocation d'accès). |

---

## 7. Résumé

Jay1Tribu est un **Service Inter-COG (Type 3)** dont les Opérateurs exécutent la messagerie, les tribus et les amis, sous gouvernance des Cores et avec transport par le MWS. Aucune archive centralisée ; transit crypté ; persistance locale exclusive via KindMother.

---

## 8. Références

| Document | Rôle |
|----------|------|
| [Jay1Tribu - Document Conceptuel](./Jay1Tribu%20-%20Document%20Conceptuel.md) | Concepts (tribus, salons, amis, rôles), contraintes. |
| [Jay1Tribu - Document Fondateur](./Jay1Tribu%20-%20Document%20Fondateur.md) | Vision et principes. |
| [MWS - Document Fondateur](../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) | Présence, découverte, transport. |

---

**Document** : Jay1Tribu — Architecture et Positionnement  
**Version** : 1.0  
**Date** : 2026-02-15  
**Statut** : Document normatif
