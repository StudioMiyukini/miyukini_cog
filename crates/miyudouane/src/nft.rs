//! @id     miyudouane_nft
//! @do     generer_auditer_regles_nftables
//! @role   securite
//! @layer  operateur
//! @human  miyudouane — génération (texte `nft -f`) et audit des règles nftables, refus par défaut.

//! # Règles nftables
//!
//! Génération **pure** (aucun appel système) du texte passé à `nft -f`.
//!
//! ## Frontière (`table inet miyudouane`, un espace de noms par frontière)
//!
//! Trois chaînes `entree` / `transit` / `sortie` (crochets `input` / `forward`
//! / `output`), toutes en `policy drop`, dans cet ordre :
//!
//! 1. bouclage (`lo`) accepté (entrée et sortie seulement) ;
//! 2. `ct state invalid` → journal `INVALIDE` + rejet ;
//! 3. pour chaque flux (marque conntrack `m`) : paquet trop grand → `TAILLE`,
//!    débit au-delà du **filet** → `DEBIT` : `ct direction original|reply
//!    limit rate over ⌈D × 5/4⌉ kbytes/second burst B bytes`, une limite par
//!    sens et par chaîne. Le débit déclaré `D` est tenu par la mise en forme
//!    `tc` ([`crate::tc`] : dépassement mis en file) ; la règle `DEBIT` n'est
//!    qu'un filet au-delà de 125 % (rapport documenté dans [`crate::tc`]) ;
//! 4. `meta mark set ct mark` (marque du paquet = marque du flux, lue par le
//!    filtre `fw` de `tc`) puis `ct state established,related` accepté
//!    (réponses des seules connexions admises) ;
//! 5. connexions nouvelles **déclarées** : `ct state new`, marques posées
//!    (`ct mark set m meta mark set m`),
//!    `comment "voie=<id> service=<s> flux=<n>"` ;
//! 6. toute source dans la zone COG → `SORTIE-COG` ; toute destination dans
//!    la zone COG → `ENTREE-COG` (INV-Z1, INV-Z2 : incidents critiques) ;
//! 7. tout le reste → `NON-DECLARE` + rejet explicite.
//!
//! ## Zone (`table bridge miyudouane_zone` + `table inet miyudouane_zone`)
//!
//! Le pont de zone ne laisse passer que l'ARP, les réponses établies et les
//! connexions déclarées entre une sandbox et le port de **sa** frontière.
//! Sandbox ⇄ sandbox → `LATERAL` (INV-Z8). L'espace de zone n'a aucune
//! adresse : `inet` tout rejeté.
//!
//! ## Journalisation (convention pour les Vigies)
//!
//! `log prefix "MDOU|<lieu>|<chaine>|<MOTIF>|" group <G>` (NFLOG) où `<lieu>`
//! est le nom de la frontière ou `zone-<zone>`, `<G>` le groupe NFLOG du plan
//! (frontières 101..103, zones 110..112) et `<MOTIF>` l'un de
//! [`MOTIFS_JOURNAL`]. Les Vigies lisent ces groupes (libnetfilter_log /
//! socket NFLOG) ; chaque motif a sa gravité ([`crate::ips::Gravite`]).
//!
//! ## Journalisation bornée (anti-inondation du relais NFLOG)
//!
//! Journaliser chaque paquet rejeté laisserait une inondation saturer le
//! tampon netlink du relais (`ENOBUFS`) ⇒ capteur muet ⇒ voie aveugle ⇒
//! coupure de toutes les voies du groupe. Chaque rejet est donc écrit en trois
//! règles ([`rejet`]) : journal limité **par flux** (ensembles dynamiques
//! `journal_v4` / `journal_v6`, clé `saddr . daddr . l4proto`, une ligne par
//! seconde, expiration 10 s, 65 536 flux) **et** par une limite **globale** de
//! la table (objet `limit journal`, 50 lignes/s, rafale 100), puis rejet
//! inconditionnel compté par le compteur nommé `rejets` (volume total rejeté,
//! journalisé ou non). Le `drop` ne dépend jamais des limites.

use std::collections::BTreeSet;
use std::fmt::Write as _;

use borderguard::{ArreteVerifie, Frontiere, Reseau, Zone};

