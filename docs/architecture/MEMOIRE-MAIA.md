# Mémoire de MAIA — mémoire stratifiée

<!-- @id arch.ia.memoire-maia -->
<!-- @role architecture-specification -->
<!-- @layer 8 -->
<!-- @human Mémoire hiérarchique de MAIA : légère, indexable, économe en tokens, utilisable par un petit modèle ou une chaîne de modèles par strates -->
<!-- @do design_maia_memory -->

- **Statut** : v1, 2026-10-24
- **Base** : [modèle zones et couches](MODELE-ZONES-ET-COUCHES-COG.md) (couche 8) · [services hybrides](SERVICES-HYBRIDES-ET-PORTAIL.md) · crate existante `miyualicia-memory` (types Fact, Preference, Episode, Persona, LearnedSkill)

## 1. Objectifs

<!-- @id arch.ia.memoire.objectifs -->
<!-- @role design-goals -->
<!-- @do state_memory_goals -->

- **Légère** : stockage local, aucun service externe, tient sur un mini-PC
- **Indexable** : identifiants hiérarchiques, plein texte, vecteurs, graphe, temps
- **Économe** : contexte assemblé **sous budget de tokens**, jamais de vidage brut
- **Hiérarchique** : du résumé global au détail, chargée par **zoom progressif**
- **Utilisable par un petit modèle** (contexte 4 k) comme par une **chaîne de modèles par strates**
- **Sûre** : zone COG, données `nationale`, provenance tracée, résistante à l'empoisonnement
- **Souveraine** : appartient à l'utilisateur, consultable, corrigeable, oubliable

## 2. Strates de résolution

<!-- @id arch.ia.memoire.strates -->
<!-- @role memory-hierarchy -->
<!-- @do define_memory_levels -->

- **N0 · Noyau** (≤ 200 tokens) — toujours chargé
  - identité de l'utilisateur, règles qu'il a posées, préférences fortes
- **N1 · Carte** (≤ 40 tokens par domaine) — table des matières
  - domaines (travail, famille, santé, projets…) + entités clés nommées une fois
- **N2 · Sujets** (≤ 150 tokens) — un résumé par sujet, projet, personne
- **N3 · Notes** (≤ 60 tokens) — faits, épisodes, procédures atomiques
- **N4 · Sources** — documents, conversations, fichiers : **jamais injectés**, référencés par empreinte
- Chaque nœud N1 / N2 **résume ses enfants** et liste leurs identifiants
- Lecture **descendante** : le modèle voit N0 + N1, puis ouvre seulement les branches utiles

## 3. Unités de mémoire

<!-- @id arch.ia.memoire.unites -->
<!-- @role memory-types -->
<!-- @do define_memory_units -->

