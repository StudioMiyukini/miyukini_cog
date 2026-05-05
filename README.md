# Miyukini COG

> Votre ecosysteme numerique souverain — un seul environnement pour gerer votre activite, vos evenements, vos finances, votre cloud et vos jeux. Sans abonnement cloud. Sans dependance externe. Vos donnees restent chez vous.

---

## A qui s'adresse Miyukini ?

| Vous etes... | Miyukini vous apporte |
|---|---|
| **Independant, auto-entrepreneur** | Comptabilite, devis, factures, suivi de tresorerie |
| **Association, collectif** | Calendrier partage, rendez-vous, contacts, documents |
| **Particulier** | Budget personnel, cloud prive, calendrier, jeux |
| **Equipe collaborative** | Suite bureautique souveraine (docs, sheets, slides, mail, message) |
| **Restaurateur, food truck** | Commande en ligne, reservation de tables, gestion de stock |
| **Developpeur curieux** | Architecture gouvernee en Rust, 70+ crates, moteur de jeu, IA locale |

---

## Ce que Miyukini fait concretement

### Services prets a l'emploi

| Service | Ce qu'il fait | Statut |
|---|---|---|
| **Miyukini Central** | Hub desktop — point d'entree unique vers tous vos services | Fonctionnel |
| **JayKoa** | Calendrier universel — vue jour, semaine, mois, planning | Fonctionnel |
| **JayKonta** | Budget personnel + comptabilite entreprise | Fonctionnel (Bourse) |
| **Miyukini Cloud** | Cloud prive avec fichiers, calendriers, contacts (WebDAV/CalDAV/CardDAV) | Fonctionnel |
| **Jay Bureau** | Suite collaborative (Docs, Sheets, Slides, Formulaire, Reunion, Club, Mail, Message) | Fonctionnel |
| **Alicia** | Assistante domotique — controle vocal, MQTT, automatisations | MVP |
| **MAIA** | IA locale (LLM, STT) — fonctionne sans internet | Fonctionnel |
| **Miyukini Whisper** | Dictee locale STT/TTS avec presets hardware | Fonctionnel |
| **MiyukiniWatch** | Tableau de bord metriques et habitudes | Fonctionnel |
| **MiyukiniClicker** | Jeu idle/clicker avec strategie et gestion de cite | Jouable |
| **MiyukiniSurvivor** | Jeu Survivor + Tower Defense — tours, troupes, vagues | Jouable |
| **MGE (Game Engine)** | Moteur de jeu multijoueur — simulation, crafting, pathfinding | En cours |

### Services en preparation

| Service | Ce qu'il fera |
|---|---|
| **JayRDV** | Prise de rendez-vous en ligne (B2B2C) |
| **JayFaim** | Reservation de tables et commande en ligne |
| **JayShop** | Boutique et point de vente unifies |
| **MiyukiniSales** | Cycle complet devis, commandes, facturation, paiements |
| **MIPowa** | Editeur de workflows visuels |
| **Jay1Tribu** | Reseau social prive pour groupes et familles |
| **JayManga** | Lecteur et bibliotheque manga/BD |

### Services retires

Voir [docs/services/DEPRECATED.md](docs/services/DEPRECATED.md) pour les details.

| Service | Date de retrait | Raison |
|---|---|---|
| **JayXpose** | 2026-04-29 | Sortie du perimetre — vitrine professionnelle non priorisee |
| **JayFestival** | 2026-04-29 | Sortie du perimetre — gestion de festivals non priorisee |

---

## Pourquoi choisir Miyukini ?

### Avantages et securite

| | Miyukini | SaaS classique (Google, Odoo...) |
|---|---|---|
| **Vos donnees** | Restent sur votre machine, chiffrees AES-256 | Stockees sur des serveurs tiers |
| **Connexion internet** | Facultative — tout fonctionne hors-ligne | Obligatoire en permanence |
| **Abonnement mensuel** | Aucun pour usage personnel | 5 a 50 euros/mois par service |
| **Nombre de services** | Illimite, tous integres dans un seul Hub | Un abonnement par service |
| **Vie privee** | Zero telemetrie, zero tracking | Profilage publicitaire |
| **Mise a jour** | Vous decidez quand et si vous mettez a jour | Imposee par l'editeur |
| **Interoperabilite** | Export iCal, WebDAV, CalDAV, CardDAV | Formats proprietaires |
| **Performance** | Application native Rust — rapide, legere | Application web — lente, gourmande |
| **Personnalisation** | Theme clair/sombre, interface modulaire | Limitee au plan paye |
| **IA integree** | LLM local, reconnaissance vocale, sans cloud | Dependante de services tiers |

### Securite en detail

