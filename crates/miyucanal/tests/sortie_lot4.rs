//! @id     miyucanal_tests_sortie_lot4
//! @do     tester_sortie_lot4
//! @role   test
//! @layer  toolkit
//! @human  miyucanal — test de sortie du lot 4 : mise à jour signée installée de bout en bout, mise à jour altérée refusée à chaque étape.

//! Sortie du lot 4 (PLAN-IMPLEMENTATION §6) : « une mise à jour signée
//! installée de bout en bout ; une mise à jour altérée refusée ».
//!
//! Chaîne : publication (Origin + éditeur) → registre officiel (têtes de la
//! clé de journal d'Origin, A-006) → annonce 2 sur 2 → relevé via le dépôt
//! (voie en mémoire) → quarantaine écoulée → installateur appelé une fois.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use miyucanal::{
    sceller_annonce, sceller_revocation, CleQuarantaine, ConfigCanal, DepotMemoire, ErreurCanal, Evenement, Installateur,
    MiseAJourVerifiee, Motif, Releveur, QUARANTAINE_SECS,
};
use miyuregistre::{
    identite_sceau, Categorie, Cible, Corps, EntreeRegistre, ErreurJournal, ErreurTemps, HorlogeConfiance, Journal,
    Publication, StockageFichier, StockageMemoire,
};
use miyusceau::{
    sceller_multi, AncresDeConfiance, CleSignature, ErreurRegistre, ErreurSceau, NiveauConfiance, PolitiqueTete,
    PreuveInclusion,
};
use sha2::{Digest, Sha256};

const T0: u64 = 1_800_000_000;
const H: u64 = 3_600;
const REGISTRE: [u8; 32] = [0x0F; 32];
const COMPOSANT: &str = "strongfather";

#[derive(Default)]
struct InstallateurFactice {
    versions: HashMap<(Categorie, String), u64>,
    installees: Vec<(String, u64, Vec<u8>)>,
}

impl Installateur for InstallateurFactice {
    fn version_installee(&self, categorie: Categorie, composant: &str) -> Option<u64> {
        self.versions.get(&(categorie, composant.to_string())).copied()
    }

    fn installer(&mut self, maj: &MiseAJourVerifiee) -> Result<(), String> {
        self.versions.insert((maj.categorie(), maj.composant().to_string()), maj.version());
        self.installees.push((maj.composant().to_string(), maj.version(), maj.artefact().to_vec()));
        Ok(())
    }
}

/// Origin (signature + journal) et éditeur (côté Franche / International) et zone COG.
struct Banc {
    origin: CleSignature,
    journal_origin: CleSignature,
    editeur: CleSignature,
    journal: Journal<StockageMemoire>,
    depot: DepotMemoire,
    releveur: Releveur<StockageMemoire>,
    horloge: HorlogeConfiance,
    installateur: InstallateurFactice,
}

fn artefact(version: u64) -> Vec<u8> {
    format!("binaire strongfather v{version}").into_bytes()
}

fn publication(version: u64, octets: &[u8]) -> Publication {
    Publication {
        categorie: Categorie::Cores,
        composant: COMPOSANT.into(),
        version,
        empreinte: Sha256::digest(octets).into(),
        taille: octets.len() as u64,
    }
}

impl Banc {
    fn new() -> Self {
        let origin = CleSignature::generer();
        let journal_origin = CleSignature::generer();
        let editeur = CleSignature::generer();
        let config = ConfigCanal::nouvelle(
            REGISTRE,
            &origin.cle_publique(),
            &journal_origin.cle_publique(),
            &editeur.cle_publique(),
        )
        .unwrap();
        let mut installateur = InstallateurFactice::default();
        installateur.versions.insert((Categorie::Cores, COMPOSANT.into()), 1);
        Self {
            journal: Journal::ouvrir(REGISTRE, StockageMemoire::new(), StockageMemoire::new()).unwrap(),
            depot: DepotMemoire::new(),
            releveur: Releveur::new(config, StockageMemoire::new()).unwrap(),
            horloge: HorlogeConfiance::new(),
            installateur,
            origin,
            journal_origin,
            editeur,
        }
    }

    fn config(&self) -> ConfigCanal {
        ConfigCanal::nouvelle(
            REGISTRE,
            &self.origin.cle_publique(),
            &self.journal_origin.cle_publique(),
            &self.editeur.cle_publique(),
        )
        .unwrap()
    }

    /// Origin inscrit une entrée au registre officiel.
    fn inscrire(&mut self, sceau: &[u8]) -> u64 {
        self.journal.ajouter(sceau).unwrap()
    }

