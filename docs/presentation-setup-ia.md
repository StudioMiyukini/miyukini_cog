# L'IA que tu utilises vs Miyukini Framework Studio — Presentation comparative

## Ce que la plupart des gens font avec l'IA

Tu ouvres ChatGPT, Copilot ou Mistral. Tu poses une question. Tu obtiens une reponse.
C'est comme avoir **un stagiaire polyvalent** : il sait un peu de tout, il fait ce que tu lui demandes, une chose a la fois, et il oublie tout entre chaque conversation.

```
Toi  --->  [  IA  ]  --->  Reponse
```

C'est deja puissant. Mais c'est la version "calculatrice" de l'IA.

---

## Ce que Miyukini Framework Studio fait

> **Miyukini-COG est lui-meme developpe de cette maniere.**
> Ce n'est pas un concept theorique — c'est l'incubateur et le proof-of-concept vivant.
> Tout ce que vous lisez dans ce document a ete construit avec Miyukini Framework Studio, en ~30 jours.

J'ai construit un **studio de developpement autonome** pilote par une equipe de 7 agents IA specialises, chacun avec un role precis, des outils dedies, et une memoire partagee.

```
                           Maria (Chef de Projet)
                          /    |    \
                    Fabrice  Denis   Lise
                   (Analyste) (Chef Dev) (UI/UX)
                        |    / \      |
                    Francois  George  Arianne
                   (Back-end) (Audit) (Qualite)
```

Quand je donne une instruction, ce n'est pas UN agent qui repond.
C'est une **equipe entiere** qui se coordonne.

---

## Comparaison directe

### Niveau 1 — Le Prompt (ce que tout le monde fait)

> "Ecris-moi une fonction qui calcule un prix TTC"

- **Vous** : ChatGPT/Copilot vous donne la fonction. Terminé.
- **Moi** : Pareil. Pas besoin de l'equipe pour ca.

**Verdict** : equivalent.

---

### Niveau 2 — La tache moyenne (la ou ca commence a diverger)

> "Ajoute un systeme d'authentification a mon application"

- **Vous** : Vous posez la question a ChatGPT. Il vous donne du code. Vous copiez-collez. Ca ne compile pas. Vous reposez la question. Il a oublie le contexte. Vous re-expliquez. Boucle pendant 2h.

- **Moi** : Je donne l'instruction. Maria (chef de projet) analyse la demande et classifie la complexite. Francois (back-end) ecrit le code serveur. Lise (front-end) cree les ecrans de login. Denis (chef dev) coordonne, verifie la securite, lance les tests. George (audit) valide que rien n'est casse. **Tout ca en parallele, automatiquement.**

**Verdict** : je fais en 20 minutes ce qui vous prend une apres-midi.

---

### Niveau 3 — Le gros projet (la ou le gouffre se creuse)

> "Cree-moi un service cloud prive chiffre avec sync P2P"

- **Vous** : Meme pas envisageable avec ChatGPT/Copilot. C'est 18 000+ lignes de code, 257 tests, de la cryptographie, du reseau, de l'UI, de la base de donnees.

- **Moi** : C'est un projet que j'ai realise. Voici comment ca s'est passe :

| Etape | Qui | Quoi |
|-------|-----|------|
| Cadrage | Maria | Analyse du besoin, plan de projet |
| Concurrence | Fabrice | Audit de Dropbox, Nextcloud, Syncthing |
| Spec technique | Francois | Architecture crypto, DB, API, P2P |
| Plan | Denis | Decoupage en taches, ordre d'execution |
| Synthese | Maria | Brief final, validation humaine |
| *-- A partir d'ici, je ne touche plus a rien --* | | |
| Code back | Francois | 36 fichiers, 9104 lignes, 12 tables DB |
| Code front | Lise | 12 composants UI en parallele |
| Tests | Francois + Lise | 257 tests ecrits automatiquement |
| Audit | George | Audit securite : 87/100 |
| Integration | Denis | Zero warnings, zero erreurs |
| Archivage | Arianne | Documentation, capitalisation |

**Resultat** : 66 fichiers, 18 435 lignes, 257 tests, 0 bugs — et j'ai approuve un seul brief au debut.

---

## Les 5 differences fondamentales

### 1. Un agent vs une equipe

