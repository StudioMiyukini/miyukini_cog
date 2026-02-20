# mge-factory-recipe

> @id mge.factory.recipe.v1  
> @role plugin  
> @domain factory  
> @do manage_recipes_transformation_ingredients_products  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-factory-recipe` |
| @id MSCM | `mge.factory.recipe.v1` |
| Domaine | factory |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Non (resolution de recette ponctuelle) |
| Headless safe | Oui |
| Complexite globale | O(r) par requete, r = nombre de recettes |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `RecipeCategory` | `Smelting, Assembling, Refining, Packaging, Custom` | Categorie de la recette. Filtre les machines compatibles |
| `RecipeStatus` | `Locked, Unlocked, Deprecated` | Statut de disponibilite. Locked = pas encore debloque |
| `ItemTier` | `Raw, Processed, Refined, Advanced, Exotic` | Tier de l'item. Influe sur la valeur et la complexite |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Recipe` | `mge.factory.recipe.v1.component.recipe` | `id: u32, name_hash: u64, category: RecipeCategory, duration_ticks: u32, status: RecipeStatus, machine_type_required: MachineType` | Definition de recette. Lie une liste d'ingredients a des produits |
| `Ingredient` | `mge.factory.recipe.v1.component.ingredient` | `recipe_id: u32, item_id: u32, quantity: u32, slot_index: u8` | Ingredient d'une recette. Un recipe_id peut avoir N ingredients |
| `Product` | `mge.factory.recipe.v1.component.product` | `recipe_id: u32, item_id: u32, quantity: u32, slot_index: u8, probability: f32` | Produit de sortie. probability = 1.0 pour garanti, < 1.0 pour sous-produit |
| `RecipeBook` | `mge.factory.recipe.v1.component.recipe_book` | `recipes: Vec<u32>, unlocked_count: u32` | Catalogue des recettes connues. recipes = liste de recipe_id |

---

## 4. Formules

