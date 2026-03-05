# Miou â€” Sous-service de Miyukini Central

## Contexte

**Miou** est l'**avatar et mascotte** de tous les COGs Miyukini. Elle est un **sous-service de Miyukini Central** : toujours prÃ©sente, elle constitue le lien Ã©motionnel et bienveillant entre l'utilisateur et son environnement COG.

Miou n'est pas un gadget visuel. Elle est le **canal privilÃ©giÃ©** par lequel le COG s'adresse Ã  l'utilisateur : accueil, suggestions, rappels, fÃ©licitations, notifications, conversation. Son rÃ´le fondamental est la **santÃ©** de l'utilisateur, son **bien-Ãªtre Ã©motionnel et physique**, son **amusement**, et surtout de **nouer une relation sincÃ¨re et Ã©motionnelle**. L'utilisateur doit sentir que Miou lui veut du bien et souhaite l'aider.

## Statut

| Attribut | Valeur |
|----------|--------|
| **Nature** | Sous-service de Miyukini Central |
| **Type de service** | Service interne COG (Type 1) â€” intÃ©grÃ© Ã  Central, pas de surface web propre |
| **PrÃ©sence** | Toujours active dÃ¨s que l'utilisateur est connectÃ© dans Central |
| **DÃ©pendances** | MiyukiniWatch (mÃ©triques), profil utilisateur, contexte applicatif (JayKoa, JayXpose, MWS, Jay1Tribu...) |

## Documents

