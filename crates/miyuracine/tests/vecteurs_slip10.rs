//! @id     miyuracine_tests_vecteurs_slip10
//! @do     verifier_vecteurs_officiels_slip10
//! @role   test
//! @layer  toolkit
//! @human  miyuracine — vecteurs officiels SLIP-10 (Ed25519).

//! Vecteurs officiels SLIP-0010 « Test vector 1 for ed25519 » et « Test vector 2
//! for ed25519 » (<https://github.com/satoshilabs/slips/blob/master/slip-0010.md>),
//! recopiés à l'identique : 2 graines × 6 nœuds = 12 nœuds ; pour chacun, code
//! de chaîne, clé privée et clé publique (`0x00 ‖ clé RFC 8032`) sont comparés.
//! L'empreinte (« fingerprint », Hash160) n'est pas vérifiée : elle sert
//! seulement à BIP-32 et demanderait RIPEMD-160.

use miyuracine::NoeudSlip10;
use std::fmt::Write as _;

struct Noeud {
    chemin: &'static [u32],
    chaine: &'static str,
    privee: &'static str,
    publique: &'static str,
}

const GRAINE_1: &str = "000102030405060708090a0b0c0d0e0f";
const VECTEUR_1: [Noeud; 6] = [
    Noeud {
        chemin: &[],
        chaine: "90046a93de5380a72b5e45010748567d5ea02bbf6522f979e05c0d8d8ca9fffb",
        privee: "2b4be7f19ee27bbf30c667b642d5f4aa69fd169872f8fc3059c08ebae2eb19e7",
        publique: "00a4b2856bfec510abab89753fac1ac0e1112364e7d250545963f135f2a33188ed",
    },
    Noeud {
        chemin: &[0],
        chaine: "8b59aa11380b624e81507a27fedda59fea6d0b779a778918a2fd3590e16e9c69",
        privee: "68e0fe46dfb67e368c75379acec591dad19df3cde26e63b93a8e704f1dade7a3",
        publique: "008c8a13df77a28f3445213a0f432fde644acaa215fc72dcdf300d5efaa85d350c",
    },
    Noeud {
        chemin: &[0, 1],
        chaine: "a320425f77d1b5c2505a6b1b27382b37368ee640e3557c315416801243552f14",
        privee: "b1d0bad404bf35da785a64ca1ac54b2617211d2777696fbffaf208f746ae84f2",
        publique: "001932a5270f335bed617d5b935c80aedb1a35bd9fc1e31acafd5372c30f5c1187",
    },
    Noeud {
        chemin: &[0, 1, 2],
        chaine: "2e69929e00b5ab250f49c3fb1c12f252de4fed2c1db88387094a0f8c4c9ccd6c",
        privee: "92a5b23c0b8a99e37d07df3fb9966917f5d06e02ddbd909c7e184371463e9fc9",
        publique: "00ae98736566d30ed0e9d2f4486a64bc95740d89c7db33f52121f8ea8f76ff0fc1",
    },
    Noeud {
        chemin: &[0, 1, 2, 2],
        chaine: "8f6d87f93d750e0efccda017d662a1b31a266e4a6f5993b15f5c1f07f74dd5cc",
        privee: "30d1dc7e5fc04c31219ab25a27ae00b50f6fd66622f6e9c913253d6511d1e662",
        publique: "008abae2d66361c879b900d204ad2cc4984fa2aa344dd7ddc46007329ac76c429c",
    },
    Noeud {
        chemin: &[0, 1, 2, 2, 1_000_000_000],
        chaine: "68789923a0cac2cd5a29172a475fe9e0fb14cd6adb5ad98a3fa70333e7afa230",
        privee: "8f94d394a8e8fd6b1bc2f3f49f5c47e385281d5c17e65324b0f62483e37e8793",
        publique: "003c24da049451555d51a7014a37337aa4e12d41e485abccfa46b47dfb2af54b7a",
    },
];

