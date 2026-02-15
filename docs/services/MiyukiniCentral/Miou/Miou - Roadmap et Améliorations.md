# Miou — Roadmap et Améliorations

Document de référence pour les améliorations planifiées de Miou : spécifications, priorité, effort, impact. Ces améliorations renforcent la relation, la personnalisation et le respect de l'utilisateur.

---

## 1. Contexte

Ce document consolide les **pistes d'amélioration prioritaires** identifiées pour Miou. Chacune est décrite avec une spécification exploitable, des critères d'acceptation et des liens vers les documents techniques concernés.

---

## 2. Priorité haute

### 2.1 Réutilisation des réponses dans les bulles

| Attribut | Valeur |
|----------|--------|
| **Priorité** | Haute |
| **Effort** | Moyen |
| **Impact** | Fort |

#### Description

Quand Miou connaît des informations sur l'utilisateur (via les réponses stockées — `reconfort`, `bonheur_quotidien`, `projet_coeur`, `activite_deconnexion`, etc.), les **réutiliser** dans les bulles pour montrer qu'elle se souvient et personnalise ses messages.

#### Exemples

| Donnée connue | Catégorie bulle | Exemple de réutilisation |
|---------------|-----------------|---------------------------|
| `reconfort` (« une tisane ») | Pause santé | « Ça fait 2h — tu m'as dit qu'une tisane te faisait du bien. Une pause ? » |
| `bonheur_quotidien` (« lire le matin ») | Accueil matin | « Bonjour {pseudo}. Un moment pour lire avant de démarrer ? » |
| `projet_coeur` (« refaire la déco ») | Retour absence | « Tu reviens. Ton projet déco avance ? » |
| `activite_deconnexion` (« courir ») | Pause santé | « 2h sur l'écran — tu m'as dit que courir te déconnecte. Et si tu sortais ? » |

#### Spécification technique

- **Variables injectables :** Étendre `BotContext` avec `user_responses_relevant` (données filtrées par catégorie de bulle).
- **Mapping donnée → catégorie :** Voir [Bot - Connaissance Utilisateur et Specs Machine](./Bot/Bot%20-%20Connaissance%20Utilisateur%20et%20Specs%20Machine.md) — section à ajouter « Réutilisation dans les bulles ».
- **Templates :** Ajouter des variantes avec placeholder `{reponse_X}` (ex. `{reconfort}`) dans les catégories concernées.
- **Condition :** N'utiliser une donnée que si `relation_level >= palier_question` et si la réponse est substantielle (longueur > 5 caractères).

#### Critères d'acceptation

- [ ] Au moins 3 catégories (pause, accueil, retour) ont des variantes avec réutilisation.
- [ ] Le moteur injecte les données uniquement quand pertinentes et non vides.
- [ ] Pas de réutilisation si l'utilisateur a effacé la réponse.

#### Références

- [Bot - Connaissance Utilisateur et Specs Machine](./Bot/Bot%20-%20Connaissance%20Utilisateur%20et%20Specs%20Machine.md)
- [Bot - Registre Questions et Paliers d'Attachement](./Bot/Bot%20-%20Registre%20Questions%20et%20Paliers%20d'Attachement.md)

---

### 2.2 Mode « Ne pas déranger »

| Attribut | Valeur |
|----------|--------|
| **Priorité** | Haute |
| **Effort** | Faible |
| **Impact** | Fort |

#### Description

Permettre à l'utilisateur d'activer un **mode Ne pas déranger (DND)** pendant lequel Miou n'affiche **aucune bulle** sauf les rappels critiques (événement imminent, pause santé après seuil très élevé — optionnel).

#### Options de configuration

| Option | Défaut | Description |
|--------|--------|-------------|
| **DND activé** | Non | Aucune bulle n'apparaît (sauf exceptions). |
| **Plage horaire** | Optionnel | Ex. 22h–8h : pas de bulles la nuit. |
| **Durée fixe** | Optionnel | Ex. « DND 2h » après activation manuelle. |
| **Exceptions** | Aucune | Ou : « Autoriser rappel événement < 1h » / « Autoriser pause santé » (à définir). |

#### Spécification technique

- **Paramètres Miou :** Ajouter `dnd_actif: bool`, `dnd_plage_debut`, `dnd_plage_fin` (Option<Time>), `dnd_fin_at` (Option<DateTime>) pour durée fixe.
- **Moteur de décision :** Contrôle en amont (priorité 0) : si DND actif ET bulle non exception → `silence`.
- **UI Paramètres :** Toggle « Ne pas déranger » + options déroulantes (plage, durée).

#### Critères d'acceptation

- [ ] Toggle DND dans Paramètres > Miou.
- [ ] Quand DND : aucune bulle (ou uniquement exceptions si implémentées).
- [ ] Option plage horaire ou durée fixe (au moins une des deux).

#### Références

- [Miou - Système de Bulles et UI](./Miou%20-%20Systeme%20de%20Bulles%20et%20UI.md)
- [Miou - Guide UI UX](./Miou%20-%20Guide%20UI%20UX.md)
- [Bot - Catalogue Complet des Triggers](./Bot/Bot%20-%20Catalogue%20Complet%20des%20Triggers.md)

---

## 3. Priorité moyenne

### 3.1 Templates saisonniers (F-01)

| Attribut | Valeur |
|----------|--------|
| **Priorité** | Moyenne |
| **Effort** | Faible |
| **Impact** | Moyen |

