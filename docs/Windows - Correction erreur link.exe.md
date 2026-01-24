# Windows — Correction erreur `link.exe` not found

## L’erreur

```
error: linker `link.exe` not found
  |  = note: program not found
note: the msvc targets depend on the msvc linker but `link.exe` was not found
note: please ensure that Visual Studio 2017 or later, or Build Tools for Visual Studio
      were installed with the Visual C++ option.
```

## Explication

Sous Windows, Rust utilise par défaut la **toolchain MSVC** (`x86_64-pc-windows-msvc`) :

1. **Compilation** : `rustc` produit des fichiers objet (`.obj`) — ça fonctionne sans outil Microsoft.
2. **Édition de liens** : pour produire l’exécutable (`.exe`), Cargo appelle le **linker Microsoft** `link.exe`.

Si Visual Studio (ou Build Tools) avec les **outils C++** n’est pas installé, `link.exe` est absent : la compilation Rust réussit, l’édition de liens échoue.

## Solution recommandée : installer les Visual Studio Build Tools

1. **Télécharger** :  
   [Build Tools pour Visual Studio 2022](https://visualstudio.microsoft.com/fr-fr/visual-cpp-build-tools/)

2. **Lancer l’installateur** et choisir la charge **« Développement Desktop en C++ »**  
   (Desktop development with C++). C’est elle qui installe le compilateur et **`link.exe`**.

3. **Redémarrer le terminal** (ou la session) après l’installation pour que le `PATH` soit à jour.

4. **Reconstruire** :
   ```powershell
   cargo build -p demo-logging-lifecycle
   cargo run -p demo-logging-lifecycle
   ```

## Variante : invitant « Développeur » de Visual Studio

Si Build Tools (ou Visual Studio) est déjà installé avec la charge C++ :

- Ouvrir **« Invite de commandes Développeur pour VS 2022 »** ou **« Developer PowerShell for VS 2022 »**  
  (menu Démarrer).
- Ces invités configurent le `PATH` pour `link.exe`.
- Depuis cette fenêtre : `cargo build -p demo-logging-lifecycle`, etc.

## Autre option : toolchain GNU (sans Visual Studio)

Pour ne pas installer Visual Studio, on peut utiliser la cible **GNU** et un linker MinGW :

1. Cible déjà ajoutée si besoin :  
   `rustup target add x86_64-pc-windows-gnu`

2. **Installer MinGW** (par ex. via [MSYS2](https://www.msys2.org/)) et mettre `bin` (avec `x86_64-w64-mingw32-gcc`) dans le `PATH`.

3. Compiler avec la cible GNU :
   ```powershell
   cargo build -p demo-logging-lifecycle --target x86_64-pc-windows-gnu
   ```

La solution la plus simple reste en général **Build Tools + charge C++**.
