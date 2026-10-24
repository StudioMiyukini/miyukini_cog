//! @id     miyudouane_zones
//! @do     rendre_zones_linux
//! @role   securite
//! @layer  operateur
//! @human  miyudouane — rendu Linux des zones : script netns + veth + sysctl + nftables, unités systemd des sandboxes (texte seulement).

//! # Rendu des zones
//!
//! Transforme un [`PlanReseau`] en configuration Linux **textuelle** (rien
//! n'est exécuté ici) :
//!
//! - [`script_mise_en_place`] : script `sh` (espaces de noms, ponts, paires
//!   veth, adresses, routes, sysctl, chargement des règles nftables) ;
//! - [`script_retrait`] : suppression des espaces de noms ;
//! - [`unites_systemd`] : une slice + un complément (`drop-in`) de service par
//!   sandbox : utilisateur dédié, espace réseau propre, système en lecture
//!   seule, DB secondaire seule écriture permise (Franche, EX-29), DB
//!   principale inaccessible (INV-Z5).
//!
//! Zone COG : aucune route par défaut (INV-Z4) ; portails sans routage
//! (`ip_forward=0`, rupture protocolaire) ; IPv6 désactivé dans tous les
//! espaces (l'adressage du doc est IPv4).

use std::fmt::Write as _;

use borderguard::Zone;

use crate::plan::{ModeFrontiere, PlanReseau, Sandbox};

/// Répertoire des règles nftables générées sur l'hôte.
pub const REPERTOIRE_REGLES: &str = "/etc/miyukini/douane";
/// Racine des données de la zone COG (rendue inaccessible aux sandboxes Franche).
pub const RACINE_DONNEES_COG: &str = "/var/lib/miyukini/cog";

/// Chemin du fichier de règles d'un espace de noms.
pub fn chemin_regles(netns: &str) -> String {
    format!("{REPERTOIRE_REGLES}/{netns}.nft")
}

fn tous_les_netns(plan: &PlanReseau) -> Vec<&str> {
    plan.zones
        .iter()
        .map(|z| z.netns.as_str())
        .chain(plan.frontieres.iter().map(|f| f.netns.as_str()))
        .chain(plan.sandboxes.iter().map(|s| s.netns.as_str()))
        .collect()
}

/// Script de mise en place (niveau 1 du doc §11 : namespaces + nftables).
/// À exécuter en root sur un hôte dédié ; jamais lancé par ce crate.
pub fn script_mise_en_place(plan: &PlanReseau) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "#!/bin/sh");
    let _ = writeln!(s, "# Généré par miyudouane — mise en place des zones COG (prototype niveau 1).");
    let _ = writeln!(s, "# Ne touche pas au réseau de l'hôte, sauf la carte physique si MCOG_IF_PHYSIQUE est défini.");
    let _ = writeln!(s, "set -eu");
    let _ = writeln!(s, "modprobe nf_conntrack_bridge");
    let _ = writeln!(s);
    let _ = writeln!(s, "# 1. Espaces de noms (zones, frontières, sandboxes)");
    for ns in tous_les_netns(plan) {
        let _ = writeln!(s, "ip netns add {ns}");
        let _ = writeln!(s, "ip -n {ns} link set lo up");
        let _ = writeln!(s, "ip netns exec {ns} sysctl -qw net.ipv6.conf.all.disable_ipv6=1");
        let _ = writeln!(s, "ip netns exec {ns} sysctl -qw net.ipv4.ip_forward=0");
    }
    let _ = writeln!(s);
    let _ = writeln!(s, "# 2. Ponts de zone (sans adresse)");
    for z in &plan.zones {
        let _ = writeln!(s, "ip -n {} link add br0 type bridge", z.netns);
        let _ = writeln!(s, "ip -n {} link set br0 up", z.netns);
    }
    let _ = writeln!(s);
    let _ = writeln!(s, "# 3. Frontières : liens veth vers les ponts de zone");
    for f in &plan.frontieres {
        for l in &f.liens {
            let Some(z) = plan.zone(l.zone) else { continue };
            let _ = writeln!(
                s,
                "ip link add {} netns {} type veth peer name {} netns {}",
                l.interface, f.netns, l.interface_zone, z.netns
            );
            let _ = writeln!(s, "ip -n {} link set {} master br0 up", z.netns, l.interface_zone);
            let _ = writeln!(s, "ip -n {} addr add {}/{} dev {}", f.netns, l.adresse, z.reseau.prefixe(), l.interface);
            let _ = writeln!(s, "ip -n {} link set {} up", f.netns, l.interface);
        }
        if f.mode == ModeFrontiere::Routage {
            let _ = writeln!(s, "ip netns exec {} sysctl -qw net.ipv4.ip_forward=1", f.netns);
        }
    }
    let _ = writeln!(s);
    let _ = writeln!(s, "# 4. Sandboxes de service");
    for sb in &plan.sandboxes {
        let Some(z) = plan.zone(sb.zone) else { continue };
        let _ = writeln!(
            s,
            "ip link add eth0 netns {} type veth peer name {} netns {}",
            sb.netns, sb.interface_zone, z.netns
        );
        let _ = writeln!(s, "ip -n {} link set {} master br0 up", z.netns, sb.interface_zone);
        let _ = writeln!(s, "ip -n {} addr add {}/{} dev eth0", sb.netns, sb.adresse, z.reseau.prefixe());
        let _ = writeln!(s, "ip -n {} link set eth0 up", sb.netns);
        match z.passerelle {
            Some(gw) => {
                let _ = writeln!(s, "ip -n {} route add default via {gw}", sb.netns);
            }
            None => {
                let _ = writeln!(s, "# {} : aucune route par défaut (zone {})", sb.netns, sb.zone);
            }
        }
    }
    let _ = writeln!(s);
    let _ = writeln!(s, "# 5. Carte physique, portée par la frontière externe (facultatif)");
    if let Some(ext) = plan.frontieres.iter().find(|f| f.mode == ModeFrontiere::Routage) {
        if let Some(i) = ext.exterieures.iter().find(|e| e.zones.contains(&Zone::International)) {
            let _ = writeln!(s, "if [ -n \"${{MCOG_IF_PHYSIQUE:-}}\" ]; then");
            let _ = writeln!(s, "\tip link set \"$MCOG_IF_PHYSIQUE\" netns {}", ext.netns);
            let _ = writeln!(s, "\tip -n {} link set \"$MCOG_IF_PHYSIQUE\" name {}", ext.netns, i.interface);
            let _ = writeln!(s, "\tip -n {} link set {} up", ext.netns, i.interface);
            let _ = writeln!(s, "fi");
        }
    }
    let _ = writeln!(s);
    let _ = writeln!(s, "# 6. Règles nftables (générées par miyudouane, auditées)");
    for ns in plan.zones.iter().map(|z| &z.netns).chain(plan.frontieres.iter().map(|f| &f.netns)) {
        let _ = writeln!(s, "ip netns exec {ns} nft -f {}", chemin_regles(ns));
    }
    s
}

