# Miou — Base Culture Populaire

Miou possède une **base de données sur la culture populaire** pour pouvoir **blaguer** ou être **espiègle** avec les bonnes références. Les références sont organisées par **thème** et par **génération** (décennie, de 1990 à 2020). En fonction de l'âge de l'utilisateur, certaines références sont plus pertinentes ; d'autres peuvent servir de **source de culture** et de découverte.

---

## 1. Contexte et objectifs

| Objectif | Description |
|----------|-------------|
| **Blagues et espièglerie** | Miou utilise des références connues pour taquiner, faire sourire, créer des ponts |
| **Pertinence par âge** | Filtrage par génération — éviter les références « trop vieilles » ou « pas encore connues » |
| **Source de culture** | Certaines références peuvent être proposées comme découverte pour les plus jeunes |

**Phrase fondatrice :**

> **Miou peut être espiègle et complice en citant la culture populaire — tout en respectant la génération de l'utilisateur.**

---

## 2. Organisation des données

### 2.1 Dimensions

| Dimension | Description | Valeurs |
|-----------|-------------|---------|
| **Thème** | Domaine culturel | Cinéma, Série TV, Musique, Jeux vidéo, Internet/Memes, Littérature, Publicité |
| **Génération** | Décennie de popularité | 1990–1999, 2000–2009, 2010–2020 |
| **Type** | Usage possible | Blague, Citation, Référence espiègle, Découverte |

### 2.2 Structure par décennie

| Décennie | Code | Période | Exemples de contexte |
|----------|------|---------|---------------------|
| 90s | `gen_1990` | 1990–1999 | Tamagotchi, Titanic, Windows 95, Spice Girls |
| 2000s | `gen_2000` | 2000–2009 | Harry Potter, Facebook, iPod, WoW |
| 2010s | `gen_2010` | 2010–2020 | Minecraft, Fortnite, Netflix, TikTok |

---

## 3. Thèmes détaillés

### 3.1 Cinéma

| Génération | Exemples de références |
|------------|------------------------|
| 1990–1999 | Titanic, Matrix, Jurassic Park, Toy Story, Forrest Gump, Lion King |
| 2000–2009 | Harry Potter, Shrek, Pirates des Caraïbes, Avatar, Inception |
| 2010–2020 | Avengers, Frozen, La La Land, Interstellar |

### 3.2 Série TV

| Génération | Exemples de références |
|------------|------------------------|
| 1990–1999 | Friends, X-Files, Buffy, Stargate |
| 2000–2009 | Lost, Breaking Bad, How I Met Your Mother, The Office |
| 2010–2020 | Game of Thrones, Stranger Things, Black Mirror |

### 3.3 Musique

| Génération | Exemples de références |
|------------|------------------------|
| 1990–1999 | Spice Girls, Nirvana, Britney Spears, Daft Punk |
| 2000–2009 | Lady Gaga, Coldplay, Rihanna, The Black Eyed Peas |
| 2010–2020 | Ed Sheeran, Billie Eilish, Dua Lipa |

### 3.4 Jeux vidéo

| Génération | Exemples de références |
|------------|------------------------|
| 1990–1999 | Super Mario 64, Pokémon, Tamagotchi, Tomb Raider |
| 2000–2009 | World of Warcraft, Minecraft (2009), Wii, GTA |
| 2010–2020 | Fortnite, Among Us, Zelda Breath of the Wild |

### 3.5 Internet et Mèmes

| Génération | Exemples de références |
|------------|------------------------|
| 1990–1999 | Dial-up, Napster, premiers forums |
| 2000–2009 | YouTube, Facebook, lolcats, « All your base » |
| 2010–2020 | TikTok, « C'est quand qu'on mange ? », challenges |

### 3.6 Littérature

| Génération | Exemples de références |
|------------|------------------------|
| 1990–1999 | Harry Potter (début), Da Vinci Code |
| 2000–2009 | Twilight, Hunger Games |
| 2010–2020 | Fifty Shades, séries YA |

### 3.7 Publicité et culture partagée

| Génération | Exemples |
|------------|----------|
| 1990–1999 | « Just do it », bande passante 56k |
| 2000–2009 | « Think different », « I'm lovin' it » |
| 2010–2020 | « Du coup », « Bref », références Netflix |

---

## 4. Filtrage par âge / génération

### 4.1 Principe

