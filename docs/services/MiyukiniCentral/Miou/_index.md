# Miou — Sous-service de Miyukini Central

## Contexte

**Miou** est l'**avatar et mascotte** de tous les COGs Miyukini. Elle est un **sous-service de Miyukini Central** : toujours présente, elle constitue le lien émotionnel et bienveillant entre l'utilisateur et son environnement COG.

Miou n'est pas un gadget visuel. Elle est le **canal privilégié** par lequel le COG s'adresse à l'utilisateur : accueil, suggestions, rappels, félicitations, notifications, conversation. Son rôle fondamental est la **santé** de l'utilisateur, son **bien-être émotionnel et physique**, son **amusement**, et surtout de **nouer une relation sincère et émotionnelle**. L'utilisateur doit sentir que Miou lui veut du bien et souhaite l'aider.

## Statut

| Attribut | Valeur |
|----------|--------|
| **Nature** | Sous-service de Miyukini Central |
| **Type de service** | Service interne COG (Type 1) — intégré à Central, pas de surface web propre |
| **Présence** | Toujours active dès que l'utilisateur est connecté dans Central |
| **Dépendances** | MiyukiniWatch (métriques), profil utilisateur, contexte applicatif (JayKoa, JayXpose, MWS, Jay1Tribu...) |

## Documents

