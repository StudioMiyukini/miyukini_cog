//! Binaire minimal Miyukini Central.
//!
//! L'UI principale est migrée vers l'app Tauri (apps/central).
//! Ce binaire permet de garder `cargo install -p miyukini-central` et
//! `cargo uninstall -p miyukini-central` fonctionnels.

fn main() {
    eprintln!(
        "Miyukini Central: l'application est désormais lancée via l'app (apps/central)."
    );
    eprintln!("Depuis la racine du dépôt: cargo run -p miyukini-central-native");
    eprintln!();
    eprintln!("Appuyez sur Entrée pour fermer...");
    let mut buf = String::new();
    let _ = std::io::stdin().read_line(&mut buf);
    std::process::exit(0);
}
