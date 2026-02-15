# MiyukiniWatch — Document Fondateur

## 1. Vision et principe

**MiyukiniWatch** est un service **silencieux** et **toujours actif** dans le COG. Il mesure les habitudes et les interactions de l'utilisateur avec l'environnement (sessions, services, amis, clics) afin d'alimenter **Miou** — l'avatar/mascotte des COGs — pour des bulles, rappels et suggestions adaptés au bien-être et aux habitudes.

### 1.1 Silencieux mais pas opaque

| Principe | Description |
|----------|-------------|
| **Silencieux** | Aucune notification, popup ou bandeau émanant de MiyukiniWatch. Le service tourne en arrière-plan sans solliciter l'attention. |
| **Pas opaque** | L'utilisateur peut **ouvrir MiyukiniWatch comme n'importe quel autre service** depuis Miyukini Central. Il y consulte les mesures prises, comprend ce qui est enregistré, et peut **effacer** tout ou partie des données ou **les laisser**. Aucune donnée cachée ; transparence totale sur le périmètre des métriques. |

### 1.2 Invariant : MiyukiniWatch ne lit pas les contenus

MiyukiniWatch **ne lit jamais** le contenu des messages, des champs saisis, des fichiers ou des pages. Il enregistre uniquement :

- **Quand** (horodatage, durée)
- **Où** (quel service, quel onglet, quel écran)
- **Qui** (amis contactés, identifiants techniques uniquement)
- **Combien** (nombre de clics, durée de session, fréquence)

Aucune analyse de texte, aucun accès au corps des messages ou aux données métier des autres services.

---

## 2. Métriques collectées

Toutes les métriques sont **locales au COG** (KindMother ou stockage dédié). Aucun envoi à un serveur tiers.

### 2.1 Sessions

| Métrique | Description | Usage possible |
|----------|-------------|----------------|
| **Début / fin de session** | Horodatage de connexion et déconnexion (ou fermeture de Central). | Durée de session ; « Tu n'es pas passé depuis X jours ». |
| **Durée de session** | Temps passé dans Central par session. | Habitudes ; Miou peut suggérer une pause après X minutes. |
| **Heure de connexion** | Plage horaire (matin, après-midi, soir). | Adapter le ton (bonjour / bonsoir) et les suggestions. |

### 2.2 Services utilisés

| Métrique | Description | Usage possible |
|----------|-------------|----------------|
| **Service ouvert** | Identifiant du service (ex. JayXpose, JayKoa) + horodatage. | « Tu reviens souvent sur JayXpose » ; « Tu n'as pas ouvert JayKoa depuis un moment ». |
| **Fréquence par service** | Nombre d'ouvertures par service sur une période (jour, semaine). | Classement des services les plus utilisés. |
| **Temps passé par service** | Durée d'affichage / focus du service par session. | Comprendre où l'utilisateur passe le plus de temps. |

### 2.3 Amis et interactions sociales (si applicable)

S'applique lorsque des services Inter-COG ou de type messagerie (ex. Jay1Tribu) exposent des notions d'« amis » ou de contacts. MiyukiniWatch ne lit pas les messages ; il enregistre uniquement :

| Métrique | Description | Usage possible |
|----------|-------------|----------------|
| **Amis contactés** | Identifiant technique du contact (pas le contenu) + horodatage de la dernière interaction. | « Tu n'as pas échangé avec [pseudo] depuis X jours ». |
| **Temps depuis dernière discussion** | Délai depuis le dernier échange avec chaque ami (en jours ou heures). | Rappels bienveillants ; « Pense à reprendre contact avec… ». |
| **Classement amis par temps passé à discuter** | Agrégat de durée (temps de session où le service de discussion était actif avec ce contact). | Miou peut mentionner les relations les plus investies ou celles délaissées. |

### 2.4 Interactions génériques