    /// Origin émet une tête (clé de journal d'Origin seule) et met à jour le dépôt (Franche).
    fn emettre_et_deposer(&mut self, t: u64) -> Vec<u8> {
        let tete = self.journal.emettre_tete(&[&self.journal_origin], t, 86_400).unwrap();
        self.depot.deposer_tete(tete.clone());
        self.depot.remplacer_entrees(self.journal.entrees(0, u64::MAX).unwrap());
        tete
    }

    /// Publie une mise à jour signée 2 sur 2 (annonce + artefact au dépôt).
    fn publier(&mut self, version: u64, t: u64) -> Vec<u8> {
        let octets = artefact(version);
        self.depot.deposer_artefact(octets.clone());
        let annonce = sceller_annonce(publication(version, &octets), t, &self.origin, &self.editeur).unwrap();
        self.inscrire(&annonce);
        annonce
    }

    fn cycle(&mut self, t: u64) -> Result<Vec<Evenement>, ErreurCanal> {
        self.releveur.cycle(&mut self.depot, &mut self.installateur, &mut self.horloge, t)
    }

    /// Avance jusqu'à la fin de la quarantaine en publiant des têtes fraîches.
    fn traverser_quarantaine(&mut self, depuis: u64) -> Vec<Evenement> {
        let mut tous = Vec::new();
        for t in [depuis + 12 * H, depuis + 24 * H, depuis + 36 * H, depuis + QUARANTAINE_SECS + 1] {
            if self.journal.derniere_tete().is_some_and(|d| d.emise_a >= t) {
                continue;
            }
            self.emettre_et_deposer(t);
            tous.extend(self.cycle(t).unwrap());
        }
        tous
    }
}

fn contient(evenements: &[Evenement], f: impl Fn(&Evenement) -> bool) -> bool {
    evenements.iter().any(f)
}

#[test]
fn sortie_mise_a_jour_signee_installee_de_bout_en_bout() {
    let mut b = Banc::new();
    b.publier(2, T0);
    b.emettre_et_deposer(T0);

    let ev = b.cycle(T0).unwrap();
    assert!(contient(&ev, |e| matches!(e, Evenement::Synchronise { de: 0, a: 1 })));
    assert!(contient(&ev, |e| matches!(e, Evenement::EnQuarantaine { version: 2, installable_a, .. }
        if *installable_a == T0 + QUARANTAINE_SECS)));
    assert!(contient(&ev, |e| matches!(e, Evenement::EnAttente { version: 2, .. })));
    assert_eq!(b.installateur.installees.len(), 0);

    let ev = b.traverser_quarantaine(T0);
    assert!(contient(&ev, |e| matches!(e, Evenement::Installee { version: 2, .. })));
    assert_eq!(b.installateur.installees, vec![(COMPOSANT.to_string(), 2, artefact(2))]);

    // Plus rien à installer : appelé une seule fois.
    b.emettre_et_deposer(T0 + QUARANTAINE_SECS + 2);
    b.cycle(T0 + QUARANTAINE_SECS + 2).unwrap();
    assert_eq!(b.installateur.installees.len(), 1);

    // Sens unique : seules des demandes génériques sont sorties.
    assert!(b.depot.demandes().iter().all(|d| matches!(
        d,
        miyucanal::Demande::Tete | miyucanal::Demande::Entrees { .. } | miyucanal::Demande::Artefact { .. }
    )));
}

#[test]
fn refus_artefact_altere() {
    let mut b = Banc::new();
    b.publier(2, T0);
    let empreinte: [u8; 32] = Sha256::digest(artefact(2)).into();
    // Dépôt compromis : autres octets sous l'empreinte annoncée (même taille, puis autre taille).
    let mut altere = artefact(2);
    altere[0] ^= 0x20;
    b.depot.remplacer_artefact(empreinte, altere);
    b.emettre_et_deposer(T0);
    b.cycle(T0).unwrap();
    let ev = b.traverser_quarantaine(T0);
    assert!(contient(&ev, |e| matches!(e, Evenement::InstallationRefusee { motif: Motif::ArtefactAltere, .. })));
    b.depot.remplacer_artefact(empreinte, b"court".to_vec());
    let ev = b.cycle(T0 + QUARANTAINE_SECS + 1).unwrap();
    assert!(contient(&ev, |e| matches!(e, Evenement::InstallationRefusee { motif: Motif::ArtefactAltere, .. })));
    assert_eq!(b.installateur.installees.len(), 0);

    // Le dépôt rétablit l'artefact authentique : installation (l'annonce était restée en attente).
    b.depot.deposer_artefact(artefact(2));
    b.cycle(T0 + QUARANTAINE_SECS + 1).unwrap();
    assert_eq!(b.installateur.installees.len(), 1);
}

