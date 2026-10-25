# Alliances — synthèse consolidée des acquis

> **Conception v2 — hors gel, non normatif.** Voir l'[index](_index.md) pour les conventions (décidé / proposé / question).

<!-- @id arch.alliance.synthese -->
<!-- @role design-consolidation -->
<!-- @layer 5-8+zones -->
<!-- @human État consolidé de la conception des Alliances : invariants candidats au gel v2, carte des services, formats à attribuer, état des questions, chemin vers le gel -->
<!-- @do consolidate_alliance_design -->

## Contexte

- Les décisions AL-D01..AL-D41 sont dispersées dans sept fiches et un registre ; cette page les ramène à ce qu'il faudra **geler** : des invariants, une carte des services, des formats
- Base : [registre](ALLIANCE-DECISIONS.md) et [revue des failles](ALLIANCE-REVUE-FAILLES.md) au 2026-10-25

## Portée / Scope

- Résumé normatif **candidat** ; en cas d'écart, le registre et les fiches font foi jusqu'au gel v2
- Hors périmètre : détail des procédures, déjà dans les fiches

## 1. Invariants candidats au gel v2

<!-- @id arch.alliance.synthese-invariants -->
<!-- @role invariants -->
<!-- @do list_alliance_invariants -->

| Id | Invariant | Source |
|---|---|---|
| AL-INV-01 | La racine de tout COG d'Alliance est détenue par l'**organisation** ; un appareil personnel n'est jamais un COG poste | AL-D06 |
| AL-INV-02 | **Aucune IA ne décide** : HECTOR et MAIA produisent des intentions ; StrongFather, le quorum et TAMR décident | AL-D03, [HECTOR §2](HECTOR.md) |
| AL-INV-03 | **Aucun membre seul** du CC ne peut écrire l'annuaire, réécrire le journal, ni ouvrir un secret détenu à seuil | AL-D02, AL-D22 |
| AL-INV-04 | **Aucune destruction à distance** : effacements seulement par cérémonie locale | INV-Z11, AL-D31, AL-D40 |
| AL-INV-05 | Tout flux vers l'extérieur passe par une **COG-Gateway** ; tout fichier externe passe le CDR et garde sa marque de provenance à vie | AL-D10, AL-D30 |
| AL-INV-06 | Une Alliance liée ne peut **jamais ouvrir de droit** chez nous ; ses objets restent identifiables et révocables | AL-D18, AL-D19 |
| AL-INV-07 | Les données d'un utilisateur **restent sur le poste** ; elles ne bougent que par un humain, chiffrées pour le compte | AL-D23, AL-D41 |
| AL-INV-08 | **Pas de donnée personnelle dans MAIA** sur un poste d'alliance, sauf politique explicite de classe 3 ; mémoire MAIA jamais dans MiyuCloud ni synchronisée | AL-D24, AL-D37, AL-D38 |
| AL-INV-09 | **Aucun secret partagé** là où une signature suffit ; TOTP seulement en compatibilité, avec anti-rejeu | AL-D33 |
| AL-INV-10 | Une politique qui élargit hors de l'enveloppe d'enrôlement exige **quorum et TAMR** | AL-D21 |
| AL-INV-11 | **NTLM jamais** ; compatibilité Windows seulement après une Alliance entièrement COG réussie | AL-D07 |
| AL-INV-12 | Données métier hors de MAIA : **Alliance-MAIA** les tient en Fiches signées ; les données sur des personnes restent dans les applications métier | AL-D39 |

| AL-INV-13 | Jamais plus de `f` membres du CC **mis à jour en même temps** | AL-D42 |
| AL-INV-15 | La **racine d'organisation** est hors ligne et à seuil ; elle ne sert qu'à fonder, refonder, déléguer l'enrôlement et changer le collège ; en bris de glace elle ne peut que **restreindre** | AL-D44, AL-D45 |
| AL-INV-14 | Une approbation de classe 3 exige `k` **personnes** distinctes ; l'auteur n'approuve pas ; une personne détient au plus une part d'une clé à seuil | AL-D43 |

| AL-INV-16 | **Restreindre est toujours plus facile qu'élargir** ; l'IA ne restreint jamais seule | AL-D48 |
| AL-INV-17 | Un poste d'alliance est **chiffré intégralement** ; un compte privilégié ne se connecte **jamais hors ligne** | AL-D47 |

## 2. Carte des services

<!-- @id arch.alliance.synthese-services -->
<!-- @role service-map -->
<!-- @do map_alliance_services -->

