# Jay1Tribu — Contraintes et Invariants

## Contexte

Ce document rassemble l'ensemble des **contraintes non négociables** et des **invariants** qui régissent Jay1Tribu. Ces règles s'appliquent à toute conception, implémentation, évolution et maintenance du service. Aucune exception ne peut être accordée sans remettre en cause les fondements de Jay1Tribu.

## Portée / Scope

- **Applicable à :** Toutes les strates impliquées (Opérateurs, Outils, Cores), toute équipe travaillant sur Jay1Tribu.
- **Audience :** Architectes, développeurs, équipes sécurité, équipes produit, auditeurs.
- **Statut :** Document normatif — contraintes non négociables.

---

## 1. Contraintes fondamentales

### 1.1 Souveraineté des données et archives

| # | Contrainte | Description | Justification |
|---|------------|-------------|---------------|
| **C-1** | **Pas d'archives centrales de contenu** | Les archives des discussions ne sont maintenues que chez les participants (leurs COGs). Aucun serveur central Miyukini ne conserve le contenu des conversations. | Souveraineté utilisateur ; philosophie fondatrice du service. |
| **C-2** | **Transit crypté** | Tout message, fichier et image en transit entre COGs est crypté. | Confidentialité ; conformité WorrySentinel et Border Guard. |
| **C-3** | **Hébergement utilisateur** | Les discussions, fichiers et images restent hébergés chez les utilisateurs (leurs COGs). | Pas de conservation à l'insu des utilisateurs. |
| **C-4** | **Persistance locale via KindMother** | Toute écriture locale (archives, fichiers, liste d'amis, paramètres tribu) passe par KindMother (WriteIntent). Aucune persistance directe. | Gouvernance ; intégrité et cohérence. |

### 1.2 Type de Service et espaces

| # | Contrainte | Description |
|---|------------|-------------|
| **C-5** | **Service Inter-COG (Type 3)** | Jay1Tribu déclare son type (Type 3) et prévoit les espaces Central (Miyukini Central) et Inter-COG (protocoles d'échange sur le MWS). |

### 1.3 Tribus et reconnexion

| # | Contrainte | Description |
|---|------------|-------------|
| **C-6** | **Livraison différée conditionnée** | La livraison différée (tribu, à la reconnexion) est conditionnée par la reconnexion du destinataire et la disponibilité de l'émetteur ; paramétrage individuel possible (restriction de ce qui est synchronisé). |

### 1.4 Rôles et présence

| # | Contrainte | Description |
|---|------------|-------------|
| **C-7** | **Rôles gouvernés** | Les rôles au sein d'une tribu sont attribués par le Chef de tribu (ou délégation) et gouvernés par Master Butler / StrongFather. |
| **C-8** | **Liste d'amis et présence** | La liste d'amis et la présence s'appuient sur le MWS ; Jay1Tribu ne duplique pas la logique de présence. |

---

## 2. Invariants architecturaux

| # | Invariant | Description |
|---|-----------|-------------|
| **INV-01** | **Service Inter-COG (Type 3)** | Jay1Tribu est et restera un Service Inter-COG. Espace Central + Protocoles Inter-COG. Pas de surface web publique de type « portail » pour la messagerie. |
| **INV-02** | **Pas de stockage MWS du contenu** | Le MWS (Relay, Tracker, Origin) ne stocke pas le contenu des messages, fichiers ou images ; routage uniquement. |
| **INV-03** | **Gouvernance par les Cores** | Toute action (envoi, création tribu, invitation, attribution de rôles) est soumise aux Cores via BondingBrother. Les Opérateurs n'agissent jamais en autonomie décisionnelle. |
| **INV-04** | **Présence déléguée au MWS** | La présence (en ligne / hors ligne) est une capacité du MWS ; Jay1Tribu la consomme, ne la réimplémente pas. |
| **INV-05** | **Archivage strictement local** | Chaque COG ne conserve que les données dont il a été partie prenante (messages envoyés ou reçus, fichiers, liste d'amis, tribus et salons auxquels il participe). |
| **INV-06** | **Chiffrement en transit obligatoire** | Aucun message, métadonnée sensible, fichier ou image ne transite en clair entre COGs. |
| **INV-07** | **Ever Buddy pour l'évolution** | Les évolutions de protocole, de format et de compatibilité sont gouvernées par Ever Buddy (versions, dépréciation). |
| **INV-08** | **TAMR pour l'humain** | Modération, litiges et révocation d'accès passent par TAMR (intervention humaine). |

---

## 3. Invariants de données

| # | Invariant | Description |
|---|-----------|-------------|
| **DAT-01** | **Pas de doublon de salon direct** | Une paire (COG A, COG B) correspond au plus à un salon direct. |
| **DAT-02** | **Identifiants stables** | Les identifiants (tribu, salon, message, ami) sont stables et uniques dans le périmètre défini (COG, ou accord Inter-COG). |
| **DAT-03** | **Horodatages cohérents** | Les horodatages sont générés et stockés de manière cohérente (ISO 8601 ou norme adoptée) ; la résolution des conflits de livraison différée peut s'appuyer sur ces timestamps. |
| **DAT-04** | **Profil / COG** | Les données sont liées au profil ou au COG ; pas d'accès croisé non autorisé (Border Guard, Master Butler). |
| **DAT-05** | **Rétention locale** | La rétention (conservation des messages et fichiers) est définie par la politique locale (WorrySentinel, KindMother) ; pas de rétention centralisée. |

---

## 4. Invariants d'intégration

| # | Invariant | Description |
|---|-----------|-------------|
| **INT-01** | **Central n'accède pas au contenu** | Miyukini Central affiche et ouvre le service Jay1Tribu ; il ne lit pas le contenu des messages pour affichage ou analyse. |
| **INT-02** | **MiyukiniWatch ne lit pas le contenu** | Si MiyukiniWatch enregistre des métadonnées liées à Jay1Tribu (conversation ouverte/fermée, ami contacté), il ne lit jamais le contenu des messages. |
| **INT-03** | **Résolution des pseudos par Jay1Tribu** | La résolution d'un identifiant technique (ex. `friend_cog_id`) en pseudo lisible est du ressort de Jay1Tribu (ou du service de contacts), pas de MiyukiniWatch ni de Miou. |
| **INT-04** | **Contrat Miou / Central** | Les APIs exposées à Miou (ex. `get_online_friends`, `get_friends_list`) ne renvoient que les données nécessaires (métadonnées, présence) ; pas de contenu des conversations. |
| **INT-05** | **Dégradation gracieuse** | Si Jay1Tribu est indisponible, Central et Miou continuent de fonctionner ; pas de crash, pas de blocage (ex. liste d'amis vide, pas de notification ami). |

---

## 5. Conformité aux Lois d'Autonomie

| Loi | Application à Jay1Tribu |
|-----|--------------------------|
| **LOI-2** | Le système accepte l'isolement ; la messagerie en temps réel n'est possible que lorsque les COGs sont connectés ; la tribu permet la livraison à la reconnexion. |
| **LOI-3** | L'état local est souverain : chaque COG est maître de ses archives (messages, fichiers, liste d'amis). |
| **LOI-4** | Horodatages et séquences locaux ; pas de dépendance à une horloge centrale pour le métier. |
| **LOI-6** | L'autonomie n'empêche pas la fédération : les COGs coopèrent via le MWS pour l'échange. |
| **LOI-7** | Évolution versionnée avec l'environnement COG (Ever Buddy). |

---

## 6. Matrice de vérification (résumé)

Lors de toute évolution ou implémentation, vérifier :

| Thème | Contraintes / Invariants à vérifier |
|-------|-------------------------------------|
| **Archives** | C-1, C-3, C-4, INV-02, INV-05, DAT-05 |
| **Transit** | C-2, INV-06 |
| **Type de Service** | C-5, INV-01 |
| **Tribus** | C-6, C-7 |
| **Présence / Amis** | C-8, INV-04, INT-03, INT-04 |
| **Gouvernance** | C-4, C-7, INV-03, INV-07, INV-08 |
| **Intégration** | INT-01 à INT-05 |

---

## 7. Références

| Document | Rôle |
|----------|------|
| [Jay1Tribu - Document Conceptuel](./Jay1Tribu%20-%20Document%20Conceptuel.md) | Concepts et contraintes conceptuelles. |
| [Jay1Tribu - Securite et Conformite](./Jay1Tribu%20-%20Securite%20et%20Conformite.md) | Sécurité, chiffrement, conformité. |
| [Jay1Tribu - Integration Central et Miou](./Jay1Tribu%20-%20Integration%20Central%20et%20Miou.md) | Contrat d'intégration. |

---

**Document** : Jay1Tribu — Contraintes et Invariants  
**Version** : 1.0  
**Date** : 2026-02-15  
**Statut** : Document normatif — contraintes non négociables
