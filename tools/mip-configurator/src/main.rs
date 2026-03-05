use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    io::{self, Write},
    path::{Component, Path, PathBuf},
};

type R<T> = Result<T, Box<dyn std::error::Error>>;
const ENV_FILE: &str = "environment.md";
const ACTIVE_FILE: &str = "profiles/active";
const SUB_FILE: &str = "config/subscriptions.md";
const STATE_FILE: &str = "config/mip-configurator.state.json";

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Config {
    date: String,
    active_profile: String,
    languages: String,
    frameworks: String,
    db: String,
    build: String,
    test: String,
    lint: String,
    fmt: String,
    tool: String,
    models: String,
    budget: String,
    security: String,
    deployment: String,
    subs_enabled: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Sub {
    provider: String,
    plan: String,
    tokens: Option<u64>,
    period: String,
    active: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct State {
    cfg: Config,
    subs: Vec<Sub>,
}
#[derive(Clone, Debug)]
struct Sandbox {
    mip: PathBuf,
    workspace: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            date: today(),
            active_profile: "anthropic-opus".into(),
            languages: "Rust".into(),
            frameworks: "Dioxus 0.6, axum".into(),
            db: "KindMother (SQLite), SQLCipher".into(),
            build: "cargo build --workspace".into(),
            test: "cargo test --workspace".into(),
            lint: "cargo clippy --workspace -- -D warnings".into(),
            fmt: "cargo fmt --all".into(),
            tool: "Cursor".into(),
            models: "Claude Sonnet/Opus".into(),
            budget: "a definir".into(),
            security: "standard".into(),
            deployment: "local / VPS".into(),
            subs_enabled: false,
        }
    }
}

impl Sandbox {
    fn discover(mut d: PathBuf) -> R<Self> {
        loop {
            let m = d.join(".mip");
            if m.is_dir() {
                return Ok(Self {
                    mip: fs::canonicalize(m)?,
                    workspace: fs::canonicalize(d)?,
                });
            }
            if !d.pop() {
                break;
            }
        }
        Err("Aucun dossier .mip trouve".into())
    }
    fn rel(&self, p: &str) -> R<PathBuf> {
        let p = Path::new(p);
        if p.is_absolute() {
            return Err("Chemin absolu interdit".into());
        }
        let mut out = PathBuf::new();
        for c in p.components() {
            match c {
                Component::Normal(v) => out.push(v),
                Component::CurDir => {}
                _ => return Err("Sortie de .mip interdite".into()),
            }
        }
        if out.as_os_str().is_empty() {
            return Err("Chemin vide".into());
        }
        Ok(self.mip.join(out))
    }
    fn read_opt(&self, p: &str) -> R<Option<String>> {
        let f = self.rel(p)?;
        if !f.exists() {
            return Ok(None);
        }
        let c = fs::canonicalize(&f)?;
        if !c.starts_with(&self.mip) {
            return Err("Sortie de .mip interdite".into());
        }
        Ok(Some(fs::read_to_string(c)?))
    }
    fn write(&self, p: &str, s: &str) -> R<()> {
        let f = self.rel(p)?;
        let parent = f.parent().ok_or("Parent introuvable")?;
        fs::create_dir_all(parent)?;
        let cp = fs::canonicalize(parent)?;
        if !cp.starts_with(&self.mip) {
            return Err("Sortie de .mip interdite".into());
        }
        fs::write(f, s)?;
        Ok(())
    }
}

fn main() -> R<()> {
    let sb = Sandbox::discover(env::current_dir()?)?;
    let mut st = load_state(&sb)?;
    match env::args()
        .nth(1)
        .unwrap_or_else(|| "config".into())
        .as_str()
    {
        "wizard" => {
            wizard(&sb, &mut st)?;
            save(&sb, &st)?;
        }
        "config" => menu(&sb, &mut st)?,
        "status" => status(&sb, &st)?,
        _ => help(),
    }
    Ok(())
}

fn help() {
    println!("mip-configurator: cargo run -p mip-configurator -- [config|wizard|status]");
}

