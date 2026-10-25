# Présentation Générale de Miyukini COG

## Qu'est-ce que Miyukini ?

**Miyukini COG** (Core-Orchestrated Governance Environment) est un environnement numérique **souverain** : il réunit vos services (agenda, comptabilité, documents, cloud privé, domotique, IA locale) sur votre propre matériel, sans abonnement cloud et sans dépendance externe critique.

Miyukini n'est **pas** un système d'exploitation. C'est un **environnement de gouvernance** qui tourne *sur* un OS : chaque action y est décidée, persistée, surveillée et tracée par des composants dédiés, les **Cores**.

### Vision en une phrase

> **Un environnement gouverné où vos services fonctionnent chez vous, hors ligne, sous votre contrôle — et où l'IA propose mais ne décide jamais.**

### Où en est le projet (octobre 2026)

- **Version 0.9.0-beta.1** : version de **recette**, à installer sur une machine dédiée pour être testée
- **Conception gelée** (doc conceptuelle v1) : toute évolution passe par un **amendement écrit avant le code** (21 à ce jour)
- **134 crates Rust**, **30 services et applications**, **3 857 tests automatisés**
- **Pas encore** : certification, audit externe, production

## Le principe : l'IA propose, les règles décident

```
   Vous / un service / MAIA
              │  intention
              ▼
      ┌───────────────┐
      │ BondingBrother│  passerelle : traduit, filtre
      └───────┬───────┘
              ▼
      ┌───────────────┐   contraintes   ┌──────────────┐
      │ StrongFather  │◄────────────────│ WorrySentinel│◄── CaringNanny
      │   DÉCIDE      │                 │  sécurité    │    (observe)
      └──┬─────────┬──┘                 └──────────────┘
   ALLOW │         │ DENY ──► refus tracé
         ▼         │
  ┌─────────────┐  └─ cas critique ──► TAMR (validation humaine)
  │ KindMother  │
  │SEUL ÉCRIVAIN│  bases chiffrées, journal d'audit chaîné
  └─────────────┘
```

- Aucun service, et pas même l'IA, n'écrit directement : il émet une **intention**.
- **StrongFather** décide, avec un **refus par défaut** ; **KindMother** est le **seul** à écrire.
- Les décisions sensibles exigent une **validation humaine** encadrée par **TAMR**.

## Les 10 Cores

| Core | Rôle | Ne fait jamais |
|------|------|----------------|
| **StrongFather** | Décide (autorise / refuse) | N'exécute ni ne persiste |
| **KindMother** | Persiste, seul validateur des écritures | Ne décide pas |
| **BondingBrother** | Passerelle : traduit, filtre, transmet | — |
| **BorderGuard** | Définit frontières et niveaux de confiance | Ne filtre pas lui-même |
| **WorrySentinel** | Gouverne la sécurité (T0 à T4) | Ne lance aucune vérification |
| **CaringNanny** | Observe l'état | Ne modifie rien |
| **MasterButler** | Registre des capacités et permissions | — |
| **LogisticsSteward** | Arbitre ressources et quotas | Ne mesure pas (le Kernel mesure) |
| **EverBuddy** | Cycle de vie, historique immuable | — |
| **TAMR** | Cadre de l'intervention humaine | — |

## Les strates

