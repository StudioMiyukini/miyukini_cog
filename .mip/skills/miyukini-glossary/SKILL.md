---
name: miyukini-glossary
description: Terminologie officielle et glossaire normatif de l'ecosysteme Miyukini COG. Utiliser quand on travaille sur du code, de la documentation, des noms de variables, des commentaires, ou toute communication impliquant la terminologie Miyukini. Inclut les correspondances termes interdits/corrects, les 8 Cores (Strate 4), BondingBrother (Strate 5), termes obsoletes (KeeperOfStorage -> KindMother), MWS (Origin, Relay, Tracker, Permis de circulation, accord d'hôte, tunnel etendu), types de COG (ORIGIN, RELAY, TRACKER, STABLE, SPECIAL, TERMINAL, LONE), types d'OS (WINDOWS, LINUX, MACOS, ANDROID, IOS), Operateurs, Outils, Services, et les phrases fondatrices.
---

# Glossaire Miyukini COG

## Regle fondamentale

Toujours utiliser la terminologie officielle. Pour le glossaire complet, voir [references/glossaire-complet.md](references/glossaire-complet.md).

## Correspondance terminologique rapide

| Terme INTERDIT | Terme CORRECT |
|----------------|---------------|
| Produit | **Operateur** |
| App | **Operateur** ou **Operateur d'Interface** |
| Creer un produit | **Deployer un Operateur** |
| Utiliser une app | **Interagir avec un Operateur** |
| Marketplace | **Registre d'Operateurs** |
| Super-Operateur | **Equipe d'Operateurs** |
| Tool | **Outil** |
| Toolkit | **Kit d'Outils** |
| Operator | **Operateur** |
| Decision Window | **Mandat de Permission** |
| Fast Path | **Chemin Mandate** |
| Hub / Dashboard | **Miyukini Central** (pour utilisateur COG) |
| Web Portal / Public Portal | **Miyukini Web Portal** ou **Portail** (pour utilisateurs externes) |
| Service Type 1 | **Service interne COG** |
| Service Type 2 | **Service a surface web externe** |
| Service Type 3 | **Service Inter-COG** |

## Nomenclature des noms

| Prefixe | Signification |
|---------|---------------|
| `MiyuXxx` | Toolkit concu par Miyukini (Strate 6) |
| `MiyukiniOpsXxx` | Operateur concu par Miyukini (Strate 7) |
| `MiyukiniXxx` | Service concu par Miyukini |
| `JayXxx` | Service officiel de la famille "Jay" |

## Les 8 Cores (Strate 4)

| Core | Domaine | Question fondamentale |
|------|---------|-----------------------|
| StrongFather | Decision strategique | "Devrait-on faire cette action ?" |
| KindMother | Donnees et persistance | "Comment les donnees sont-elles persistees ?" |
| Caring Nanny | Observation d'etat | "Dans quel etat est le systeme ?" |
| Master Butler | Capacites et permissions | "Qu'est-ce qui est possible ?" |
| Border Guard | Frontieres et confiance | "Ou sont les frontieres ?" |
| Ever Buddy | Cycle de vie et evolution | "Comment evoluer sans rompre ?" |
| WorrySentinel | Gouvernance de securite | "Quel niveau de securite ?" |
| TAMR | Intervention humaine | "Quand l'humain intervient-il ?" |

**Regle invariante :** Les Cores decident ou gouvernent, mais n'executent jamais.

**Terme obsolete (ne plus utiliser) :** KeeperOfStorage — remplacer par **KindMother** (domaine donnees et persistance, integrite du stockage).

## BondingBrother (Strate 5)

**Core de mediation** (hors les 8 Cores de gouvernance Strate 4). Traduit les intentions des Operateurs vers les Cores et les reponses vers les resultats. Pas d'autorite ; mediation uniquement. Question : *"Comment traduire cette intention pour les autorites ?"*

## Hierarchie des concepts

- **Service** = Capacite percue par l'utilisateur
- **Operateur** = Entite fonctionnelle gouvernee qui execute
- **Outil** = Capacite executable gouvernee, sans autorite
- **Kit d'Outils** = Composition officielle d'Outils

## Services Fondamentaux

Les Services Fondamentaux font partie de l'environnement versionne du COG.

| Service | Role | Cible |
|---------|------|-------|
| **Miyukini Central** | Hub de gestion des Services | Utilisateur du COG |
| **Miyukini Web Portal** (Portail) | Hub des surfaces web | Utilisateurs externes (web) |

**Regle canonique :** Central = COG, Portail = Web.

## Types de Services

