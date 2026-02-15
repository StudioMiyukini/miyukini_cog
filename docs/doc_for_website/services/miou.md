# Miou — Assistant IA

## Assistant Intelligent Intégré

**Miou** est l'assistant IA intégré à Miyukini Central. Il accompagne l'utilisateur dans l'utilisation du système et peut répondre à des questions, guider les tutoriels et interagir en mode conversationnel.

## Rôle

> Miou **accompagne** et **guide** l'utilisateur sans décider à sa place.

Miou aide à la découverte des services, aux tutoriels, au mode chatbot et à l'adaptation des préférences. Il reste gouverné par les Cores et ne modifie jamais les données sans validation.

## Fonctionnalités Principales

| Fonction | Description |
|----------|-------------|
| **Mode Chatbot** | Conversation naturelle, réponses contextuelles |
| **Tutoriels** | Accompagnement pas à pas (MWS, Central, etc.) |
| **Base culture pop** | Références et personnalité (pop culture) |
| **Préférences** | Adaptation aux habitudes utilisateur |
| **Voix / TTS** | Synthèse vocale (eSpeak) pour l’accessibilité |

## Architecture

```
┌─────────────────────────────────────────────────┐
│                     MIOU                         │
│  ┌─────────────┐ ┌─────────────┐ ┌────────────┐│
│  │   Bot /     │ │  Tutoriels  │ │  Voix TTS  ││
│  │   LLM       │ │  Moteur     │ │  eSpeak    ││
│  └─────────────┘ └─────────────┘ └────────────┘│
│  ┌─────────────┐ ┌─────────────┐                │
│  │ Connaissances│ │ Préférences │                │
│  │ (catalogue)  │ │ utilisateur │                │
│  └─────────────┘ └─────────────┘                │
└─────────────────────────────────────────────────┘
```

## Bot et Personnalité

- **Intelligence** : Règles, templates, et intégration LLM (protocole MIP-Miou, mémoire)
- **Personnalité** : Ton bienveillant, références culture pop, paliers d’attachement
- **Triggers** : Catalogue de déclencheurs (questions, intents, contexte)
- **Templates** : Banques de réponses et de scénarios

## Protocole MIP-Miou

- **MIP** : Index et balisage sémantique (MSCM)
- **Mémoire** : Contexte utilisateur, historique de conversation, spécifications machine
- **Consentement** : Proto-IA scan et consentement avant usage LLM

## Tutoriels

- Introduction au MWS (Webway) et connexion
- Introduction à Miyukini Central
- Registre des identifiants UI pour les tutoriels

## Catalogue et moteurs

- **Catalogue exhaustif des connaissances** : domaines couverts (système, services, culture pop, tutoriels)
- **Moteur de tutoriels** : étapes guidées, registre des identifiants UI, progression
- **Moteur de génération** : templates et LLM (protocole MIP-Miou), banques de templates (Volume 1 et 2)
- **Gamification et progression** : paliers d'attachement, registre des questions et réponses, jalons
- **Système de bulles et UI** : interface conversationnelle, design cohérent avec Central

## Sécurité et Gouvernance

- Miou ne modifie pas les données sensibles sans validation Cores/TAMR
- Données de conversation et préférences stockées localement (KindMother)
- Conformité avec les niveaux de sécurité et la vie privée
- **Consentement** : proto-IA scan et consentement utilisateur avant usage LLM
- Intégration TTS (eSpeak) pour l'accessibilité ; voix et audio documentés