use crate::compilation::{Crochet, RegleCompilee};
use crate::plan::{DefinitionFrontiere, PlanReseau};
use crate::tc::{filet_kio_s, rafale_octets, Sens};

/// Copie de la marque conntrack dans la marque du paquet (classification `tc`).
pub const COPIE_MARQUE: &str = "meta mark set ct mark";

/// Préfixe commun des journaux Douane.
pub const PREFIXE_JOURNAL: &str = "MDOU";
/// Motifs de journal (dernier champ du préfixe).
pub const MOTIFS_JOURNAL: [&str; 8] =
    ["NON-DECLARE", "SORTIE-COG", "ENTREE-COG", "INVALIDE", "DEBIT", "TAILLE", "LATERAL", "FERME"];

/// Débit de journalisation par flux (clé `source . destination . protocole`) :
/// une ligne par flux et par seconde au plus.
pub const JOURNAL_PAR_FLUX: &str = "1/second";
/// Durée de vie d'un flux dans les ensembles de journalisation (secondes).
pub const JOURNAL_EXPIRATION_S: u32 = 10;
/// Taille maximale des ensembles dynamiques de journalisation (par famille).
pub const JOURNAL_TAILLE_ENSEMBLE: u32 = 65_536;
/// Débit global des `log` d'une table (objet `limit journal`), paquets/s.
pub const JOURNAL_DEBIT_GLOBAL: u32 = 50;
/// Rafale admise par la limite globale (paquets).
pub const JOURNAL_RAFALE_GLOBALE: u32 = 100;
/// Nom du compteur nommé du volume total rejeté d'une table.
pub const COMPTEUR_REJETS: &str = "rejets";

/// Rejet journalisé, en trois règles (anti-inondation du relais NFLOG) :
///
/// 1. et 2. (une seule si `sel` porte déjà `ip` ou `ip6`)
///    `<sel> update @journal_v4|v6 { saddr . daddr . l4proto limit rate
///    1/second } limit name "journal" log prefix "MDOU|…|" group G` : au plus
///    une ligne par flux et par seconde, et au plus [`JOURNAL_DEBIT_GLOBAL`]
///    lignes/s pour toute la table (aucun verdict : la règle suivante s'applique) ;
/// 3. `<sel> counter name "rejets" drop` : rejet **inconditionnel**, volume
///    total compté à part (`nft list counter … rejets`).
///
/// Le sélecteur `sel` est répété : ce sont des correspondances sans état, sauf
/// `limit rate over` (règle `DEBIT`) dont chaque copie voit le même trafic.
pub(crate) fn rejet(sel: &str, lieu: &str, chaine: &str, motif: &str, groupe: u16) -> Vec<String> {
    let sel = if sel.is_empty() { String::new() } else { format!("{sel} ") };
    let log = format!("limit name \"journal\" log prefix \"{PREFIXE_JOURNAL}|{lieu}|{chaine}|{motif}|\" group {groupe}");
    // Un sélecteur déjà lié à une famille n'est journalisé que dans celle-ci
    // (`ip saddr … ip6 saddr` serait contradictoire).
    let mots: Vec<&str> = sel.split_whitespace().collect();
    let v4 = mots.contains(&"ip");
    let v6 = mots.contains(&"ip6");
    let mut regles = Vec::with_capacity(3);
    if v4 || !v6 {
        regles.push(format!(
            "{sel}update @journal_v4 {{ ip saddr . ip daddr . meta l4proto limit rate {JOURNAL_PAR_FLUX} }} {log}"
        ));
    }
    if v6 || !v4 {
        regles.push(format!(
            "{sel}update @journal_v6 {{ ip6 saddr . ip6 daddr . meta l4proto limit rate {JOURNAL_PAR_FLUX} }} {log}"
        ));
    }
    regles.push(format!("{sel}counter name \"{COMPTEUR_REJETS}\" drop"));
    regles
}

