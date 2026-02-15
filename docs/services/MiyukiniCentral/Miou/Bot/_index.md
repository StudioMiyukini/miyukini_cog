# Bot Miou — Documentation exhaustive

## Contexte

Le **Bot** (ou **Proto-IA**) est la **première couche d'intelligence** de Miou. Il constitue le moteur déterministe qui génère toutes les bulles de Miou : accueil, rappels, suggestions, félicitations, notifications. Il est **toujours actif** dès la première connexion, sans dépendance à un LLM ni à des ressources significatives.

Le Bot repose sur trois piliers :
1. **Banque de templates** — phrases pré-écrites organisées par catégorie et variante.
2. **Moteur de décision** — règles de priorité, conditions d'évaluation, sélection de la bulle à afficher.
3. **Intégration données** — agrégats MiyukiniWatch, profil utilisateur, contexte applicatif (JayKoa, JayXpose, MWS, Jay1Tribu).

## Statut

| Attribut | Valeur |
|----------|--------|
| **Nature** | Composant interne de Miou (sous-service de Miyukini Central) |
| **Type** | Moteur à règles déterministe |
| **Dépendances** | MiyukiniWatch (métriques), profil utilisateur, services applicatifs (JayKoa, JayXpose, Jay1Tribu, MWS) |
| **Ressources** | CPU négligeable, RAM < 10 Mo |

## Documents

| Document | Description | Lignes |
|----------|-------------|--------|
| [Bot - Document Fondateur et Architecture](./Bot%20-%20Document%20Fondateur%20et%20Architecture.md) | Vision, mission, principes, architecture technique complète, composants, flux, cycle de vie, contraintes, relation écosystème, spécifications techniques, interfaces, tests, extensions. | 600+ |
| [Bot - Banque de Templates](./Bot%20-%20Banque%20de%20Templates.md) | Structure des templates, syntaxe, toutes les catégories et variantes, variables, exemples de sortie, règles de sélection par catégorie, localisation, variantes contextuelles, templates Rite/Connexion. | 600+ |
| [Bot - Moteur de Décision et Règles](./Bot%20-%20Moteur%20de%20Decision%20et%20Regles.md) | Algorithme de décision, graphe de priorité, conditions d'évaluation, anti-répétition, gestion des états, sélection de variantes, conflits, seuils configurables, edge cases, diagrammes. | 600+ |
| [Bot - Intégration et Flux de Données](./Bot%20-%20Integration%20et%20Flux%20de%20Donnees.md) | Intégration MiyukiniWatch, profil, contexte applicatif, format des agrégats, construction du contexte, variables injectables, sécurité, schémas, API internes. | 600+ |
| [Bot - Intelligence et Personnalité de Miou](./Bot%20-%20Intelligence%20et%20Personnalite%20de%20Miou.md) | Personnalité structurée, états émotionnels contextuels, adaptation au ton, règles anti-culpabilisation, cohérence narrative. | 200+ |
| [Bot - Catalogue Complet des Triggers](./Bot%20-%20Catalogue%20Complet%20des%20Triggers.md) | Tous les déclencheurs (temporels, événementiels, conditionnels), ordre d'évaluation, exclusions, cooldowns. | 250+ |
| [Bot - Banque de Templates Volume 2](./Bot%20-%20Banque%20de%20Templates%20Volume%202.md) | Catégories enrichies : accueil nuit, streaks, retours, encouragement, jalons, observations, variantes additionnelles. | 400+ |
| [Bot - Capacités Avancées et Jalons](./Bot%20-%20Capacites%20Avancees%20et%20Jalons.md) | Jalons complets, streaks, observation contextuelle, personnalisation par service favori, mapping agrégats. | 200+ |
| [Bot - Connaissance Utilisateur et Specs Machine](./Bot%20-%20Connaissance%20Utilisateur%20et%20Specs%20Machine.md) | Specs machine (RAM, stockage, OS), demandes/commentaires, taquinerie, curiosité, stockage chiffré des réponses utilisateur. | 200+ |
| [Bot - Registre Questions et Paliers d'Attachement](./Bot%20-%20Registre%20Questions%20et%20Paliers%20d'Attachement.md) | Paliers (inconnue → grande sœur), registre de questions par palier, confirmation relation. | 250+ |

## Arborescence

```
docs/services/MiyukiniCentral/Miou/Bot/
├── _index.md
├── Bot - Document Fondateur et Architecture.md
├── Bot - Banque de Templates.md
├── Bot - Banque de Templates Volume 2.md
├── Bot - Moteur de Decision et Regles.md
├── Bot - Integration et Flux de Donnees.md
├── Bot - Intelligence et Personnalite de Miou.md
├── Bot - Catalogue Complet des Triggers.md
├── Bot - Capacites Avancees et Jalons.md
├── Bot - Connaissance Utilisateur et Specs Machine.md
└── Bot - Registre Questions et Paliers d'Attachement.md
```

## Concepts clés

| Concept | Description |
|---------|-------------|
| **Template** | Phrase pré-écrite avec variables (ex. `{pseudo}`, `{jours}`). Remplie dynamiquement par le moteur. |
| **Catégorie** | Regroupement de templates par type de message (accueil matin, pause santé, rappel ami, etc.). |
| **Variante** | Une des phrases possibles dans une catégorie. Anti-répétition : piocher une variante non utilisée récemment. |
| **Moteur de décision** | Évalue les conditions (MiyukiniWatch, profil, contexte) et choisit la catégorie puis la variante à afficher. |
| **Contexte** | Agrégat des données disponibles au moment de la génération (session, services, amis, événements, badges). |
| **Invariant** | Le Bot ne lit jamais de contenu (messages, saisies, fichiers) sauf les réponses explicites aux questions de Miou — stockées localement, chiffrées. |
| **Déclencheur** | Événement ou condition qui lance une tentative de génération. |
| **Jalon** | Étape significative (streak, premier service, badge) déclenchant une félicitation. |
| **État émotionnel** | Contexte (accueillant, tendre, célébrant) qui adapte le ton de Miou. |
| **Palier d'attachement** | Niveau de relation (inconnue → … → grande sœur), confirmé par l'utilisateur. |

## Voir aussi

- [Miou — Documentation complète](../_index.md)
- [Miou - Moteur de Génération Templates et LLM](../Miou%20-%20Moteur%20de%20Generation%20Templates%20et%20LLM.md)
- [Miou - Proto-IA Scan et Consentement LLM](../Miou%20-%20Proto-IA%20Scan%20et%20Consentement%20LLM.md)
- [MiyukiniWatch — Document Fondateur](../../../MiyukiniWatch/MiyukiniWatch%20-%20Document%20Fondateur.md)

---

*Bot Miou : première couche d'intelligence, toujours active, déterministe, au service de la relation entre Miou et l'utilisateur.*

*Dernière mise à jour : 2026-02-15*
