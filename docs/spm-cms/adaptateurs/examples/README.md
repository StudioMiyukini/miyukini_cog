# Exemples d'Implémentations d'Adaptateurs

Ce dossier contient des exemples d'implémentations concrètes d'adaptateurs produits pour différents scénarios.

## Structure

Les exemples sont organisés par type d'implémentation :

- **memory/** : Adaptateurs mémoire (référence, utilisés pour les tests)
- **database/** : Adaptateurs avec base de données (exemples conceptuels)
- **kindmother/** : Adaptateurs avec KindMother (quand disponible)

## Références

Pour comprendre comment créer un adaptateur, voir :

- **Guide général :** `../Miyukini Framework - Guide Adaptateurs Produits.md`
- **Guides spécifiques :** `../Miyukini Framework - Adaptateur *.md`

## Implémentations mémoire

Les implémentations mémoire sont disponibles dans les crates SPM :

- `crates/miyukini-spm-cms-content/src/memory.rs`
- `crates/miyukini-spm-cms-hierarchy/src/memory.rs`
- `crates/miyukini-spm-cms-taxonomies/src/memory.rs`
- `crates/miyukini-spm-cms-media/src/memory.rs`
- `crates/miyukini-spm-cms-publication/src/memory.rs`
- `crates/miyukini-spm-cms-search/src/memory.rs`

Ces implémentations servent de référence pour comprendre le comportement attendu des adaptateurs.

## Exemples à venir

Des exemples d'implémentations avec base de données et KindMother seront ajoutés au fur et à mesure du développement.