/// Script de retrait (supprime tous les espaces de noms du plan).
pub fn script_retrait(plan: &PlanReseau) -> String {
    let mut s = String::from("#!/bin/sh\n# Généré par miyudouane — retrait des zones COG.\nset -u\n");
    for ns in tous_les_netns(plan).into_iter().rev() {
        let _ = writeln!(s, "ip netns del {ns} 2>/dev/null || true");
    }
    s
}

fn unite_sandbox(sb: &Sandbox) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "# Généré par miyudouane — sandbox {}/{} (EX-21, EX-29).", sb.zone, sb.service);
    let _ = writeln!(s, "[Service]");
    let _ = writeln!(s, "User={}", sb.utilisateur);
    let _ = writeln!(s, "Group={}", sb.utilisateur);
    let _ = writeln!(s, "Slice={}", sb.slice());
    let _ = writeln!(s, "NetworkNamespacePath=/run/netns/{}", sb.netns);
    for directive in [
        "NoNewPrivileges=yes",
        "PrivateTmp=yes",
        "PrivateDevices=yes",
        "ProtectSystem=strict",
        "ProtectHome=yes",
        "ProtectKernelTunables=yes",
        "ProtectKernelModules=yes",
        "ProtectKernelLogs=yes",
        "ProtectControlGroups=yes",
        "ProtectClock=yes",
        "ProtectHostname=yes",
        "RestrictNamespaces=yes",
        "RestrictRealtime=yes",
        "RestrictSUIDSGID=yes",
        "LockPersonality=yes",
        "MemoryDenyWriteExecute=yes",
        "CapabilityBoundingSet=",
        "AmbientCapabilities=",
        "SystemCallArchitectures=native",
        "SystemCallFilter=@system-service",
        "SystemCallFilter=~@privileged @resources @mount @debug @reboot @swap @raw-io",
        "RestrictAddressFamilies=AF_INET AF_UNIX",
        "UMask=0077",
    ] {
        let _ = writeln!(s, "{directive}");
    }
    match &sb.db_secondaire {
        Some(db) => {
            let _ = writeln!(s, "ReadWritePaths={}", db.display());
            let _ = writeln!(s, "InaccessiblePaths={RACINE_DONNEES_COG}");
        }
        None => {
            let _ = writeln!(s, "# Zone {} : aucune écriture locale, données via KindMother (INV-Z8).", sb.zone);
        }
    }
    s
}

fn slice_sandbox(sb: &Sandbox) -> String {
    let r = sb.ressources;
    format!(
        "# Généré par miyudouane — plafonds de {}/{} (contrat de service, §10).\n[Slice]\nCPUQuota={}%\nMemoryMax={}M\nTasksMax={}\nIOWeight={}\n",
        sb.zone, sb.service, r.cpu_pourcent, r.memoire_mio, r.taches_max, r.poids_io
    )
}

/// Unités systemd des sandboxes : `(chemin relatif, contenu)`.
///
/// Le complément s'applique au service `<service>-<zone>.service` fourni par
/// le déploiement (le binaire et son `ExecStart` ne sont pas générés ici).
pub fn unites_systemd(plan: &PlanReseau) -> Vec<(String, String)> {
    let mut unites = Vec::new();
    for sb in &plan.sandboxes {
        unites.push((sb.slice(), slice_sandbox(sb)));
        unites.push((
            format!("{}-{}.service.d/miyudouane.conf", sb.service, sb.zone.nom()),
            unite_sandbox(sb),
        ));
    }
    unites
}