| Strate | Nom | Rôle |
|--------|-----|------|
| **A** | MiyukiniAdmin | Opérateur souverain, en zone COG uniquement, jamais exposé |
| **S** | Services / Opérateurs | Ce que vous utilisez ; émettent des intentions |
| **T** | Toolkits | Capacités techniques composables, cataloguées par MasterButler |
| **C** | Cores | Les 10 Cores de gouvernance |
| **K** | Kernel | Socle neutre : configuration, identité, temps, journal, cycle de vie |
| **H** | Classes de services H0–H3 | Degré d'exposition d'un service (de « jamais exposé » à « actif vers l'extérieur ») |

## Les zones : rien n'entre sans contrôle, rien ne sort de la zone COG

```
  EXTÉRIEUR          Internet · Fédération (autres COG) · Origin (canal officiel)
      │
      ▼
  ┌────────┐
  │ DOUANE │  pare-feu nftables, IDS/IPS, journaux bornés
  └───┬────┘
      ▼
  ┌──────────────────────────────┐
  │ ZONE FRANCHE                 │  façades web, DB de session,
  │                              │  quarantaine + désarmement des fichiers (CDR)
  └──────────────┬───────────────┘
                 ▼
            ┌─────────┐
            │ PORTAIL │  voies déclarées uniquement
            └────┬────┘
                 ▲  la zone COG TIRE ce dont elle a besoin
  ┌──────────────┴───────────────┐
  │ ZONE COG — aucune sortie     │  Cores, services, MAIA,
  │                              │  bases principales chiffrées
  └──────────────────────────────┘
```

- La **zone COG ne sort jamais** : une tentative de sortie est bloquée et déclenche une **alerte critique**.
- Les **Vigies** surveillent chaque voie ; un capteur muet ou une anomalie **coupe la voie** (fail-closed).
- Chaque service tourne sous son propre compte, avec un **profil AppArmor** en mode enforce, et n'a droit qu'aux flux locaux qu'il déclare.

## Identité et confiance

- Chaque COG possède une **racine** (clé hors ligne) ; personnes et appareils reçoivent un **passeport**, et un **visa** pour accéder à un service.
- Tous les titres sont des **Sceaux** signés ; les révocations sont publiées dans des **registres Merkle** vérifiables.
- **Aucun secret partagé** là où une signature suffit.
- Les mises à jour arrivent par un **canal officiel** signé (2 sur 2) ; c'est vous qui les relevez.

## Les lois d'autonomie

1. **LOI-1** : aucune dépendance externe critique à l'exécution
2. **LOI-2** : l'isolement est un état normal
3. **LOI-3** : l'état local est souverain
4. **LOI-4** : pas de temps global requis
5. **LOI-5** : le coût est proportionnel au matériel
6. **LOI-6** : l'autonomie n'empêche pas la fédération, qui reste explicite

## MAIA, l'IA locale

- Modèles **locaux**, adaptés au matériel ; aucune donnée envoyée à un service d'IA externe.
- **Mémoire chiffrée par utilisateur**, jamais publiée ni synchronisée ; **oubli réel** à la demande (effacement physique vérifié).
- **Fiches fédérées** : MAIA peut s'appuyer sur des connaissances d'autres COG, marquées étrangères et corroborées ; elle ne publie que ce que **vous** avez étiqueté et signé.
- **Bornée** : MAIA ne décide rien ; elle propose, les Cores décident.

## Services intégrés

| Domaine | Services |
|---------|----------|
| Hub | **Miyukini Central** |
| Organisation | **JayKoa** (calendrier universel), **JayRDV** (rendez-vous), **MiyukiniWatch** |
| Finances | **JayKonta**, **MiyukiniSales** |
| Collaboratif | **Jay Bureau** (Docs, Sheets, Slides, Formulaire, Réunion, Club, Mail, Message), **Jay1Tribu** |
| Fichiers | **MiyuCloud** (cloud privé), **JayCloud** (sauvegardes) |
| IA | **MAIA**, **Miyukini Whisper** |
| Domotique | **Alicia** |

## Fédération

```
  Candidat ──(≥ 2 membres confirment)──► Invité ──(≥ 4 membres, 7 jours)──► Membre
                                                                           │
                       recours accepté ◄── Suspendu ◄── accusation au quorum de 6
                                              │
                                              └── 72 h sans recours ──► Révoqué
```

- Les COG se reconnaissent entre eux ; une faute se **prouve** par des preuves signées, jamais par simple réputation.
- La Fédération fonctionne **sans Origin** ; un COG peut aussi rester **seul**, sans réseau.

## Et ensuite : les Alliances (conception v2)

Pour les organisations : une **Alliance** est une fédération **privée** de COG, administrée par un **centre de commandement** de 6 à 7 COG au quorum (tolérant aux membres compromis), avec un annuaire « AD maison » (**Alliance-Admin**) et une IA d'administration bornée (**HECTOR**). Voir la [conception des Alliances](../../architecture/alliance/_index.md).

## Pour qui ?

- **Indépendants et professions réglementées** : agenda, rendez-vous, comptabilité et IA locale, sans sortir les données de l'entreprise
- **TPE, PME, associations, petites collectivités** : un seul serveur, budget maîtrisé, données en local
- **Équipes** : suite bureautique souveraine sans cloud tiers
- **Particuliers** : cloud privé, budget, domotique, assistant vocal local
- **Développeurs** : architecture gouvernée en Rust, contrats explicites, sécurité vérifiable

## Ce que Miyukini n'est pas

| Ce n'est pas | C'est |
|--------------|-------|
| Un OS | Un **environnement de gouvernance** qui tourne *sur* un OS |
| Un framework où chacun fait comme il veut | Un environnement **gouverné** : les Cores sont le socle non substituable |
| Un CMS ou un SaaS de plus | Un **écosystème** de services gouvernés, chez vous |
| Une IA qui agit à votre place | Une IA **bornée** qui propose ; les règles et vous décidez |

## Commencer

- 📐 **Architecture** : comprendre la structure du système
- ⚙️ **Cores** : les 10 piliers de gouvernance
- 🔧 **Outils** : les capacités techniques disponibles
- 📱 **Services** : les applications intégrées
- 🌐 **MWS** : le réseau de fédération
- 🔒 **Sécurité** : protection et confiance
