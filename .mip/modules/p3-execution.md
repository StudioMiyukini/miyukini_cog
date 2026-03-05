# Module MIP — P3 Implémentation

> Ce module est chargé au début de P3 (toutes classes).

---

## Agents

**François** (back-end) + **Lise** (front-end) en PARALLÈLE. Denis coordonne les checkpoints.

**Exécution par subagent frais** : Chaque tâche est exécutée par un subagent frais pour éviter la pollution de contexte.

---

## Test fumée prioritaire (AVANT TDD tâche par tâche)

Denis écrit un **test d'intégration de bout en bout** du parcours principal (happy path). Ce test DOIT :
- **Compiler** (sinon le plan a un défaut structurel -> corriger avant de continuer)
- **Échouer** (la fonctionnalité n'existe pas encore — c'est attendu)

```python
# Exemple test fumée (Python)
def test_smoke_feature_happy_path():
    # Compile mais échoue (RED) -> attendu
    result = FeatureModule(config=default_config())
    assert result.is_ok()
```

```rust
// Exemple test fumée (Rust)
#[test]
fn smoke_feature_happy_path() {
    // Compile mais échoue (RED) -> attendu
    let result = FeatureModule::new(Config::default());
    assert!(result.is_ok());
}
```

```typescript
// Exemple test fumée (TypeScript)
test('smoke feature happy path', () => {
    // Compile mais échoue (RED) -> attendu
    const result = new FeatureModule(defaultConfig());
    expect(result.isOk()).toBe(true);
});
```

---

## Pré-vol par tâche (avant d'écrire le code)

1. **Lire la tâche** depuis le plan exhaustif (fichier, code attendu, test)
2. **Spot-check docs** si la tâche touche une API externe ou un pattern framework :
   - Vérifier le pattern via la documentation
   - Ex : composant React -> vérifier la syntaxe des hooks
   - Ex : handler Express -> vérifier la chaîne middleware
3. **Charger le contexte anti-patterns** : patterns-and-lessons.md section « Erreurs à ne pas répéter » (lignes 27-35) ou MEMORY.md synthèse « Erreurs critiques » (~10 lignes). Pièges framework = extrait pertinent (ex. dioxus RSX pour Lise). 1× au démarrage P3, pas avant chaque tâche.

---

## Cycle TDD par tâche (10 pas)

| # | Pas | Action |
|---|-----|--------|
| 1 | **START** | Annoter la tâche dans le plan : `Démarré à HH:MM` |
| 2 | **RED** | Écrire le test qui échoue |
| 3 | **GREEN** | Écrire le code minimal pour faire passer le test |
| 4 | **REFACTOR** | Nettoyer si nécessaire |
| 5 | **VERIFY** | Lancer les tests du projet (ex : `cargo test -p {crate}`, `npm test`, `pytest`) |
| 6 | **LINT** | Lancer le linter (ex : `cargo clippy`, `eslint`, `ruff`) — sortie propre |
| 7 | **COMMIT** | Commit atomique avec message conventionnel |
| 8 | **PUSH** | `git push` sur la branche feature |
| 9 | **LOG** | TodoWrite : marquer la tâche `completed` |
| 10 | **TRACK** | Annoter le plan : `Terminé à HH:MM avec [model] pour N tokens (mesurés)` |

> Les commandes build/test/lint sont lues depuis `.mip/environment.md` (pas en dur).

---

## Checkpoint intermédiaire (toutes les 5 tâches)

Denis réalise un mini-audit :

1. Build des paquets modifiés
2. Lint des paquets modifiés
3. Vérifier que les tâches précédentes ne sont pas cassées
4. **Spot-check sécurité Victor** (si la tâche touche auth, crypto, validation, secrets) :
   - Grep des patterns dangereux : `unwrap()`, URLs en dur, secrets en dur, `eval()`, SQL non paramétré
   - Vérifier les algorithmes crypto (tableau approuvé)
   - Vérifier la validation des entrées utilisateur
5. **Spot-check efficience Jean** : tokens/ligne, fichiers chargés inutilement, boucles de correction >3 tentatives. Si anomalie -> recommandation (ajustement prompt, changement de modèle). Autorité CONSULTATIVE.
6. Si régression -> corriger avant de continuer
7. `git push` — pousser l'état courant

---

## Annonce d'étape

À chaque étape complétée du plan :

```
[YYYY-MM-DD HH:MM] Étape X/<total> — <nom> terminée.
  Tâches : X/Y terminées | Tests : X passés | Commits : N
  Prochaine étape : <nom>
```

**Comportement par mode** :
- **FULL** : Annonce informative, exécution continue
- **BIG_STEPS** : Annonce informative en P3, gate entre P3->P4
- **GUIDED** : Attendre validation utilisateur (VALIDER / MODIFIER / ANNULER / SAUTER)

---

## Parallélisme

François et Lise travaillent simultanément quand leurs tâches sont indépendantes. Les tâches avec dépendances sont séquencées par Denis.

> Pour T4-T5 : voir `.mip/modules/mass.md` pour la parallélisation avancée par DAG et vagues.

---

## Auto-correction

Si un test échoue :

1. Lire le message d'erreur, identifier la cause (analyse de cause racine)
2. Vérifier contre la doc si problème de pattern/API
3. Corriger et re-tester (**tentative 1**)
4. Si échec -> corriger différemment (**tentative 2**)
5. Si échec -> **frein d'urgence** avec diagnostic complet

---

## Gate P3

Chaque tâche passe test + lint.

### Gate BIG_STEPS (P3->P4)

Denis présente un résumé :

```
[YYYY-MM-DD HH:MM] Résumé P3 — Implémentation terminée.
  Étapes : X/X | Tâches : X/X | Tests : X passés, Y échoués | Commits : N
  Auto-corrections : N | Lignes écrites : N
  Continuer vers P4 (Intégration et Audit) ?
  [CONTINUER] / [CORRIGER : <instructions>] / [ARRÊTER]
```
