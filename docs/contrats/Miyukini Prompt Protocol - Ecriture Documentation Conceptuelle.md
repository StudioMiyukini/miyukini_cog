# Protocole d'Ã©criture de la documentation conceptuelle et contractuelle d'un nouveau systÃ¨me
**Version : v1.0**  
**Statut : Normatif â€“ Processus contrÃ´lÃ©**  

---

## 1. Contexte

Ce protocole complÃ¨te le [Protocole d'implÃ©mentation gÃ©nÃ©rale](..//_index.md) en dÃ©finissant les rÃ¨gles spÃ©cifiques Ã  la **rÃ©daction de documentation** (et non de code). Il guide l'agents IA *"planificateur"* en mode PLAN pour composer une to-do liste complÃ¨te qui faciletera l'allocation des tÃ¢ches aux agents IA.

### 1.1 PortÃ©e / Scope

Ce document dÃ©finit un **protocole strict d'utilisation d'agents IA** pour l'Ã©criture de la documentation d'un systÃ¨me logiciel â€” **cette documentation fera rÃ©fÃ©rence**.

Le processus :
- est **suivi et automatisÃ©**,
- repose sur un **cycle fermÃ©**, dÃ©terminÃ© et explicite,
- impose des **contrÃ´les stricts Ã  chaque Ã©tape**,
- autorise le **travail en parallÃ¨le**, sous contraintes,
- intÃ¨gre la **gestion des limites de contexte et de coÃ»ts LLM**,
- interdit toute dÃ©rive implicite ou interprÃ©tation libre.
- DÃ©finie les rÃ¨gles de nomenclature des tÃ¢ches

## 2. Cycle global d'Ã©criture (obligatoire)

Le cycle est **ordonnÃ©, fermÃ© et non contournable** :

1. **Planification**
2. **Distribution des tÃ¢ches aux agents**
3. **VÃ©rification, corrections et tests**
4. **Gel et versionnement**

ðŸ‘‰ Aucune Ã©tape ne peut Ãªtre sautÃ©e ou fusionnÃ©e.

---

## 3. Phase 1 â€” Planification

### 3.1 Titre de l'Ã©tape en cours

Renseigner obligatoirement le titre de l'Ã©tape de planification active.

---

### 3.2 Explication rapide

Description synthÃ©tique de :
- le pÃ©rimÃ¨tre couvert,
- les limites explicites,
- le maintien de la liste des tÃ¢ches par l'agent planificateur,
- le lancement des nouvelles tÃ¢ches Ã  de nouveaux agents Auto Cursor si les conditions sont rÃ©unies.

---

### 3.3 SÃ©lection du modÃ¨le IA (obligatoire)

voir "AI Mode Selector.md"

### 3.4 Prompt engineering â€” Mode PLAN

Le prompt de planification DOIT inclure :

#### a) DÃ©finition de l'agent
- RÃ´le prÃ©cis
- Poste
- CompÃ©tences requises
- ResponsabilitÃ©s
- Ce que l'agent ne doit jamais faire

#### b) Cadre de travail
- Documentation autorisÃ©e (liste fermÃ©e)
- Outils autorisÃ©s
- Outils interdits

#### c) Construction de la structure
- 1er document = document "fondateur" si pas dÃ©jÃ  prÃ©sent
- CrÃ©ation des documents nommÃ©s mais vides
- Respecter la taxonomie et les nomenclatures du projet

Les dÃ©pendances doivent Ãªtre :
- explicitÃ©es,
- annotÃ©es,
- accompagnÃ©es d'un ordre strict si nÃ©cessaire

#### d) Contraintes absolues
- âŒ Ne pas anticiper les Ã©tapes suivantes
- âŒ Ne pas fusionner plusieurs documents
- âŒ Ne pas corriger hors pÃ©rimÃ¨tre

#### e) Mini log de planification
- AmbiguÃ¯tÃ©s dÃ©tectÃ©es
- DÃ©pendances critiques
- DÃ©cisions structurantes

---

## 4. Phase 2 â€” Distribution des tÃ¢ches

**Une to-do liste est gÃ©nÃ©rÃ©** et **gÃ©rÃ©**en fonction de ce qui suit :

Chaque tÃ¢che issue du plan est **dÃ©lÃ©guÃ©e par l'agent de planification** Ã  un nouvel agent si les conditions sont rÃ©unies.

Contexte vierge obligatoire.
**Pas de tÃ¢che impliquant plusieurs documents Ã  Ã©crire**
**Pas de batch/vague/groupe de tÃ¢ches**
**1 agent = 1 document**

*Chaque tÃ¢che du plan ou to-do a une nomenclature dÃ©fini* : [xx] - [du document Ã  produire]
Le [xx] est prÃ©fixe de regroupement d'Ã©criture parallÃ¨le. Je peux lancer toutes les tÃ¢ches "01" en mÃªme temps avec des agents diffÃ©rents pour chaque tÃ¢che.

Une limite maximale d'agents simultanÃ©s est dÃ©finie Ã  **4**. Donc aucun groupement de prÃ©fixe ne doit avoir plus de 4 itÃ©rations.

Lance les agents automatiquement en suivant le plan jusqu'Ã  un arret ou la fin de la tÃ¢che

Il est interdit d'avoir des tÃ¢ches qui demande l'Ã©criture de plusieurs fichiers.

### 4.1 RÃ¨gle d'arrÃªt stricte

Un agent DOIT S'ARRÃŠTER IMMÃ‰DIATEMENT si :

- une ambiguÃ¯tÃ© bloquante est dÃ©tectÃ©e,
- une dÃ©pendance manquante est rencontrÃ©e,
- la fenÃªtre de contexte devient insuffisante,
- la tÃ¢che et le test unitaire (si prÃ©sent) sont terminÃ©s et corrects.

ðŸ‘‰ Dans ces cas :

- aucun fichier partiel n'est gÃ©nÃ©rÃ©,
- l'agent informe l'agent planificateur qui continuera son orchestration.

---

## 5. Phase 3 â€” VÃ©rification, corrections et tests

### 5.1 VÃ©rification globale

Le systÃ¨me documentaire est analysÃ© pour :
- incohÃ©rences inter-documents,
- non-conformitÃ©s Ã  la documentation de rÃ©fÃ©rence,
- violations de rÃ¨gles ou d'invariants,
- comportements implicites ou non documentÃ©s.

### 5.2 Corrections

Toute correction est traitÃ©e comme une nouvelle tÃ¢che.  
Les rÃ¨gles de la Phase 2 s'appliquent intÃ©gralement.  
Aucune correction "rapide" hors protocole n'est autorisÃ©e.

### 5.3 Tests

- ExÃ©cution des tests de cohÃ©rence et de complÃ©tude
- Validation de la structure documentaire complÃ¨te
- Aucune validation partielle n'est acceptÃ©e

### 5.4 Audit du systÃ¨me

RÃ©daction d'un audit formel incluant :
- erreurs rencontrÃ©es,
- corrections appliquÃ©es,
- risques Ã©vitÃ©s,
- points de vigilance futurs.

---

## 6. Phase 4 â€” Gel et versionnement

### 6.1 Gel

- RÃ©daction d'un document de gel officiel
- Liste exhaustive des Ã©lÃ©ments gelÃ©s
- Interdiction de toute modification implicite

### 6.2 Versionnement

- Attribution d'une version explicite (ex : v1.2.0)
- Distinction versions majeures / mineures
- RÃ¨gles d'Ã©volution futures
- Conditions de dÃ©gel et de migration

ðŸ‘‰ AprÃ¨s gel :

- toute modification impose un nouveau cycle complet

---

## 7. Annexes

### 7.1 Modes IA

**AI Mode 1 : Libre**
- Abonnement non limitÃ©

**AI Mode 2 : DÃ©gradÃ©**
- Budget On Demand limitÃ© (50$)
- â†’ PrioritÃ© au mode Auto

**AI Mode 3 : FermÃ©**
- Cursor Auto uniquement

### 7.2 ModÃ¨les premium accessibles

- Composer 1
- Opus 4.5
- Sonnet 4.5
- GPT 5.2 Codex (Low / High / Extra High)
- GPT 5.2
- Gemini 3 Flash
- GPT 5.1 mini
- Kimi K2

### 7.3 RÃ¨gle de consommation IA

Ã€ chaque [pause], l'agent doit :

- soit continuer explicitement,
- soit passer la main Ã  un nouvel agent.

Un agent dont la fenÃªtre de contexte est saturÃ©e :

- s'arrÃªte,
- ne gÃ©nÃ¨re aucun document, mÃªme partiel.

---

## 8. Conclusion

Ce protocole garantit :

- une documentation conforme aux standards du projet,
- une discipline stricte des agents IA,
- une traÃ§abilitÃ© complÃ¨te,
- une maÃ®trise des coÃ»ts et du contexte,
- une base stable pour audit, gel et certification.

Toute rÃ©daction de documentation hors de ce protocole est considÃ©rÃ©e comme non conforme.