const GRAINE_2: &str = "fffcf9f6f3f0edeae7e4e1dedbd8d5d2cfccc9c6c3c0bdbab7b4b1aeaba8a5a29f9c999693908d8a8784817e7b7875726f6c696663605d5a5754514e4b484542";
const VECTEUR_2: [Noeud; 6] = [
    Noeud {
        chemin: &[],
        chaine: "ef70a74db9c3a5af931b5fe73ed8e1a53464133654fd55e7a66f8570b8e33c3b",
        privee: "171cb88b1b3c1db25add599712e36245d75bc65a1a5c9e18d76f9f2b1eab4012",
        publique: "008fe9693f8fa62a4305a140b9764c5ee01e455963744fe18204b4fb948249308a",
    },
    Noeud {
        chemin: &[0],
        chaine: "0b78a3226f915c082bf118f83618a618ab6dec793752624cbeb622acb562862d",
        privee: "1559eb2bbec5790b0c65d8693e4d0875b1747f4970ae8b650486ed7470845635",
        publique: "0086fab68dcb57aa196c77c5f264f215a112c22a912c10d123b0d03c3c28ef1037",
    },
    Noeud {
        chemin: &[0, 2_147_483_647],
        chaine: "138f0b2551bcafeca6ff2aa88ba8ed0ed8de070841f0c4ef0165df8181eaad7f",
        privee: "ea4f5bfe8694d8bb74b7b59404632fd5968b774ed545e810de9c32a4fb4192f4",
        publique: "005ba3b9ac6e90e83effcd25ac4e58a1365a9e35a3d3ae5eb07b9e4d90bcf7506d",
    },
    Noeud {
        chemin: &[0, 2_147_483_647, 1],
        chaine: "73bd9fff1cfbde33a1b846c27085f711c0fe2d66fd32e139d3ebc28e5a4a6b90",
        privee: "3757c7577170179c7868353ada796c839135b3d30554bbb74a4b1e4a5a58505c",
        publique: "002e66aa57069c86cc18249aecf5cb5a9cebbfd6fadeab056254763874a9352b45",
    },
    Noeud {
        chemin: &[0, 2_147_483_647, 1, 2_147_483_646],
        chaine: "0902fe8a29f9140480a00ef244bd183e8a13288e4412d8389d140aac1794825a",
        privee: "5837736c89570de861ebc173b1086da4f505d4adb387c6a1b1342d5e4ac9ec72",
        publique: "00e33c0f7d81d843c572275f287498e8d408654fdf0d1e065b84e2e6f157aab09b",
    },
    Noeud {
        chemin: &[0, 2_147_483_647, 1, 2_147_483_646, 2],
        chaine: "5d70af781f3a37b829f0d060924d5e960bdc02e85423494afc0b1a41bbe196d4",
        privee: "551d333177df541ad876a60ea71f00447931c0a9da16f227c11ea080d7391b8d",
        publique: "0047150c75db263559a70d5778bf36abbab30fb061ad69f69ece61a72b0cfa4fc0",
    },
];

fn hex(octets: &[u8]) -> String {
    octets.iter().fold(String::new(), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}

fn depuis_hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

fn verifier(graine: &str, noeuds: &[Noeud]) -> usize {
    let maitre = NoeudSlip10::maitre(&depuis_hex(graine)).unwrap();
    for n in noeuds {
        let noeud = maitre.deriver(n.chemin).unwrap();
        assert_eq!(
            hex(noeud.code_chaine().as_slice()),
            n.chaine,
            "chaîne {:?}",
            n.chemin
        );
        assert_eq!(
            hex(noeud.cle_privee().as_slice()),
            n.privee,
            "privée {:?}",
            n.chemin
        );
        let publique = format!("00{}", hex(&noeud.cle_signature().cle_publique()));
        assert_eq!(publique, n.publique, "publique {:?}", n.chemin);
    }
    noeuds.len()
}

#[test]
fn vecteurs_officiels_ed25519() {
    let total = verifier(GRAINE_1, &VECTEUR_1) + verifier(GRAINE_2, &VECTEUR_2);
    assert_eq!(total, 12);
}

#[test]
fn derivation_pas_a_pas_identique_au_chemin() {
    let maitre = NoeudSlip10::maitre(&depuis_hex(GRAINE_2)).unwrap();
    let pas_a_pas = maitre
        .enfant_durci(0)
        .unwrap()
        .enfant_durci(2_147_483_647)
        .unwrap();
    let direct = maitre.deriver(&[0, 2_147_483_647]).unwrap();
    assert_eq!(*pas_a_pas.cle_privee(), *direct.cle_privee());
    // L'indice 2³¹ − 1 est le dernier admis ; 2³¹ est refusé (déjà durci).
    assert!(maitre.enfant_durci(0x8000_0000).is_err());
}
