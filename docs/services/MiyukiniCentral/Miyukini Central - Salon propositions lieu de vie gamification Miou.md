# Miyukini Central — Salon : propositions lieu de vie, gamification et Miou

Document de propositions pour faire du **Salon** (anciennement Magasin) la page d'accueil du COG : lieu de vie chaleureux, juste après le rite d'entrée ou la connexion, avec une bulle de dialogue Miou, de la gamification incarnée et des mesures utilisateur.

**Contexte :** Le Salon est l'onglet par défaut affiché après Connexion / Rite d'Entrée. Miou accompagne déjà l'utilisateur à la voix sur les écrans Connexion et Rite d'Entrée ; l'objectif est d'étendre cette incarnation au Salon et d'en faire un « lieu de vie » plutôt qu'un simple hub de services.

**Liens structurants :**
- **MiyukiniWatch** (service silencieux) mesure les habitudes et interactions ; l'utilisateur peut l'ouvrir comme tout autre service pour consulter et effacer les données. Voir [MiyukiniWatch — Document Fondateur](../MiyukiniWatch/MiyukiniWatch%20-%20Document%20Fondateur.md).
- **Miou** (avatar/mascotte des COGs) communique via des **bulles en bas à droite** de l'affichage ; elle s'adapte grâce à MiyukiniWatch et au profil. Rôle : santé, bien-être, amusement, relation sincère. Voir [Miou — Documentation complète](./Miou/_index.md).

---

## 1. Enrichir le Salon (page d'accueil)

### 1.1 UI « lieu de vie » — ambiance salon

| Proposition | Description |
|-------------|-------------|
| **Palette et atmosphère** | Tons chauds (ambre, terracotta, bois), lumière douce, ombres légères ; éviter le « dashboard froid ». Option thème « Salon » distinct du thème Gaming actuel. |
| **Espace central** | Zone principale type « canapé » : contenu mis en avant (suggestions Miou, activité du jour) au centre, pas une grille de cartes seule. |
| **Coin et repères** | Zones nommées : « Coin des Services », « Coin Webway », « Table du jour » (suggestions), « Où en es-tu ? » (progression). |
| **Micro-interactions** | Transitions douces à l’entrée (fade-in, léger parallax), reflets ou lueur sur les cartes au survol, sons discrets (optionnel, cohérents avec Miou). |
| **Rythme visuel** | Pas de surcharge : une hero « bienvenue [pseudo] » + bulle Miou + un bloc d’actions suggérées + accès aux services. |

### 1.2 Bulle / fenêtre de dialogue Miou

| Proposition | Description |
|-------------|-------------|
| **Bulle de suggestion** | Une bulle (type bulle de BD ou carte arrondie) affichée en évidence, avec avatar/icône Miou (🌸 ou mascotte dédiée) et un court message contextuel. |
| **Contenu dynamique** | Message basé sur : heure de la journée, dernier passage, métriques **MiyukiniWatch** (voir section 3 et [MiyukiniWatch](../MiyukiniWatch/MiyukiniWatch%20-%20Document%20Fondateur.md)), habitudes (services les plus ouverts, onglet favori). |
| **Exemples de messages** | « Bonjour [pseudo], tu n’es pas passé depuis 3 jours — JayKoa a peut-être des événements à te rappeler. » / « Ce soir, rien de prévu dans ton calendrier. Veux-tu faire un tour au Webway ? » / « Tu reviens souvent sur JayXpose : ta vitrine est à jour ? » |
| **Actions rapides** | 1–2 boutons dans la bulle : « Voir le calendrier », « Ouvrir JayXpose », « Explorer le Webway », « C’est tout pour l’instant ». |
| **Dismiss / rappel** | Possibilité de fermer la bulle pour la session ; elle réapparaît au prochain lancement ou après un délai configurable. |
| **Placement** | **Bulles Miou : en bas à droite** (position canonique). Voir [Miou — Système de Bulles et UI](./Miou/Miou%20-%20Systeme%20de%20Bulles%20et%20UI.md). Sur le Salon, une zone « Table du jour » peut reprendre le message au centre (above the fold). |