| Métrique | Description | Usage possible |
|----------|-------------|----------------|
| **Nombre de clics** | Compteur global ou par zone (optionnel : par service). Agrégat, pas de traçage de cible précise. | Indicateur d'activité ; éviter surcharge (gamification positive uniquement). |

---

## 3. Interface utilisateur (ouvrable comme un autre service)

MiyukiniWatch apparaît dans la liste des services de Miyukini Central (Salon / Bibliothèque). Lorsque l'utilisateur l'ouvre :

### 3.1 Ce que l'utilisateur voit

| Élément | Description |
|---------|-------------|
| **Résumé des mesures** | Synthèse lisible : dernières sessions, services les plus utilisés, amis récemment contactés / temps depuis dernière discussion, agrégats de clics (si exposés). |
| **Périmètre explicite** | Liste claire des types de données enregistrés (sessions, services, amis, clics) et rappel : « MiyukiniWatch ne lit pas le contenu de tes messages ni de tes saisies. » |
| **Historique / détail** | Possibilité de consulter des périodes (jour, semaine, mois) selon la rétention configurée. |

### 3.2 Maîtrise des données

| Action | Description |
|--------|-------------|
| **Effacer** | Bouton(s) pour effacer tout l'historique MiyukiniWatch ou une plage (ex. « Effacer les données du dernier mois »). Après effacement, Miou continuera à fonctionner avec des messages génériques jusqu'à ce que de nouvelles métriques soient collectées. |
| **Laisser** | L'utilisateur peut ne rien effacer ; les données restent pour alimenter Miou et les suggestions. |
| **Désactiver la collecte** | Option (dans MiyukiniWatch ou dans Paramètres Miyukini) pour désactiver toute nouvelle collecte. Les données déjà présentes restent consultables et effaçables. |

---

## 4. Gouvernance et rétention

| Règle | Description |
|-------|-------------|
| **Local uniquement** | Toutes les données restent sur le COG. Aucune télémetrie externe. |
| **Rétention** | Politique configurable (ex. 90 jours) ; au-delà, les agrégats peuvent être purgés automatiquement ou proposés à l'effacement. |
| **Transparence** | La liste des métriques (section 2) est documentée et reprise dans l'interface du service. |
| **Sécurité** | Accès aux données MiyukiniWatch réservé au profil connecté ; pas d'export par défaut vers l'extérieur du COG. |

---

## 5. Lien avec Miou

Les **agrégats** produits par MiyukiniWatch (sans accès aux détails bruts si on souhaite limiter l'exposition) sont utilisés par **Miou** pour :

- Adapter le **contenu des bulles** (messages contextuels en bas à droite).
- Proposer des **rappels** ou **notifications** bienveillantes (pause, ami non contacté, événement à venir).
- Renforcer la **relation sincère et émotionnelle** : l'utilisateur doit sentir que Miou connaît ses habitudes pour mieux l'aider, pas pour le surveiller.

Le rôle de Miou reste la **santé**, le **bien-être émotionnel et physique**, l'**amusement** et une **relation de confiance**. MiyukiniWatch est un outil au service de ce rôle, dans un cadre strict : pas de lecture de contenus, transparence et maîtrise utilisateur.

---

## 6. Type de Service et espaces

| Attribut | Valeur |
|----------|--------|
| **Type** | Service interne COG (Type 1) |
| **Espace** | Miyukini Central uniquement |
| **Surface externe** | Aucune |

---

## 7. Références

- [Miou — Documentation complète](../MiyukiniCentral/Miou/_index.md)
- [Miyukini Central — Salon propositions lieu de vie gamification Miou](../MiyukiniCentral/Miyukini%20Central%20-%20Salon%20propositions%20lieu%20de%20vie%20gamification%20Miou.md)
- [Types de Services et Espaces](../../reference/Miyukini%20Conceptual%20References%20-%20Types%20de%20Services%20et%20Espaces.md)

---

*Document fondateur MiyukiniWatch — Service silencieux de mesure des habitudes au service de Miou et du bien-être utilisateur.*