#### Description

Adapter les bulles d'accueil (et éventuellement autres) selon la **saison** ou des **dates festives** : Noël, Nouvel An, été, rentrée, etc. Renforce le lien avec le monde réel et le caractère vivant de Miou.

#### Périodes ciblées

| Période | Plage (approximative) | Exemple bulle accueil |
|---------|------------------------|------------------------|
| Noël | 20–26 décembre | « Joyeux Noël, {pseudo} ! Le Salon t'attend. » |
| Nouvel An | 30 déc – 2 jan | « Bonne année, {pseudo} ! » |
| Été | 21 juin – 21 sept | « L'été est là. Une session au frais ? » |
| Rentrée | 1–15 septembre | « La rentrée — ton COG est prêt pour la suite. » |
| Halloween | 28 oct – 1 nov | Optionnel, léger |

#### Spécification technique

- **Trigger F-01** : Déjà listé dans [Bot - Catalogue Complet des Triggers](./Bot/Bot%20-%20Catalogue%20Complet%20des%20Triggers.md). Condition : `date in plage_saison`.
- **Catégorie :** `accueil_saison` ou variante de `accueil_matin`/`accueil_soir` selon priorité (saison > heure).
- **Templates :** Ajouter dans [Bot - Banque de Templates Volume 2](./Bot/Bot%20-%20Banque%20de%20Templates%20Volume%202.md) section `accueil_saison`.

#### Critères d'acceptation

- [ ] Au moins 3 périodes (Noël, Nouvel An, été) avec templates dédiés.
- [ ] Priorité : si date dans plage ET première connexion → bulle saison. Sinon accueil standard.

#### Références

- [Bot - Catalogue Complet des Triggers](./Bot/Bot%20-%20Catalogue%20Complet%20des%20Triggers.md) — section 7 Triggers futurs
- [Bot - Banque de Templates Volume 2](./Bot/Bot%20-%20Banque%20de%20Templates%20Volume%202.md)

---

### 3.2 Variantes selon palier

| Attribut | Valeur |
|----------|--------|
| **Priorité** | Moyenne |
| **Effort** | Moyen |
| **Impact** | Fort |

#### Description

Les **variantes** des catégories (accueil, pause, retour, etc.) diffèrent selon le **palier d'attachement** actuel. Plus le palier est élevé, plus le ton est familier et personnalisé.

#### Mapping palier → ton

| Palier | Accueil matin | Pause santé | Retour absence |
|--------|---------------|-------------|----------------|
| Inconnue | « Bienvenue dans Miyukini Central. » | « Ça fait 2h — une pause ? » | « Content de te revoir. » |
| Connaissance | « Bonjour {pseudo}. » | « {duree} de session — accorde-toi une pause. » | « Ça fait {jours} jours — content de te revoir. » |
| Pote | « Salut {pseudo} ! » | « 2h — pause ? Tu vas en avoir besoin. » | « Te revoilà ! {jours} jours, c'est long. » |
| Amie | « Hey {pseudo}, bien dormi ? » | « Tu t'oublies — une pause s'impose. » | « {pseudo}, tu me manquais. » |
| Amie proche+ | « Salut toi. » | « Pause. Je ne veux pas que tu t'épuises. » | « Enfin. Comment ça va ? » |

#### Spécification technique

- **Sélecteur de variante :** Prendre en entrée `relation_level` en plus de la catégorie.
- **Structure templates :** Soit variantes taguées par palier (ex. `am1_inconnue`, `am1_pote`), soit un tableau palier → liste de variantes.
- **Fallback :** Si pas de variante pour le palier, utiliser celle du palier inférieur le plus proche.

#### Critères d'acceptation

- [ ] Au moins 3 catégories (accueil, pause, retour) ont des variantes par palier.
- [ ] Le moteur sélectionne la variante selon `relation_level`.
- [ ] Transition fluide entre paliers (pas de rupture de ton).

#### Références

- [Bot - Registre Questions et Paliers d'Attachement](./Bot/Bot%20-%20Registre%20Questions%20et%20Paliers%20d'Attachement.md)
- [Bot - Banque de Templates](./Bot/Bot%20-%20Banque%20de%20Templates.md)

---

## 4. Synthèse

| Amélioration | Priorité | Effort | Impact | Statut |
|--------------|----------|--------|--------|--------|
| Réutilisation des réponses | Haute | Moyen | Fort | Spécifié |
| Mode Ne pas déranger | Haute | Faible | Fort | Spécifié |
| Templates saisonniers | Moyenne | Faible | Moyen | Spécifié |
| Variantes selon palier | Moyenne | Moyen | Fort | Spécifié |

---

## 5. Références

- [Miou - Document Fondateur](./Miou%20-%20Document%20Fondateur.md)
- [Miou - Système de Bulles et UI](./Miou%20-%20Systeme%20de%20Bulles%20et%20UI.md)
- [Bot - Connaissance Utilisateur et Specs Machine](./Bot/Bot%20-%20Connaissance%20Utilisateur%20et%20Specs%20Machine.md)
- [Bot - Registre Questions et Paliers d'Attachement](./Bot/Bot%20-%20Registre%20Questions%20et%20Paliers%20d'Attachement.md)

---

*Roadmap Miou : personnalisation, respect, saisonnalité. L'utilisateur au centre.*

*Dernière mise à jour : 2026-02-15*
