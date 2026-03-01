# {NOM_SERVICE} — Brainstorming Initial

<!--
@id brainstorm.{nom_service_snake}.cadrage
@do analyze_need_for_{nom_service_snake}
@role planning
@layer service
@human Brainstorming initial — cadrage et ideation du service {NOM_SERVICE}
-->

## Contexte

{Description en 2-3 phrases : quel service, pour qui, pourquoi.}

## Portee / Scope

- **Applicable a :** Phase exploratoire, brainstorming initial, collecte d'idees.
- **Audience :** Equipe projet, parties prenantes, utilisateur fondateur.
- **Statut :** Document de brainstorming (pre-fondateur). A transformer en Document Fondateur apres validation.
- **Redige par :** Maria (Chef de Projet)
- **Date :** {AAAA-MM-JJ}

---

## Proposition de nom : {NOM_SERVICE}

**Justification :** {Pourquoi ce nom. Coherence avec la nomenclature Jay* ou Miyukini*.}

**Nom interne suggere :** {NomService}
**Alternatives :** {Alternative1}, {Alternative2}, {Alternative3}

---

<!-- ============================================================ -->
<!-- PHASE 1 — CADRAGE                                            -->
<!-- ============================================================ -->

## 1. Analyse du besoin

<!--
@id brainstorm.{nom_service_snake}.analyse_besoin
@do identify_target_audience_and_problems
@role planning
@layer service
@human Analyse du public cible et des problemes a resoudre
-->

### 1.1 Public cible

**Public primaire :**
- {Qui sont les utilisateurs principaux ?}
- {Tranche d'age, profil, contexte d'utilisation}

**Public secondaire :**
- {Qui sont les utilisateurs secondaires ?}

**Public tertiaire (futur) :**
- {Extensions futures du public ?}

### 1.2 Contexte et problemes a resoudre

| Probleme | Description |
|----------|-------------|
| **{Probleme 1}** | {Description} |
| **{Probleme 2}** | {Description} |
| **{Probleme 3}** | {Description} |
| **{Probleme 4}** | {Description} |

### 1.3 Positionnement