```
Match recette :
  for each recipe in recipe_book:
    if recipe.machine_type_required == machine.machine_type:
      if all(ingredient.item_id in input_slots AND quantity >= required):
        match = true

Validation ingredients :
  valid = true
  for each ingredient in recipe.ingredients:
    slot = input_slots[ingredient.slot_index]
    if slot.item_id != ingredient.item_id OR slot.quantity < ingredient.quantity:
      valid = false

Sous-produit (probabilite) :
  for each product in recipe.products:
    if product.probability < 1.0:
      if rng.next_f32() <= product.probability:
        produce(product)
    else:
      produce(product)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `match_recipe` | `mge.factory.recipe.v1.fn.match_recipe` | 2020 | CraftRequest (event), RecipeBook, Recipe, InputSlot, Machine | Machine (active_recipe_id) | RecipeMatched | O(r) | Cherche une recette compatible avec les inputs et la machine. Assigne si trouvee |
| `validate_ingredients` | `mge.factory.recipe.v1.fn.validate_ingredients` | 2021 | Recipe, Ingredient, InputSlot | none | RecipeFailed | O(i) | Verifie que les slots contiennent les quantites requises. Emet RecipeFailed si non |
| `apply_recipe` | `mge.factory.recipe.v1.fn.apply_recipe` | 2022 | ProductionCompleted (event), Recipe, Product | OutputSlot | RecipeCompleted | O(p) | Quand la production est terminee, genere les produits (avec probabilite pour sous-produits) |
| `unlock_recipe` | `mge.factory.recipe.v1.fn.unlock_recipe` | 2023 | UnlockRecipeRequest (event), RecipeBook, Recipe | Recipe (status), RecipeBook | none | O(1) | Debloque une recette (Locked → Unlocked) |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `RecipeMatched` | `mge.factory.recipe.v1.event.recipe_matched` | `machine_entity: EntityId, recipe_id: u32, duration_ticks: u32` | `match_recipe` | machine (start production), ui |
| `RecipeCompleted` | `mge.factory.recipe.v1.event.recipe_completed` | `machine_entity: EntityId, recipe_id: u32, products: Vec<(u32, u32)>` | `apply_recipe` | ui (production log), analytics |
| `RecipeFailed` | `mge.factory.recipe.v1.event.recipe_failed` | `machine_entity: EntityId, recipe_id: u32, missing_item_id: u32` | `validate_ingredients` | ui (warning), logistics (request item) |

---

## 7. Invariants

- Un `recipe_id` est unique dans le `RecipeBook`.
- Chaque `Ingredient` reference un `recipe_id` existant.
- Chaque `Product` reference un `recipe_id` existant.
- `Product.probability` est borne entre 0.0 (exclus) et 1.0 (inclus).
- Une recette `Locked` ne peut pas etre utilisee par `match_recipe`.
- Le nombre d'ingredients par recette ne depasse pas le nombre de slots d'entree de la machine.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `max_ingredients_per_recipe` | `u32` | 4 | [1, 8] | Nombre max d'ingredients par recette |
| `max_products_per_recipe` | `u32` | 3 | [1, 6] | Nombre max de produits par recette |
| `default_recipe_duration` | `u32` | 120 | [10, 600] | Duree de base d'une recette (ticks) |
| `byproduct_rng_seed_offset` | `u64` | 0 | [0, u64::MAX] | Offset seed pour les sous-produits |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Definit les recettes (ingredients → produits) | Ne gere pas le cycle machine (→ machine) |
| Matche une recette aux inputs disponibles | Ne transporte pas les items (→ conveyor) |
| Valide les ingredients requis | Ne gere pas le stockage global (→ logistics) |
| Genere les produits avec probabilite | Ne gere pas le cout financier (→ tycoon) |
| Gere le deblocage de recettes | Ne gere pas le rendu des recettes |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Recipe, Ingredient, Product, RecipeBook, InputSlot, Machine, CraftRequest, ProductionCompleted |
| Ecrit | Machine (active_recipe_id), Recipe (status), RecipeBook, OutputSlot |
| Emet | RecipeMatched, RecipeCompleted, RecipeFailed |
| Ne touche jamais | MachineState, ProcessingTimer, Conveyor, LogisticsNode, StorageContainer |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-factory-recipe/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.factory.recipe.v1, trait Plugin impl
    ├── components.rs     # Recipe, Ingredient, Product, RecipeBook
    ├── systems.rs        # match_recipe, validate_ingredients, apply_recipe, unlock_recipe
    └── events.rs         # RecipeMatched, RecipeCompleted, RecipeFailed
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 3 enumerations (RecipeCategory, RecipeStatus, ItemTier)
- [ ] Formules de matching et probabilite documentees
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : recipe match, ingredient validation, byproduct probability, unlock
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.factory.recipe.v1","k":"p","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.recipe.v1.component.recipe","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.recipe.v1.component.ingredient","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.recipe.v1.component.product","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.recipe.v1.component.recipe_book","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.recipe.v1.fn.match_recipe","k":"s","d":"factory","r":["RecipeBook","Recipe","InputSlot","Machine"],"w":["Machine"],"e":["RecipeMatched"],"p":2020,"c":"O(r)"},
  {"i":"mge.factory.recipe.v1.fn.validate_ingredients","k":"s","d":"factory","r":["Recipe","Ingredient","InputSlot"],"w":[],"e":["RecipeFailed"],"p":2021,"c":"O(i)"},
  {"i":"mge.factory.recipe.v1.fn.apply_recipe","k":"s","d":"factory","r":["Recipe","Product"],"w":["OutputSlot"],"e":["RecipeCompleted"],"p":2022,"c":"O(p)"},
  {"i":"mge.factory.recipe.v1.fn.unlock_recipe","k":"s","d":"factory","r":["RecipeBook","Recipe"],"w":["Recipe","RecipeBook"],"e":[],"p":2023,"c":"O(1)"},
  {"i":"mge.factory.recipe.v1.event.recipe_matched","k":"e","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.recipe.v1.event.recipe_completed","k":"e","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.recipe.v1.event.recipe_failed","k":"e","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let iron_bar_recipe = world.spawn();
world.insert(iron_bar_recipe, Recipe {
    id: 101,
    name_hash: 0xCAFEBABE,
    category: RecipeCategory::Smelting,
    duration_ticks: 120,
    status: RecipeStatus::Unlocked,
    machine_type_required: MachineType::Smelter,
});
world.insert(iron_bar_recipe, Ingredient {
    recipe_id: 101,
    item_id: 1, // iron_ore
    quantity: 2,
    slot_index: 0,
});
world.insert(iron_bar_recipe, Product {
    recipe_id: 101,
    item_id: 10, // iron_bar
    quantity: 1,
    slot_index: 0,
    probability: 1.0,
});

let book = world.spawn();
world.insert(book, RecipeBook {
    recipes: vec![101],
    unlocked_count: 1,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Factory - Index](_index.md) | Vue d'ensemble du pack |
| [mge-factory-machine](mge-factory-machine.md) | Plugin machines (consomme les recettes) |