#[test]
fn refus_signature_unique() {
    let mut b = Banc::new();
    let octets = artefact(2);
    b.depot.deposer_artefact(octets.clone());
    let entree = EntreeRegistre { inscrite_a: T0, corps: Corps::Publication(publication(2, &octets)) };
    // Origin seul (clé éditeur hors ligne absente), puis éditeur seul.
    b.inscrire(&sceller_multi(&entree, &[&b.origin]).unwrap());
    b.inscrire(&sceller_multi(&entree, &[&b.editeur]).unwrap());
    b.emettre_et_deposer(T0);
    let ev = b.cycle(T0).unwrap();
    let refus = ev
        .iter()
        .filter(|e| {
            matches!(e, Evenement::AnnonceRefusee {
                motif: Motif::Signature(ErreurSceau::SeuilNonAtteint { requis: 2, obtenu: 1 }),
                ..
            })
        })
        .count();
    assert_eq!(refus, 2);
    b.traverser_quarantaine(T0);
    assert_eq!(b.installateur.installees.len(), 0);

}

#[test]
fn tetes_signees_par_la_seule_cle_de_journal_d_origin() {
    let mut b = Banc::new();
    b.publier(2, T0);
    // Tête de la seule clé de journal d'Origin : acceptée (chemin par défaut).
    b.emettre_et_deposer(T0);
    assert!(b.cycle(T0).is_ok());
    assert!(b.releveur.miroir().suivi().etat().is_some());

    // Éditeur seul, Origin (clé de signature) seul, clé inconnue : refusées.
    let intrus = CleSignature::generer();
    let fautives = [b.editeur.graine(), b.origin.graine(), intrus.graine()].map(|g| CleSignature::depuis_graine(&g));
    for (k, cle) in fautives.iter().enumerate() {
        let t = T0 + H * (k as u64 + 1);
        let tete = b.journal.emettre_tete(&[cle], t, 86_400).unwrap();
        b.depot.deposer_tete(tete);
        assert!(matches!(
            b.cycle(t),
            Err(ErreurCanal::Registre(ErreurJournal::Registre(ErreurRegistre::Sceau(ErreurSceau::SignataireInconnu))))
        ));
    }
    // Une signature valide de la clé de journal accompagnée d'un intrus reste acceptée (intrus non compté).
    let tete = b.journal.emettre_tete(&[&b.journal_origin, &intrus], T0 + 4 * H, 86_400).unwrap();
    b.depot.deposer_tete(tete);
    assert!(b.cycle(T0 + 4 * H).is_ok());
}

#[test]
fn configuration_des_tetes_jamais_plus_laxiste() {
    let b = Banc::new();
    let (origin, journal, editeur) = (b.origin.cle_publique(), b.journal_origin.cle_publique(), b.editeur.cle_publique());
    // Clé de journal d'Origin dédiée.
    assert!(ConfigCanal::nouvelle(REGISTRE, &origin, &origin, &editeur).is_err());
    assert!(ConfigCanal::nouvelle(REGISTRE, &origin, &editeur, &editeur).is_err());

    let ancres = |cles: &[[u8; 32]]| {
        let mut ancres = AncresDeConfiance::new();
        for cle in cles {
            ancres.ajouter(cle, NiveauConfiance::Reconnu).unwrap();
        }
        ancres
    };
    let politique = |seuil| PolitiqueTete::nouvelle(seuil, 86_400).unwrap();
    // Sans la clé de journal, seuil partiel, âge plus long : refus.
    assert!(b.config().avec_tetes(ancres(&[origin, editeur]), politique(2)).is_err());
    assert!(b.config().avec_tetes(ancres(&[journal, origin]), politique(1)).is_err());
    assert!(b.config().avec_tetes(ancres(&[journal]), PolitiqueTete::nouvelle(1, 2 * 86_400).unwrap()).is_err());

    // Plus stricte : 2 sur 2 {journal d'Origin, Origin} ; la clé de journal seule ne suffit plus.
    let stricte = b.config().avec_tetes(ancres(&[journal, origin]), PolitiqueTete::canal_officiel()).unwrap();
    let mut b = b;
    b.releveur = Releveur::new(stricte, StockageMemoire::new()).unwrap();
    b.publier(2, T0);
    b.emettre_et_deposer(T0);
    assert!(matches!(
        b.cycle(T0),
        Err(ErreurCanal::Registre(ErreurJournal::Registre(ErreurRegistre::Sceau(ErreurSceau::SeuilNonAtteint {
            requis: 2,
            obtenu: 1
        }))))
    ));
    let tete = b.journal.emettre_tete(&[&b.journal_origin, &b.origin], T0 + 1, 86_400).unwrap();
    b.depot.deposer_tete(tete);
    assert!(b.cycle(T0 + 1).is_ok());
}