| Document | Description |
|----------|-------------|
| [Miou - Document Fondateur](./Miou%20-%20Document%20Fondateur.md) | IdentitÃ©, personnalitÃ©, mission, relation avec l'utilisateur, principes fondateurs. |
| [Miou - Systeme de Bulles et UI](./Miou%20-%20Systeme%20de%20Bulles%20et%20UI.md) | Canal de communication (bulles en bas Ã  droite), placement, forme, comportement, file d'attente, dismiss. |
| [Miou - Moteur de Generation Templates et LLM](./Miou%20-%20Moteur%20de%20Generation%20Templates%20et%20LLM.md) | Architecture : Proto-IA (Bot + templates, toujours active) + LLM local optionnel. Construction du prompt, modÃ¨les compatibles, lazy loading. |
| [Miou - Architecture LLM Cloud Inter-COG](./Miou%20-%20Architecture%20LLM%20Cloud%20Inter-COG.md) | Ã‰volution vers un Service Inter-COG (Type 3) : COG dÃ©diÃ© GPU (Mistral 7B via Ollama), protocole MiouCloudRequest/Response, flux MWS, dÃ©gradation gracieuse (LOI-1/LOI-2), souverainetÃ© Ã©tendue au rÃ©seau COG personnel. |
| [Miou - Proto-IA Scan et Consentement LLM](./Miou%20-%20Proto-IA%20Scan%20et%20Consentement%20LLM.md) | Scan des specs au premier lancement, proposition de consentement (bulle Miou), refus â†’ relance aprÃ¨s long dÃ©lai, toggle manuel, transparence non intrusive. |
| [Bot â€” Documentation exhaustive](./Bot/_index.md) | PremiÃ¨re couche d'intelligence : Document Fondateur, Banque de Templates (Vol. 1 et 2), Moteur de DÃ©cision, IntÃ©gration, Intelligence et PersonnalitÃ©, Catalogue Triggers, CapacitÃ©s AvancÃ©es. 9 documents. |
| [Miou - Gamification et Progression](./Miou%20-%20Gamification%20et%20Progression.md) | Badges, Ã©tapes, progression visible, ton positif, pas de punition. |
| [Miou - Voix et Audio](./Miou%20-%20Voix%20et%20Audio.md) | SystÃ¨me de voix existant (Rite d'EntrÃ©e, Connexion), roadmap voix sur le Salon, cohÃ©rence sonore. |
| [Miou - IntÃ©gration TTS eSpeak](./Miou%20-%20Integration%20TTS%20eSpeak.md) | TTS eSpeak NG : moteur lÃ©ger, config activable/dÃ©sactivable, stratÃ©gie hybride MP3. LLM non implÃ©mentÃ© en 0.1.x. |
| [Miou - Guide UI UX](./Miou%20-%20Guide%20UI%20UX.md) | Principes UI/UX, design system, bulles, accessibilitÃ©, gamification, ParamÃ¨tres, do's/don'ts. |
| [Miou - Roadmap et AmÃ©liorations](_index.md) | AmÃ©liorations prioritaires : rÃ©utilisation des rÃ©ponses, mode DND, templates saisonniers, variantes selon palier. |
| [Miou - Onglet Service Mode Chatbot](./Miou%20-%20Onglet%20Service%20Mode%20Chatbot.md) | Onglet dÃ©diÃ©, interface type ChatGPT, transparence base de connaissance (voir/modifier/supprimer). |
| [Miou - Catalogue Exhaustif des Connaissances](./Miou%20-%20Catalogue%20Exhaustif%20des%20Connaissances.md) | Liste exhaustive des connaissances que Miou peut collecter : clÃ©s, sources, catÃ©gories, schÃ©ma. |
| [Miou - PrÃ©fÃ©rences Utilisateur et Adaptation Central](./Miou%20-%20Pr%C3%A9f%C3%A9rences%20Utilisateur%20et%20Adaptation%20Central.md) | GoÃ»ts, prÃ©fÃ©rences, humeur â€” adaptation du thÃ¨me, frÃ©quence et orientation des actions Miou. |
| [Miou - Base Culture Populaire](./Miou%20-%20Base%20Culture%20Populaire.md) | Base de rÃ©fÃ©rences pop culture pour blagues et espiÃ¨glerie, organisÃ©e par thÃ¨me et gÃ©nÃ©ration (1990â€“2020). |
| [Miou - Protocole MIP-Miou et MÃ©moire LLM](./Miou%20-%20Protocole%20MIP-Miou%20et%20Memoire%20LLM.md) | Archivage et indexation des donnÃ©es chatbot, table Miou LLM Memory, tags et catÃ©gories pour retrieval LLM. |
| [Miou - Guide Implementation Complet](./Miou%20-%20Guide%20Implementation%20Complet.md) | Guide exhaustif : pÃ©rimÃ¨tre 0.1.x, architecture, composants, donnÃ©es, ordre de livraison, critÃ¨res d'acceptation. |

## Arborescence

```
docs/services/MiyukiniCentral/Miou/
â”œâ”€â”€ _index.md
â”œâ”€â”€ Miou - Document Fondateur.md
â”œâ”€â”€ Miou - Architecture LLM Cloud Inter-COG.md
â”œâ”€â”€ Miou - Systeme de Bulles et UI.md
â”œâ”€â”€ Miou - Moteur de Generation Templates et LLM.md
â”œâ”€â”€ Miou - Proto-IA Scan et Consentement LLM.md
â”œâ”€â”€ Miou - Gamification et Progression.md
â”œâ”€â”€ Miou - Voix et Audio.md
â”œâ”€â”€ Miou - Guide UI UX.md
â”œâ”€â”€ Miou - Roadmap et AmÃ©liorations.md
â”œâ”€â”€ Miou - Onglet Service Mode Chatbot.md
â”œâ”€â”€ Miou - Catalogue Exhaustif des Connaissances.md
â”œâ”€â”€ Miou - PrÃ©fÃ©rences Utilisateur et Adaptation Central.md
â”œâ”€â”€ Miou - Base Culture Populaire.md
â”œâ”€â”€ Miou - Protocole MIP-Miou et Memoire LLM.md
â”œâ”€â”€ Miou - Integration TTS eSpeak.md
â”œâ”€â”€ Miou - Guide Implementation Complet.md
â”œâ”€â”€ data/
â”‚   â””â”€â”€ miou_popculture_db.json
â””â”€â”€ Bot/
    â”œâ”€â”€ _index.md
    â”œâ”€â”€ Bot - Document Fondateur et Architecture.md
    â”œâ”€â”€ Bot - Banque de Templates.md
    â”œâ”€â”€ Bot - Moteur de Decision et Regles.md
    â”œâ”€â”€ Bot - Integration et Flux de Donnees.md
    â”œâ”€â”€ Bot - Intelligence et Personnalite de Miou.md
    â”œâ”€â”€ Bot - Catalogue Complet des Triggers.md
    â”œâ”€â”€ Bot - Banque de Templates Volume 2.md
    â””â”€â”€ Bot - Capacites Avancees et Jalons.md
```

## Concepts clÃ©s

| Concept | Description |
|---------|-------------|
| **Sous-service** | Miou vit Ã  l'intÃ©rieur de Miyukini Central ; elle n'a pas son propre onglet dans la TabBar mais dispose de son propre overlay (bulles). |
| **Bulles** | Canal de communication principal, affichÃ©es **en bas Ã  droite** de l'Ã©cran Central. |
| **MiyukiniWatch** | Service silencieux fournissant les mÃ©triques d'usage (sessions, services, amis, clics) sans lire les contenus. |
| **Proto-IA + LLM** | Proto-IA (Bot) = premiÃ¨re couche, toujours active (templates + rÃ¨gles). LLM = couche complÃ©mentaire optionnelle si specs suffisantes et consentement utilisateur. |
| **Relation sincÃ¨re** | Objectif : l'utilisateur doit sentir que Miou le connaÃ®t, lui veut du bien, et n'est pas lÃ  pour vendre ni surveiller. |
| **Scan specs + consentement** | Ã€ la premiÃ¨re connexion, Miou scanne l'environnement ; si specs suffisantes, elle propose le LLM. L'utilisateur accepte ou refuse. Toggle manuel dans ParamÃ¨tres. Transparence accessible sans Ãªtre intrusive. |

## Voir aussi

- [MiyukiniWatch â€” Document Fondateur](../../MiyukiniWatch/MiyukiniWatch%20-%20Document%20Fondateur.md)
- [Miyukini Central â€” Salon propositions](../Miyukini%20Central%20-%20Salon%20propositions%20lieu%20de%20vie%20gamification%20Miou.md)
- [Types de Services et Espaces](..//..//..//miyukini-webway-system//reference//_index.md)

---

*Miou : avatar des COGs, sous-service de Miyukini Central, au service de la santÃ©, du bien-Ãªtre et d'une relation sincÃ¨re avec l'utilisateur.*

*DerniÃ¨re mise Ã  jour : 2026-02-15*


