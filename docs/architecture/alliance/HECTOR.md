# HECTOR — l'IA du COG-CC

> **Conception v2 — hors gel, non normatif.** Voir l'[index](_index.md) pour les conventions (décidé / proposé / question).

<!-- @id arch.alliance.hector -->
<!-- @role design-ai-framework -->
<!-- @layer 8 -->
<!-- @human Framework IA qui pilote à moitié un COG-CC : rôles superviseur / relève / traitants, confinement des prompts et des données, procédures automatiques, double analyse -->
<!-- @do operate_command_center_ai -->

## Contexte

- Un COG-CC reçoit en continu des alertes et des demandes courantes (mot de passe perdu, déverrouillage, accès à un groupe) ; les traiter à la main ne tient pas à l'échelle d'une organisation
- Une IA qui agit sur un annuaire est une **cible d'injection** de premier ordre : elle doit pouvoir trier et préparer, pas détenir l'autorité

## Portée / Scope

- Rôles des COG du CC dans HECTOR, confinement, catalogue de procédures, chaîne de décision, procédure « mot de passe perdu »
- Le quorum et les classes d'actions sont dans [COG-CC](COG-CC.md)

## 1. Ce qui est décidé

<!-- @id arch.alliance.hector-decide -->
<!-- @role owner-decisions -->
<!-- @do record_hector_decisions -->

- **Décidé (propriétaire)**
  - Sur les COG du CC, l'agent IA passe de **MAIA à HECTOR**
  - HECTOR gère les **alertes** et les **demandes courantes**, automatiquement
  - HECTOR **n'a pas d'autre prompt** que ceux du COG-CC et ne travaille **qu'avec la base** de celui-ci
  - Il interprète des demandes **prédéfinies ou proches**, qu'il analyse comme **malveillantes ou légitimes**, après le filtrage déjà fait par **BorderGuard**
  - Rôles : un COG du CC **superviseur**, un autre en **relève** (fail-over), les 4 autres **sous-agents de traitement**

## 2. Principe directeur

<!-- @id arch.alliance.hector-principe -->
<!-- @role design-principle -->
<!-- @do bound_ai_authority -->

- **Proposé — HECTOR propose, il ne décide pas**
  - HECTOR **classe** une demande vers une procédure du catalogue et **remplit ses champs typés**
  - L'exécution est une **procédure déterministe**, signée, sans IA
  - La décision reste à **StrongFather** du CC (classe 1), au **quorum** (classe 2), au quorum **et** à TAMR (classe 3) — cohérent avec D1 et D2
  - Conséquence : une injection réussie peut au pire **mal classer** une demande ; elle ne peut ni inventer une action, ni changer sa classe, ni contourner le contrôle d'identité

## 3. Rôles

<!-- @id arch.alliance.hector-roles -->
<!-- @role topology -->
<!-- @do assign_hector_roles -->

| Rôle | Nombre | Fait | Ne fait pas |
|---|---|---|---|
| **Superviseur** | 1 | reçoit les demandes, les répartit, consolide les analyses, soumet à la décision | ne traite pas seul une demande de classe ≥ 1 |
| **Relève** | 1 | suit l'état du superviseur en continu, reprend à sa défaillance | ne soumet rien tant que le superviseur est sain |
| **Traitants** | n − 2 (≥ 4) | analysent en parallèle : classement, risque, champs | ne voient pas l'analyse des autres avant d'avoir rendu la leur |

- **Proposé**
  - Bascule : superviseur muet 3 périodes ou mis à l'écart (COG-CC §4) ⇒ la relève prend la main, **confirmée par le quorum** ; un traitant est promu relève
  - **Rotation** périodique du superviseur ([AL-Q12](ALLIANCE-DECISIONS.md)) : limite la durée d'une compromission discrète
  - Rôles HECTOR et répartition de charge de la base sont **indépendants** : un traitant sert aussi des lectures

## 4. Double analyse

<!-- @id arch.alliance.hector-double-analyse -->
<!-- @role cross-check -->
<!-- @do cross_check_requests -->

- **Proposé**
  - Toute demande de classe ≥ 1 est analysée par **2 traitants au moins**, tirés au sort, **à l'aveugle**
  - Accord exigé sur : procédure, champs extraits, verdict légitime / malveillant
  - Désaccord ⇒ **pas d'automatisme** : la demande monte d'une classe (au plus classe 3, TAMR)
  - Variante plus forte ([AL-Q13](ALLIANCE-DECISIONS.md)) : deux **modèles différents** sur les traitants, pour qu'une même injection ne trompe pas les deux

## 5. Confinement

<!-- @id arch.alliance.hector-confinement -->
<!-- @role containment -->
<!-- @do confine_hector -->