#[test]
fn refus_annonce_hors_registre() {
    let mut b = Banc::new();
    // 1. Annonce 2 sur 2 jamais inscrite : jamais vue, jamais installée.
    let octets = artefact(2);
    b.depot.deposer_artefact(octets.clone());
    let hors = sceller_annonce(publication(2, &octets), T0, &b.origin, &b.editeur).unwrap();
    b.publier(3, T0); // une autre annonce, inscrite
    b.emettre_et_deposer(T0);
    b.cycle(T0).unwrap();

    // 2. Livrée hors registre avec une preuve d'inclusion forgée : refus.
    let fausse = PreuveInclusion { index: 0, taille: 1, chemin: vec![] };
    let mut horloge = b.horloge.clone();
    assert!(matches!(
        b.releveur.verifier_annonce_incluse(&hors, &fausse, &mut horloge, T0),
        Err(ErreurCanal::Registre(ErreurJournal::Registre(ErreurRegistre::PreuveInvalide)))
    ));
    // L'annonce inscrite, elle, est prouvée.
    let incluse = b.journal.entree(0).unwrap();
    let vraie = b.journal.preuve_inclusion(0, 1).unwrap();
    assert!(b.releveur.verifier_annonce_incluse(&incluse, &vraie, &mut horloge, T0).is_ok());

    // 3. Dépôt compromis : substitue une entrée non couverte par la racine signée.
    let substituee = b.publier(4, T0 + H);
    let tete = b.journal.emettre_tete(&[&b.journal_origin], T0 + H, 86_400).unwrap();
    b.depot.deposer_tete(tete);
    let mut entrees = b.journal.entrees(0, u64::MAX).unwrap();
    entrees[1] = hors.clone();
    assert_ne!(entrees[1], substituee);
    b.depot.remplacer_entrees(entrees);
    assert!(matches!(b.cycle(T0 + H), Err(ErreurCanal::Registre(ErreurJournal::RacineDifferente))));

    b.depot.remplacer_entrees(b.journal.entrees(0, u64::MAX).unwrap());
    let ev = b.cycle(T0 + H).unwrap();
    assert!(contient(&ev, |e| matches!(e, Evenement::Annulee { version: 3, .. })));
    b.traverser_quarantaine(T0 + H);
    // Seule la dernière version inscrite (v4) est installée ; v2 (hors registre) jamais.
    assert_eq!(b.installateur.installees.iter().map(|i| i.1).collect::<Vec<_>>(), vec![4]);
}

#[test]
fn refus_tete_perimee_ou_en_recul() {
    let mut b = Banc::new();
    b.publier(2, T0);
    let ancienne = b.emettre_et_deposer(T0);
    b.cycle(T0).unwrap();
    b.emettre_et_deposer(T0 + H);
    b.cycle(T0 + H).unwrap();

    // Retour arrière : le dépôt rejoue l'ancienne tête.
    b.depot.deposer_tete(ancienne.clone());
    assert!(matches!(
        b.cycle(T0 + 2 * H),
        Err(ErreurCanal::Registre(
            ErreurJournal::Temps(ErreurTemps::AnterieurMedian { .. })
                | ErreurJournal::Registre(ErreurRegistre::RetourArriere)
        ))
    ));

    // Gel : le dépôt ne fournit plus de tête récente ; quarantaine écoulée mais rien d'installé.
    let t = T0 + QUARANTAINE_SECS + 1;
    assert!(matches!(
        b.cycle(t),
        Err(ErreurCanal::Registre(ErreurJournal::Registre(ErreurRegistre::TeteGelee { .. })))
    ));
    let mut horloge = b.horloge.clone();
    assert!(matches!(
        b.releveur.installer_pretes(&mut b.depot, &mut b.installateur, &mut horloge, t),
        Err(ErreurCanal::Registre(ErreurJournal::Registre(ErreurRegistre::TeteGelee { .. })))
    ));

    // Tête encore fraîche mais émise avant la fin de la quarantaine : attente.
    let mut b = Banc::new();
    b.publier(2, T0);
    b.emettre_et_deposer(T0);
    b.cycle(T0).unwrap();
    b.emettre_et_deposer(T0 + 47 * H);
    let ev = b.cycle(T0 + QUARANTAINE_SECS + 1).unwrap();
    assert!(contient(&ev, |e| matches!(e, Evenement::EnAttente { .. })));
    assert_eq!(b.installateur.installees.len(), 0);

    // Horloge murale recalée sous le Median-Time-Past : refus.
    assert!(matches!(b.cycle(T0 - H), Err(ErreurCanal::Temps(ErreurTemps::AnterieurMedian { .. }))));
    assert_eq!(b.installateur.installees.len(), 0);
}