fn load_state(sb: &Sandbox) -> R<State> {
    if let Some(raw) = sb.read_opt(STATE_FILE)? {
        return Ok(serde_json::from_str(&raw)?);
    }
    let mut cfg = Config::default();
    if let Some(a) = sb.read_opt(ACTIVE_FILE)? {
        let s = a.trim();
        if !s.is_empty() {
            cfg.active_profile = s.into();
        }
    }
    Ok(State {
        cfg,
        subs: parse_subs(&sb.read_opt(SUB_FILE)?.unwrap_or_default()),
    })
}

fn menu(sb: &Sandbox, st: &mut State) -> R<()> {
    loop {
        println!(
            "\n=== MIP Configurator ===\nPerimetre verrouille: {}",
            sb.mip.display()
        );
        let o = vec![
            "Statut",
            "Setup wizard",
            "Profil actif",
            "Commandes",
            "Abonnements",
            "Sauvegarder+Quitter",
            "Quitter sans sauvegarde",
        ];
        match pick("Action", &o, 0)? {
            0 => status(sb, st)?,
            1 => wizard(sb, st)?,
            2 => set_profile(sb, st)?,
            3 => set_cmds(st)?,
            4 => set_subs(st)?,
            5 => {
                save(sb, st)?;
                println!("Sauvegarde OK.");
                break;
            }
            6 => break,
            _ => {}
        }
    }
    Ok(())
}

fn wizard(sb: &Sandbox, st: &mut State) -> R<()> {
    println!("\n=== Setup Wizard MIP ===");
    set_profile(sb, st)?;
    st.cfg.languages = ask("Langages", &st.cfg.languages)?;
    st.cfg.frameworks = ask("Frameworks", &st.cfg.frameworks)?;
    st.cfg.db = ask("Bases de donnees", &st.cfg.db)?;
    st.cfg.security = ask("Niveau securite", &st.cfg.security)?;
    st.cfg.deployment = ask("Deploiement", &st.cfg.deployment)?;
    st.cfg.tool = ask("Outil IA principal", &st.cfg.tool)?;
    st.cfg.models = ask("Modeles IA", &st.cfg.models)?;
    st.cfg.budget = ask("Budget IA", &st.cfg.budget)?;
    set_cmds(st)?;
    st.cfg.subs_enabled = ask_bool("Activer suivi abonnements", st.cfg.subs_enabled)?;
    if st.cfg.subs_enabled {
        set_subs(st)?;
    }
    st.cfg.date = today();
    Ok(())
}

fn set_profile(sb: &Sandbox, st: &mut State) -> R<()> {
    let mut p = profile_slugs(sb)?;
    if p.is_empty() {
        p.push(st.cfg.active_profile.clone());
    }
    let d = p
        .iter()
        .position(|v| v == &st.cfg.active_profile)
        .unwrap_or(0);
    let i = pick(
        "Profil MIP actif",
        &p.iter()
            .map(std::string::String::as_str)
            .collect::<Vec<_>>(),
        d,
    )?;
    st.cfg.active_profile = p[i].clone();
    Ok(())
}

fn profile_slugs(sb: &Sandbox) -> R<Vec<String>> {
    let mut v = Vec::new();
    for d in ["profiles/builtin", "profiles/custom"] {
        let p = sb.rel(d)?;
        if !p.exists() {
            continue;
        }
        let c = fs::canonicalize(&p)?;
        if !c.starts_with(&sb.mip) {
            return Err("Sortie de .mip interdite".into());
        }
        for e in fs::read_dir(c)? {
            let e = e?;
            let p = e.path();
            if p.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
                    if !stem.trim().is_empty() {
                        v.push(stem.to_string());
                    }
                }
            }
        }
    }
    v.sort();
    v.dedup();
    Ok(v)
}

fn set_cmds(st: &mut State) -> R<()> {
    st.cfg.build = ask("Build", &st.cfg.build)?;
    st.cfg.test = ask("Test", &st.cfg.test)?;
    st.cfg.lint = ask("Lint", &st.cfg.lint)?;
    st.cfg.fmt = ask("Format", &st.cfg.fmt)?;
    Ok(())
}