- **Entrée :** `date_naissance` ou `annee_generation` (décennie de naissance de l'utilisateur).
- **Règle :** Les références sont filtrées pour privilégier celles de la **génération de l'utilisateur** (± 1 décennie).
- **Découverte :** Les références « plus vieilles » peuvent être proposées comme culture (« Tu connais Titanic ? ») — optionnel.

### 4.2 Matrice de pertinence

| Âge utilisateur (année naissance) | Références prioritaires | Références secondaires | Découverte possible |
|----------------------------------|------------------------|------------------------|---------------------|
| ~1990 | gen_1990, gen_2000 | gen_2010 | — |
| ~2000 | gen_2000, gen_2010 | gen_1990 | gen_1990 |
| ~2010 | gen_2010 | gen_2000 | gen_1990, gen_2000 |

*(Exemple indicatif — à affiner selon la logique métier.)*

### 4.3 Absence de date de naissance

Si `date_naissance` n'est pas renseignée :
- Miou utilise un **mélange** des trois générations.
- Peut proposer dans le chatbot : « Tu es né(e) dans les années 90, 2000 ou 2010 ? Ça m'aide à choisir mes références. »
- Option : défaut « génération 2000 » (la plus large).

---

## 5. Usage dans les bulles et le chatbot

### 5.1 Catégories de bulles concernées

| Catégorie | Usage |
|-----------|-------|
| `taquinerie_innocente` | Référence pop pour taquiner |
| `curiosite_utilisateur` | « Tu préfères Harry Potter ou Hunger Games ? » |
| Réponse chatbot | Espièglerie, citation, clin d'œil |

### 5.2 Ton

- **Jamais condescendant** : pas de « tu es trop jeune pour connaître ».
- **Source de culture** : « Tu connais Titanic ? C'est un classique — je peux te le raconter. »
- **Complice** : « Comme disait Shrek : les oignons ont des couches. »

### 5.3 Exemples de répliques

| Contexte | Référence | Réplique possible |
|----------|-----------|-------------------|
| Pause après longue session | Minecraft | « Tu construis un château ou tu te déconnectes ? » |
| Retour après absence | Friends | « So no one told you life was gonna be this way… » (taquinerie) |
| Utilisateur fatigué | Napster / dial-up | « Tu te sens en 56k ce matin ? » |
| Confusion | Matrix | « Tu as choisi la pilule rouge, je me souviens. » |

---

## 6. Schéma de données (résumé)

```rust
// Entrée de la base culture
struct PopCultureEntry {
    id: String,
    theme: Theme,           // Cinema, Series, Music, Games, Internet, etc.
    generation: Generation,  // gen_1990, gen_2000, gen_2010
    title: String,          // "Titanic", "Friends"
    context: String,        // Contexte d'usage : "blague pause", "taquinerie"
    quote_or_hook: Option<String>,  // Citation ou accroche réutilisable
    tags: Vec<String>,      // ["amour", "long", "épique"]
}
```

---

## 7. Fichier de données

La base est alimentée par le fichier **`data/miou_popculture_db.json`** :

- Entrées par génération (`gen_1990`, `gen_2000`, `gen_2010`)
- Chaque entrée : `id`, `theme`, `title`, `year`, `quote`, `hooks[]`, `contexts[]`, `tags[]`
- Thèmes : cinema, series, music, games, internet, literature, culture

**Chemin :** `docs/services/MiyukiniCentral/Miou/data/miou_popculture_db.json`

---

## 8. Extension de la base

La base peut être **enrichie** avec :
- Une liste d'entrées par thème et par génération
- Des variantes de répliques (templates avec `{reference}`)
- Des règles d'injection (quand utiliser une référence)

**Référence technique :** Voir [Bot - Banque de Templates](./Bot/Bot%20-%20Banque%20de%20Templates%20Volume%202.md) pour l'intégration des variantes.

---

## 9. Références

- [Miou - Onglet Service Mode Chatbot](./Miou%20-%20Onglet%20Service%20Mode%20Chatbot.md)
- [Miou - Catalogue Exhaustif des Connaissances](./Miou%20-%20Catalogue%20Exhaustif%20des%20Connaissances.md)
- [Bot - Intelligence et Personnalité](./Bot/Bot%20-%20Intelligence%20et%20Personnalite%20de%20Miou.md)
- [Bot - Connaissance Utilisateur et Specs Machine](./Bot/Bot%20-%20Connaissance%20Utilisateur%20et%20Specs%20Machine.md)

---

**Version :** 1.0  
**Statut :** Spécification base culture populaire