| Service | Où | Rôle | Fiche |
|---|---|---|---|
| **Alliance-Admin** | COG-CC | annuaire, authentification, politiques | [fiche](ALLIANCE-ADMIN.md) |
| **COG-Admin** | COG poste | client d'Alliance-Admin ; présente les politiques à StrongFather du poste | [fiche](ALLIANCE-ADMIN.md) §5 |
| **HECTOR** | COG-CC | IA de tri et de préparation des demandes ; aucune autorité | [fiche](HECTOR.md) |
| **COG-Gateway** | COG dédié | pare-feu de l'Alliance : Internet, Fédération, télétravail, Alliances liées | [fiche](COG-GATEWAY.md) |
| **Alliance-Files** | CC (index) + postes + COG de stockage | synchronisation des fichiers d'équipe ; sauvegardes | [fiche](ALLIANCE-FILES.md) |
| **Alliance-MAIA** | CC (index) + COG désignés | base de connaissances métier en Fiches signées, lue par MAIA | [vue globale §8](ALLIANCE-VUE-GLOBALE.md) |
| **COG-Auth** | téléphone, clé FIDO2 | second facteur : approbations signées, TOTP de compatibilité | [fiche](COG-AUTH.md) |
| **Alliance-CompatAD** | sandbox du CC | traduction Kerberos / LDAP / SMB pour Windows (après AL-D07) | [Alliance-Admin §4](ALLIANCE-ADMIN.md) |

## 3. Chaîne de décision

| Classe | Exemples | Qui décide |
|---|---|---|
| 0 | consultation | aucune décision |
| 1 | P-001 mot de passe perdu, compte verrouillé, groupe ordinaire | StrongFather du CC, après double analyse HECTOR |
| 2 | nouveau poste, export avec accord, politique dans l'enveloppe, mise à jour du CC par vagues (AL-D42) | quorum `q` |
| 3 | compte privilégié, politique hors enveloppe, export sans accord, ouverture de domaines personnels MAIA, récupération d'archive | quorum `q` et TAMR par le collège de `k` personnes (AL-D43) |

## 4. Paramètres chiffrés

| Paramètre | Valeur | Statut |
|---|---|---|
| Membres du CC | 6 min, 7 recommandés, 12 max par CC | décidé (AL-D08), plafond par le Sceau |
| Tolérance | n=6 → f=1, q=4 · n=7 → f=2, q=5 · n=12 → f=3, q=8 | calcul |
| COG-Gateway | 1 min ; 2 dès le télétravail | décidé (AL-D12) ; 2 proposé (AL-F12) |
| Attestations d'intégrité | toutes les 30 s | proposé |
| Fraîcheur des révocations | 15 min, fail-closed | v1 |
| Mot de passe temporaire P-001 | 1 h | proposé |
| Codes de secours COG-Auth | 8, hachés | décidé (AL-D35) |
| Récupération d'archive MAIA sans consentement | 2 ans, hors `sante`/`finances`/`tiers` | décidé (AL-D26, AL-D27) |

## 5. Formats à attribuer par amendement

<!-- @id arch.alliance.synthese-formats -->
<!-- @role format-backlog -->
<!-- @do list_formats_to_assign -->

- Aucun code n'est attribué ici (C-82 : amendement avant le code) ; liste de ce qu'il faudra attribuer au gel v2

| Objet | Signé par | Proche de |
|---|---|---|
| Passeport d'alliance | CC (quorum ou délégation) | Sceau `0x01` existant, nature à préciser |
| Visa d'alliance | un membre sain du CC (AL-D46) | visa v1 |
| Arrêté d'alliance (politique) | quorum du CC | — |
| Attestation d'intégrité d'un membre | membre du CC | — |
| Tête de journal du CC | quorum (HotStuff) | registres Merkle v1 |
| Acte de genèse / refondation | racine d'organisation à seuil | — |
| Accord de liaison entre Alliances | deux CC et leurs Gateways | reconnaissance `0x07` |
| Demande et réponse d'approbation COG-Auth | appareil de l'utilisateur | preuve de possession (lot 2 §7) |
| Intention HECTOR | membre traitant | intention v1 |
| Paquet d'export / sauvegarde | poste d'origine, chiffré pour le compte | `maia-memoire/v1` pour la mémoire |
| Fiche Alliance-MAIA | auteur autorisé | Fiche `0x09` |

## 6. État des questions

- **Tranchées** : 48 décisions (AL-D01..AL-D48)
- **Ouvertes, en attente d'accord** : AL-Q07 (poste partagé), AL-Q32 (sauvegarde MAIA au CC)
- **Ouvertes, issues de la revue** : AL-Q48, AL-Q50, AL-Q51 ([revue](ALLIANCE-REVUE-FAILLES.md))
- **Ouvertes, autres** : AL-Q06, Q08, Q11, Q12, Q13, Q15, Q16, Q17, Q18, Q21, Q33 (juridique), Q19 (moment du gel)

## 7. Chemin vers le gel v2

- **Préalables structurels** (à trancher avant le gel, ils changent la forme du système) : ~~AL-Q42~~ (AL-D42), ~~AL-Q43~~ (AL-D43), ~~AL-Q44~~ (AL-D44), ~~AL-Q45~~ (AL-D45), ~~AL-Q46~~ (AL-D46) — **tous tranchés** (connexion en lecture seule)
- **Préalables hors conception** : lot 0 bis (CRA, AI Act) ; AIPD de l'Alliance (AL-Q48) avant le pilote
- **Peuvent rester ouvertes dans le gel** : réglages chiffrés (Q12, Q16, Q17, Q18), matériel (Q15), juridique (Q33)

---

Date de mise a jour : 2026-10-25