#[test]
fn refus_revocation_pendant_quarantaine() {
    for par_artefact in [false, true] {
        let mut b = Banc::new();
        let annonce = b.publier(2, T0);
        b.emettre_et_deposer(T0);
        b.cycle(T0).unwrap();

        // Pendant la quarantaine, l'éditeur (ou Origin) révoque.
        let cible = if par_artefact {
            Cible::Artefact(Sha256::digest(artefact(2)).into())
        } else {
            Cible::Annonce(identite_sceau(&annonce).unwrap())
        };
        let revocation = sceller_revocation(cible, "faille découverte", T0 + 20 * H, &[&b.editeur]).unwrap();
        b.inscrire(&revocation);
        b.emettre_et_deposer(T0 + 20 * H);
        let ev = b.cycle(T0 + 20 * H).unwrap();
        assert!(contient(&ev, |e| matches!(e, Evenement::Revocation { .. })));
        assert!(contient(&ev, |e| matches!(e, Evenement::Annulee { version: 2, .. })));

        b.traverser_quarantaine(T0);
        assert_eq!(b.installateur.installees.len(), 0);
        assert_eq!(b.releveur.en_attente().len(), 0);
    }
}

#[test]
fn refus_retrogradation() {
    let mut b = Banc::new();
    b.installateur.versions.insert((Categorie::Cores, COMPOSANT.into()), 5);
    b.publier(4, T0);
    b.publier(5, T0);
    b.emettre_et_deposer(T0);
    let ev = b.cycle(T0).unwrap();
    assert!(contient(&ev, |e| matches!(e, Evenement::AnnonceRefusee {
        motif: Motif::Retrogradation { reference: 5, proposee: 4 },
        ..
    })));
    assert!(contient(&ev, |e| matches!(e, Evenement::AnnonceRefusee {
        motif: Motif::Retrogradation { reference: 5, proposee: 5 },
        ..
    })));

    // v7 en attente, puis v6 annoncée : refusée ; v8 remplace v7.
    b.publier(7, T0 + H);
    b.publier(6, T0 + H);
    b.publier(8, T0 + H);
    b.emettre_et_deposer(T0 + H);
    let ev = b.cycle(T0 + H).unwrap();
    assert!(contient(&ev, |e| matches!(e, Evenement::AnnonceRefusee {
        motif: Motif::Retrogradation { reference: 7, proposee: 6 },
        ..
    })));
    assert!(contient(&ev, |e| matches!(e, Evenement::Annulee { version: 7, .. })));
    b.traverser_quarantaine(T0 + H);
    assert_eq!(b.installateur.installees.iter().map(|i| i.1).collect::<Vec<_>>(), vec![8]);
}

#[test]
fn quarantaine_comptee_depuis_la_premiere_vue_locale() {
    let mut b = Banc::new();
    // Annonce antidatée de trois jours : la quarantaine part quand même de T0.
    b.publier(2, T0 - 72 * H);
    b.emettre_et_deposer(T0);
    b.cycle(T0).unwrap();
    b.emettre_et_deposer(T0 + H);
    let ev = b.cycle(T0 + H).unwrap();
    assert!(contient(&ev, |e| matches!(e, Evenement::EnAttente { .. })));
    assert_eq!(b.installateur.installees.len(), 0);
    assert!(ConfigCanal::nouvelle(REGISTRE, &[1; 32], &[2; 32], &[1; 32]).is_err());
}

static N: AtomicU64 = AtomicU64::new(0);

fn chemin_temporaire() -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "miyucanal-sortie-{}-{}.mrgj",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    ))
}

/// Secret local du COG (zone COG) dont dérive la clé de scellement de la quarantaine.
const SECRET_LOCAL: [u8; 32] = [0x5A; 32];