| Document | Description |
|----------|-------------|
| [Miou - Document Fondateur](./Miou%20-%20Document%20Fondateur.md) | Identité, personnalité, mission, relation avec l'utilisateur, principes fondateurs. |
| [Miou - Systeme de Bulles et UI](./Miou%20-%20Systeme%20de%20Bulles%20et%20UI.md) | Canal de communication (bulles en bas à droite), placement, forme, comportement, file d'attente, dismiss. |
| [Miou - Moteur de Generation Templates et LLM](./Miou%20-%20Moteur%20de%20Generation%20Templates%20et%20LLM.md) | Architecture : Proto-IA (Bot + templates, toujours active) + LLM local optionnel. Construction du prompt, modèles compatibles, lazy loading. |
| [Miou - Architecture LLM Cloud Inter-COG](./Miou%20-%20Architecture%20LLM%20Cloud%20Inter-COG.md) | Évolution vers un Service Inter-COG (Type 3) : COG dédié GPU (Mistral 7B via Ollama), protocole MiouCloudRequest/Response, flux MWS, dégradation gracieuse (LOI-1/LOI-2), souveraineté étendue au réseau COG personnel. |
| [Miou - Proto-IA Scan et Consentement LLM](./Miou%20-%20Proto-IA%20Scan%20et%20Consentement%20LLM.md) | Scan des specs au premier lancement, proposition de consentement (bulle Miou), refus → relance après long délai, toggle manuel, transparence non intrusive. |
| [Bot — Documentation exhaustive](./Bot/_index.md) | Première couche d'intelligence : Document Fondateur, Banque de Templates (Vol. 1 et 2), Moteur de Décision, Intégration, Intelligence et Personnalité, Catalogue Triggers, Capacités Avancées. 9 documents. |
| [Miou - Gamification et Progression](./Miou%20-%20Gamification%20et%20Progression.md) | Badges, étapes, progression visible, ton positif, pas de punition. |
| [Miou - Voix et Audio](./Miou%20-%20Voix%20et%20Audio.md) | Système de voix existant (Rite d'Entrée, Connexion), roadmap voix sur le Salon, cohérence sonore. |
| [Miou - Intégration TTS eSpeak](./Miou%20-%20Integration%20TTS%20eSpeak.md) | TTS eSpeak NG : moteur léger, config activable/désactivable, stratégie hybride MP3. LLM non implémenté en 0.1.x. |
| [Miou - Guide UI UX](./Miou%20-%20Guide%20UI%20UX.md) | Principes UI/UX, design system, bulles, accessibilité, gamification, Paramètres, do's/don'ts. |
| [Miou - Roadmap et Améliorations](./Miou%20-%20Roadmap%20et%20Améliorations.md) | Améliorations prioritaires : réutilisation des réponses, mode DND, templates saisonniers, variantes selon palier. |
| [Miou - Onglet Service Mode Chatbot](./Miou%20-%20Onglet%20Service%20Mode%20Chatbot.md) | Onglet dédié, interface type ChatGPT, transparence base de connaissance (voir/modifier/supprimer). |
| [Miou - Catalogue Exhaustif des Connaissances](./Miou%20-%20Catalogue%20Exhaustif%20des%20Connaissances.md) | Liste exhaustive des connaissances que Miou peut collecter : clés, sources, catégories, schéma. |
| [Miou - Préférences Utilisateur et Adaptation Central](./Miou%20-%20Pr%C3%A9f%C3%A9rences%20Utilisateur%20et%20Adaptation%20Central.md) | Goûts, préférences, humeur — adaptation du thème, fréquence et orientation des actions Miou. |
| [Miou - Base Culture Populaire](./Miou%20-%20Base%20Culture%20Populaire.md) | Base de références pop culture pour blagues et espièglerie, organisée par thème et génération (1990–2020). |
| [Miou - Protocole MIP-Miou et Mémoire LLM](./Miou%20-%20Protocole%20MIP-Miou%20et%20Memoire%20LLM.md) | Archivage et indexation des données chatbot, table Miou LLM Memory, tags et catégories pour retrieval LLM. |
| [Miou - Guide Implementation Complet](./Miou%20-%20Guide%20Implementation%20Complet.md) | Guide exhaustif : périmètre 0.1.x, architecture, composants, données, ordre de livraison, critères d'acceptation. |

## Arborescence

```
docs/services/MiyukiniCentral/Miou/
├── _index.md
├── Miou - Document Fondateur.md
├── Miou - Architecture LLM Cloud Inter-COG.md
├── Miou - Systeme de Bulles et UI.md
├── Miou - Moteur de Generation Templates et LLM.md
├── Miou - Proto-IA Scan et Consentement LLM.md
├── Miou - Gamification et Progression.md
├── Miou - Voix et Audio.md
├── Miou - Guide UI UX.md
├── Miou - Roadmap et Améliorations.md
├── Miou - Onglet Service Mode Chatbot.md
├── Miou - Catalogue Exhaustif des Connaissances.md
├── Miou - Préférences Utilisateur et Adaptation Central.md
├── Miou - Base Culture Populaire.md
├── Miou - Protocole MIP-Miou et Memoire LLM.md
├── Miou - Integration TTS eSpeak.md
├── Miou - Guide Implementation Complet.md
├── data/
│   └── miou_popculture_db.json
└── Bot/
    ├── _index.md
    ├── Bot - Document Fondateur et Architecture.md
    ├── Bot - Banque de Templates.md
    ├── Bot - Moteur de Decision et Regles.md
    ├── Bot - Integration et Flux de Donnees.md
    ├── Bot - Intelligence et Personnalite de Miou.md
    ├── Bot - Catalogue Complet des Triggers.md
    ├── Bot - Banque de Templates Volume 2.md
    └── Bot - Capacites Avancees et Jalons.md
```

## Concepts clés

| Concept | Description |
|---------|-------------|
| **Sous-service** | Miou vit à l'intérieur de Miyukini Central ; elle n'a pas son propre onglet dans la TabBar mais dispose de son propre overlay (bulles). |
| **Bulles** | Canal de communication principal, affichées **en bas à droite** de l'écran Central. |
| **MiyukiniWatch** | Service silencieux fournissant les métriques d'usage (sessions, services, amis, clics) sans lire les contenus. |
| **Proto-IA + LLM** | Proto-IA (Bot) = première couche, toujours active (templates + règles). LLM = couche complémentaire optionnelle si specs suffisantes et consentement utilisateur. |
| **Relation sincère** | Objectif : l'utilisateur doit sentir que Miou le connaît, lui veut du bien, et n'est pas là pour vendre ni surveiller. |
| **Scan specs + consentement** | À la première connexion, Miou scanne l'environnement ; si specs suffisantes, elle propose le LLM. L'utilisateur accepte ou refuse. Toggle manuel dans Paramètres. Transparence accessible sans être intrusive. |

## Voir aussi

- [MiyukiniWatch — Document Fondateur](../../MiyukiniWatch/MiyukiniWatch%20-%20Document%20Fondateur.md)
- [Miyukini Central — Salon propositions](../Miyukini%20Central%20-%20Salon%20propositions%20lieu%20de%20vie%20gamification%20Miou.md)
- [Types de Services et Espaces](../../../reference/Miyukini%20Conceptual%20References%20-%20Types%20de%20Services%20et%20Espaces.md)

---

*Miou : avatar des COGs, sous-service de Miyukini Central, au service de la santé, du bien-être et d'une relation sincère avec l'utilisateur.*

*Dernière mise à jour : 2026-02-15*
