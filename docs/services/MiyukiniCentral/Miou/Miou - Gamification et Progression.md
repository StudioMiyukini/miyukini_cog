# Miou — Gamification et Progression

Système de gamification incarnée par Miou : badges, étapes de progression, ton positif. L'objectif est de renforcer l'engagement et le plaisir d'usage sans jamais culpabiliser.

---

## 1. Principes

| Principe | Description |
|----------|-------------|
| **Gamification positive uniquement** | Pas de punition, pas de niveau qui baisse, pas de perte, pas de streak perdue. Si l'utilisateur est absent 2 mois, il retrouve tout intact. |
| **Incarnée par Miou** | Les badges et étapes ne sont pas un tableau froid ; c'est Miou qui les annonce dans ses bulles (« Tu as débloqué… »). |
| **Discrète** | Les badges sont consultables mais ne s'imposent pas. Pas de popup forcée, pas de fanfare. |
| **Significative** | Chaque badge correspond à un usage réel du COG, pas à du remplissage (pas de « clique 1000 fois »). |

---

## 2. Badges

### 2.1 Catalogue des badges

| Badge | Icône | Condition de déblocage | Bulle Miou (exemple) |
|-------|-------|----------------------|----------------------|
| **Habitant installé** | 🏠 | Rite d'Entrée terminé (premier compte créé) | « Bienvenue chez toi — ton COG est prêt. » |
| **Première clé** | 🔑 | Première connexion réussie après le Rite | « Tu as franchi le seuil. Ton COG t'attend. » |
| **Fidèle (7 jours)** | 📅 | 7 jours distincts de connexion | « 7 jours avec ton COG — tu prends tes marques. » |
| **Assidu (30 jours)** | 🌟 | 30 jours distincts de connexion | « Un mois ensemble — c'est devenu une habitude. » |
| **Explorateur** | 🧭 | Avoir ouvert au moins 3 services différents | « Tu as exploré 3 coins de ton COG — curiosité récompensée. » |
| **Webway connecté** | 🌐 | Première connexion MWS réussie | « Ton COG est relié au monde — bienvenue sur le Webway. » |
| **Exposant actif** | 🏪 | Avoir créé un profil exposant dans JayXpose | « Ton profil exposant est en place — belle initiative. » |
| **Vitrine en ligne** | 🖼️ | Vitrine JayXpose publiée sur le web | « Ta vitrine est visible sur le web — le monde peut te découvrir. » |
| **Premier événement** | 📆 | Avoir créé un événement dans JayKoa | « Premier événement dans ton calendrier — ça se fête. » |
| **Ami connecté** | 💬 | Premier échange via Jay1Tribu | « Premier échange avec un ami — le Webway prend vie. » |
| **Cercle d'amis** | 👥 | 3 amis ajoutés dans Jay1Tribu | « 3 amis dans ton cercle — ton réseau grandit. » |
| **Souverain numérique** | 👑 | Tous les badges précédents débloqués | « Tous les badges réunis — tu es un vrai souverain numérique. » |

### 2.2 Règles

| Règle | Description |
|-------|-------------|
| **Permanents** | Une fois débloqué, un badge ne peut pas être perdu. |
| **Pas de compteur intrusif** | Pas de « 12/30 jours » affiché en permanence. L'utilisateur ne voit la progression que s'il la consulte. |
| **Annonce unique** | Miou annonce le badge une seule fois dans une bulle. L'utilisateur peut retrouver ses badges dans son profil. |
| **Pas de classement** | Pas de leaderboard, pas de comparaison avec d'autres COGs. La gamification est personnelle. |

---

## 3. Étapes de progression

Complémentaire aux badges, les étapes sont des **jalons narratifs** que Miou commente au fil du temps :

| Étape | Condition | Commentaire Miou |
|-------|-----------|------------------|
| **Emménagement** | Rite d'Entrée terminé | « Tu viens d'emménager — prends ton temps pour découvrir les lieux. » |
| **Premiers pas** | 3 sessions complétées | « Tu commences à trouver tes repères. » |
| **Routine installée** | 7 jours de connexion | « Ton COG fait partie de ta routine — j'aime ça. » |
| **Maison décorée** | Au moins 2 services utilisés régulièrement | « Tu as trouvé tes coins préférés. » |
| **Porte ouverte au monde** | Webway connecté + 1 ami | « Ton COG n'est plus seul — tu as ouvert la porte au monde. » |
| **Chez soi** | 30 jours + tous services explorés | « C'est chez toi maintenant. Vraiment. » |

Les étapes utilisent la **métaphore de la maison** (emménagement → routine → décoration → ouverture au monde → chez soi) en cohérence avec le vocabulaire Miyukini (« Habitant », « Rite d'Entrée », « Salon »).

---

## 4. Affichage des badges

### 4.1 Dans le profil utilisateur

Les badges débloqués apparaissent dans la fenêtre Profil (`profile_window`) :

- Rangée d'icônes avec tooltip au survol (nom + date de déblocage).
- Badges verrouillés grisés avec indication de la condition (optionnel, discret).

### 4.2 Dans les bulles Miou

Quand un badge est débloqué :

1. Miou affiche une bulle de type **Félicitation** (priorité basse, ne coupe pas un rappel important).
2. Texte : phrase personnalisée (voir tableau ci-dessus).
3. Icône du badge visible dans la bulle.
4. Bouton optionnel : « Voir mes badges ».

### 4.3 Dans le Salon

Option (futur) : zone « Mes étapes » dans le Salon, résumant la progression (petites icônes, discret, non intrusif).

---

## 5. Données nécessaires

Toutes les données de progression proviennent de **MiyukiniWatch** et de l'état des services :

| Donnée | Source |
|--------|--------|
| Nombre de jours de connexion | MiyukiniWatch (sessions) |
| Services ouverts / utilisés | MiyukiniWatch (services) |
| Connexion MWS réussie | État MWS dans Central |
| Profil exposant créé | JayXpose (état interne) |
| Vitrine publiée | JayXpose (état interne) |
| Événement créé | JayKoa (état interne) |
| Amis ajoutés / échanges | Jay1Tribu (métadonnées MiyukiniWatch) |

**Invariant :** La gamification ne lit aucun contenu. Uniquement des compteurs et des états (existe / n'existe pas).

---

## 6. Références

- [Miou - Document Fondateur](./Miou%20-%20Document%20Fondateur.md)
- [Miou - Système de Bulles et UI](./Miou%20-%20Systeme%20de%20Bulles%20et%20UI.md)
- [MiyukiniWatch — Document Fondateur](../../MiyukiniWatch/MiyukiniWatch%20-%20Document%20Fondateur.md)

---

*Gamification positive, incarnée par Miou. Pas de punition, pas de pression — seulement des petites victoires et une mascotte qui s'en réjouit.*