/// État persisté par la zone COG après un relevé.
struct Persiste {
    etat: Vec<u8>,
    temps: Vec<u8>,
    quarantaine: Vec<u8>,
    horloge: Vec<u8>,
}

fn persister(releveur: &Releveur<StockageFichier>, horloge: &HorlogeConfiance) -> Persiste {
    let (etat, temps) = releveur.etat_persistant().unwrap();
    Persiste {
        etat,
        temps,
        quarantaine: releveur.etat_quarantaine(&CleQuarantaine::deriver(&SECRET_LOCAL)),
        horloge: horloge.encoder(),
    }
}

/// Redémarrage : miroir fichier rouvert, états repris, horloge de confiance restaurée.
fn redemarrer(b: &mut Banc, chemin: &std::path::Path, p: &Persiste) -> Releveur<StockageFichier> {
    b.horloge = HorlogeConfiance::decoder(&p.horloge).unwrap();
    Releveur::reprendre(
        b.config(),
        StockageFichier::ouvrir(chemin).unwrap(),
        &p.etat,
        &p.temps,
        &p.quarantaine,
        &CleQuarantaine::deriver(&SECRET_LOCAL),
    )
    .unwrap()
}

fn cycle_fichier(b: &mut Banc, r: &mut Releveur<StockageFichier>, t: u64) -> Result<Vec<Evenement>, ErreurCanal> {
    r.cycle(&mut b.depot, &mut b.installateur, &mut b.horloge, t)
}

fn installable_a(evenements: &[Evenement], composant: &str) -> Option<u64> {
    evenements.iter().find_map(|e| match e {
        Evenement::EnQuarantaine { composant: c, installable_a, .. } if c == composant => Some(*installable_a),
        _ => None,
    })
}

#[test]
fn reprise_apres_24_h_reprend_le_decompte_et_installe_a_48_h() {
    let chemin = chemin_temporaire();
    let mut b = Banc::new();
    let mut r = Releveur::new(b.config(), StockageFichier::ouvrir(&chemin).unwrap()).unwrap();
    b.publier(2, T0);
    b.emettre_et_deposer(T0);
    let ev = cycle_fichier(&mut b, &mut r, T0).unwrap();
    assert_eq!(installable_a(&ev, COMPOSANT), Some(T0 + QUARANTAINE_SECS));
    b.emettre_et_deposer(T0 + 24 * H);
    cycle_fichier(&mut b, &mut r, T0 + 24 * H).unwrap();
    let p = persister(&r, &b.horloge);
    drop(r);

    // Redémarrage après 24 h : le décompte reprend depuis la première vue (T0).
    let mut r = redemarrer(&mut b, &chemin, &p);
    assert_eq!(r.miroir().taille(), 1);
    b.emettre_et_deposer(T0 + 30 * H);
    let ev = cycle_fichier(&mut b, &mut r, T0 + 30 * H).unwrap();
    assert_eq!(installable_a(&ev, COMPOSANT), Some(T0 + QUARANTAINE_SECS));
    assert!(!contient(&ev, |e| matches!(e, Evenement::QuarantaineAlteree { .. })));
    assert_eq!(b.installateur.installees.len(), 0);
    // L'état persisté avant le premier relevé réussi garde les premières vues.
    assert_eq!(r.etat_quarantaine(&CleQuarantaine::deriver(&SECRET_LOCAL)), p.quarantaine);

    // 48 h au total : installée.
    b.emettre_et_deposer(T0 + QUARANTAINE_SECS + 1);
    let ev = cycle_fichier(&mut b, &mut r, T0 + QUARANTAINE_SECS + 1).unwrap();
    assert!(contient(&ev, |e| matches!(e, Evenement::Installee { version: 2, .. })));
    assert_eq!(b.installateur.installees, vec![(COMPOSANT.to_string(), 2, artefact(2))]);
    let _ = std::fs::remove_file(&chemin);
}

