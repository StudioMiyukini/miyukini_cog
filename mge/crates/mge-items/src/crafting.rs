use serde::{Deserialize, Serialize};

use crate::rarity::Rarity;
use crate::socketables::RuneKind;

/// Ingredient specification in a cube recipe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecipeIngredient {
    ItemOfRarity(Rarity),
    Rune(RuneKind),
    AnyItem,
}

/// Static definition of a transmutation cube recipe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CubeRecipe {
    pub id: u32,
    pub name: String,
    pub inputs: Vec<RecipeIngredient>,
    pub output_description: String,
}

/// Error returned when no recipe matches the provided ingredients.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecipeError {
    NoMatchingRecipe,
    InsufficientIngredients,
}

/// Find the first recipe whose rarity-based inputs match the provided rarities (order-independent).
pub fn match_recipe<'a>(
    recipes: &'a [CubeRecipe],
    provided_rarities: &[Rarity],
) -> Result<&'a CubeRecipe, RecipeError> {
    for recipe in recipes {
        let expected: Vec<Rarity> = recipe
            .inputs
            .iter()
            .filter_map(|i| if let RecipeIngredient::ItemOfRarity(r) = i { Some(*r) } else { None })
            .collect();

        if expected.is_empty() || expected.len() != provided_rarities.len() {
            continue;
        }

        let mut sorted_expected = expected.clone();
        let mut sorted_provided = provided_rarities.to_vec();
        sorted_expected.sort();
        sorted_provided.sort();

        if sorted_expected == sorted_provided {
            return Ok(recipe);
        }
    }
    Err(RecipeError::NoMatchingRecipe)
}

/// MVP transmutation cube recipes.
pub fn mvp_recipes() -> Vec<CubeRecipe> {
    vec![
        CubeRecipe {
            id: 1,
            name: "3 Normal Items → 1 Magic Item".into(),
            inputs: vec![
                RecipeIngredient::ItemOfRarity(Rarity::Normal),
                RecipeIngredient::ItemOfRarity(Rarity::Normal),
                RecipeIngredient::ItemOfRarity(Rarity::Normal),
            ],
            output_description: "Random magic item of the same type".into(),
        },
        CubeRecipe {
            id: 2,
            name: "3 Magic Items → 1 Rare Item".into(),
            inputs: vec![
                RecipeIngredient::ItemOfRarity(Rarity::Magic),
                RecipeIngredient::ItemOfRarity(Rarity::Magic),
                RecipeIngredient::ItemOfRarity(Rarity::Magic),
            ],
            output_description: "Random rare item".into(),
        },
        CubeRecipe {
            id: 3,
            name: "3× El → 1× Eld (rune upgrade)".into(),
            inputs: vec![
                RecipeIngredient::Rune(RuneKind::El),
                RecipeIngredient::Rune(RuneKind::El),
                RecipeIngredient::Rune(RuneKind::El),
            ],
            output_description: "Eld rune".into(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_normals_match_recipe_1() {
        let recipes = mvp_recipes();
        let rarities = [Rarity::Normal, Rarity::Normal, Rarity::Normal];
        let result = match_recipe(&recipes, &rarities);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, 1);
    }

    #[test]
    fn three_magic_match_recipe_2() {
        let recipes = mvp_recipes();
        let rarities = [Rarity::Magic, Rarity::Magic, Rarity::Magic];
        let result = match_recipe(&recipes, &rarities);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, 2);
    }

    #[test]
    fn mixed_rarities_no_match() {
        let recipes = mvp_recipes();
        let rarities = [Rarity::Normal, Rarity::Magic, Rarity::Rare];
        assert!(match_recipe(&recipes, &rarities).is_err());
    }

    #[test]
    fn mvp_recipe_count_is_three() {
        assert_eq!(mvp_recipes().len(), 3);
    }
}
