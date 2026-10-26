# COG « poste » à 100 % — définition de fini et suivi

<!-- @id impl.poste100 -->
<!-- @role definition-of-done -->
<!-- @layer 0-9+zones -->
<!-- @human Liste vérifiable de ce qu'un COG poste autonome doit faire pour être considéré complet (v1), avec l'état de chaque point -->
<!-- @do track_cog_poste_completion -->

## Contexte

- Demande du propriétaire (2026-10-26) : « Implémente jusqu'à 100 % d'un COG poste. Utilise un mini modèle via llama.cpp pour MAIA et ALICIA »
- « Poste » = **COG autonome v1** installé sur une machine (doc gelée v1.22) ; le poste membre d'une Alliance (client COG-Admin, enrôlement par l'organisation) relève de la conception v2 hors gel et n'est pas dans ce périmètre tant que le gel v2 n'est pas prononcé

## Portée / Scope

- Un point est **fait** quand il est implémenté, testé (tests automatisés et, s'il touche au système, en VM), commité et consigné par amendement si un format change
- Hors périmètre : Alliances (v2), certification CSPN, audit externe (démarches hors code)

## 1. Socle et frontières

| # | Point | État |
|---|---|---|
| S1 | Cores réels, KindMother seul écrivain, refus par défaut | fait (lot C) |
| S2 | Zone COG sans sortie, Douane générée et auditée, Vigies, relais NFLOG | fait (lots 3, A-015, A-016) |
| S3 | AppArmor enforce pour chaque service, flux locaux par compte | fait (A-016) |
| S4 | Débit `tc` généré par la Douane | fait (A-023, A-025) ; intégré aux installateurs (unité `miyukini-douane-externe`, ordre strict → tc → filet), vérifié en VM (interfaces factices) |
| S5 | Portail scindé entrée / sortie | fait (A-027, vérifié en VM) ; CDR et descripteurs par les Cores réels à brancher |
| S6 | Façade Franche réelle : image signée, sandbox, réseau, quotas disque et sessions, révocations rafraîchies | à faire |

## 2. Identité

| # | Point | État |
|---|---|---|
| I1 | Racine hors ligne, passeports, preuve de possession | fait (lot 2) |
| I2 | Délégation de clé d'usage `0x45`, clés `visas` (5) et `fiches` (6) | fait (A-024) ; migration des émetteurs et mode strict avec I3 |
| I3 | Guichet des visas en zone COG (visa ≤ 15 min) | fait (A-029) ; placement des délégations par l'installateur à faire |
| I4 | Routes mémoire MAIA authentifiées par passeport et visa | à faire (après I3) |
| I5 | Défis et nonces persistants (plusieurs instances) ; preuve de possession liée au canal TLS | fait (A-029, A-030) ; liaison TLS fournie, non branchée (aucun vérificateur ne termine TLS) |
| I6 | Liaison d'identité et de clé d'appareil dans MiyuCloud ; régénération d'identité mobile | fait (A-030) ; PostgreSQL et Android non testés ; rotation côté central-remote à faire |

## 3. Données

| # | Point | État |
|---|---|---|
| D1 | Bases chiffrées, oubli réel (secure_delete, WAL, oubli typé), nettoyage des bases antérieures | fait (A-020, A-021) |
| D2 | Sauvegardes : 30 j, oublis réappliqués à la restauration, chiffrées | fait (A-025) ; restauration à exposer à MiyukiniAdmin |
| D3 | Mémoire d'Alicia dans KindMother | fait (A-019) |

## 4. IA locale

| # | Point | État |
|---|---|---|
| A1 | **Mini modèle réel via llama.cpp** (licence Apache-2.0 / MIT, C-72), livré par le canal `MMIA`, empreinte vérifiée | fait (A-026) ; paquet modèle séparé installé et vérifié par les installateurs ; re-signature canal à faire |
| A2 | MAIA répond avec ce modèle, sous la porte des Cores, mémoire et Fiches branchées | fait (A-026) ; installé (`miyukini-maia-llm`, 11436) et vérifié en VM |
| A3 | **Alicia** utilise le même moteur local (aucun service externe) | fait (A-026) ; installé (`miyukini-maia-llm`, 11436) et vérifié en VM |
| A4 | Mention IA (AI Act art. 50) dans MAIA et Alicia | fait (A-026) ; installé (`miyukini-maia-llm`, 11436) et vérifié en VM |
| A5 | Interface de la mémoire (voir, corriger, exporter, oublier, valider) | à faire (après I4) |
| A6 | Publication des Fiches, état persistant | fait (A-017, A-018) |
| A7 | TAMR branché sur la republication d'une donnée étrangère | fait (A-028) ; vue fédérale à câbler au pont (avec F2) |

## 5. Fédération et échanges

| # | Point | État |
|---|---|---|
| F1 | Statuts fédéraux persistants, anti-retour, fuzzing | fait (A-019, A-020) |
| F2 | Transport des Fiches et requêtes entre COG (voie fédérée du Portail) | à faire (après S5) |
| F3 | Transport du commérage fédéral | à faire |

## 6. Livraison et conformité

| # | Point | État |
|---|---|---|
| L1 | Paquet signé, reproductible, installation et mise à jour vérifiées | fait (beta.1) |
| L2 | Signalement des vulnérabilités (CRA) : SECURITY.md, security.txt, processus 24 h / 72 h | partiel : documents rédigés ([processus](../securite/PROCESSUS-SIGNALEMENT-CRA.md)) ; attend contact de sécurité, clé éventuelle, rôles, compte ENISA et route `/.well-known/security.txt` (propriétaire) |
| L3 | Aucun `unwrap` hors tests hors P0/P1 (432 restants, surtout MiyuCloud et MAIA) | à faire |
| L4 | Couverture ≥ 80 % mesurée sur les crates de sécurité | fait : 23/23 crates ≥ 80 % des lignes, total 89,4 % (COUVERTURE-SECURITE.md v1.1) |
| L5 | Installation complète en VM : tous les points ci-dessus, de bout en bout | partiel : VM 183/183 contrôles, 134/134 scénarios (S4, A1–A4) ; paquet de recette reconstruit non revalidé |

---

Date de mise a jour : 2026-10-26