fn set_subs(st: &mut State) -> R<()> {
    loop {
        println!("\n--- Abonnements ---");
        if st.subs.is_empty() {
            println!("Aucun abonnement.")
        } else {
            for (i, s) in st.subs.iter().enumerate() {
                println!(
                    "{}. {} plan={} tokens={:?} period={} active={}",
                    i + 1,
                    s.provider,
                    s.plan,
                    s.tokens,
                    s.period,
                    s.active
                );
            }
        }
        let o = ["Ajouter/remplacer", "Supprimer", "Retour"];
        match pick("Action abonnements", &o, 0)? {
            0 => {
                let provider = ask("Provider", "anthropic")?.to_lowercase();
                let plan = ask("Plan", "pro")?;
                let t = ask("Tokens/period (vide=inconnu)", "")?;
                let tokens = if t.trim().is_empty() {
                    None
                } else {
                    Some(t.trim().parse::<u64>()?)
                };
                let period = ask("Period (monthly/daily/annual)", "monthly")?;
                let active = ask_bool("Actif", true)?;
                let sub = Sub {
                    provider: provider.clone(),
                    plan,
                    tokens,
                    period,
                    active,
                };
                if let Some(x) = st.subs.iter_mut().find(|s| s.provider == provider) {
                    *x = sub
                } else {
                    st.subs.push(sub)
                }
            }
            1 => {
                if st.subs.is_empty() {
                    continue;
                }
                let names: Vec<&str> = st.subs.iter().map(|s| s.provider.as_str()).collect();
                let i = pick("Supprimer lequel", &names, 0)?;
                st.subs.remove(i);
            }
            2 => break,
            _ => {}
        }
    }
    Ok(())
}

fn status(sb: &Sandbox, st: &State) -> R<()> {
    println!("\n=== Statut MIP ===");
    println!("Workspace: {}", sb.workspace.display());
    println!("MIP root : {}", sb.mip.display());
    println!("Date setup: {}", st.cfg.date);
    println!("Profil    : {}", st.cfg.active_profile);
    println!("Tool/Model: {} / {}", st.cfg.tool, st.cfg.models);
    println!("Build/Test: {} | {}", st.cfg.build, st.cfg.test);
    println!(
        "Subs      : {} ({} fournisseur(s))",
        if st.cfg.subs_enabled {
            "actif"
        } else {
            "desactive"
        },
        st.subs.len()
    );
    Ok(())
}

fn save(sb: &Sandbox, st: &State) -> R<()> {
    sb.write(STATE_FILE, &serde_json::to_string_pretty(st)?)?;
    sb.write(ACTIVE_FILE, &format!("{}\n", st.cfg.active_profile))?;
    sb.write(ENV_FILE, &render_env(&st.cfg))?;
    if st.cfg.subs_enabled {
        sb.write(SUB_FILE, &render_subs(&st.subs))?;
    }
    Ok(())
}

fn render_env(c: &Config) -> String {
    format!("# Configuration environnement MIP\n\n## TL;DR\n\nGenere par mip-configurator (outil externe dedie MIP).\n\n## Metadonnees\n- Date de configuration : {d}\n- Version MIP : v2.1\n- Reconfigurable via : `/mip_setup`\n\n## Stack technique\n- Langage(s) : {l}\n- Framework(s) : {f}\n- Base(s) de donnees : {db}\n\n## Commandes standard\n- Build : `{b}`\n- Test : `{t}`\n- Lint : `{li}`\n- Format : `{fo}`\n\n## Securite\n- Niveau : {sec}\n\n## Infrastructure\n- Deploiement : {dep}\n\n## Outil IA\n- Outil principal : {tool}\n- Modele(s) : {m}\n- Budget : {bu}\n- Abonnements : {sub}\n\n## Garde-fou\n- Cet outil n'ecrit que dans `.mip/`\n",d=c.date,l=c.languages,f=c.frameworks,db=c.db,b=c.build,t=c.test,li=c.lint,fo=c.fmt,sec=c.security,dep=c.deployment,tool=c.tool,m=c.models,bu=c.budget,sub=if c.subs_enabled{"`.mip/config/subscriptions.md`"}else{"desactive"})
}