/// Objets de journalisation d'une table (à écrire juste après `table … {`) :
/// ensembles dynamiques par flux, limite globale, compteur des rejets.
pub(crate) fn objets_journal(texte: &mut String) {
    for (famille, adresse) in [("v4", "ipv4_addr"), ("v6", "ipv6_addr")] {
        let _ = writeln!(texte, "\tset journal_{famille} {{");
        let _ = writeln!(texte, "\t\ttype {adresse} . {adresse} . inet_proto");
        let _ = writeln!(texte, "\t\tsize {JOURNAL_TAILLE_ENSEMBLE}");
        let _ = writeln!(texte, "\t\tflags dynamic,timeout");
        let _ = writeln!(texte, "\t\ttimeout {JOURNAL_EXPIRATION_S}s");
        let _ = writeln!(texte, "\t}}");
    }
    let _ = writeln!(texte, "\tlimit journal {{");
    let _ = writeln!(texte, "\t\trate {JOURNAL_DEBIT_GLOBAL}/second burst {JOURNAL_RAFALE_GLOBALE} packets");
    let _ = writeln!(texte, "\t}}");
    let _ = writeln!(texte, "\tcounter {COMPTEUR_REJETS} {{");
    let _ = writeln!(texte, "\t}}");
}

fn adresse(r: &Reseau) -> String {
    let hote = if r.est_v4() { 32 } else { 128 };
    if r.prefixe() == hote {
        r.adresse().to_string()
    } else {
        r.to_string()
    }
}

fn ensemble(rs: &[&Reseau]) -> String {
    if let [r] = rs {
        adresse(r)
    } else {
        format!("{{ {} }}", rs.iter().map(|r| adresse(r)).collect::<Vec<_>>().join(", "))
    }
}

/// Sélecteurs d'adresses par famille : `[(famille, "ip saddr … ip daddr …")]`.
pub(crate) fn selecteurs_adresses(sources: &[Reseau], destinations: &[Reseau]) -> Vec<String> {
    let mut sortie = Vec::new();
    for (v4, mot) in [(true, "ip"), (false, "ip6")] {
        let s: Vec<&Reseau> = sources.iter().filter(|r| r.est_v4() == v4).collect();
        let d: Vec<&Reseau> = destinations.iter().filter(|r| r.est_v4() == v4).collect();
        let s_ok = sources.is_empty() || !s.is_empty();
        let d_ok = destinations.is_empty() || !d.is_empty();
        if !(s_ok && d_ok) || (s.is_empty() && d.is_empty()) {
            continue;
        }
        let mut sel = String::new();
        if !s.is_empty() {
            let _ = write!(sel, "{mot} saddr {}", ensemble(&s));
        }
        if !d.is_empty() {
            if !sel.is_empty() {
                sel.push(' ');
            }
            let _ = write!(sel, "{mot} daddr {}", ensemble(&d));
        }
        sortie.push(sel);
    }
    sortie
}

fn commentaire(r: &RegleCompilee) -> String {
    format!("comment \"voie={} service={} flux={}\"", r.voie, r.service, r.index_flux)
}

fn regle_nouvelle(r: &RegleCompilee, iif: Option<&str>, oif: Option<&str>, poser_marque: bool) -> Vec<String> {
    let mut prefixe = String::new();
    if let Some(i) = iif {
        let _ = write!(prefixe, "iifname \"{i}\" ");
    }
    if let Some(o) = oif {
        let _ = write!(prefixe, "oifname \"{o}\" ");
    }
    let marque = if poser_marque {
        format!("ct mark set 0x{m:08x} meta mark set 0x{m:08x} ", m = r.marque)
    } else {
        String::new()
    };
    selecteurs_adresses(&r.sources, &r.destinations)
        .into_iter()
        .map(|sel| {
            format!(
                "{prefixe}{sel} {} dport {} meta length <= {} ct state new {marque}counter accept {}",
                r.protocole,
                r.port,
                r.taille_max_paquet,
                commentaire(r)
            )
        })
        .collect()
}

/// `<iif|oif> "lo" accept` suivi des règles de rejet.
fn avec_lo(sens: &str, rejets: Vec<String>) -> Vec<String> {
    let mut v = vec![format!("{sens} \"lo\" accept")];
    v.extend(rejets);
    v
}

fn entete(texte: &mut String, titre: &str, lignes: &[String]) {
    let _ = writeln!(texte, "#!/usr/sbin/nft -f");
    let _ = writeln!(texte, "# Généré par miyudouane — ne pas modifier à la main.");
    let _ = writeln!(texte, "# {titre}");
    for l in lignes {
        let _ = writeln!(texte, "# {l}");
    }
    let _ = writeln!(texte, "flush ruleset");
    let _ = writeln!(texte);
}

