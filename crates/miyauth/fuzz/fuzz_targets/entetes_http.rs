//! @id     miyauth_fuzz_entetes_http
//! @do     fuzzer_entetes_http_miyauth
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — analyseurs d'en-têtes de `miyauth::http` : base64 strict, PoP, autorisation, requêtes signées (EX-10, C-26).

//! Entrée : jusqu'à quatre valeurs d'en-tête séparées par `\n`.
//!
//! Propriétés : aucune panique ; base64 strictement canonique (décodé puis
//! ré-encodé = identique) ; toute valeur admise se reformate et se relit à
//! l'identique ; une requête signée par une clé fixe est admise une fois puis
//! refusée comme rejeu, et refusée si un seul champ change.

#![no_main]

use ed25519_dalek::{Signer, SigningKey};
use libfuzzer_sys::fuzz_target;
use miyauth::http::{
    analyser_autorisation_passeport, analyser_autorisation_visa, analyser_pop, b64_decoder, b64_encoder,
    formater_autorisation_passeport, formater_autorisation_visa, formater_pop, message_requete_signee,
    service::analyser_cle_publique, verifier_requete_signee, EntetesSignees, RegistreNonces,
};
use sha2::{Digest, Sha256};

fn signee(champs: &[&str]) {
    let cle = SigningKey::from_bytes(&[64; 32]);
    let (usage, methode, chemin) =
        (champs[0], champs.get(1).copied().unwrap_or("GET"), champs.get(2).copied().unwrap_or("/"));
    let corps = champs.get(3).copied().unwrap_or("").as_bytes();
    let horodatage = 1_800_000_000 + (usage.len() as u64);
    let nonce: Vec<u8> = (0..16 + (chemin.len() % 49) as u8).collect();
    let empreinte: [u8; 32] = Sha256::digest(corps).into();
    let message = message_requete_signee(usage, methode, chemin, horodatage, &nonce, &empreinte);
    let e = EntetesSignees {
        signataire: cle.verifying_key().to_bytes(),
        horodatage,
        nonce,
        signature: cle.sign(&message).to_bytes(),
    };
    let [s, h, n, g] = e.en_entetes();
    let relue = EntetesSignees::analyser(Some(&s.1), Some(&h.1), Some(&n.1), Some(&g.1)).expect("en-têtes relus");
    assert_eq!(relue, e);
    let nonces = RegistreNonces::new(8, 300);
    let longueurs_ok = usage.len() < 0xFFFF && methode.len() < 0xFFFF && chemin.len() < 0xFFFF;
    let r = verifier_requete_signee(&relue, usage, methode, chemin, corps, &nonces, horodatage);
    if longueurs_ok {
        r.expect("requête signée valide refusée");
        assert!(
            verifier_requete_signee(&relue, usage, methode, chemin, corps, &nonces, horodatage).is_err(),
            "rejeu admis"
        );
    }
    let nonces = RegistreNonces::new(8, 300);
    let autre_chemin = format!("{chemin}x");
    assert!(
        verifier_requete_signee(&relue, usage, methode, &autre_chemin, corps, &nonces, horodatage).is_err(),
        "chemin modifié admis"
    );
    if corps != b"autre" {
        assert!(
            verifier_requete_signee(&relue, usage, methode, chemin, b"autre", &nonces, horodatage).is_err(),
            "corps modifié admis"
        );
    }
}

fuzz_target!(|octets: &[u8]| {
    let Ok(texte) = std::str::from_utf8(octets) else { return };
    let champs: Vec<&str> = texte.splitn(4, '\n').collect();
    let premier = champs[0];

    if let Some(v) = b64_decoder(premier) {
        assert_eq!(b64_encoder(&v), premier, "base64 non canonique");
    }
    if let Ok(p) = analyser_pop(premier) {
        assert_eq!(analyser_pop(&formater_pop(&p.id_defi, &p.signature)).expect("PoP relue"), p);
    }
    if let Ok((visa, passeport)) = analyser_autorisation_visa(premier) {
        let f = formater_autorisation_visa(&visa, &passeport);
        assert_eq!(analyser_autorisation_visa(&f).expect("autorisation relue"), (visa, passeport));
    }
    if let Ok(passeport) = analyser_autorisation_passeport(premier) {
        let f = formater_autorisation_passeport(&passeport);
        assert_eq!(analyser_autorisation_passeport(&f).expect("autorisation relue"), passeport);
    }
    if let Some(cle) = analyser_cle_publique(premier) {
        assert_eq!(analyser_cle_publique(&b64_encoder(&cle)), Some(cle));
    }
    let entetes = EntetesSignees::analyser(
        champs.first().copied(),
        champs.get(1).copied(),
        champs.get(2).copied(),
        champs.get(3).copied(),
    );
    if let Ok(e) = entetes {
        let [s, h, n, g] = e.en_entetes();
        assert_eq!(
            EntetesSignees::analyser(Some(&s.1), Some(&h.1), Some(&n.1), Some(&g.1)).expect("en-têtes relus"),
            e
        );
        let nonces = RegistreNonces::new(8, 300);
        let _ = verifier_requete_signee(&e, "origin/fuzz", "POST", "/", b"", &nonces, e.horodatage);
    }
    signee(&champs);
});
