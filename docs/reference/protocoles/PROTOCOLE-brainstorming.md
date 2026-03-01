# Protocole de Brainstorming — Services Miyukini COG

<!--
@id protocole.brainstorming
@do define_standard_brainstorming_process_for_new_services
@role governance
@layer reference
@human Protocole normalise pour le brainstorming de tout nouveau service dans l'ecosysteme Miyukini COG. Definit les phases, checklists, roles et balisage MSCM obligatoire.
-->

## Contexte

Ce document definit le **protocole standard de brainstorming** pour la conception de tout nouveau service dans l'ecosysteme Miyukini COG. Il garantit une approche structuree, reproductible et conforme aux standards du projet.

## Portee / Scope

- **Applicable a :** Tout nouveau service Miyukini (Strate 7) ou toolkit (Strate 6).
- **Audience :** Tous les agents de l'equipe projet.
- **Statut :** Standard officiel. Tout brainstorming de service doit suivre ce protocole.
- **Maintenu par :** Maria (Chef de Projet)

---

## 1. Vue d'ensemble du processus

Le brainstorming d'un nouveau service suit **5 phases sequentielles**. Chaque phase produit des livrables specifiques et se termine par une checklist de validation. Le passage a la phase suivante est conditionne par la validation de la checklist.

```
Phase 1         Phase 2        Phase 3            Phase 4         Phase 5
CADRAGE    -->  IDEATION  -->  PROFONDEUR    -->  DECISIONS  -->  SYNTHESE
(Maria)         (Equipe)       (Maria+Denis)      (Utilisateur)   (Maria)
    |               |              |                   |               |
    v               v              v                   v               v
BRAINSTORM-     BRAINSTORM-    BRAINSTORM-        BRAINSTORM-     Document
initial.md      initial.md     approfondissement  initial.md      Fondateur
(sections       (sections      .md                (section        (norme)
 1-4)            5-10)                             decisions)
```

### Balisage MSCM du processus

Chaque document de brainstorming doit porter le balisage MSCM suivant :

```
<!--
@id brainstorm.{nom_service}.{phase}
@do {description_fonctionnelle}
@role planning
@layer service
@human {description_humaine}
-->
```

---

## 2. Phase 1 — Cadrage

<!--
@id protocole.brainstorming.phase1
@do analyze_need_and_define_scope
@role planning
@layer reference
@human Phase de cadrage : comprendre le besoin, le public, les problemes a resoudre, et le positionnement du service.
-->

### Objectif

Comprendre le **pourquoi** du service. Qui en a besoin, quel probleme il resout, comment il se positionne dans l'ecosysteme.

### Responsable

**Maria** (Chef de Projet), avec l'utilisateur comme source d'information.

### Activites

1. Recevoir et analyser la demande de l'utilisateur
2. Identifier le public cible (primaire, secondaire, tertiaire)
3. Lister les problemes a resoudre
4. Definir le positionnement (ce que le service est / ce qu'il n'est pas)
5. Proposer un nom de service (nomenclature Jay* ou Miyukini*)
6. Formuler les questions ouvertes pour l'utilisateur

### Livrables

- Fichier `BRAINSTORM-initial.md` (sections 1 a 4 du template)
- Liste des questions ouvertes

### Balisage MSCM obligatoire

Le fichier doit contenir en en-tete :
```
<!--
@id brainstorm.{nom_service}.cadrage
@do analyze_need_for_{nom_service}
@role planning
@layer service
@human Brainstorming initial — cadrage du service {Nom}
-->
```

### Checklist de validation Phase 1