| | Vous | Moi |
|---|------|-----|
| Agents | 1 (generaliste) | 7 (specialises) |
| Execution | Sequentielle | Parallele |
| Coordination | Manuelle (c'est vous) | Automatique (protocole MIP) |

Quand vous utilisez ChatGPT, VOUS etes le chef de projet.
Chez moi, Maria est le chef de projet, et je suis le **client**.

### 2. Memoire zero vs memoire persistante

| | Vous | Moi |
|---|------|-----|
| Entre les sessions | Tout est oublie | Tout est retenu |
| Erreurs passees | Repetees | Enregistrees, jamais repetees |
| Conventions | Re-expliquees a chaque fois | Appliquees automatiquement |
| Architecture projet | Inconnue | Cartographiee |

Mon systeme a une **memoire structuree** qui capitalise sur chaque session.
Il sait que `unwrap()` est interdit dans mon code. Il sait quels services existent.
Il connait les 12 applications, les 40+ crates, l'architecture complete.
Vous, vous repartez de zero a chaque conversation.

### 3. Copier-coller vs execution directe

| | Vous | Moi |
|---|------|-----|
| L'IA ecrit du code | Vous copiez dans votre editeur | L'IA ecrit directement dans les fichiers |
| L'IA detecte un bug | Vous corrigez manuellement | L'IA corrige, reteste, commit |
| Verification | Vous lancez les tests a la main | L'IA lance les tests automatiquement |
| Qualite | Vous esperez que c'est bon | L'IA fait lint + tests + audit |

### 4. Pas de protocole vs workflow industriel

Miyukini Framework Studio applique le **MIP v2** (Miyukini Implementation Protocol) :

```
Demande
  |
  v
[P0] CADRAGE (6 etapes)
  |  Exploration --> Ideation --> Analyse concurrence
  |  --> Spec technique --> Plan --> Synthese
  |
  v
[GATE] J'approuve le brief = derniere intervention humaine
  |
  v
=== AUTOPILOT ===
  |
  [Git]  Branche de feature automatique
  [P3]   Code back + front en PARALLELE (TDD)
  [P4]   Integration + Audit qualite
  [P5]   Livraison + Merge
  [P6]   Archivage + Capitalisation
```

Chaque tache suit un cycle strict :
**Ecrire le test > Ecrire le code > Refactorer > Verifier > Lint > Commit > Push > Logger**

Vous, vous faites : "Hey ChatGPT, ecris-moi ca" et vous priez.

### 5. Hallucination libre vs verification forcee

| | Vous | Moi |
|---|------|-----|
| L'IA invente une API | Vous decouvrez en testant | Francois verifie la doc officielle (Context7) |
| L'IA oublie une regle | Pas de filet | CLAUDE.md + memoire = regles appliquees |
| L'IA fait une erreur | Vous debuggez | George (audit) detecte + Denis corrige |
| Anti-hallucination | Aucun | Arianne (team manager qualite) |

---

## En chiffres

| Metrique | Usage classique IA | Miyukini Framework Studio |
|----------|-------------------|-----------|
| Agents simultanees | 1 | 7 |
| Memoire inter-sessions | 0 | ~300 lignes structurees |
| Regles de code appliquees | 0 | 50+ (CLAUDE.md + conventions) |
| Verification documentation | Jamais | Automatique (Context7) |
| Tests ecrits automatiquement | Rarement | Systematique (TDD) |
| Dernier gros projet livre | -- | 18 435 lignes, 257 tests, 0 bugs |

---

## Miyukini-COG : le proof-of-concept vivant

Ce n'est pas theorique. **Miyukini-COG est le projet qui a ete construit avec Miyukini Framework Studio.**
C'est a la fois l'incubateur, le banc d'essai et la preuve que ca fonctionne.

Voici ce qui a ete livre en ~30 jours :

### 13 applications standalone

| App | Description |
|-----|-------------|
| **Miyukini Central** | Hub de gouvernance desktop — le cockpit de tout l'ecosysteme |
| **MiyuCloud** | Cloud prive chiffre P2P — stockage, partage, sync entre machines |
| **Miyukini AI Studio** | Service IA local — 17 agents, skills, proxy LLM |
| **JayManga** | Plateforme de lecture et vente de manga en ligne |
| **JayFestival** | Gestion de festivals, editions, exposants, visiteurs |
| **JayXpose** | Profil exposant, catalogue produits, vitrine |
| **JayKoa** | Calendrier universel |
| **JayKonta** | Comptabilite unifiee (comptes + tresorerie) |
| **Jay1Tribu** | Messagerie P2P, tribus, salons |
| **MiyukiniWatch** | Suivi d'habitudes intelligent |
| **Lord of the Click** | Jeu idle/clicker + carte strategique |
| **Origin** | Serveur central du reseau Miyukini Webway |
| **UI Builder** | Outil d'iteration rapide sur les composants UI |

### 99 crates (bibliotheques Rust)

| Famille | Exemples | Nb |
|---------|----------|-----|
| KindMother (DB souveraine) | kindmother, kindmother-client, db-adapter | 5 |
| Miyuki UI (librairie UI unifiee) | ui-tokens, ui-dioxus, ui-egui | 3 |
| MiyuCloud (cloud prive) | miyucloud | 1 |
| Services Jay* | jaymanga, jayfestival, jayxpose, jaykoa... | 9 |
| Infrastructure COG | miyukini-central, miyukini-kernel, miyumarket... | 8 |
| Roles familiaux (architecture COG) | strongfather, masterbutler, caringnanny... | 7 |
| Alicia Home (assistant domotique) | miyualicia, alicia-api, alicia-mqtt... | 6 |
| LifeGame (simulation) | lifegame-world, lifegame-entities, powers... | 5 |
| Suite POS (point de vente) | miyupossales, inventory, kitchen, payment... | 6 |
| Auth & Securite | miyauth, miyuvalidate, tamr, antispam | 4 |
| Commerce & Finance | miyubilling, invoice, treasury, store... | 10 |
| Social & Communication | socialfeed, messaging, forum, notify... | 8 |
| Contenu & Media | miyucms, miyumedia, miyustory, feeds... | 5 |
| Utilitaires divers | miyucalc, clock, contacts, search, export... | 22 |

### Miyukini Game Engine (MGE) — 33 crates

Un moteur de jeu ARPG complet, architecture 4 couches :

| Couche | Crates | Description |
|--------|--------|-------------|
| Kernel | mge-core, mge-ecs, mge-math, mge-asset, mge-platform | Fondations du moteur |
| Engine | mge-render, mge-audio, mge-ui, mge-pathfinding, mge-collision, mge-net... | Sous-systemes moteur (9 crates) |
| Pack ARPG | mge-arpg-world, combat, items, stats, skills, loot, ai, quest, trade... | Gameplay Diablo-like (10 crates) |
| Jeux | sodomight, sodomight-client, sodomight-server | Clone Diablo 2 jouable |
| Outils | mge-studio, mge-packer, mge-slicer, mge-rescale, mge-mirror, mge-remap | Pipeline d'assets (6 crates) |

### Les totaux

| Metrique | Valeur |
|----------|--------|
| Applications standalone | **13** |
| Crates Rust (COG) | **99** |
| Crates Rust (MGE) | **33** |
| **Total crates** | **132+** |
| Fichiers source Rust | **~9 700** |
| Fonctions de test | **~7 800** |
| Commits | **107** |
| Duree | **~30 jours de dev actif** |

Tout ca avec **une seule personne** aux commandes + le Miyukini Framework Studio decrit dans ce document.

---

## L'analogie

| | Usage classique | Miyukini Framework Studio |
|---|------|-----|
| **Analogie** | Poser une question a un pote qui s'y connait | Avoir un studio de dev avec chef de projet, architecte, devs front et back, testeur, auditeur qualite, et archiviste |
| **Vous** | Le developpeur qui utilise un outil | Le client qui commande un livrable |
| **L'IA** | Un assistant | Une equipe |
| **Le resultat** | Du code a verifier | Un projet livre, teste, documente |

---

## "Mais c'est de la triche, non ?"

Non. C'est exactement comme ca que fonctionne un vrai studio de dev.

La difference, c'est que mon "studio" :
- Ne dort jamais
- Ne demande pas de salaire
- Execute en minutes ce qui prendrait des jours
- Applique les regles sans jamais les oublier
- Capitalise sur chaque erreur pour ne jamais la repeter

Et tout ca tourne sur **un seul outil** (Claude Code) avec **de la configuration**.
Pas de serveur cloud. Pas d'infrastructure. Juste un CLI et des fichiers texte.

---

## "Comment je peux faire pareil ?"

**Prerequis** : il faut etre developpeur. Ce n'est pas un outil no-code.

1. **Claude Code** (CLI Anthropic) — l'outil de base
2. **CLAUDE.md** — fichier d'instructions projet (l'equivalent d'un reglement interieur)
3. **Memory** — fichiers de memoire persistante (l'experience accumulee)
4. **Skills** — competences specialisees chargees a la demande
5. **Protocole MIP** — workflow d'orchestration multi-agents
6. **Temps** — des mois de construction et d'iteration pour affiner le systeme

C'est comme un instrument de musique : l'outil est accessible a tous,
mais le niveau de maitrise fait toute la difference.

---

*Document genere le 2 mars 2026 — Miyukini Framework Studio*