### 1.3 Contenu et structure de la page

| Proposition | Description |
|-------------|-------------|
| **Titre personnalisé** | « Bienvenue, [pseudo] » ou « Bonsoir, [pseudo] » selon l’heure, au lieu d’un générique « Bienvenue dans Miyukini Central ». |
| **Bloc « À faire / suggéré »** | Liste courte (3–5 items) : tâches ou suggestions dérivées des métriques (ex. « Mettre à jour ta vitrine », « Événement demain 9h »). |
| **Services en « lieu de vie »** | Présenter les services comme des « pièces » ou « coins » (Coin Exposant, Coin Calendrier, Coin Réseau) avec libellés chaleureux et icônes cohérentes. |
| **Nouvelles / annonces COG** | Petit bandeau ou ligne « Dernière nouvelle Miyukini » (optionnel, si contenu disponible). |

---

## 2. Gamification incarnée par Miou

| Proposition | Description |
|-------------|-------------|
| **Miou comme guide** | Miou n’est pas un simple avatar : elle commente les petites victoires (« Ta vitrine est à jour »), encourage (« Plus qu’un pas pour finir ta config Webway »), félicite (« Premier événement créé dans JayKoa ! »). |
| **Phrases courtes et voix** | Réutiliser le système audio existant (voix Miou) pour des répliques ponctuelles sur le Salon (optionnel, désactivable) : connexion, première visite du jour, objectif atteint. |
| **Progression visible** | Indicateurs simples que Miou peut commenter : « Premier lancement », « 7 jours d’utilisation », « Tous les Cores verts », « Premier service installé », « Webway connecté ». |
| **Badges / étapes** | Badges discrets (icône + tooltip) : « Habitant installé », « Exposant actif », « Calendrier en usage », « Réseau connecté ». Miou peut les mentionner dans la bulle (« Tu as débloqué le badge Réseau connecté »). |
| **Pas de punition** | Gamification positive uniquement : pas de « niveau qui baisse » ni de pression ; Miou encourage sans culpabiliser. |
| **Cohérence narrative** | Ton de Miou identique à Connexion / Rite d’Entrée : bienveillant, invitant, léger (« Entre donc », « Rejoins moi à l’intérieur »). |

---

## 3. Mesures utilisateur (métriques et habitudes)

Les métriques servent à **alimenter la bulle Miou** et les suggestions du Salon, dans le respect de la souveraineté (données locales, pas de télémetrie externe). Elles sont **déléguées au service MiyukiniWatch** : voir [MiyukiniWatch — Document Fondateur](../MiyukiniWatch/MiyukiniWatch%20-%20Document%20Fondateur.md) pour le périmètre complet (sessions, services, amis, clics, pas de lecture de contenus). L'utilisateur peut ouvrir MiyukiniWatch comme tout autre service pour consulter et effacer ces données.

### 3.1 Métriques à collecter (côté COG — via MiyukiniWatch)

| Métrique | Usage possible |
|----------|----------------|
| **Dernière connexion** | « Tu n’es pas passé depuis X jours » ; « Bon retour ». |
| **Heure de connexion** | Adapter le message (bonjour / bonsoir) et les suggestions (matin : calendrier, soir : détente). |
| **Services les plus ouverts** | Suggérer le service favori ou « Tu n’as pas ouvert JayKoa depuis un moment ». |
| **Onglet principal** | Salon vs Bibliothèque vs Webway : adapter le message (ex. « Tu passes souvent par la Bibliothèque — un service à découvrir ? »). |
| **État des services** | Vitrine à jour ou pas, événements à venir (JayKoa), connexion MWS. |
| **Nombre de lancements / jours actifs** | Déblocage de badges ou messages du type « 7 jours avec ton COG ». |
| **Rite d’entrée vs Connexion** | Premier passage après Rite : message « Bienvenue dans ton nouveau chez-toi » ; après Connexion : « Content de te revoir ». |
| **Amis contactés / temps depuis dernière discussion** | Rappels bienveillants : « Pense à reprendre contact avec [pseudo] ». Voir MiyukiniWatch. |
| **Nombre de clics (agrégat)** | Indicateur d'activité ; usage limité pour ne pas surcharger (gamification positive). |

