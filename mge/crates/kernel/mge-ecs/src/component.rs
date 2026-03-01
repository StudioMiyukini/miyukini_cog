// @id: mge-ecs-component @do: define @role: kernel @layer: 1 @human: miyuk
//
//! Component trait and typed column storage (safe, no unsafe code).
//!
//! Uses `std::any::Any` downcasting instead of raw pointer manipulation
//! to comply with `unsafe_code = "forbid"`.

use std::any::{Any, TypeId};

use crate::errors::EcsError;

// ---------------------------------------------------------------------------
// AnyVecLen — trait object helper for len() without knowing T
// ---------------------------------------------------------------------------

/// Trait allowing us to query the length of a type-erased `Vec<T>`.
pub(crate) trait AnyVecLen: Any {
    /// Returns the number of elements.
    fn vec_len(&self) -> usize;

    /// Returns a reference to self as `&dyn Any`.
    fn as_any(&self) -> &dyn Any;

    /// Returns a mutable reference to self as `&mut dyn Any`.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> AnyVecLen for Vec<T> {
    fn vec_len(&self) -> usize {
        self.len()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// ---------------------------------------------------------------------------
// TypedColumn — SoA column using Any for type safety
// ---------------------------------------------------------------------------

/// Colonne typee utilisant `Any` pour eviter tout code unsafe.
/// Chaque colonne stocke un `Vec<T>` wrape dans `Box<dyn AnyVecLen>`.
pub struct TypedColumn {
    /// Donnees stockees sous forme de `Box<dyn AnyVecLen>`.
    /// Le type reel est `Vec<T>` pour le type T de cette colonne.
    inner: Box<dyn AnyVecLen>,
    /// `TypeId` du composant stocke.
    type_id: TypeId,
}

impl std::fmt::Debug for TypedColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypedColumn")
            .field("type_id", &self.type_id)
            .field("len", &self.inner.vec_len())
            .finish()
    }
}

impl TypedColumn {
    /// Cree une nouvelle colonne typee pour le type T.
    pub fn new<T: 'static + Clone + std::fmt::Debug>() -> Self {
        let inner: Vec<T> = Vec::new();
        Self {
            inner: Box::new(inner),
            type_id: TypeId::of::<T>(),
        }
    }

    /// Ajoute un element a la colonne.
    /// Retourne `Err` si le type ne correspond pas.
    pub fn push<T: 'static + Clone + std::fmt::Debug>(
        &mut self,
        value: T,
    ) -> Result<(), EcsError> {
        let vec = self
            .inner
            .as_any_mut()
            .downcast_mut::<Vec<T>>()
            .ok_or(EcsError::TypeMismatch)?;
        vec.push(value);
        Ok(())
    }

    /// Obtient une reference a l'element a l'index donne.
    pub fn get<T: 'static>(&self, index: usize) -> Result<&T, EcsError> {
        let vec = self
            .inner
            .as_any()
            .downcast_ref::<Vec<T>>()
            .ok_or(EcsError::TypeMismatch)?;
        vec.get(index).ok_or(EcsError::IndexOutOfBounds { index })
    }

    /// Obtient une reference mutable a l'element a l'index donne.
    pub fn get_mut<T: 'static>(&mut self, index: usize) -> Result<&mut T, EcsError> {
        let vec = self
            .inner
            .as_any_mut()
            .downcast_mut::<Vec<T>>()
            .ok_or(EcsError::TypeMismatch)?;
        vec.get_mut(index)
            .ok_or(EcsError::IndexOutOfBounds { index })
    }

    /// Supprime l'element a l'index donne par swap-remove.
    /// Retourne l'element supprime si les types correspondent.
    pub fn swap_remove<T: 'static>(&mut self, index: usize) -> Result<T, EcsError> {
        let vec = self
            .inner
            .as_any_mut()
            .downcast_mut::<Vec<T>>()
            .ok_or(EcsError::TypeMismatch)?;
        if index >= vec.len() {
            return Err(EcsError::IndexOutOfBounds { index });
        }
        Ok(vec.swap_remove(index))
    }

    /// Retourne le nombre d'elements dans la colonne.
    pub fn len(&self) -> usize {
        self.inner.vec_len()
    }

    /// Retourne `true` si la colonne est vide.
    pub fn is_empty(&self) -> bool {
        self.inner.vec_len() == 0
    }

    /// Retourne le `TypeId` du composant stocke dans cette colonne.
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typed_column_push_and_get() {
        let mut col = TypedColumn::new::<i32>();
        col.push(42_i32).unwrap();
        col.push(99_i32).unwrap();

        assert_eq!(col.len(), 2);
        assert_eq!(*col.get::<i32>(0).unwrap(), 42);
        assert_eq!(*col.get::<i32>(1).unwrap(), 99);
    }

    #[test]
    fn test_typed_column_get_mut() {
        let mut col = TypedColumn::new::<i32>();
        col.push(10_i32).unwrap();

        let val = col.get_mut::<i32>(0).unwrap();
        *val = 20;

        assert_eq!(*col.get::<i32>(0).unwrap(), 20);
    }

    #[test]
    fn test_typed_column_swap_remove() {
        let mut col = TypedColumn::new::<i32>();
        col.push(1_i32).unwrap();
        col.push(2_i32).unwrap();
        col.push(3_i32).unwrap();

        let removed = col.swap_remove::<i32>(0).unwrap();
        assert_eq!(removed, 1);
        assert_eq!(col.len(), 2);
        // After swap_remove(0), last element (3) moved to index 0.
        assert_eq!(*col.get::<i32>(0).unwrap(), 3);
        assert_eq!(*col.get::<i32>(1).unwrap(), 2);
    }

    #[test]
    fn test_typed_column_type_mismatch() {
        let mut col = TypedColumn::new::<i32>();
        let result = col.push("oops".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_typed_column_index_out_of_bounds() {
        let col = TypedColumn::new::<i32>();
        let result = col.get::<i32>(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_typed_column_is_empty() {
        let col = TypedColumn::new::<f64>();
        assert!(col.is_empty());
    }
}