fn chaine(texte: &mut String, nom: &str, crochet: &str, regles: &[String]) {
    let _ = writeln!(texte, "\tchain {nom} {{");
    let _ = writeln!(texte, "\t\ttype filter hook {crochet} priority filter; policy drop;");
    for r in regles {
        let _ = writeln!(texte, "\t\t{r}");
    }
    let _ = writeln!(texte, "\t}}");
}

/// Génère les règles d'une frontière depuis un arrêté vérifié et ses règles
/// compilées. Les voies de `voies_coupees` sont omises (coupure de voie, EX-49).
pub fn generer_frontiere(
    arrete: &ArreteVerifie,
    regles: &[RegleCompilee],
    plan: &PlanReseau,
    voies_coupees: &BTreeSet<String>,
) -> Option<String> {
    let a = arrete.arrete();
    let def = plan.frontiere(a.frontiere)?;
    let cog = plan.zone(Zone::Cog).map(|z| z.reseau);
    let lieu = a.frontiere.nom();
    let g = def.groupe_nflog;
    let actives: Vec<&RegleCompilee> = regles.iter().filter(|r| !voies_coupees.contains(&r.voie)).collect();

    let mut texte = String::new();
    let mut infos = vec![
        format!("frontiere={lieu} arrete={} empreinte={}", a.numero, arrete.empreinte_hex()),
        format!("validite={}..{} emetteur={}", a.pas_avant, a.pas_apres, a.emetteur),
        format!("journal : nflog groupe {g}, prefixe \"{PREFIXE_JOURNAL}|{lieu}|<chaine>|<MOTIF>|\""),
    ];
    if !voies_coupees.is_empty() {
        infos.push(format!("voies coupees : {}", voies_coupees.iter().cloned().collect::<Vec<_>>().join(",")));
    }
    entete(&mut texte, "Douane — règles de frontière (refus par défaut)", &infos);
    let _ = writeln!(texte, "table inet miyudouane {{");
    objets_journal(&mut texte);

    // Limites par marque (une fois par flux actif, dans chaque chaîne).
    let mut marques: Vec<&RegleCompilee> = Vec::new();
    for r in &actives {
        if !marques.iter().any(|m| m.marque == r.marque) {
            marques.push(r);
        }
    }

    for crochet in [Crochet::Entree, Crochet::Transit, Crochet::Sortie] {
        let nom = crochet.chaine();
        let mut lignes = Vec::new();
        match crochet {
            Crochet::Entree => lignes.push("iif \"lo\" accept".to_string()),
            Crochet::Sortie => lignes.push("oif \"lo\" accept".to_string()),
            Crochet::Transit => {}
        }
        lignes.extend(rejet("ct state invalid", lieu, nom, "INVALIDE", g));
        for m in &marques {
            lignes.extend(rejet(
                &format!("ct mark 0x{:08x} meta length > {}", m.marque, m.taille_max_paquet),
                lieu,
                nom,
                "TAILLE",
                g,
            ));
            // Filet de sécurité au-delà de la mise en forme `tc` (crate::tc) :
            // plafond × 5/4, par sens, même rafale que la feuille HTB.
            for sens in [Sens::Original, Sens::Reponse] {
                lignes.extend(rejet(
                    &format!(
                        "ct mark 0x{:08x} ct direction {} limit rate over {} kbytes/second burst {} bytes",
                        m.marque,
                        sens.nom(),
                        filet_kio_s(m.debit_max_kio_s),
                        rafale_octets(m.debit_max_kio_s)
                    ),
                    lieu,
                    nom,
                    "DEBIT",
                    g,
                ));
            }
        }
        // Marque du paquet = marque du flux : classification `tc` (filtre `fw`).
        lignes.push(COPIE_MARQUE.to_string());
        lignes.push("ct state established,related counter accept".to_string());
        for r in actives.iter().filter(|r| r.crochet == crochet) {
            lignes.extend(regle_nouvelle(r, r.iif.as_deref(), r.oif.as_deref(), true));
        }
        if let Some(cog) = cog {
            let mot = if cog.est_v4() { "ip" } else { "ip6" };
            lignes.extend(rejet(&format!("{mot} saddr {cog}"), lieu, nom, "SORTIE-COG", g));
            lignes.extend(rejet(&format!("{mot} daddr {cog}"), lieu, nom, "ENTREE-COG", g));
        }
        lignes.extend(rejet("", lieu, nom, "NON-DECLARE", g));
        let crochet_nft = match crochet {
            Crochet::Entree => "input",
            Crochet::Transit => "forward",
            Crochet::Sortie => "output",
        };
        chaine(&mut texte, nom, crochet_nft, &lignes);
    }
    let _ = writeln!(texte, "}}");
    Some(texte)
}