- [ ] Le public cible est identifie (au moins primaire et secondaire)
- [ ] Au moins 3 problemes concrets a resoudre sont listes
- [ ] Le positionnement est clair (ce que le service est / n'est pas)
- [ ] Un nom de service est propose (avec alternatives)
- [ ] Les questions ouvertes sont formulees (minimum 5)
- [ ] Le fichier suit le template normalise
- [ ] Le balisage MSCM est present

### Critere de passage

La Phase 1 est complete quand le fichier `BRAINSTORM-initial.md` (sections 1-4) est redige et que les questions ouvertes sont transmises a l'utilisateur.

---

## 3. Phase 2 — Ideation

<!--
@id protocole.brainstorming.phase2
@do brainstorm_features_and_mechanics
@role planning
@layer reference
@human Phase d'ideation : lister les fonctionnalites, imaginer les mecaniques, definir l'UX, l'architecture technique et les risques.
-->

### Objectif

Explorer le **quoi** et le **comment**. Lister les fonctionnalites, imaginer les mecaniques, definir les interfaces, evaluer les risques.

### Responsable

**Maria** (coordination), avec contributions de **Denis** (technique), **Lise** (UX), **Francois** (faisabilite back-end).

### Activites

1. Lister les fonctionnalites par priorite (MVP / v1.0 / futur)
2. Definir les mecaniques cles du service
3. Esquisser les interfaces utilisateur (ecrans principaux)
4. Positionner le service dans l'architecture COG (strate, cores, toolkits)
5. Identifier les risques et contraintes
6. Estimer les couts et ressources (fourchettes)

### Livrables

- Fichier `BRAINSTORM-initial.md` complet (sections 5 a 13 du template)

### Balisage MSCM obligatoire

Les sections techniques doivent contenir :
```
<!--
@id brainstorm.{nom_service}.architecture
@do define_cog_architecture_for_{nom_service}
@role architecture
@layer service
@human Architecture COG : strates, operateurs, toolkits, cores
-->
```

### Checklist de validation Phase 2

- [ ] Les fonctionnalites MVP sont listees avec priorites
- [ ] Les fonctionnalites v1.0 et futures sont separees
- [ ] Au moins 2 interfaces sont esquissees (ecrans principaux)
- [ ] Le service est positionne dans l'architecture COG :
  - [ ] Type de service identifie (Type 1, 2 ou 3)
  - [ ] Strate identifiee
  - [ ] Operateurs listes
  - [ ] Toolkits a creer et a reutiliser listes
  - [ ] Cores utilises identifies
- [ ] La structure des crates est esquissee
- [ ] Le schema DB est esquisse
- [ ] La conformite avec les 8 Lois d'Autonomie est verifiee
- [ ] Les risques sont identifies avec probabilite/impact/mitigation
- [ ] Les couts sont estimes en fourchettes (optimiste/pessimiste)
- [ ] La repartition par agent est definie

### Critere de passage

La Phase 2 est complete quand le fichier `BRAINSTORM-initial.md` est complet (toutes les sections) et les questions ouvertes sont en attente de reponse.

---

## 4. Phase 3 — Profondeur

<!--
@id protocole.brainstorming.phase3
@do deepen_key_axes_with_detailed_analysis
@role planning
@layer reference
@human Phase de profondeur : approfondir les axes cles (parcours utilisateur, contenu, technique, UX).
-->

### Objectif

Approfondir les axes cles pour preparer la redaction du Document Fondateur. Produire des specifications detaillees sur les parcours utilisateur, le format de donnees, les algorithmes, la narration, les flux.

### Responsable

**Maria** (coordination), avec **Denis** (validation technique).

### Prerequis

- Phase 2 complete
- Decisions de l'utilisateur sur les questions ouvertes (au moins les questions critiques : nom, scope MVP, cible, modele economique)

### Activites

Les axes a approfondir dependent du service. Voici les axes **standards** (obligatoires) et **optionnels** :

**Axes standards (obligatoires pour tout service) :**

1. **Parcours utilisateur detaille**
   - Premier lancement (onboarding)
   - Session type (utilisation quotidienne)
   - Cas limites (erreurs, absence, retour)

2. **Format de donnees**
   - Schema des entites principales (JSON/Rust)
   - Convention d'identifiants
   - Organisation du stockage

3. **Roadmap MVP**
   - Phases de developpement
   - Jalons et livrables par phase
   - Dependances entre phases
   - Criteres de succes

**Axes optionnels (selon le type de service) :**

4. **Contenu / catalogue** — si le service a du contenu a creer (exercices, articles, produits)
5. **Algorithme / moteur** — si le service a un moteur de traitement (adaptatif, recommandation, calcul)
6. **Narration / univers** — si le service a une dimension narrative (jeux, gamification)
7. **Flux parent/admin** — si le service a un mode administration ou supervision
8. **Integration inter-services** — si le service interagit avec d'autres services Jay*
9. **Protocole reseau** — si le service est Inter-COG (Type 3)

### Livrables

- Fichier `BRAINSTORM-approfondissement.md`
- Mise a jour de `BRAINSTORM-initial.md` avec les decisions validees

### Balisage MSCM obligatoire

```
<!--
@id brainstorm.{nom_service}.approfondissement
@do deepen_analysis_for_{nom_service}
@role planning
@layer service
@human Brainstorming approfondissement — axes detailles du service {Nom}
-->
```

Chaque axe approfondi doit avoir son propre balisage :
```
<!--
@id brainstorm.{nom_service}.axe.{nom_axe}
@do detail_{nom_axe}_for_{nom_service}
@role planning
@layer service
@human Approfondissement de l'axe {nom_axe}
-->
```

### Checklist de validation Phase 3

- [ ] Les decisions de l'utilisateur sont integrees dans le BRAINSTORM-initial.md
- [ ] Le parcours utilisateur est detaille (onboarding + session type + fin + retour)
- [ ] Le format de donnees est defini (schema, identifiants, stockage)
- [ ] La roadmap MVP est definie (phases, jalons, livrables, dependances)
- [ ] Les criteres de succes du MVP sont listes (mesurables)
- [ ] Les axes optionnels pertinents sont traites
- [ ] Le balisage MSCM est present sur chaque axe

### Critere de passage

La Phase 3 est complete quand le fichier `BRAINSTORM-approfondissement.md` est redige et que toutes les informations necessaires au Document Fondateur sont disponibles.

---

## 5. Phase 4 — Decisions

<!--
@id protocole.brainstorming.phase4
@do collect_and_record_user_decisions
@role planning
@layer reference
@human Phase de decisions : collecter les reponses de l'utilisateur aux questions ouvertes et les integrer dans les documents.
-->

### Objectif

S'assurer que toutes les questions ouvertes ont une reponse (ou sont explicitement differees) et que les decisions sont enregistrees.

### Responsable

**Maria** (collecte), **Utilisateur** (decision), **Arianne** (archivage).

### Activites

1. Presenter les questions ouvertes a l'utilisateur
2. Collecter les reponses
3. Mettre a jour le `BRAINSTORM-initial.md` avec une section "Decisions validees"
4. Identifier les questions non resolues et les classer :
   - **Bloquantes** : empechent le passage au Document Fondateur
   - **Non bloquantes** : peuvent etre resolues pendant le developpement
5. Archiver les decisions (Arianne)

### Livrables

- Section "Decisions validees" dans `BRAINSTORM-initial.md`
- Liste des questions non resolues classees par criticite

### Format de la section Decisions

```markdown
## Decisions validees par l'utilisateur ({date})

| Question | Decision | Date | Statut |
|----------|----------|------|--------|
| Nom du service | JayXxx | 2026-XX-XX | Definitif |
| Scope MVP | ... | ... | Definitif |
| ... | ... | ... | ... |
```

### Checklist de validation Phase 4

- [ ] Toutes les questions bloquantes ont une reponse
- [ ] Les decisions sont integrees dans le BRAINSTORM-initial.md
- [ ] Les questions non resolues sont classees (bloquante / non bloquante)
- [ ] Les decisions sont archivees par Arianne

### Critere de passage

La Phase 4 est complete quand **aucune question bloquante** ne reste ouverte.

---

## 6. Phase 5 — Synthese (Document Fondateur)

<!--
@id protocole.brainstorming.phase5
@do synthesize_into_founding_document
@role planning
@layer reference
@human Phase de synthese : rediger le Document Fondateur norme a partir des brainstorms.
-->

### Objectif

Produire le **Document Fondateur** norme du service, pret a etre transmis a Denis pour la documentation technique.

### Responsable

**Maria** (redaction), **Denis** (review technique), **Arianne** (archivage).

### Activites

1. Synthetiser les brainstorms en Document Fondateur
2. Appliquer la structure normee (voir section 8 de ce protocole)
3. Faire reviewer par Denis (coherence technique)
4. Faire archiver par Arianne
5. Transmettre a Denis pour la documentation technique

### Livrables

- `{NomService} - Document Fondateur.md` (dans `docs/services/{NomService}/`)
- Archivage par Arianne

### Balisage MSCM obligatoire

```
<!--
@id service.{nom_service}.fondateur
@do define_founding_vision_for_{nom_service}
@role governance
@layer service
@human Document Fondateur du service {Nom} — vision, perimetre, architecture
-->
```

### Checklist de validation Phase 5

- [ ] Le Document Fondateur suit la structure normee (section 8)
- [ ] Toutes les sections obligatoires sont presentes
- [ ] Le balisage MSCM est complet
- [ ] Le positionnement dans l'architecture COG est clair
- [ ] La conformite avec les Lois d'Autonomie est confirmee
- [ ] Denis a relu et valide la coherence technique
- [ ] Arianne a archive le document
- [ ] Les brainstorms sont references dans le Document Fondateur

### Critere de passage

La Phase 5 est complete quand le Document Fondateur est redige, relu par Denis, et archive par Arianne. Le workflow standard prend le relais :

```
Document Fondateur → Denis (doc technique) → Francois (back) + Lise (front)
→ Denis (tests finaux) → George (audit) → Arianne (archivage)
```

---

## 7. Roles et responsabilites par phase

| Phase | Maria | Denis | Francois | Lise | George | Arianne | Utilisateur |
|-------|-------|-------|----------|------|--------|---------|-------------|
| 1. Cadrage | **Redige** | -- | -- | -- | -- | -- | Source |
| 2. Ideation | **Redige** | Contribue (technique) | Contribue (faisabilite) | Contribue (UX) | -- | -- | -- |
| 3. Profondeur | **Redige** | **Valide** (technique) | Contribue | Contribue | -- | -- | -- |
| 4. Decisions | **Collecte** | -- | -- | -- | -- | **Archive** | **Decide** |
| 5. Synthese | **Redige** | **Review** | -- | -- | -- | **Archive** | Valide |

---

## 8. Structure normee du Document Fondateur

Tout Document Fondateur produit en Phase 5 doit suivre cette structure :

```markdown
# {NomService} — Document Fondateur

<!--
@id service.{nom_service}.fondateur
@do define_founding_vision_for_{nom_service}
@role governance
@layer service
@human Document Fondateur du service {Nom}
-->

## Contexte
[Pourquoi ce service existe, quel probleme il resout]

## Portee / Scope
- Applicable a : ...
- Audience : ...
- Statut : Document fondateur normatif.

## 1. Vision
[Phrase de vision + principes fondateurs]

## 2. Type de Service
[Type 1 (interne COG), Type 2 (COG + web), ou Type 3 (Inter-COG)]

## 3. Capacites cles
[Tableau des capacites principales]

## 4. Architecture COG
### Operateurs (Strate 7)
### Toolkits (Strate 6)
### Cores (Strate 4)

## 5. Dependances
[Services et Cores dont ce service depend]

## 6. Lois d'Autonomie
[Comment le service respecte chaque loi applicable]

## 7. Resume
[Synthese en 2-3 phrases]

## 8. References
[Liens vers brainstorms, doc technique, etc.]
```

---

## 9. Arborescence des fichiers

Pour chaque service brainstorme, l'arborescence suivante est creee :

```
docs/services/{nom-service-kebab}/
  |-- BRAINSTORM-initial.md              -- Phase 1+2 : cadrage et ideation
  |-- BRAINSTORM-approfondissement.md    -- Phase 3 : profondeur
  |-- {NomService} - Document Fondateur.md  -- Phase 5 : synthese
  |-- _index.md                          -- Index du dossier (optionnel)
```

Convention de nommage :
- Dossier : `kebab-case` du nom conceptuel (ex: `accompagnement-scolaire`, `gestion-evenements`)
- Fichiers brainstorm : prefixe `BRAINSTORM-` en majuscules
- Document Fondateur : nomenclature standard Miyukini `<Nom> - <Type>.md`

---

## 10. Balisage MSCM — Recapitulatif

### Balises obligatoires par document

| Document | @id | @do | @role | @layer |
|----------|-----|-----|-------|--------|
| BRAINSTORM-initial | `brainstorm.{service}.cadrage` | `analyze_need_for_{service}` | `planning` | `service` |
| BRAINSTORM-approfondissement | `brainstorm.{service}.approfondissement` | `deepen_analysis_for_{service}` | `planning` | `service` |
| Document Fondateur | `service.{service}.fondateur` | `define_founding_vision_for_{service}` | `governance` | `service` |

### Balises optionnelles par section

Chaque section technique majeure peut porter son propre balisage :

```
<!--
@id brainstorm.{service}.{section}
@do {action}
@role {role}
@layer service
@human {description}
-->
```

Exemples de sections balisables :
- `brainstorm.jayecole.architecture` — architecture COG
- `brainstorm.jayecole.axe.parcours_utilisateur` — parcours utilisateur
- `brainstorm.jayecole.axe.contenu_pedagogique` — contenu
- `brainstorm.jayecole.roadmap` — roadmap MVP

---

## 11. Anti-patterns a eviter

| Anti-pattern | Pourquoi c'est un probleme | Alternative |
|-------------|---------------------------|-------------|
| Sauter la Phase 1 | On risque de construire la mauvaise chose | Toujours commencer par le cadrage |
| Brainstormer sans questions ouvertes | L'utilisateur n'est pas implique dans les decisions | Toujours formuler des questions |
| Promettre des delais en Phase 1-2 | Les estimations sont impossibles sans profondeur | Estimer uniquement en Phase 3 (fourchettes) |
| Melanger MVP et futur | Scope creep garanti | Toujours separer clairement MVP / v1.0 / futur |
| Ignorer les Lois d'Autonomie | Non-conformite architecturale | Verifier les 8 Lois en Phase 2 |
| Ne pas baliser MSCM | Document non indexable par MIP | Balisage obligatoire |
| Rediger le Document Fondateur sans decisions | Fondations instables | Attendre Phase 4 |
| Estimer en valeur unique | Fausse precision | Toujours en fourchette (optimiste/pessimiste) |

---

## 12. References

| Document | Role |
|----------|------|
| `docs/reference/protocoles/templates/TEMPLATE-brainstorm-service.md` | Template reutilisable pour le brainstorm initial |
| `CLAUDE.md` | Conventions projet, stack technique, equipe |
| `.cursor/skills/miyukini-mscm-mip/SKILL.md` | Balisage MSCM et indexation MIP |
| `.cursor/skills/miyukini-services/SKILL.md` | Patterns de services (data/, auth/, etc.) |
| `.cursor/skills/miyukini-architecture/SKILL.md` | Architecture COG, strates, Cores |

---

*Protocole redige par Maria, Chef de Projet Miyukini AI Studio*
*Version 1.0 — 2026-02-27*
*Tout brainstorming de nouveau service doit suivre ce protocole.*
