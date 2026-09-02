//! @id     games_allumina_build
//! @do     main
//! @role   ui
//! @layer  core
//! @human  games allumina — module « build ».

//! Build script: on Windows, embed the application icon and version metadata
//! into the executable so it looks like a proper distributable (taskbar icon,
//! Explorer "Details" tab). No-op on every other platform and whenever the
//! icon file is absent (e.g. a fresh checkout that hasn't generated `dist/`).

fn main() {
    #[cfg(windows)]
    {
        let icon = "dist/allumina.ico";
        if std::path::Path::new(icon).exists() {
            let mut res = winresource::WindowsResource::new();
            res.set_icon(icon);
            res.set("ProductName", "Allumina");
            res.set("FileDescription", "Allumina — ARPG");
            res.set("CompanyName", "Studio Miyukini");
            res.set("LegalCopyright", "© Studio Miyukini");
            if let Err(e) = res.compile() {
                println!("cargo:warning=icon embed skipped: {e}");
            }
        }
        println!("cargo:rerun-if-changed=dist/allumina.ico");
    }
}
