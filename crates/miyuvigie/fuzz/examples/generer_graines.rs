//! @id     miyuvigie_fuzz_generer_graines
//! @do     generer_corpus_initial_fuzzing
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — génère le corpus initial déterministe des cibles des Vigies (EX-10).

//! `cargo run --example generer_graines` (stable) : écrit `seeds/<cible>/*`.

use std::fs;
use std::path::Path;

fn ecrire(cible: &str, nom: &str, texte: &str) {
    let dossier = Path::new(env!("CARGO_MANIFEST_DIR")).join("seeds").join(cible);
    fs::create_dir_all(&dossier).expect("dossier de graines");
    fs::write(dossier.join(nom), texte).expect("écriture de graine");
}

fn main() {
    let lignes = [
        "1729760000.123456 hote nflog-110: MDOU|zone-cog|transit|SORTIE-COG| IN=sv0 OUT=z-pi SRC=10.77.0.10 DST=93.184.216.34 LEN=60 TTL=64 DF PROTO=TCP SPT=40000 DPT=443 SYN",
        "1729760000.123456 hote nflog-102: MDOU|portail-interne|entree|ENTREE-COG| IN=pi-franche OUT= SRC=10.77.1.10 DST=10.77.0.253 LEN=60 PROTO=TCP SPT=40000 DPT=7443",
        "1729760000.123456 hote nflog-101: MDOU|externe|transit|NON-DECLARE| IN=ext0 OUT=ext-franche SRC=8.8.8.8 DST=10.77.0.10 LEN=60 PROTO=TCP SPT=1 DPT=22",
        "1729760000.123456 hote nflog-111: MDOU|zone-franche|transit|NON-DECLARE| IN=z-pi OUT=sv1 SRC=10.77.1.253 DST=10.77.1.11 LEN=60 PROTO=TCP SPT=40000 DPT=22",
        "1729760000.123456 hote nflog-110: MDOU|zone-cog|transit|LATERAL| IN=sv0 OUT=sv1 SRC=10.77.0.10 DST=10.77.0.11 LEN=60 PROTO=UDP SPT=40000 DPT=53",
        "Oct 24 10:00:00 hote kernel: ligne quelconque",
    ];
    for (i, l) in lignes.iter().enumerate() {
        ecrire("journal_douane", &format!("ligne_{i}"), l);
    }
    ecrire("journal_douane", "lot", &lignes.join("\n"));

    let loc = u32::from_ne_bytes([10, 77, 0, 10]);
    let ext = u32::from_ne_bytes([93, 184, 216, 34]);
    let portail = u32::from_ne_bytes([10, 77, 0, 250]);
    let lo = u32::from_ne_bytes([127, 0, 0, 1]);
    let tcp = format!(
        "  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode\n\
         \x20  0: {lo:08X}:1538 00000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 1111 1\n\
         \x20  1: {loc:08X}:9C40 {ext:08X}:01BB 01 00000000:00000000 00:00000000 00000000  1000        0 2222 1\n\
         \x20  2: {loc:08X}:9C41 {portail:08X}:1B59 02 00000000:00000000 00:00000000 00000000  1000        0 3333 1\n\
         \x20  3: n'importe quoi\n"
    );
    ecrire("proc_net_tcp", "tcp", &tcp);
    let tcp6 = "  sl  local_address                         remote_address                        st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode\n\
                \x20  0: 00000000000000000000000001000000:1F90 00000000000000000000000000000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 4444 1\n\
                \x20  1: 0000000000000000FFFF00000A4D000A:9C40 0000000000000000FFFF00005DB8D822:01BB 01 00000000:00000000 00:00000000 00000000  1000        0 5555 1\n";
    ecrire("proc_net_tcp", "tcp6", tcp6);

    ecrire(
        "regles",
        "jeu",
        "# essai\nvigies-regles 1\n\
         regle R-FR-EXT si zone_src=franche et zone_dst=ext et verdict=autorise et octets>1048576 alors elevee flux_hors_contrat\n\
         regle R-PORT si zone_src=cog et port_dst=22 et voie=- alors critique sortie_zone_cog\n\
         regle R-VOIE si voie=jayoffice et verdict=bloque alors info mouvement_lateral\n",
    );
    ecrire("regles", "reserve", "vigies-regles 1\nregle R-EX12-SORTIE si zone_src=cog alors info flux_hors_contrat\n");
}