- **Fait** — affirmation sur le monde ou l'utilisateur
- **Préférence** — goût, habitude, réglage
- **Règle** — consigne durable donnée **explicitement par l'utilisateur** (jamais déduite)
- **Épisode** — événement daté (conversation, action, résultat)
- **Procédure** — savoir-faire appris (compétence)
- **Entité** — fiche personne / lieu / objet / projet
- **Résumé** — nœud de synthèse N1 / N2 (généré, régénérable)
- Champs communs :
  - `id` hiérarchique (§4)
  - texte court
  - importance (0–9), confiance (0–9)
  - dates : création, dernier usage
  - **provenance** : `utilisateur`, `deduit`, `etranger`
  - liens : parent, entités, sources (N4)
  - version (ne s'écrase jamais, §9)

## 4. Identifiants hiérarchiques

<!-- @id arch.ia.memoire.identifiants -->
<!-- @role memory-indexing -->
<!-- @do address_memory_by_path -->

- Chemin pointé, même convention que les `@id` MSCM : `domaine.sujet.note`
- Exemples : `travail.cog.f12`, `famille.lea.p3`, `sante.sommeil.e7`
- Préfixe = sous-arbre : `travail.cog` désigne tout le projet
- Suffixe de type : `f` fait, `p` préférence, `r` règle, `e` épisode, `k` procédure, `n` entité, `s` résumé
- Stables et courts : un modèle peut les citer et demander à les ouvrir

## 5. Format compact en contexte

<!-- @id arch.ia.memoire.format -->
<!-- @role token-format -->
<!-- @do render_memory_compactly -->

- **Une ligne par unité**, ordre de champs fixe, aucun JSON
- Grammaire : `id texte [· c<confiance>] [· <date relative>] [· ↗<ids liés>]`
- Confiance affichée seulement si < 7 ; dates **relatives** (« il y a 3 j ») ; entités nommées une fois en N1 puis réutilisées
- Branches non ouvertes résumées en une ligne avec `…` et leur identifiant
- Bloc délimité, annoncé comme **données** :

```
<memoire>
N0 Utilisateur : Miyukini, dev Rust, serveur maison Ubuntu. Répondre en français, concis.
N1 travail(3 projets) · famille(Léa=fille) · sante(sommeil) · maison(domotique)
travail.cog.s1 Projet COG : écosystème souverain Rust, sécurité en cours (lot 2) ↗travail.cog.f12
travail.cog.f12 Racine de confiance sur USB, sauvegarde SLIP-39 2 sur 3 · il y a 1 j
famille.lea.s1 …
</memoire>
```

## 6. Indexation et rappel

<!-- @id arch.ia.memoire.indexation -->
<!-- @role retrieval -->
<!-- @do index_and_recall_memory -->

- **Stockage** : SQLite (libSQL) via KindMother, chiffré, une base par utilisateur
- **Index** :
  - chemin (préfixe) → navigation hiérarchique
  - plein texte FTS5 (BM25) → mots exacts, noms propres
  - vecteurs locaux **quantifiés** (int8, petit modèle d'embedding local) → sens
  - graphe d'entités et de liens → voisinage
  - temps → récence, épisodes
- **Score de rappel** = pertinence × importance × fraîcheur × confiance
- **Diversité** : pas plus de N notes d'une même branche par réponse

## 7. Assemblage sous budget

<!-- @id arch.ia.memoire.budget -->
<!-- @role context-assembly -->
<!-- @do assemble_context_under_token_budget -->

- Budgets par profil :
  - petit modèle (contexte 4 k) : mémoire ≤ 600 tokens
  - moyen (8 k) : ≤ 1 500 tokens
  - grand (32 k et +) : ≤ 4 000 tokens
- Ordre de remplissage :
  1. N0 (toujours)
  2. N1 compressée
  3. N2 / N3 par score décroissant, jusqu'au budget
  4. le reste → une ligne `…` avec identifiant
- Zoom à la demande : outil `memoire.ouvrir(id)` (lecture seule, sans effet de bord)
- Comptage réel des tokens avec le tokenizer du modèle actif

## 8. Chaîne de modèles par strates

<!-- @id arch.ia.memoire.strata -->
<!-- @role model-pipeline -->
<!-- @do run_memory_through_model_strata -->

- **S0 · Déterministe** (sans LLM) : rappel par index, score, budget
- **S1 · Petit modèle** (≈ 1–3 B) : reclasse, fusionne les doublons, compresse les notes trop longues pour tenir dans le budget
- **S2 · Modèle principal** : répond à l'utilisateur à partir du paquet final
- Un petit modèle seul peut tout faire : S0 + S2 réduit, budget petit modèle
- Écriture : S1 **propose** des notes candidates à partir de la conversation ; elles ne deviennent mémoire qu'après validation (§9)

## 9. Écriture, consolidation, oubli

<!-- @id arch.ia.memoire.cycle-de-vie -->
<!-- @role memory-lifecycle -->
<!-- @do write_consolidate_forget_memory -->

- **Écriture**
  - propos explicites de l'utilisateur → acceptés directement
  - déductions → confiance réduite, validées en lot par l'utilisateur
  - types sensibles (santé, finances, tiers) → validation TAMR
  - **Règles** : uniquement sur demande explicite de l'utilisateur
- **Versionnage** : jamais d'écrasement ; une correction crée une nouvelle version qui remplace l'ancienne ; historique dans le registre (KindMother)
- **Consolidation (« sommeil »)**, tâche de fond hors usage :
  - fusion des doublons
  - régénération des résumés N1 / N2
  - baisse d'importance des notes jamais rappelées
  - détection des contradictions → question à l'utilisateur
- **Oubli** : l'utilisateur peut oublier une note ou une branche → effacement réel et tracé (pierre tombale sans contenu)
- **Consultation** : l'utilisateur voit, corrige, exporte toute sa mémoire

## 10. Sécurité

<!-- @id arch.ia.memoire.securite -->
<!-- @role memory-security -->
<!-- @layer 8 -->
<!-- @do protect_memory_from_poisoning_and_injection -->

- Mémoire en **zone COG**, étiquetée `nationale` : ne sort jamais, ni vers la Franche ni vers la Fédération
- **Une mémoire par utilisateur**, clé de chiffrement propre ; aucun partage entre personnes du COG
- **Provenance `etranger`** (Fiches fédérées, documents relayés) :
  - jamais écrite en mémoire longue sans validation de l'utilisateur
  - ne peut jamais créer de **Règle**
- Contenu mémoire rendu comme **données délimitées**, jamais comme consignes système
- Les **Règles** n'existent que par décision explicite de l'utilisateur : une injection ne peut pas installer de consigne durable
- Aucun apprentissage (fine-tuning) automatique sur la mémoire
- Accès de MAIA à la mémoire par intention lue par StrongFather ; ouverture de branche sensible tracée

## 11. Intégration

<!-- @id arch.ia.memoire.integration -->
<!-- @role integration-plan -->
<!-- @do integrate_maia_memory -->

- Partir des types de `miyualicia-memory` (Fact, Preference, Episode, Persona, LearnedSkill) : une mémoire commune MAIA / Alicia
- Ajouter : identifiants hiérarchiques, strates N0–N4, Règle, Résumé, provenance, budget en tokens
- Remplacer `ContextOptions` (nombres d'éléments) par un **budget en tokens**
- Stockage par KindMother ; embeddings calculés localement par MAIA

## 12. Exigences

<!-- @id arch.ia.memoire.exigences -->
<!-- @role security-requirements -->
<!-- @do derive_memory_requirements -->

- **EX-50** Contexte mémoire assemblé sous budget de tokens mesuré, N0 toujours présent
- **EX-51** Identifiants hiérarchiques stables, ouverture de branche à la demande
- **EX-52** Notes versionnées, aucune écriture destructive hors oubli demandé
- **EX-53** Provenance obligatoire ; `etranger` jamais en mémoire longue sans validation
- **EX-54** Règles créées uniquement par l'utilisateur
- **EX-55** Une mémoire chiffrée par utilisateur, étiquetée `nationale`
- **EX-56** Mémoire consultable, corrigeable, exportable et oubliable par l'utilisateur
