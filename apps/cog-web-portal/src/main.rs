//! COG Web Portal — Point d'entrée du portail HTTP externe.
//!
//! @id: cog_web_portal_main @do: start_portal_http_server
//! @role: infra @layer: service
//! @human: Lance le serveur HTTP axum du COG Web Portal.

fn main() {
    println!("COG Web Portal — stub (E00 smoke test placeholder)");
}

#[cfg(test)]
mod tests {
    /// Smoke test E00 — RED intentionnel.
    /// Ce test échoue pour valider la structure du plan avant TDD.
    /// Il sera rendu GREEN en E04 lors de l'implémentation réelle.
    #[test]
    fn smoke_portal_not_yet_implemented() {
        // Ce test doit échouer : le portail n'est pas encore implémenté.
        // RED intentionnel — invariant I-6 TDD.
        panic!("E00 SMOKE RED — portal not yet implemented. Start E01.");
    }
}