### 3.2 Stockage et gouvernance

| Proposition | Description |
|-------------|-------------|
| **Local uniquement** | Toutes les métriques restent sur le COG (MiyukiniWatch, ex. KindMother ou stockage dédié) ; pas d’envoi à un serveur tiers. |
| **Transparence** | L'utilisateur ouvre **MiyukiniWatch** pour voir les mesures ; liste lisible des types de données (sessions, services, amis, clics). Option dans Paramètres Miyukini : « Données utilisées pour les suggestions Miou » pointant vers ce service (dernière connexion, services ouverts, etc.). |
| **Désactivation** | Possibilité de désactiver la collecte MiyukiniWatch ou les suggestions personnalisées ; la bulle Miou affiche alors des messages génériques (bienvenue, lien vers services). |
| **Rétention** | Politique claire (ex. 90 jours) ; l'utilisateur peut effacer tout ou partie des données depuis MiyukiniWatch. |

---

## 4. Synthèse des priorités

| Priorité | Élément | Justification |
|----------|---------|---------------|
| **P0** | Bulle Miou avec message contextuel + 1–2 actions | Cœur de la « vie » du Salon et lien direct avec l’incarnation Miou. |
| **P0** | UI Salon « lieu de vie » (titres, zones, ambiance) | Différenciation par rapport à un simple hub. |
| **P1** | Métriques minimales (dernière connexion, services ouverts) | Nécessaires pour personnaliser la bulle. |
| **P1** | Messages Miou selon métriques (retour, heure, service favori) | Donne l’impression que Miou « connaît » l’utilisateur. |
| **P2** | Badges / progression + phrases Miou associées | Renforce la gamification sans complexifier. |
| **P2** | Voix Miou ponctuelle sur le Salon (optionnel) | Cohérence avec Connexion / Rite d’entrée. |
| **P3** | Thème « Salon » (palette chaleureuse dédiée) | Améliore l’ambiance sans bloquer le reste. |
| **P3** | Paramètres (désactiver suggestions, voir données) | Confiance et souveraineté. |

---

## 5. Références techniques actuelles

**Documents liés :** [MiyukiniWatch — Document Fondateur](../MiyukiniWatch/MiyukiniWatch%20-%20Document%20Fondateur.md) (service silencieux, consulter / effacer les mesures) · [Miou — Documentation complète](./Miou/_index.md) (bulles en bas à droite, rôle bien-être).

- **Salon** : `MainTab::Salon`, vue dans `apps/central/src/services/home.rs` (`HomeView`), contenu `ServiceGrid` « Services populaires ».
- **Rite d’entrée** : `apps/central/src/screens/rite_entree.rs` ; après création du compte → `App` affiche Header + main avec `MainTab::Salon`.
- **Connexion** : `apps/central/src/screens/connexion.rs` ; Miou accueil (voix + phrases) ; après login → même flux Salon.
- **État** : `apps/central/src/state.rs` — `AppState` (current_user, main_tab, services, last_login_*, pas encore de métriques « habitudes »). À terme : délégation à MiyukiniWatch.
- **Voix Miou** : `apps/central/src/audio.rs` ; sons dans `miyuclicker_data_dir` (ex. `login_retour_a.mp3`, `login_new_ask_name.mp3`).

---

*Document créé pour orienter l’évolution du Salon (page d’accueil du COG) : lieu de vie, gamification incarnée par Miou, et mesures utilisateur au service des suggestions.*