{NOM_SERVICE} n'est **pas** {ce qu'il n'est pas}. C'est {ce qu'il est} qui :
- {Proposition de valeur 1}
- {Proposition de valeur 2}
- {Proposition de valeur 3}

---

<!-- ============================================================ -->
<!-- PHASE 2 — IDEATION                                           -->
<!-- ============================================================ -->

## 2. Fonctionnalites cles

<!--
@id brainstorm.{nom_service_snake}.fonctionnalites
@do list_features_by_priority
@role planning
@layer service
@human Liste des fonctionnalites par priorite (MVP, v1.0, futur)
-->

### 2.1 MVP (Version 0.1)

| Fonctionnalite | Description | Priorite |
|----------------|-------------|----------|
| **{Fonctionnalite 1}** | {Description} | Critique |
| **{Fonctionnalite 2}** | {Description} | Critique |
| **{Fonctionnalite 3}** | {Description} | Important |
| **{Fonctionnalite 4}** | {Description} | Important |

### 2.2 Version 1.0

| Fonctionnalite | Description | Priorite |
|----------------|-------------|----------|
| **{Fonctionnalite 5}** | {Description} | Eleve |
| **{Fonctionnalite 6}** | {Description} | Moyen |

### 2.3 Extensions futures

| Fonctionnalite | Description |
|----------------|-------------|
| **{Fonctionnalite 7}** | {Description} |
| **{Fonctionnalite 8}** | {Description} |

---

## 3. Mecaniques cles

<!--
@id brainstorm.{nom_service_snake}.mecaniques
@do define_core_service_mechanics
@role planning
@layer service
@human Mecaniques fondamentales du service
-->

### 3.1 {Mecanique principale 1}

{Description detaillee de la mecanique. Comment ca fonctionne pour l'utilisateur.}

### 3.2 {Mecanique principale 2}

{Description detaillee.}

### 3.3 {Mecanique principale 3}

{Description detaillee.}

---

## 4. {Axe specifique au service}

<!--
Adapter cette section au service. Exemples :
- Pour un service educatif : "Programme et contenu"
- Pour un service de gestion : "Processus metier"
- Pour un jeu : "Gameplay et boucle de jeu"
- Pour un service social : "Interactions et moderation"
-->

{Contenu specifique au domaine du service.}

---

## 5. Interface utilisateur

<!--
@id brainstorm.{nom_service_snake}.interface
@do sketch_main_user_interfaces
@role planning
@layer service
@human Esquisse des interfaces utilisateur principales
-->

### 5.1 Principes UX/UI

**Navigation :**
- {Principe 1}
- {Principe 2}

**Design :**
- {Principe 1}
- {Principe 2}

**Accessibilite :**
- {Principe 1}
- {Principe 2}

### 5.2 Ecrans principaux

1. **{Ecran 1}** : {description}
2. **{Ecran 2}** : {description}
3. **{Ecran 3}** : {description}
4. **{Ecran 4}** : {description}

---

## 6. {Interface secondaire — si applicable}

<!--
Exemples :
- Dashboard parent (service educatif)
- Dashboard admin (service de gestion)
- Mode organisateur (service evenementiel)
Supprimer cette section si non applicable.
-->

{Contenu.}

---

## 7. Considerations techniques

<!--
@id brainstorm.{nom_service_snake}.architecture
@do define_cog_architecture_for_{nom_service_snake}
@role architecture
@layer service
@human Architecture COG : strates, operateurs, toolkits, cores
-->

### 7.1 Positionnement dans l'architecture COG

**Type de Service : {Type 1 (interne COG) / Type 2 (COG + web) / Type 3 (Inter-COG)}**
- {Description de l'acces}
- {Description du stockage}
- {Conformite Lois d'Autonomie}

**Strate 7 — Service**
```
{NomService}
|-- {Interface principale} (Dioxus)
|-- {Interface secondaire si applicable} (Dioxus)
|-- {Moteur metier}
```

**Strate 7 — Operateurs**
```
{NomService}.{Operateur1}  -> {Description}
{NomService}.{Operateur2}  -> {Description}
{NomService}.{Operateur3}  -> {Description}
```

**Strate 6 — Toolkits requis**
```
Toolkits existants a reutiliser :
  {Toolkit1}  -> {Usage}
  {Toolkit2}  -> {Usage}

Toolkits a creer :
  {Toolkit3}  -> {Description}
  {Toolkit4}  -> {Description}
```

**Strate 4 — Cores utilises**
```
KindMother      -> {Usage specifique}
StrongFather    -> {Usage specifique}
CaringNanny     -> {Usage specifique}
MasterButler    -> {Usage specifique}
WorrySentinel   -> {Usage specifique}
EverBuddy       -> {Usage specifique}
{Supprimer les Cores non utilises}
```

### 7.2 Structure des crates

```
crates/{nom_service_snake}/
  Cargo.toml
  src/
    lib.rs
    data/
      mod.rs
      types.rs
      kindmother_db.rs
    services/
      {service1}.rs
      {service2}.rs
    auth/
      mod.rs
    export/
      mod.rs

crates/{toolkit_nouveau}/
  Cargo.toml
  src/
    lib.rs
    admin_cell.rs
    context.rs
    errors.rs
    {metier}.rs
```

### 7.3 Schema de base de donnees (KindMother, esquisse)

```sql
-- {Table principale}
CREATE TABLE {table_principale} (
    id TEXT PRIMARY KEY,          -- UUID v4
    {champ1} TEXT NOT NULL,
    {champ2} TEXT,
    created_at TEXT NOT NULL,     -- ISO 8601
    updated_at TEXT NOT NULL
);

-- {Table secondaire}
CREATE TABLE {table_secondaire} (
    id TEXT PRIMARY KEY,
    {fk} TEXT NOT NULL REFERENCES {table_principale}(id),
    {champ1} TEXT NOT NULL,
    {champ2} REAL,
    created_at TEXT NOT NULL
);
```

### 7.4 Conformite avec les Lois d'Autonomie

| Loi | Application a {NOM_SERVICE} |
|-----|----------------------------|
| **LOI-1** (pas de dependance externe) | {Comment le service respecte cette loi} |
| **LOI-2** (isolement = normal) | {Comment le service respecte cette loi} |
| **LOI-3** (etat local souverain) | {Comment le service respecte cette loi} |
| **LOI-4** (pas de temps global) | {Comment le service respecte cette loi} |
| **LOI-5** (cout proportionnel au hardware) | {Comment le service respecte cette loi} |
| **LOI-6** (federation possible) | {Comment le service respecte cette loi} |
| **LOI-7** (Cores immuables) | {Comment le service respecte cette loi} |
| **LOI-8** (migration = diplomatie) | {Comment le service respecte cette loi} |

---

## 8. Risques et contraintes

<!--
@id brainstorm.{nom_service_snake}.risques
@do identify_risks_and_mitigations
@role planning
@layer service
@human Identification des risques projet et des contraintes
-->

### 8.1 Risques reglementaires

{RGPD, CNIL, regulations specifiques au domaine. Supprimer si non applicable.}

### 8.2 Risques projet

| Risque | Probabilite | Impact | Mitigation |
|--------|-------------|--------|-----------|
| **{Risque 1}** | {Elevee/Moyenne/Faible} | {Eleve/Moyen/Faible} | {Mitigation} |
| **{Risque 2}** | {Elevee/Moyenne/Faible} | {Eleve/Moyen/Faible} | {Mitigation} |
| **{Risque 3}** | {Elevee/Moyenne/Faible} | {Eleve/Moyen/Faible} | {Mitigation} |

### 8.3 Contraintes specifiques

1. {Contrainte 1}
2. {Contrainte 2}
3. {Contrainte 3}

---

## 9. Estimation des couts et ressources

<!--
@id brainstorm.{nom_service_snake}.couts
@do estimate_costs_and_resources
@role planning
@layer service
@human Estimation des couts en fourchettes et repartition des ressources
-->

### 9.1 Couts de developpement (en jours-homme estimes)

| Poste | Optimiste | Pessimiste | Notes |
|-------|-----------|------------|-------|
| **{Poste 1}** | {X} j | {Y} j | {Notes} |
| **{Poste 2}** | {X} j | {Y} j | {Notes} |
| **{Poste 3}** | {X} j | {Y} j | {Notes} |
| **TOTAL MVP** | **{X} j** | **{Y} j** | {Conversion en mois} |

### 9.2 Repartition par agent

| Agent | Responsabilite | Charge estimee |
|-------|---------------|---------------|
| **Maria** | {Responsabilite} | {X-Y} j |
| **Denis** | {Responsabilite} | {X-Y} j |
| **Francois** | {Responsabilite} | {X-Y} j |
| **Lise** | {Responsabilite} | {X-Y} j |
| **George** | {Responsabilite} | {X-Y} j |
| **Arianne** | {Responsabilite} | {X-Y} j |

### 9.3 Ressources externes potentielles

| Ressource | Pourquoi | Cout estime |
|-----------|----------|-------------|
| **{Ressource 1}** | {Justification} | {Cout} |
| **{Ressource 2}** | {Justification} | {Cout} |

---

## 10. Questions ouvertes (a clarifier avec l'utilisateur)

<!--
Formuler au minimum 5 questions.
Les questions doivent couvrir :
- Le nom du service
- Le scope du MVP
- La cible geographique / marche
- Le modele economique
- La priorite dans la roadmap
Plus des questions specifiques au domaine.
-->

1. **Nom du service** : {NomPropose} convient-il ? Alternatives ?
2. **Scope MVP** : {Quelles fonctionnalites inclure/exclure ?}
3. **Cible** : {Geographique, demographique, sectorielle ?}
4. **Modele economique** : Gratuit ? Freemium ? Payant ?
5. **Priorite** : Ou se situe {NOM_SERVICE} dans la roadmap Miyukini ?
6. {Question specifique 1}
7. {Question specifique 2}
8. {Question specifique 3}

---

## 11. Prochaines etapes proposees

1. **Validation de ce brainstorming** par l'utilisateur (reponses aux questions ouvertes)
2. **Brainstorming approfondissement** (Phase 3 du protocole)
3. **Redaction du Document Fondateur** (Phase 5)
4. **Transmission a Denis** pour la documentation technique
5. **Archivage** par Arianne

---

*Document redige par Maria, Chef de Projet Miyukini AI Studio*
*Brainstorming initial — En attente de validation utilisateur*
*Protocole de reference : `docs/reference/protocoles/PROTOCOLE-brainstorming.md`*