- **Prompts**
  - **Catalogue de prompts** scellé au **quorum** du CC, identique sur tous les membres (comparé par l'intégrité collective, COG-CC §4)
  - Aucun prompt système fourni par un utilisateur, une politique ou une donnée ; aucun ajout à chaud
- **Données**
  - Accès en **lecture seule** à la base du CC, par des **requêtes nommées** du catalogue (pas de requête libre)
  - Aucune écriture : HECTOR produit des **intentions** (comme un opérateur, D1), validées par StrongFather avant KindMother
  - Aucun accès réseau, aucun outil hors catalogue, aucune mémoire personnelle (les règles MAIA N0–N3 ne s'appliquent pas)
  - **Aucun accès** aux sauvegardes de mémoire MAIA tenues par le CC (AL-D25) : magasin séparé, hors de ses requêtes nommées
- **Texte de l'utilisateur**
  - Normalisé comme une Fiche (A-009 : NFKC, invisibles, balisage, homoglyphes) puis **traité comme une donnée**, jamais comme une consigne
  - Les champs extraits sont **typés et validés** (identifiant de compte existant, poste enrôlé…) avant toute suite
- **Modèle**
  - Poids livrés par le **canal officiel** (artefact `MMIA`, A-007), empreinte identique sur tous les membres

## 6. Catalogue de procédures

<!-- @id arch.alliance.hector-catalogue -->
<!-- @role procedure-catalog -->
<!-- @do list_hector_procedures -->

- **Proposé** — chaque procédure fixe : déclencheurs reconnus, champs, contrôles d'identité, classe, action déterministe, notification

| Id | Procédure | Classe |
|---|---|---|
| P-001 | Mot de passe perdu (compte ordinaire) | 1 |
| P-002 | Compte verrouillé après échecs | 1 |
| P-003 | Demande d'ajout à un groupe ordinaire | 1 (accord du responsable du groupe) |
| P-004 | Nouveau poste pour un utilisateur existant | 2 |
| P-005 | Alerte WorrySentinel d'un poste : tri et proposition | 1 → 3 selon niveau |
| P-006 | Demande d'export des données d'un utilisateur (AL-D23) | 2 ou 3 ; HECTOR **prépare** la demande, l'export est fait par un humain |
| P-0xx | Toute demande d'un **compte privilégié** | 3, jamais automatique |

- Demande « proche » d'une procédure : HECTOR peut la rapprocher, mais la procédure et sa classe restent celles du catalogue ; aucune correspondance sûre ⇒ ticket humain

## 7. Procédure P-001 « mot de passe perdu »

<!-- @id arch.alliance.hector-p001 -->
<!-- @role procedure -->
<!-- @do reset_lost_password -->

- **Décidé (propriétaire)** : protocole d'identification ; s'il réussit, **mot de passe temporaire** et **changement obligatoire** à la prochaine connexion ; le tout automatiquement
- **Proposé — déroulé**
  1. **Canal** : écran de connexion d'un **poste enrôlé** (le poste est authentifié par son identité même si l'utilisateur ne l'est pas) ; BorderGuard a déjà filtré
  2. **Classement** : 2 traitants à l'aveugle (§4) ⇒ P-001, champ `compte`
  3. **Contrôles déterministes** (sans IA) : compte existant et **non privilégié** ; pas de réinitialisation dans les 24 h ; débit par compte et par poste ; aucune alerte WorrySentinel ouverte sur le compte ou le poste
  4. **Identification** — un facteur **que l'IA n'évalue pas**, dans cet ordre (**décidé, AL-D36**) :
     - approbation sur un **autre appareil enrôlé** de l'utilisateur (passeport valide), par une **approbation signée** dans [COG-Auth](COG-AUTH.md) ; ou
     - code à usage unique confirmé par le **responsable** désigné dans l'annuaire ; ou
     - vérification en présence (guichet)
     - **Refusé** : questions secrètes, informations connues de collègues
  5. **Décision** : StrongFather du CC (classe 1), inscrite au journal
  6. **Action** : mot de passe temporaire aléatoire, affiché **une fois** sur le poste demandeur, validité courte (défaut proposé : 1 h), drapeau **changement obligatoire** ; clés Kerberos du compte régénérées ; sessions existantes et tickets révoqués
  7. **Notification** sur les autres appareils de l'utilisateur : « votre mot de passe a été réinitialisé depuis le poste X » (détection d'abus)
- Échec à l'une des étapes ⇒ refus neutre (sans dire laquelle), ticket humain, signal WorrySentinel si le débit est anormal

## 8. Points à trancher

- [AL-Q12](ALLIANCE-DECISIONS.md) rotation du superviseur · [AL-Q13](ALLIANCE-DECISIONS.md) diversité des modèles · [AL-Q15](ALLIANCE-DECISIONS.md) modèle local de HECTOR et matériel des COG du CC

---

Date de mise a jour : 2026-10-25
