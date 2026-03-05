# MiyukiniWatch â€” Document Fondateur

## 1. Vision et principe

**MiyukiniWatch** est un service **silencieux** et **toujours actif** dans le COG. Il mesure les habitudes et les interactions de l'utilisateur avec l'environnement (sessions, services, amis, clics) afin d'alimenter **Miou** â€” l'avatar/mascotte des COGs â€” pour des bulles, rappels et suggestions adaptÃ©s au bien-Ãªtre et aux habitudes.

### 1.1 Silencieux mais pas opaque

| Principe | Description |
|----------|-------------|
| **Silencieux** | Aucune notification, popup ou bandeau Ã©manant de MiyukiniWatch. Le service tourne en arriÃ¨re-plan sans solliciter l'attention. |
| **Pas opaque** | L'utilisateur peut **ouvrir MiyukiniWatch comme n'importe quel autre service** depuis Miyukini Central. Il y consulte les mesures prises, comprend ce qui est enregistrÃ©, et peut **effacer** tout ou partie des donnÃ©es ou **les laisser**. Aucune donnÃ©e cachÃ©e ; transparence totale sur le pÃ©rimÃ¨tre des mÃ©triques. |

### 1.2 Invariant : MiyukiniWatch ne lit pas les contenus

MiyukiniWatch **ne lit jamais** le contenu des messages, des champs saisis, des fichiers ou des pages. Il enregistre uniquement :

- **Quand** (horodatage, durÃ©e)
- **OÃ¹** (quel service, quel onglet, quel Ã©cran)
- **Qui** (amis contactÃ©s, identifiants techniques uniquement)
- **Combien** (nombre de clics, durÃ©e de session, frÃ©quence)

Aucune analyse de texte, aucun accÃ¨s au corps des messages ou aux donnÃ©es mÃ©tier des autres services.

---

## 2. MÃ©triques collectÃ©es

Toutes les mÃ©triques sont **locales au COG** (KindMother ou stockage dÃ©diÃ©). Aucun envoi Ã  un serveur tiers.

### 2.1 Sessions

| MÃ©trique | Description | Usage possible |
|----------|-------------|----------------|
| **DÃ©but / fin de session** | Horodatage de connexion et dÃ©connexion (ou fermeture de Central). | DurÃ©e de session ; Â« Tu n'es pas passÃ© depuis X jours Â». |
| **DurÃ©e de session** | Temps passÃ© dans Central par session. | Habitudes ; Miou peut suggÃ©rer une pause aprÃ¨s X minutes. |
| **Heure de connexion** | Plage horaire (matin, aprÃ¨s-midi, soir). | Adapter le ton (bonjour / bonsoir) et les suggestions. |

### 2.2 Services utilisÃ©s

| MÃ©trique | Description | Usage possible |
|----------|-------------|----------------|
| **Service ouvert** | Identifiant du service (ex. JayXpose, JayKoa) + horodatage. | Â« Tu reviens souvent sur JayXpose Â» ; Â« Tu n'as pas ouvert JayKoa depuis un moment Â». |
| **FrÃ©quence par service** | Nombre d'ouvertures par service sur une pÃ©riode (jour, semaine). | Classement des services les plus utilisÃ©s. |
| **Temps passÃ© par service** | DurÃ©e d'affichage / focus du service par session. | Comprendre oÃ¹ l'utilisateur passe le plus de temps. |

### 2.3 Amis et interactions sociales (si applicable)

S'applique lorsque des services Inter-COG ou de type messagerie (ex. Jay1Tribu) exposent des notions d'Â« amis Â» ou de contacts. MiyukiniWatch ne lit pas les messages ; il enregistre uniquement :

| MÃ©trique | Description | Usage possible |
|----------|-------------|----------------|
| **Amis contactÃ©s** | Identifiant technique du contact (pas le contenu) + horodatage de la derniÃ¨re interaction. | Â« Tu n'as pas Ã©changÃ© avec [pseudo] depuis X jours Â». |
| **Temps depuis derniÃ¨re discussion** | DÃ©lai depuis le dernier Ã©change avec chaque ami (en jours ou heures). | Rappels bienveillants ; Â« Pense Ã  reprendre contact avecâ€¦ Â». |
| **Classement amis par temps passÃ© Ã  discuter** | AgrÃ©gat de durÃ©e (temps de session oÃ¹ le service de discussion Ã©tait actif avec ce contact). | Miou peut mentionner les relations les plus investies ou celles dÃ©laissÃ©es. |

### 2.4 Interactions gÃ©nÃ©riques

| MÃ©trique | Description | Usage possible |
|----------|-------------|----------------|
| **Nombre de clics** | Compteur global ou par zone (optionnel : par service). AgrÃ©gat, pas de traÃ§age de cible prÃ©cise. | Indicateur d'activitÃ© ; Ã©viter surcharge (gamification positive uniquement). |