/// Règles de fermeture d'une frontière : tout rejeté (aucun arrêté en vigueur,
/// arrêté expiré, gel). Seul le bouclage reste ouvert.
pub fn generer_fermeture(frontiere: Frontiere, def: &DefinitionFrontiere, motif: &str) -> String {
    let lieu = frontiere.nom();
    let g = def.groupe_nflog;
    let motif_sur: String = motif.chars().filter(|c| c.is_ascii_alphanumeric() || *c == ' ' || *c == '-').collect();
    let mut texte = String::new();
    entete(
        &mut texte,
        "Douane — fermeture de frontière (tout rejeté)",
        &[format!("frontiere={lieu} motif={motif_sur}")],
    );
    let _ = writeln!(texte, "table inet miyudouane {{");
    objets_journal(&mut texte);
    chaine(&mut texte, "entree", "input", &avec_lo("iif", rejet("", lieu, "entree", "FERME", g)));
    chaine(&mut texte, "transit", "forward", &rejet("", lieu, "transit", "FERME", g));
    chaine(&mut texte, "sortie", "output", &avec_lo("oif", rejet("", lieu, "sortie", "FERME", g)));
    let _ = writeln!(texte, "}}");
    texte
}

/// Port de pont côté zone pour une extrémité de règle : sandbox du service si
/// l'extrémité est la zone elle-même, sinon port du lien de la frontière.
fn port_zone(plan: &PlanReseau, def: &DefinitionFrontiere, zone: Zone, r: &RegleCompilee, cote_sandbox: bool) -> Option<String> {
    if cote_sandbox {
        plan.sandbox(zone, &r.service).map(|s| s.interface_zone.clone())
    } else {
        def.lien(zone).map(|l| l.interface_zone.clone())
    }
}

/// Génère les règles d'un espace de zone (pont filtré) depuis les règles
/// compilées de toutes les frontières qui touchent la zone.
pub fn generer_zone(zone: Zone, frontieres: &[(Frontiere, Vec<RegleCompilee>)], plan: &PlanReseau) -> Option<String> {
    let defz = plan.zone(zone)?;
    let lieu = format!("zone-{}", zone.nom());
    let g = defz.groupe_nflog;
    let mut declarees = Vec::new();
    for (frontiere, regles) in frontieres {
        let def = plan.frontiere(*frontiere)?;
        for r in regles {
            // Côté de la zone traversé par cette règle.
            // Côté de la zone traversé par cette règle : la sandbox émet (zone
            // source) vers le port de la frontière, ou le reçoit (zone destination).
            let sandbox_emet = match r.crochet {
                Crochet::Entree | Crochet::Transit if r.zone_source == zone => true,
                Crochet::Sortie | Crochet::Transit if r.zone_destination == zone => false,
                _ => continue,
            };
            let sandbox = port_zone(plan, def, zone, r, true)?;
            let frontiere = port_zone(plan, def, zone, r, false)?;
            let (iif, oif) = if sandbox_emet { (sandbox, frontiere) } else { (frontiere, sandbox) };
            declarees.extend(regle_nouvelle(r, Some(&iif), Some(&oif), false));
        }
    }
    let mut texte = String::new();
    entete(
        &mut texte,
        "Douane — pont de zone filtré (segmentation par service, refus par défaut)",
        &[
            format!("zone={} reseau={}", zone.nom(), defz.reseau),
            format!("journal : nflog groupe {g}, prefixe \"{PREFIXE_JOURNAL}|{lieu}|<chaine>|<MOTIF>|\""),
            "prerequis : module nf_conntrack_bridge".into(),
        ],
    );
    let _ = writeln!(texte, "table bridge miyudouane_zone {{");
    objets_journal(&mut texte);
    let mut lignes = vec![
        "ether type arp accept".to_string(),
    ];
    lignes.extend(rejet("ct state invalid", &lieu, "transit", "INVALIDE", g));
    lignes.push("ct state established,related counter accept".to_string());
    lignes.extend(declarees);
    lignes.extend(rejet("iifname \"sv*\" oifname \"sv*\"", &lieu, "transit", "LATERAL", g));
    if zone == Zone::Cog {
        lignes.extend(rejet(&format!("ip saddr {}", defz.reseau), &lieu, "transit", "SORTIE-COG", g));
    }
    lignes.extend(rejet("", &lieu, "transit", "NON-DECLARE", g));
    chaine(&mut texte, "transit", "forward", &lignes);
    let _ = writeln!(texte, "}}");
    let _ = writeln!(texte);
    let _ = writeln!(texte, "table inet miyudouane_zone {{");
    objets_journal(&mut texte);
    chaine(&mut texte, "entree", "input", &avec_lo("iif", rejet("", &lieu, "entree", "NON-DECLARE", g)));
    chaine(&mut texte, "transit", "forward", &rejet("", &lieu, "transit", "NON-DECLARE", g));
    chaine(&mut texte, "sortie", "output", &avec_lo("oif", rejet("", &lieu, "sortie", "SORTIE-COG", g)));
    let _ = writeln!(texte, "}}");
    Some(texte)
}