fn render_subs(v: &[Sub]) -> String {
    let mut s =
        String::from("# Abonnements et quotas tokens\n\nGenere par mip-configurator.\n\n```yaml\n");
    if v.is_empty() {
        s.push_str("# Aucun fournisseur configure\n");
    } else {
        for x in v {
            s.push_str(&format!(
                "{}:\n  plan: {}\n  tokens_period: {}\n  period: {}\n  active: {}\n\n",
                x.provider,
                x.plan,
                x.tokens
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| "null".into()),
                x.period,
                x.active
            ));
        }
    }
    s.push_str("```\n");
    s
}

fn parse_subs(raw: &str) -> Vec<Sub> {
    let mut v = Vec::new();
    let mut in_yaml = false;
    let mut cur: Option<Sub> = None;
    for line in raw.lines() {
        let l = line.trim_end();
        if l.trim() == "```yaml" {
            in_yaml = true;
            continue;
        }
        if in_yaml && l.trim() == "```" {
            if let Some(s) = cur.take() {
                v.push(s)
            }
            break;
        }
        if !in_yaml || l.trim().is_empty() || l.trim_start().starts_with('#') {
            continue;
        }
        if !l.starts_with(' ') && l.ends_with(':') {
            if let Some(s) = cur.take() {
                v.push(s)
            }
            cur = Some(Sub {
                provider: l.trim_end_matches(':').to_string(),
                plan: "unknown".into(),
                tokens: None,
                period: "monthly".into(),
                active: false,
            });
            continue;
        }
        if let Some(s) = cur.as_mut() {
            if let Some((k, val)) = l.trim().split_once(':') {
                let val = val.trim();
                match k.trim() {
                    "plan" => s.plan = val.to_string(),
                    "tokens_period" => {
                        s.tokens = if val == "null" {
                            None
                        } else {
                            val.parse().ok()
                        }
                    }
                    "period" => s.period = val.to_string(),
                    "active" => s.active = val == "true",
                    _ => {}
                }
            }
        }
    }
    v
}

fn ask(label: &str, default: &str) -> R<String> {
    print!("- {label} [{default}] : ");
    io::stdout().flush()?;
    let mut b = String::new();
    io::stdin().read_line(&mut b)?;
    let t = b.trim();
    Ok(if t.is_empty() {
        default.to_string()
    } else {
        t.to_string()
    })
}
fn ask_bool(label: &str, default: bool) -> R<bool> {
    let h = if default { "Y/n" } else { "y/N" };
    loop {
        print!("- {label} [{h}] : ");
        io::stdout().flush()?;
        let mut b = String::new();
        io::stdin().read_line(&mut b)?;
        let v = b.trim().to_lowercase();
        if v.is_empty() {
            return Ok(default);
        }
        if ["y", "yes", "oui", "o", "1", "true"].contains(&v.as_str()) {
            return Ok(true);
        }
        if ["n", "no", "non", "0", "false"].contains(&v.as_str()) {
            return Ok(false);
        }
        println!("Reponse invalide.");
    }
}
fn pick(label: &str, options: &[&str], default_idx: usize) -> R<usize> {
    if options.is_empty() {
        return Err("Menu vide".into());
    }
    println!("\n{label}");
    for (i, o) in options.iter().enumerate() {
        println!("  {}. {}", i + 1, o);
    }
    loop {
        print!("Choix [{}] : ", default_idx + 1);
        io::stdout().flush()?;
        let mut b = String::new();
        io::stdin().read_line(&mut b)?;
        let t = b.trim();
        if t.is_empty() {
            return Ok(default_idx);
        }
        if let Ok(n) = t.parse::<usize>() {
            if (1..=options.len()).contains(&n) {
                return Ok(n - 1);
            }
        }
        println!("Choix invalide.");
    }
}
fn today() -> String {
    Utc::now().date_naive().to_string()
}
