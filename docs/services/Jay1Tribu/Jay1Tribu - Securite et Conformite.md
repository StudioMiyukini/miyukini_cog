# Jay1Tribu — Sécurité et Conformité

## Contexte

**Jay1Tribu** est un Service Inter-COG dont les données (messages, fichiers, images) sont sensibles. Ce document décrit les exigences de **sécurité** (chiffrement, contrôles d'accès, modèle de menaces) et de **conformité** (gouvernance Cores, Lois d'Autonomie, bonnes pratiques).

## Portée / Scope

- **Applicable à :** Conception sécurité, chiffrement, contrôles d'accès, audit, conformité.
- **Audience :** Équipes sécurité, architectes, développeurs, auditeurs.
- **Statut :** Document normatif de référence sécurité.

### Hors périmètre

- Choix précis d'algorithmes et de bibliothèques (à figer en implémentation avec WorrySentinel).

---

## 1. Classification des données

| Classe | Exemples | Niveau de sensibilité | Règle |
|--------|----------|------------------------|-------|
| **Contenu de message** | Texte, pièces jointes, images | Élevé | Transit crypté obligatoire ; au repos selon politique WorrySentinel / KindMother (chiffrement disque ou DB). |
| **Métadonnées de conversation** | Identifiants salon, tribu, participants, horodatages | Moyen | Transit crypté ; exposition minimale (seuls les participants et le MWS pour le routage, sans lecture du contenu). |
| **Liste d'amis** | Identifiants COG, pseudos | Moyen | Local uniquement ; pas de partage avec des tiers. |
| **Paramètres tribu** | Rôles, règles d'invitation | Moyen | Local + échange Inter-COG crypté pour synchronisation entre membres. |
| **Présence** | En ligne / hors ligne | Faible | Fournie par le MWS ; Jay1Tribu consomme sans la dupliquer. |

**Principe :** Toute donnée échangée entre COGs est considérée comme sensible jusqu'à preuve contraire ; le défaut est le chiffrement.

---

## 2. Chiffrement

### 2.1 En transit

| Exigence | Description |
|----------|-------------|
| **Obligation** | L'ensemble des données qui transitent entre COGs (messages, fichiers, images, métadonnées sensibles) est crypté. Aucune exception. |
| **Périmètre** | Transport via le MWS : le canal (TLS ou équivalent) et/ou le payload applicatif (chiffrement de bout en bout) doivent garantir la confidentialité et l'intégrité. |
| **Spécification technique** | Les mécanismes (chiffrement de bout en bout, échange de clés, gestion des identités) seront définis dans une spécification technique et validés avec WorrySentinel et Border Guard. |

### 2.2 Au repos

| Exigence | Description |
|----------|-------------|
| **Autorité** | La classification et le chiffrement au repos dans chaque COG relèvent de **WorrySentinel** et **KindMother** (niveaux de sécurité, politique de résidence des données). |
| **Cohérence** | Si le COG applique le chiffrement de base de données (ex. SQLCipher via kindmother-db-key), les données Jay1Tribu sont incluses dans ce périmètre. |
| **Pas de stockage central** | Puisqu'il n'y a pas d'archives centrales, la question du chiffrement au repos ne se pose que localement (chaque COG). |

---

## 3. Contrôles d'accès et permissions

| Niveau | Mécanisme | Rôle |
|--------|-----------|------|
| **Décision** | StrongFather | Autorise ou refuse toute action (envoi, création tribu, invitation, attribution de rôles). |
| **Capacités** | Master Butler | Registre des permissions : qui peut créer un salon, inviter, être Chef de tribu, envoyer des fichiers, etc. |
| **Frontières** | Border Guard | Définit qui peut communiquer avec qui (COGs de confiance, règles Inter-COG). |
| **Persistance** | KindMother | Valide les WriteIntent ; refuse toute écriture non autorisée ou non conforme. |
| **Sécurité** | WorrySentinel | Niveaux de sécurité des contenus, règles de rétention, politique de chiffrement. |

**Règle :** Aucun accès en écriture ou en lecture sensible sans passage par les Cores (via BondingBrother).

---

## 4. Modèle de menaces (résumé)

| Menace | Mitigation |
|--------|------------|
| **Interception du transit** | Chiffrement systématique (TLS et/ou E2E). |
| **Archives volées ou exposées** | Pas d'archives centrales ; au repos gouverné par WorrySentinel/KindMother (chiffrement DB si activé). |
| **Usurpation d'identité COG** | Authentification et identité COG gérées par le MWS et les Cores ; Jay1Tribu s'appuie sur ces garanties. |
| **Accès non autorisé à un salon / tribu** | Permissions et rôles gouvernés par Master Butler / StrongFather ; Border Guard pour les frontières. |
| **Modération / abus** | TAMR : points d'intervention humaine (modération, litiges, révocation d'accès). |
| **Déni de service** | Caring Nanny : observation de l'état ; réduction ou suspension possible en environnement dégradé. |

---

## 5. Conformité

### 5.1 Lois d'Autonomie

Jay1Tribu respecte les Lois d'Autonomie Miyukini (LOI-2, LOI-3, LOI-4, LOI-6, LOI-7) ; voir [Contraintes et Invariants](./Jay1Tribu%20-%20Contraintes%20et%20Invariants.md).

### 5.2 Contraintes non négociables

Les contraintes C-1 à C-8 et les invariants documentés dans [Jay1Tribu - Contraintes et Invariants](./Jay1Tribu%20-%20Contraintes%20et%20Invariants.md) sont impératives. Toute évolution doit les préserver.

### 5.3 Audit et traçabilité

- Les décisions de gouvernance (StrongFather, KindMother) peuvent faire l'objet de logs d'audit selon la politique du COG (WorrySentinel, TAMR).
- Aucun contenu de message ne doit être logué en clair ; seuls les événements (envoi, réception, création salon, etc.) et les identifiants techniques peuvent être tracés, selon la politique de confidentialité.

---

## 6. Bonnes pratiques de développement

| Pratique | Description |
|----------|-------------|
| **Pas de secret en clair** | Clés et secrets (chiffrement, authentification) ne doivent jamais être stockés ou logués en clair. |
| **Dépendances à jour** | Bibliothèques de chiffrement et de communication maintenues et patchées (WorrySentinel, Ever Buddy). |
| **Principe du moindre privilège** | Les Opérateurs et Outils n'accèdent qu'aux données et capacités strictement nécessaires. |
| **Revue sécurité** | Les changements affectant le chiffrement, le transport ou les permissions font l'objet d'une revue alignée avec WorrySentinel et Border Guard. |

---

## 7. Références

| Document | Rôle |
|----------|------|
| [Jay1Tribu - Contraintes et Invariants](./Jay1Tribu%20-%20Contraintes%20et%20Invariants.md) | Contraintes C-1 à C-8, invariants. |
| [Jay1Tribu - Document Conceptuel](./Jay1Tribu%20-%20Document%20Conceptuel.md) | Concepts, gouvernance Cores. |
| [Security — Liste des Mesures de Sécurité](../../security/reference/Security%20-%20Liste%20des%20Mesures%20de%20Securite%20Miyukini%20COG%20et%20MWS.md) | Référence sécurité Miyukini COG et MWS. |

---

**Document** : Jay1Tribu — Sécurité et Conformité  
**Version** : 1.0  
**Date** : 2026-02-15  
**Statut** : Document normatif
