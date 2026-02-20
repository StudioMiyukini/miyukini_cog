//! EventQueue — file d'événements typés, buffer double.
//!
//! Écriture pendant le tick N, lecture au tick N+1.
//!
//! @id mge.kernel.event_queue
//! @role simulation
//! @layer kernel
//! @human File d'événements typés avec double buffer
//! @do store_and_serve_typed_events

use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::event::Event;

/// File d'événements typés. Double buffer : write pendant tick N, lecture au tick N+1.
pub struct EventQueue {
    write_buffer: HashMap<TypeId, Vec<Box<dyn Any + Send + Sync>>>,
    read_buffer: HashMap<TypeId, Vec<Box<dyn Any + Send + Sync>>>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            write_buffer: HashMap::new(),
            read_buffer: HashMap::new(),
        }
    }

    /// Écrit un événement dans le buffer d'écriture (lu au tick suivant).
    pub fn emit<E: Event>(&mut self, event: E) {
        self.write_buffer
            .entry(TypeId::of::<E>())
            .or_default()
            .push(Box::new(event));
    }

    /// Swap des buffers : write ↔ read, clear write. Appelé au début de chaque tick.
    pub fn swap(&mut self) {
        std::mem::swap(&mut self.write_buffer, &mut self.read_buffer);
        self.write_buffer.clear();
    }

    /// Retourne true si le buffer de lecture contient au moins un événement de type E.
    pub fn has<E: Event>(&self) -> bool {
        self.read_buffer
            .get(&TypeId::of::<E>())
            .is_some_and(|v| !v.is_empty())
    }

    /// Nombre d'événements de type E dans le buffer de lecture.
    pub fn count<E: Event>(&self) -> usize {
        self.read_buffer
            .get(&TypeId::of::<E>())
            .map_or(0, Vec::len)
    }

    /// Itère sur les événements du type E (buffer de lecture = tick précédent).
    pub fn iter<E: Event>(&self) -> EventIter<'_, E> {
        let vec = self.read_buffer.get(&TypeId::of::<E>());
        let slice = vec.map(|v| v.as_slice()).unwrap_or(&[]);
        EventIter {
            inner: slice.iter(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Itérateur sur les événements de type E.
pub struct EventIter<'a, E> {
    inner: std::slice::Iter<'a, Box<dyn Any + Send + Sync>>,
    _phantom: std::marker::PhantomData<E>,
}

impl<'a, E: Event> Iterator for EventIter<'a, E> {
    type Item = &'a E;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|boxed| {
            boxed.downcast_ref::<E>().expect("EventIter: invariant TypeId violated")
        })
    }
}