| Type | Nom | Description | Espaces |
|------|-----|-------------|---------|
| **1** | Service interne COG | Aucune surface externe | Central uniquement |
| **2** | Service a surface web externe | Gestion + surface web | Central + Portail |
| **3** | Service Inter-COG | Interactions entre COGs | Central + Protocoles Inter-COG |

**Regle :** Tout Service doit declarer son type et prevoir les espaces correspondants.

## Phrases fondatrices cles

> "Dans Miyukini, les utilisateurs n'installent pas d'applications. Ils interagissent avec des Operateurs gouvernes."

> "Un Outil fait, mais ne decide jamais."

> "La complexite est geree par la collaboration, pas par l'accumulation."

> "Un Mandat de Permission n'est pas une optimisation. C'est un acte de gouvernance delegue."

> "Central = COG, Portail = Web."

> "Les Services Fondamentaux font partie de l'environnement versionne du COG."

> "Tout Service doit declarer son type et prevoir les espaces correspondants."

## Miyukini Webway System (MWS) — termes cles

Le MWS est la couche de **presence, decouverte et transport** des COGs sur le reseau. Racine documentaire : `docs/miyukini-webway-system`.

| Terme | Signification |
|-------|---------------|
| **Origin** | Point d'origine du MWS ; cumule les fonctions relay et tracker ; source de verite unique. |
| **Relay** | Duplication d'Origin ; verification de conformite (Passeport COG, cle Cores, blocs MIP) ; delivrance du **Permis de circulation** (accord relay). |
| **Tracker** | Douanier du reseau ; controle d'identite et **controle tracker** (verification du Permis) ; pools par version des Cores ; catalogue web (port 80). |
| **Permis de circulation** | Accord emis par un relay (ou Origin) apres verification conforme ; autorise la connexion au maillage via les trackers officiels. |
| **Accord d'hôte** | Autorisation delivree par le COG Hebergeur pour acceder a ses ressources (Lobby, services) ; distinct du Permis de circulation. |
| **Visa de Connexion** | Synonyme courant de accord d'hôte (Connexion Inter-COG). |
| **COG participant** | COG qui se declare sur le Webway, expose des surfaces (Lobbys), decouvre les autres. |
| **COG Tracker** | COG qui endosse le role Tracker (port 21000) ; devoir de protection du reseau (systemes passifs et actifs). |
| **Tunnel etendu multi-tenant** | Relay de transport : les COGs s'enregistrent avec un token et un `cog_id`, le relay route le trafic vers le bon tunnel. |
| **Passeport COG** | Document presente au relay : cog_id, cog_type, os_type, core_version, services, environment_health, previous_permis ; base de la verification en 3 phases (cle Cores, blocs Services, sante). |

### Types de COG (`cog_type`)

| Type | Description |
|------|-------------|
| **ORIGIN** | Point central de verite unique du MWS ; une seule adresse IP et/ou URL. |
| **RELAY** | COG de controle d'integrite ; duplication d'Origin ; verification 3 phases. |
| **TRACKER** | Mapping et controle ; douanier du reseau ; pools par version. |
| **STABLE** | COG d'utilisateur commun ; usage personnel ou professionnel standard. |
| **SPECIAL** | COG professionnel a forte utilisation reseau et/ou services larges. |
| **TERMINAL** | COG embarque mobile ; enfant d'un COG Stable du meme utilisateur. |
| **LONE** | COG structurellement et volontairement isole du reseau. |

### Types d'OS (`os_type`)

| Type | Description |
|------|-------------|
| **WINDOWS** | Microsoft Windows (10, 11, Server) |
| **LINUX** | Distributions Linux (Ubuntu, Debian, Fedora, etc.) |
| **MACOS** | Apple macOS |
| **ANDROID** | Google Android (pour COGs TERMINAL) |
| **IOS** | Apple iOS (pour COGs TERMINAL) |

**Regle MWS :** Un COG ne doit se connecter qu'aux **trackers officiels** (liste remise avec le Permis de circulation par le relay).

## References

*Chemins `docs/` relatifs a la racine du workspace.*

- **Glossaire (skill)** : [references/glossaire-complet.md](references/glossaire-complet.md)
- **Glossaire officiel** : `docs/reference/Miyukini Conceptual References - Glossaire.md`
- **Types de Services** : `docs/reference/Miyukini Conceptual References - Types de Services et Espaces.md`
- **Miyukini Central** : `docs/reference/Miyukini Conceptual References - Miyukini Central Hub Services.md`
- **Miyukini Web Portal** : `docs/services/MiyukiniWebPortal/`
- **MWS (Webway)** : `docs/miyukini-webway-system/` — Document Fondateur : `MWS - Document Fondateur.md` ; index : `reference/_index.md` ; acteurs : `acteurs/` (Origin, Relays, Trackers)