/// Audit structurel d'un texte nftables généré (défense en profondeur, sans
/// dépendre du générateur) :
///
/// - chaque chaîne de base est en `policy drop`, aucune `policy accept` ;
/// - chaque chaîne se termine par un rejet ;
/// - chaque `accept` est soit le bouclage, soit l'ARP du pont, soit
///   `ct state established,related`, soit une connexion **nouvelle** portant
///   `comment "voie=<id> …"` avec `<id>` dans `voies_declarees`.
///
/// Renvoie la liste des écarts (vide = conforme).
pub fn auditer(texte: &str, voies_declarees: &BTreeSet<String>) -> Vec<String> {
    let mut ecarts = Vec::new();
    let mut dans_chaine = false;
    let mut politique_drop = false;
    let mut derniere = String::new();
    for (n, brute) in texte.lines().enumerate() {
        let l = brute.trim();
        let n = n + 1;
        if l.contains("policy accept") {
            ecarts.push(format!("ligne {n} : policy accept"));
        }
        if l.starts_with("chain ") {
            dans_chaine = true;
            politique_drop = false;
            derniere.clear();
            continue;
        }
        if dans_chaine && l == "}" {
            if !politique_drop {
                ecarts.push(format!("ligne {n} : chaîne sans policy drop"));
            }
            if !derniere.ends_with(" drop") {
                ecarts.push(format!("ligne {n} : chaîne non terminée par un rejet"));
            }
            dans_chaine = false;
            continue;
        }
        if !dans_chaine || l.is_empty() {
            continue;
        }
        if l.starts_with("type filter hook") {
            politique_drop = l.ends_with("policy drop;");
            continue;
        }
        derniere = l.to_string();
        let accepte = l.split_whitespace().any(|m| m == "accept");
        if !accepte {
            continue;
        }
        let permis_fixe = l == "iif \"lo\" accept"
            || l == "oif \"lo\" accept"
            || l == "ether type arp accept"
            || l == "ct state established,related counter accept";
        if permis_fixe {
            continue;
        }
        let voie = l
            .split("comment \"voie=")
            .nth(1)
            .and_then(|s| s.split(' ').next())
            .map(str::to_owned);
        match voie {
            Some(v) if voies_declarees.contains(&v) && l.contains("ct state new") => {}
            Some(v) => ecarts.push(format!("ligne {n} : accept sur voie non déclarée « {v} »")),
            None => ecarts.push(format!("ligne {n} : accept sans voie déclarée")),
        }
    }
    if dans_chaine {
        ecarts.push("chaîne non fermée".into());
    }
    ecarts
}
