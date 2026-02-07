# JayBudget - Analyse des besoins (point d'entree Purse)

## Contexte

Ce document formalise l'analyse des besoins Purse du service COG JayKonta.
Le perimetre couvre budget personnel, budgets occasionnels, objectifs, alertes et exports.

References principales :
- `docs/services/JayKonta/JayKonta - Document Fondateur.md`
- `docs/services/JayKonta/JayKonta - Contrats Service Operateurs et Toolkits.md`
- `docs/services/JayKonta/reference/JayKonta - Niveaux Securite et Protection Donnees.md`

## Portee

- Public : particuliers, foyers
- In scope : compte Purse, mouvements, categories, budgets occasionnels, objectifs, alertes, rapports
- Out of scope : devis, facturation legale, AP/AR entreprise

## Objectifs metier

- O1 : donner une vision claire du solde et des depenses
- O2 : piloter des budgets occasionnels (vacances, noel, projets)
- O3 : suivre des objectifs d'epargne ou de depense
- O4 : fournir des alertes utiles sans bruit
- O5 : garantir protection donnees niveau 2

## Personas et attentes

| Persona | Attente principale | Risque de non couverture |
|---------|--------------------|--------------------------|
| Solo budget | Saisie rapide et vue simple | abandon et retour tableur |
| Foyer | Separation budgets perso/commun | confusion et surdepenses |
| Projet occasionnel | budget dedie et suivi reel | depassement non detecte |
| Epargnant | objectifs et progression | perte de motivation |

## Besoins fonctionnels

### B1. Acces Purse

| ID | Besoin | Contrat associe |
|----|--------|-----------------|
| PUR-01 | Creation compte Purse | CK-SVC-01, CK-TK-01 |
| PUR-02 | Connexion/session | CK-TK-01 |
| PUR-03 | Contexte Purse isole | CK-SVC-02 |

### B2. Mouvements et categories

| ID | Besoin | Contrat associe |
|----|--------|-----------------|
| PUR-04 | Enregistrer mouvements | CK-OP-02, CK-TK-11 |
| PUR-05 | Gerer categories | CK-TK-11 |
| PUR-06 | Solde et synthese | CK-OP-01, CK-TK-51 |
| PUR-07 | Historique et recherche | CK-TK-11 |

### B3. Budgets occasionnels

| ID | Besoin | Contrat associe |
|----|--------|-----------------|
| PUR-08 | Creer budget occasionnel | CK-OP-03, CK-TK-61 |
| PUR-09 | Affecter depenses au budget | CK-OP-03, CK-TK-61 |
| PUR-10 | Suivre detail budget | CK-TK-61 |

### B4. Objectifs et alertes

| ID | Besoin | Contrat associe |
|----|--------|-----------------|
| PUR-11 | Definir objectifs | CK-TK-61 |
| PUR-12 | Suivre progression objectifs | CK-TK-61 |
| PUR-15 | Configurer alertes | CK-TK-61, CK-AUD-01 |
| PUR-16 | Rappels optionnels JayKoa | CK-INT-03 |

### B5. Rapports et export

| ID | Besoin | Contrat associe |
|----|--------|-----------------|
| PUR-13 | Rapports personnels | CK-TK-51 |
| PUR-14 | Export PDF/CSV personnel | CK-TK-51, CK-AUD-02 |

## Besoins non fonctionnels

| ID | Exigence | Cible |
|----|----------|-------|
| NFR-PUR-01 | Securite donnees | Niveau 2 minimum |
| NFR-PUR-02 | Residence donnees | Selon policy service |
| NFR-PUR-03 | Audit actions sensibles | Ecritures et exports traces |
| NFR-PUR-04 | Perf dashboard | < 3 s mediane |
| NFR-PUR-05 | Perf saisie mouvement | < 2 s mediane |
| NFR-PUR-06 | Simplicite UX | 3 actions max pour saisir depense |
| NFR-PUR-07 | Mobile/desktop | parcours principaux sur 2 formats |

## Priorisation

| Priorite | Besoins |
|----------|---------|
| P0 | PUR-01 a PUR-07 |
| P1 | PUR-08 a PUR-10 |
| P2 | PUR-11 a PUR-14 |
| P3 | PUR-15 a PUR-16 |

## Criteres d'acceptation globaux

- CA-1 : un utilisateur enregistre un mouvement en moins de 2 secondes
- CA-2 : un budget occasionnel affiche depense/restant correctement
- CA-3 : un objectif affiche progression et alerte
- CA-4 : un export personnel respecte le scope utilisateur

## Definition of done (document)

- besoins traces vers contrats
- priorisation explicite
- criteres mesurables presents
- risques et dependances identifies

## Risques et mitigations

- R1 : surcharge fonctionnelle Purse
- mitigation : maintenir focus budget personnel, pas de logique entreprise

- R2 : alertes trop frequentes
- mitigation : seuils configurables et granularite utilisateur

- R3 : confusion categorie/projet occasionnel
- mitigation : UX separant categories globales et budgets dedies

## References complementaires

- `docs/services/JayKonta/JayKonta - Bornage Implementation.md`
- `docs/services/JayKonta/publics/Purse/Purse - Operateurs et Toolkits.md`
- `docs/services/JayKonta/publics/Purse/Purse - Parcours Capacites Livrables.md`

## Statut

- Version : 2.0
- Date : 2026-02-07
- Statut : Analyse enrichie
