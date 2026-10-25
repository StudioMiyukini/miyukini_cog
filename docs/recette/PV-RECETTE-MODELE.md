# Procès-verbal de recette — modèle

<!-- @id recette.pv -->
<!-- @role report-template -->
<!-- @layer 0-9+zones -->
<!-- @human Modèle de procès-verbal de la recette 0.9.0-beta.1 : version, environnement, cas, anomalies, décision -->
<!-- @do fill_acceptance_report -->

- **Mode d'emploi** : copier ce fichier hors dépôt (un PV par passage), remplacer chaque `…`, joindre les sorties TAP
- **Références** : [plan](PLAN-RECETTE-BETA.md) · [cas](CAS-DE-TEST-BETA.md) · [limites](LIMITES-CONNUES-BETA.md)
- Ne jamais coller de secret (clé, mot de passe, contenu de `.env`) dans le PV : noter l'empreinte SHA-256 seulement

## 1. Identification

<!-- @id recette.pv.identification -->
<!-- @role report-header -->
<!-- @do record_identification -->

| Champ | Valeur |
|---|---|
| Référence du PV | `PV-0.9.0-beta.1-…` |
| Version recettée | `0.9.0-beta.…` |
| Commit (en-tête du `MANIFESTE`) | `…` |
| État de l'arbre (`propre` / `modifié`) | `…` |
| Empreinte du `MANIFESTE` (reçue / calculée) | `…` / `…` |
| Dates de début et de fin | `…` |
| Testeur · Mainteneur · Propriétaire | `…` · `…` · `…` |

## 2. Environnement

<!-- @id recette.pv.environnement -->
<!-- @role report-environment -->
<!-- @do record_environment -->

| Champ | Valeur |
|---|---|
| Hyperviseur et version | `…` |
| Ubuntu (`lsb_release -a`) et noyau (`uname -r`) | `…` |
| vCPU · RAM · disque | `…` · `…` · `…` |
| Instantané vierge (nom, date) | `…` |
| `systemd-detect-virt --vm` | `…` |
| AppArmor actif (`aa-status`) · Landlock disponible | `…` · `…` |
| `br_netfilter` chargé (doit être non) | `…` |
| PostgreSQL (`psql --version`) | `…` |
| Écarts à l'environnement du [plan §3](PLAN-RECETTE-BETA.md) | `…` |

## 3. Critères d'entrée

<!-- @id recette.pv.entree -->
<!-- @role report-entry -->
<!-- @do check_entry_criteria -->

| Critère | Oui / Non | Remarque |
|---|---|---|
| Arbre propre, commit étiqueté | … | |
| `tester.sh` 100 et banc 100 sans `not ok` côté mainteneur | … | |
| Empreinte du `MANIFESTE` reçue hors bande | … | |
| VM vierge et instantané pris | … | |
| Limites connues lues et acceptées | … | |

## 4. Résultats par phase

<!-- @id recette.pv.resultats -->
<!-- @role report-results -->
<!-- @do record_results -->

| Phase | Cas | Passés | Échoués | Non exécutés / sans objet |
|---|---|---|---|---|
| A. Installation | RB-001 à RB-006 | … | … | … |
| B. Fail-closed | RB-007 à RB-012 | … | … | … |
| C. MiyuCloud, portail | RB-013 à RB-028 | … | … | … |
| C. MAIA, KindMother | RB-029 à RB-036 | … | … | … |
| C. Zone COG, AppArmor, sas | RB-037 à RB-046 | … | … | … |
| C. Banc d'intrusion | RB-047 à RB-052 | … | … | … |
| C. Relais NFLOG | RB-053 à RB-055 | … | … | … |
| D. Redémarrage | RB-056 à RB-060 | … | … | … |
| E. Sauvegarde, restauration | RB-061 à RB-063 | … | … | … |
| F. Mise à jour, retour arrière | RB-064 à RB-067 | … | … | … |
| G. Désinstallation | RB-068 | … | … | … |
| **Total** | **68** | … | … | … |

### 4.1 Sorties automatiques

| Script | Attendu | Obtenu | Fichier joint |
|---|---|---|---|
| `tester.sh` | 100 `ok` | … | `…` |
| `tester.sh --banc` | 100 `ok` | … | `…` |
| `scenarios.sh` | 0 `not ok` | … | `…` |
| `essai_nflog.sh` | 0 `KO` | … | `…` |

### 4.2 Détail par cas

Une ligne par cas ; résultat : `P` passé · `E` échoué · `N` non exécuté · `S` sans objet (amendement refusé) · `L` limite attendue.

| Cas | Résultat | Date et heure | Preuve (extrait, fichier) | Anomalie |
|---|---|---|---|---|
| RB-001 | … | … | … | … |
| RB-002 | … | … | … | … |
| … | … | … | … | … |
| RB-068 | … | … | … | … |

## 5. Anomalies

<!-- @id recette.pv.anomalies -->
<!-- @role report-defects -->
<!-- @do list_defects -->

| Id | Cas | Gravité (B / M / m) | Résumé | Étapes | Journaux | Statut | Contournement |
|---|---|---|---|---|---|---|---|
| AN-001 | RB-… | … | … | … | … | ouverte / corrigée / acceptée | … |

- Total : bloquantes ouvertes `…` · majeures ouvertes `…` · mineures ouvertes `…`
- Écarts classés « limite attendue » (références `L-NN`) : `…`

## 6. Amendements en attente

<!-- @id recette.pv.amendements -->
<!-- @role report-amendments -->
<!-- @do record_amendment_decisions -->

| Amendement | Décision du propriétaire (validé / refusé / en attente) | Date | Cas concernés |
|---|---|---|---|
| A-010 | … | … | RB-032 |
| A-011 | … | … | RB-029 à RB-032 |
| A-012 | … | … | RB-045 |
| A-013 | … | … | RB-014, RB-015, RB-031, RB-063 |
| A-014 | … | … | — |
| A-015 | … | … | RB-035, RB-036, RB-053 à RB-055 |

## 7. Critères de sortie

<!-- @id recette.pv.sortie -->
<!-- @role report-exit -->
<!-- @do check_exit_criteria -->

| Critère | Atteint (oui / non) | Remarque |
|---|---|---|
| 100 % des cas bloquants passés | … | |
| ≥ 95 % des autres cas passés | … | |
| 0 anomalie bloquante ouverte | … | |
| Majeures : contournement écrit et accepté | … | |
| Aucun `DENIED` AppArmor inexpliqué, aucune écoute hors boucle locale | … | |
| Restauration et retour arrière démontrés (RB-062, RB-067) | … | |

## 8. Décision

<!-- @id recette.pv.decision -->
<!-- @role report-decision -->
<!-- @do record_decision -->

- **Décision** : GO · GO sous réserves · NO-GO (barrer les mentions inutiles)
- **Réserves** : `…`
- **Actions et échéances** (mainteneur) : `…`
- **Version suivante à recetter** : `0.9.0-beta.…` · cas à rejouer : `…`
- **Nettoyage** : identifiants de test détruits (`~/.cog-scenarios/admin.env`), archives de sauvegarde supprimées ou conservées chiffrées, VM remise à l'instantané vierge : `oui / non`

| Rôle | Nom | Date | Signature |
|---|---|---|---|
| Testeur | … | … | |
| Mainteneur | … | … | |
| Propriétaire | … | … | |