| Couche | Protection |
|---|---|
| **Donnees au repos** | Base chiffree AES-256-CBC (libSQL + SQLCipher) |
| **Mots de passe** | Hachage Argon2id (resistant aux attaques GPU) |
| **Sessions admin** | Tokens JWT avec expiration |
| **Connexions reseau** | TLS 1.2+ obligatoire (rustls) |
| **Acces aux donnees** | Chaque service n'accede qu'a ses propres donnees |
| **Ecritures** | Systeme WriteIntent — toute modification est auditee |
| **Code** | Rust — zero `unsafe`, pas de failles memoire |
| **Rate limiting** | Protection anti-abus sur tous les endpoints |
| **Firewall / SELinux** | Configures en production |
| **RGPD** | Donnees locales = conformite native |

---

## Comment ca marche (en bref)

```
Vous
  |
  v
Miyukini Central (le Hub)
  |
  +---> JayKoa         (votre calendrier)
  +---> JayKonta       (vos finances)
  +---> Jay Bureau     (vos documents collaboratifs)
  +---> Miyukini Cloud (vos fichiers)
  +---> MAIA           (votre IA locale)
  +---> Jeux           (vos loisirs)
  +---> ...
```

Vous lancez **une seule application**. Elle vous donne acces a tous vos services. Chaque service garde son propre etat, meme en arriere-plan. Pas besoin de navigateur, pas besoin de compte cloud, pas besoin d'internet.

---

## Pour les curieux — L'architecture

Miyukini n'est pas un simple logiciel. C'est un **environnement gouverne** (COG — Core-Orchestrated Governance) organise en strates, comme un pays numerique :

| Strate | Role | En langage simple |
|---|---|---|
| **Kernel** | Fondations techniques | L'electricite et la plomberie |
| **Cores** (x9) | Gouvernance du systeme | Les ministeres qui fixent les regles |
| **Toolkits** (x49) | Capacites metier | La boite a outils universelle |
| **Services** | Ce que vous utilisez | Les guichets ouverts au public |
| **Admin** | Autorite souveraine | Le bureau du president |

**Principes fondateurs :**

- **Autonomie** — fonctionne sans reseau, sans cloud, sans service tiers
- **Souverainete** — vos donnees, vos regles, votre environnement
- **Gouvernance** — chaque composant a un role strict, rien n'est implicite
- **Federation** — plusieurs COG peuvent echanger comme des pays souverains, par "diplomatie" (visas, passeports, ponts)

### Chiffres cles

```
  9 Cores de gouvernance
 49 Toolkits implementes (crates Rust)
 70+ crates dans le workspace
 13 services et applications (dont 9 dans la suite Jay Bureau)
  2 jeux jouables
  1 moteur de jeu (MGE)
  1 assistante domotique (Alicia)
  1 IA locale (MAIA — LLM + STT)
  1 cloud prive (Miyukini Cloud)
```

---

## Vibe Coding — Un projet pilote par IA

Miyukini COG est l'un des plus gros projets entierement concu en **Vibe Coding** — pilotage integral par agents IA (Claude, GPT-4, Gemini), sous supervision humaine.

L'auteur n'est pas codeur de formation. Il definit l'intention, la vision et les contraintes. L'IA genere, structure et implemente. Pour garder la coherence a cette echelle, le projet a invente ses propres protocoles :

| Protocole | Role |
|---|---|
| **MSCM** | Balisage semantique dans le code — chaque bloc porte son identite |
| **MIP** | Index structurel global — l'IA navigue le projet via un graphe JSON |
| **Skills IA** | Instructions normatives pour les agents — conventions, architecture, glossaire |

> *"Documenter d'abord. Indexer pour que l'IA navigue. Implementer en parallele. Auditer. Puis passer au suivant."*

---

## Technologies

| Composant | Technologie |
|---|---|
| Langage principal | **Rust** (zero unsafe, performance native) |
| Interface desktop | **Dioxus** (UI reactive, pur Rust) |
| Base de donnees | **libSQL** + chiffrement AES-256 |
| IA locale | **llama.cpp** (LLM) + **Whisper** (STT) |
| Cloud prive | **WebDAV / CalDAV / CardDAV** |
| Domotique | **MQTT** + HTTP + automatisations |
| Reseau | **TLS 1.2+** (rustls), protocole binaire custom |
| Serveur Origin | **Relay + Tracker + Web + Admin** (multi-port) |

---

## Demarrage rapide

```bash
# Cloner le depot
git clone <repo-url> miyukini-cog
cd miyukini-cog

# Compiler et lancer le Hub
cargo build -p miyukini-central-native
cargo run -p miyukini-central-native
```

Prerequis : [Rust](https://rustup.rs/) (edition 2021, version 1.75+).

---

## Licence

- **Usage personnel** (personne physique, non commercial) : **gratuit**
- **Usage professionnel** (entreprise, association, administration) : **licence commerciale requise**

Details : [Politique de licence](docs/legal/Miyukini%20-%20Politique%20de%20Licence.md)

---

> *"Miyukini n'est pas un logiciel. C'est un environnement gouverne dans lequel vos services operent — souverainement, localement, sans compromis."*

**Version** : 0.1.0
**Derniere mise a jour** : 2026-05-05