#[test]
fn etat_de_quarantaine_altere_relance_sans_jamais_raccourcir() {
    const OS: &str = "noyau-linux";
    let preparer = |b: &mut Banc, chemin: &std::path::Path| {
        let mut r = Releveur::new(b.config(), StockageFichier::ouvrir(chemin).unwrap()).unwrap();
        b.publier(2, T0);
        let octets = format!("{OS} v1").into_bytes();
        b.depot.deposer_artefact(octets.clone());
        let p = Publication {
            categorie: Categorie::Os,
            composant: OS.into(),
            version: 1,
            empreinte: Sha256::digest(&octets).into(),
            taille: octets.len() as u64,
        };
        let annonce = sceller_annonce(p, T0, &b.origin, &b.editeur).unwrap();
        b.inscrire(&annonce);
        b.emettre_et_deposer(T0);
        cycle_fichier(b, &mut r, T0).unwrap();
        b.emettre_et_deposer(T0 + 24 * H);
        cycle_fichier(b, &mut r, T0 + 24 * H).unwrap();
        persister(&r, &b.horloge)
    };

    // 1. Un enregistrement altéré (première vue « reculée » de 3 jours pour
    //    raccourcir) : refusé, sa quarantaine repart ; l'autre reprend.
    let chemin = chemin_temporaire();
    let mut b = Banc::new();
    let mut p = preparer(&mut b, &chemin);
    // Enregistrements triés (Cores < OS) : le premier est celui de strongfather.
    let position_vue = 4 + 1 + 32 + 4 + 32 + 8;
    p.quarantaine[position_vue..position_vue + 8].copy_from_slice(&(T0 - 72 * H).to_be_bytes());
    let mut r = redemarrer(&mut b, &chemin, &p);
    let t = T0 + 25 * H;
    b.emettre_et_deposer(t);
    let ev = cycle_fichier(&mut b, &mut r, t).unwrap();
    assert!(contient(&ev, |e| matches!(e, Evenement::QuarantaineAlteree { refuses: 1, illisible: false })));
    assert_eq!(installable_a(&ev, COMPOSANT), Some(t + QUARANTAINE_SECS));
    assert_eq!(installable_a(&ev, OS), Some(T0 + QUARANTAINE_SECS));
    // À 48 h de la première vue : seul l'enregistrement intact est installé.
    b.emettre_et_deposer(T0 + QUARANTAINE_SECS + 1);
    cycle_fichier(&mut b, &mut r, T0 + QUARANTAINE_SECS + 1).unwrap();
    assert_eq!(b.installateur.installees.iter().map(|i| i.0.as_str()).collect::<Vec<_>>(), vec![OS]);
    // L'annonce relancée attend ses 48 h complètes.
    b.emettre_et_deposer(t + QUARANTAINE_SECS - H);
    cycle_fichier(&mut b, &mut r, t + QUARANTAINE_SECS - H).unwrap();
    assert_eq!(b.installateur.installees.len(), 1);
    b.emettre_et_deposer(t + QUARANTAINE_SECS + 1);
    cycle_fichier(&mut b, &mut r, t + QUARANTAINE_SECS + 1).unwrap();
    assert_eq!(b.installateur.installees.len(), 2);
    let _ = std::fs::remove_file(&chemin);

    // 2. État illisible (absent ou étranger) : toutes les quarantaines repartent.
    for illisible in [Vec::new(), b"pas un etat de quarantaine".to_vec()] {
        let chemin = chemin_temporaire();
        let mut b = Banc::new();
        let mut p = preparer(&mut b, &chemin);
        p.quarantaine = illisible;
        let mut r = redemarrer(&mut b, &chemin, &p);
        b.emettre_et_deposer(t);
        let ev = cycle_fichier(&mut b, &mut r, t).unwrap();
        assert!(contient(&ev, |e| matches!(e, Evenement::QuarantaineAlteree { refuses: 0, illisible: true })));
        assert_eq!(installable_a(&ev, COMPOSANT), Some(t + QUARANTAINE_SECS));
        assert_eq!(installable_a(&ev, OS), Some(t + QUARANTAINE_SECS));
        b.emettre_et_deposer(T0 + QUARANTAINE_SECS + 1);
        cycle_fichier(&mut b, &mut r, T0 + QUARANTAINE_SECS + 1).unwrap();
        assert_eq!(b.installateur.installees.len(), 0);
        let _ = std::fs::remove_file(&chemin);
    }

    // 3. Clé locale différente (état d'un autre COG) : tout est refusé, relancé.
    let chemin = chemin_temporaire();
    let mut b = Banc::new();
    let p = preparer(&mut b, &chemin);
    b.horloge = HorlogeConfiance::decoder(&p.horloge).unwrap();
    let mut r = Releveur::reprendre(
        b.config(),
        StockageFichier::ouvrir(&chemin).unwrap(),
        &p.etat,
        &p.temps,
        &p.quarantaine,
        &CleQuarantaine::deriver(&[0x11; 32]),
    )
    .unwrap();
    b.emettre_et_deposer(t);
    let ev = cycle_fichier(&mut b, &mut r, t).unwrap();
    assert!(contient(&ev, |e| matches!(e, Evenement::QuarantaineAlteree { refuses: 2, illisible: false })));
    assert_eq!(installable_a(&ev, OS), Some(t + QUARANTAINE_SECS));
    let _ = std::fs::remove_file(&chemin);
}

