# Miou — Onglet Service et Mode Chatbot

Miou dispose d'un **onglet de service dédié** dans Miyukini Central, offrant un **mode chatbot** de type ChatGPT pour discuter et enrichir ses connaissances sur l'utilisateur. L'interface est transparente : l'utilisateur peut consulter, modifier et supprimer les données de la base de connaissance.

---

## 1. Contexte et positionnement

| Aspect | Description |
|--------|-------------|
| **Type** | Onglet de service Miou (comme JayKoa, JayXpose) dans la TabBar de Central |
| **Accès** | Bibliothèque > Carte Miou, ou onglet dédié si Miou est « installée » comme service |
| **Rôle** | Canal conversationnel privilégié, enrichissement des connaissances, transparence totale |

**Phrase fondatrice :**

> **L'onglet Miou est le lieu où l'utilisateur converse librement avec Miou et contrôle tout ce qu'elle sait de lui.**

---

## 2. Interface type ChatGPT

### 2.1 Structure générale

| Zone | Description |
|------|-------------|
| **En-tête** | Titre « Miou », indication du palier de relation (optionnel), lien vers « Mes connaissances » |
| **Zone conversation** | Historique des échanges (bulles utilisateur / réponses Miou), scroll automatique |
| **Zone saisie** | Champ texte multiligne + bouton Envoyer (style ChatGPT / Claude) |
| **Sidebar (optionnelle)** | Accès rapide : « Ce que Miou sait de moi », Paramètres Miou |

### 2.2 Comportement conversationnel

| Fonctionnalité | Description |
|----------------|-------------|
| **Saisie libre** | L'utilisateur tape un message, Miou répond (Proto-IA ou LLM selon config) |
| **Contexte** | Miou a accès au contexte : profil, connaissances stockées, session, palier |
| **Enrichissement** | Miou peut poser des questions, l'utilisateur peut divulguer des infos — stockage explicite |
| **Ton** | Adapté au palier d'attachement (Connaissance → Grande sœur) |
| **Références culturelles** | Miou peut blaguer ou être espiègle avec des références pop culture (voir [Miou - Base Culture Populaire](./Miou%20-%20Base%20Culture%20Populaire.md)) |

### 2.3 Invariants

- **Aucune collecte passive** : Miou ne lit que ce que l'utilisateur envoie dans le chat.
- **Stockage explicite** : Les infos extraites sont enregistrées uniquement après validation ou accord implicite (ex. « Tu peux retenir ça »).
- **100 % local** : Aucune donnée envoyée hors du COG.

---

## 3. Base de connaissance — Transparence

### 3.1 Accès « Ce que Miou sait de moi »

Depuis l'onglet Miou (ou Paramètres > Miou), l'utilisateur accède à la **liste complète** des connaissances stockées.

| Action | Description |
|--------|-------------|
| **Voir** | Affichage de chaque donnée : clé (ex. `reconfort`), valeur, date d'enregistrement |
| **Modifier** | Édition inline ou formulaire — mise à jour immédiate |
| **Supprimer** | Bouton par entrée — confirmation légère, suppression définitive |

### 3.2 Organisation de l'affichage

Les données sont regroupées par **catégorie** (voir [Miou - Catalogue Exhaustif des Connaissances](./Miou%20-%20Catalogue%20Exhaustif%20des%20Connaissances.md)) :

- Identité et préférences pratiques
- Loisirs et habitudes
- Émotions et bien-être
- Projets et valeurs
- Soutien et présence
- Données Rite d'Entrée (profil Central)

### 3.3 Options globales

| Option | Effet |
|--------|-------|
| **Effacer tout** | « Réinitialiser la connaissance de Miou » — supprime toutes les réponses, remet le palier à Inconnue (ou propose de garder les données tout en réinitialisant le palier) |
| **Exporter** | Export JSON ou CSV des données (pour portabilité, backup) |
| **Importer** | Restauration depuis un export (à définir) |

---

## 4. Flux conversationnel et enrichissement

### 4.1 Déclenchement de questions

Miou peut **proposer** des questions dans le chat :

- Selon le palier d'attachement
- Selon les thèmes non encore couverts
- En réponse à un message de l'utilisateur (« Et toi, qu'est-ce qui te fait du bien ? »)

### 4.2 Extraction et confirmation

Quand l'utilisateur partage une info pertinente :

| Mode | Description |
|------|-------------|
| **Explicite** | Miou demande : « Tu veux que je retienne ça ? » — boutons Oui / Non |
| **Implicite** | Si l'utilisateur dit clairement « retiens que… » — enregistrement direct |
| **Déduit** | Miou ne déduit jamais sans confirmation — pas d'extraction passive |

### 4.3 Intégration avec les bulles

Les données enrichies via le chatbot sont **réutilisées** dans les bulles (pause santé, accueil, retour). Voir [Miou - Roadmap et Améliorations](./Miou%20-%20Roadmap%20et%20Améliorations.md) — réutilisation des réponses.

---

## 5. Références

- [Miou - Catalogue Exhaustif des Connaissances](./Miou%20-%20Catalogue%20Exhaustif%20des%20Connaissances.md)
- [Miou - Préférences Utilisateur et Adaptation Central](./Miou%20-%20Pr%C3%A9f%C3%A9rences%20Utilisateur%20et%20Adaptation%20Central.md)
- [Miou - Base Culture Populaire](./Miou%20-%20Base%20Culture%20Populaire.md)
- [Bot - Connaissance Utilisateur et Specs Machine](./Bot/Bot%20-%20Connaissance%20Utilisateur%20et%20Specs%20Machine.md)
- [Bot - Registre Questions et Paliers d'Attachement](./Bot/Bot%20-%20Registre%20Questions%20et%20Paliers%20d'Attachement.md)

---

**Version :** 1.0  
**Statut :** Spécification onglet service et mode chatbot
