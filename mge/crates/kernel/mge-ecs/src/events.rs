// @id: mge-ecs-events @do: define @role: kernel @layer: 1 @human: miyuk
//
//! Double-buffered event system for inter-system communication.
//!
//! Events written in one frame are readable in the following frame.
//! The scheduler calls `swap_buffers()` once per frame.

/// File d'evenements double-buffer pour communication entre systemes.
/// Les evenements ecrits dans un frame sont lisibles dans le frame suivant.
#[derive(Debug)]
pub struct Events<T: std::fmt::Debug> {
    /// Buffer courant (ecritures).
    write_buffer: Vec<T>,
    /// Buffer precedent (lectures).
    read_buffer: Vec<T>,
}

impl<T: std::fmt::Debug> Events<T> {
    /// Cree une nouvelle file d'evenements vide.
    pub fn new() -> Self {
        Self {
            write_buffer: Vec::new(),
            read_buffer: Vec::new(),
        }
    }

    /// Envoie un evenement (ecrit dans le buffer courant).
    pub fn send(&mut self, event: T) {
        self.write_buffer.push(event);
    }

    /// Lit tous les evenements du frame precedent.
    pub fn read(&self) -> &[T] {
        &self.read_buffer
    }

    /// Swap les buffers. Appele une fois par frame par le scheduler.
    /// Le write buffer devient le read buffer, et le write buffer est vide.
    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.write_buffer, &mut self.read_buffer);
        self.write_buffer.clear();
    }

    /// Retourne le nombre d'evenements dans le buffer de lecture.
    pub fn read_count(&self) -> usize {
        self.read_buffer.len()
    }

    /// Retourne le nombre d'evenements en attente d'ecriture.
    pub fn pending_count(&self) -> usize {
        self.write_buffer.len()
    }
}

impl<T: std::fmt::Debug> Default for Events<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Wrapper pour ecrire des evenements depuis un systeme.
pub struct EventWriter<'a, T: std::fmt::Debug> {
    events: &'a mut Events<T>,
}

impl<'a, T: std::fmt::Debug> EventWriter<'a, T> {
    /// Cree un nouveau writer lie a une file d'evenements.
    pub fn new(events: &'a mut Events<T>) -> Self {
        Self { events }
    }

    /// Envoie un evenement.
    pub fn send(&mut self, event: T) {
        self.events.send(event);
    }
}

/// Wrapper pour lire des evenements depuis un systeme.
pub struct EventReader<'a, T: std::fmt::Debug> {
    events: &'a Events<T>,
}

impl<'a, T: std::fmt::Debug> EventReader<'a, T> {
    /// Cree un nouveau reader lie a une file d'evenements.
    pub fn new(events: &'a Events<T>) -> Self {
        Self { events }
    }

    /// Lit tous les evenements du frame precedent.
    pub fn read(&self) -> &[T] {
        self.events.read()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct DamageEvent {
        amount: f32,
    }

    #[test]
    fn test_send_and_read_events() {
        let mut events = Events::<DamageEvent>::new();

        // Frame 1 : ecrire des evenements.
        events.send(DamageEvent { amount: 10.0 });
        events.send(DamageEvent { amount: 25.5 });

        // Pas encore lisible (dans le write buffer).
        assert!(events.read().is_empty());
        assert_eq!(events.pending_count(), 2);

        // Swap : les evenements passent dans le read buffer.
        events.swap_buffers();

        // Maintenant lisible.
        assert_eq!(events.read().len(), 2);
        assert!((events.read()[0].amount - 10.0).abs() < f32::EPSILON);
        assert!((events.read()[1].amount - 25.5).abs() < f32::EPSILON);

        // Un nouveau swap efface les anciens evenements.
        events.swap_buffers();
        assert!(events.read().is_empty());
    }

    #[test]
    fn test_event_writer_and_reader() {
        let mut events = Events::<DamageEvent>::new();

        {
            let mut writer = EventWriter::new(&mut events);
            writer.send(DamageEvent { amount: 5.0 });
        }

        events.swap_buffers();

        {
            let reader = EventReader::new(&events);
            assert_eq!(reader.read().len(), 1);
            assert!((reader.read()[0].amount - 5.0).abs() < f32::EPSILON);
        }
    }

    #[test]
    fn test_events_default() {
        let events = Events::<DamageEvent>::default();
        assert!(events.read().is_empty());
        assert_eq!(events.pending_count(), 0);
    }
}