---

## 3. Interface utilisateur (ouvrable comme un autre service)

MiyukiniWatch apparaÃ®t dans la liste des services de Miyukini Central (Salon / BibliothÃ¨que). Lorsque l'utilisateur l'ouvre :

### 3.1 Ce que l'utilisateur voit

| Ã‰lÃ©ment | Description |
|---------|-------------|
| **RÃ©sumÃ© des mesures** | SynthÃ¨se lisible : derniÃ¨res sessions, services les plus utilisÃ©s, amis rÃ©cemment contactÃ©s / temps depuis derniÃ¨re discussion, agrÃ©gats de clics (si exposÃ©s). |
| **PÃ©rimÃ¨tre explicite** | Liste claire des types de donnÃ©es enregistrÃ©s (sessions, services, amis, clics) et rappel : Â« MiyukiniWatch ne lit pas le contenu de tes messages ni de tes saisies. Â» |
| **Historique / dÃ©tail** | PossibilitÃ© de consulter des pÃ©riodes (jour, semaine, mois) selon la rÃ©tention configurÃ©e. |

### 3.2 MaÃ®trise des donnÃ©es

| Action | Description |
|--------|-------------|
| **Effacer** | Bouton(s) pour effacer tout l'historique MiyukiniWatch ou une plage (ex. Â« Effacer les donnÃ©es du dernier mois Â»). AprÃ¨s effacement, Miou continuera Ã  fonctionner avec des messages gÃ©nÃ©riques jusqu'Ã  ce que de nouvelles mÃ©triques soient collectÃ©es. |
| **Laisser** | L'utilisateur peut ne rien effacer ; les donnÃ©es restent pour alimenter Miou et les suggestions. |
| **DÃ©sactiver la collecte** | Option (dans MiyukiniWatch ou dans ParamÃ¨tres Miyukini) pour dÃ©sactiver toute nouvelle collecte. Les donnÃ©es dÃ©jÃ  prÃ©sentes restent consultables et effaÃ§ables. |

---

## 4. Gouvernance et rÃ©tention

| RÃ¨gle | Description |
|-------|-------------|
| **Local uniquement** | Toutes les donnÃ©es restent sur le COG. Aucune tÃ©lÃ©metrie externe. |
| **RÃ©tention** | Politique configurable (ex. 90 jours) ; au-delÃ , les agrÃ©gats peuvent Ãªtre purgÃ©s automatiquement ou proposÃ©s Ã  l'effacement. |
| **Transparence** | La liste des mÃ©triques (section 2) est documentÃ©e et reprise dans l'interface du service. |
| **SÃ©curitÃ©** | AccÃ¨s aux donnÃ©es MiyukiniWatch rÃ©servÃ© au profil connectÃ© ; pas d'export par dÃ©faut vers l'extÃ©rieur du COG. |

---

## 5. Lien avec Miou

Les **agrÃ©gats** produits par MiyukiniWatch (sans accÃ¨s aux dÃ©tails bruts si on souhaite limiter l'exposition) sont utilisÃ©s par **Miou** pour :

- Adapter le **contenu des bulles** (messages contextuels en bas Ã  droite).
- Proposer des **rappels** ou **notifications** bienveillantes (pause, ami non contactÃ©, Ã©vÃ©nement Ã  venir).
- Renforcer la **relation sincÃ¨re et Ã©motionnelle** : l'utilisateur doit sentir que Miou connaÃ®t ses habitudes pour mieux l'aider, pas pour le surveiller.

Le rÃ´le de Miou reste la **santÃ©**, le **bien-Ãªtre Ã©motionnel et physique**, l'**amusement** et une **relation de confiance**. MiyukiniWatch est un outil au service de ce rÃ´le, dans un cadre strict : pas de lecture de contenus, transparence et maÃ®trise utilisateur.

---

## 6. Type de Service et espaces

| Attribut | Valeur |
|----------|--------|
| **Type** | Service interne COG (Type 1) |
| **Espace** | Miyukini Central uniquement |
| **Surface externe** | Aucune |

---

## 7. RÃ©fÃ©rences

- [Miou â€” Documentation complÃ¨te](../MiyukiniCentral/Miou/_index.md)
- [Miyukini Central â€” Salon propositions lieu de vie gamification Miou](../MiyukiniCentral/Miyukini%20Central%20-%20Salon%20propositions%20lieu%20de%20vie%20gamification%20Miou.md)
- [Types de Services et Espaces](..//..//miyukini-webway-system//reference//_index.md)

---

*Document fondateur MiyukiniWatch â€” Service silencieux de mesure des habitudes au service de Miou et du bien-Ãªtre utilisateur.*