#[test]
fn recul_d_horloge_sous_le_mtp_refuse_apres_redemarrage() {
    let chemin = chemin_temporaire();
    let mut b = Banc::new();
    let mut r = Releveur::new(b.config(), StockageFichier::ouvrir(&chemin).unwrap()).unwrap();
    b.publier(2, T0);
    b.emettre_et_deposer(T0);
    cycle_fichier(&mut b, &mut r, T0).unwrap();
    b.emettre_et_deposer(T0 + 24 * H);
    cycle_fichier(&mut b, &mut r, T0 + 24 * H).unwrap();
    let p = persister(&r, &b.horloge);
    drop(r);

    let mut r = redemarrer(&mut b, &chemin, &p);
    // MTP restauré (fenêtre paire {T0, T0 + 24 h} : médiane haute).
    assert_eq!(b.horloge.mtp(), Some(T0 + 24 * H));
    assert!(matches!(
        cycle_fichier(&mut b, &mut r, T0 + H),
        Err(ErreurCanal::Temps(ErreurTemps::AnterieurMedian { .. }))
    ));
    // Horloge murale avancée au-delà de 48 h sans tête postérieure : rien d'installé.
    let ev = cycle_fichier(&mut b, &mut r, T0 + 24 * H + 60).unwrap();
    assert_eq!(installable_a(&ev, COMPOSANT), Some(T0 + QUARANTAINE_SECS));
    assert!(matches!(
        r.installer_pretes(&mut b.depot, &mut b.installateur, &mut b.horloge, T0 + QUARANTAINE_SECS + H),
        Err(ErreurCanal::Registre(ErreurJournal::Registre(ErreurRegistre::TeteGelee { .. })))
    ));
    assert_eq!(b.installateur.installees.len(), 0);
    let _ = std::fs::remove_file(&chemin);
}

#[test]
fn revocations_par_origin_ou_editeur_seulement() {
    let intrus = CleSignature::generer();
    for (signataire, acceptee) in [("origin", true), ("editeur", true), ("journal", false), ("intrus", false)] {
        let mut b = Banc::new();
        let annonce = b.publier(2, T0);
        b.emettre_et_deposer(T0);
        b.cycle(T0).unwrap();
        let cle = match signataire {
            "origin" => &b.origin,
            "editeur" => &b.editeur,
            "journal" => &b.journal_origin,
            _ => &intrus,
        };
        let cible = Cible::Annonce(identite_sceau(&annonce).unwrap());
        let revocation = sceller_revocation(cible, "faille", T0 + H, &[cle]).unwrap();
        let index = b.inscrire(&revocation);
        b.emettre_et_deposer(T0 + H);
        let ev = b.cycle(T0 + H).unwrap();
        assert_eq!(contient(&ev, |e| matches!(e, Evenement::Revocation { .. })), acceptee, "{signataire}");
        assert_eq!(contient(&ev, |e| matches!(e, Evenement::Annulee { version: 2, .. })), acceptee, "{signataire}");
        assert_eq!(contient(&ev, |e| matches!(e, Evenement::EntreeIgnoree { index: i } if *i == index)), !acceptee);
        b.traverser_quarantaine(T0);
        assert_eq!(b.installateur.installees.len(), usize::from(!acceptee), "{signataire}");
    }
}

#[test]
fn os_et_modeles_ia_par_le_meme_canal() {
    let mut b = Banc::new();
    for (categorie, composant) in [(Categorie::Os, "noyau-linux"), (Categorie::ModeleIa, "maia-7b.gguf")] {
        let octets = format!("{composant} v1").into_bytes();
        b.depot.deposer_artefact(octets.clone());
        let p = Publication {
            categorie,
            composant: composant.into(),
            version: 1,
            empreinte: Sha256::digest(&octets).into(),
            taille: octets.len() as u64,
        };
        b.inscrire(&sceller_annonce(p, T0, &b.origin, &b.editeur).unwrap());
    }
    b.emettre_et_deposer(T0);
    b.cycle(T0).unwrap();
    b.traverser_quarantaine(T0);
    let mut installes: Vec<_> = b.installateur.installees.iter().map(|i| i.0.clone()).collect();
    installes.sort();
    assert_eq!(installes, vec!["maia-7b.gguf".to_string(), "noyau-linux".to_string()]);
}
