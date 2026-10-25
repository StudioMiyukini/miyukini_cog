# Alliances, COG-CC, HECTOR et Alliance-Admin — index

> **Conception v2 — hors gel, non normatif.** Ces documents prolongent la [doc conceptuelle gelée v1](../GEL-DOC-CONCEPTUELLE-v1.md) sans la modifier. En cas de conflit, la doc gelée prime. Une notion ne devient normative que par un amendement `A-NNN` (ou un gel v2), écrit avant le code (C-82).

<!-- @id arch.alliance.index -->
<!-- @role design-index -->
<!-- @layer 5-8+zones -->
<!-- @human Point d'entrée de la conception des Alliances (fédérations privées), des centres de commandement COG-CC, de l'IA HECTOR et de l'annuaire Alliance-Admin -->
<!-- @do navigate_alliance_design -->

## Contexte

- La v1 gelée décrit des COG **souverains** (un propriétaire, une racine) réunis dans une **Fédération** publique (candidat → invité → membre → blacklisté)
- Besoin nouveau : des **organisations** avec beaucoup d'utilisateurs qui changent de poste ou travaillent à distance, sur un réseau privé sécurisé, avec une administration centrale (équivalent Active Directory) **sans renoncer au socle de sécurité COG**
- Statut : conception en cours, implémentation à venir

## Portée / Scope

- **Dans le périmètre** : les trois échelles (Alliance, COG-CC, COG poste), l'IA HECTOR, l'annuaire Alliance-Admin et son client COG-Admin, la compatibilité avec les postes Windows
- **Hors périmètre** : changement de la Fédération publique v1, formats binaires définitifs (codes Sceau, natures), attribués seulement par amendement

## Documents

| Document | Contenu |
|---|---|
| [Vue globale des Alliances](ALLIANCE-VUE-GLOBALE.md) | Trois échelles, place dans le modèle v1, confiance, réseau, propagation des politiques |
| [COG-CC](COG-CC.md) | Centre de commandement : ≥ 6 COG, quorum, rôles, base répliquée, intégrité collective |
| [COG-Gateway](COG-GATEWAY.md) | Pare-feu de l'Alliance : sortie sans compromission, navigation, Fédération, liaisons inter-Alliances |
| [HECTOR](HECTOR.md) | Framework IA du COG-CC : superviseur, relève, traitants, confinement, procédures |
| [Alliance-Admin](ALLIANCE-ADMIN.md) | Annuaire et authentification « AD maison » en Rust, client COG-Admin, compatibilité Windows |
| [Alliance-Files](ALLIANCE-FILES.md) | Synchronisation des fichiers entre COG, droits par Alliance-Admin, défense contre la propagation |
| [COG-Auth](COG-AUTH.md) | Authentificateur (type Google Authenticator) : approbations signées, TOTP de compatibilité |
| [Registre des décisions](ALLIANCE-DECISIONS.md) | Décisions du propriétaire, propositions, questions ouvertes |
| [Synthèse consolidée](ALLIANCE-SYNTHESE.md) | Invariants candidats au gel v2, carte des services, chaîne de décision, formats à attribuer, chemin vers le gel |
| [Revue des failles](ALLIANCE-REVUE-FAILLES.md) | Failles et manquements (AL-F01..F15), recommandations, questions AL-Q42..Q51 |

## Lexique

- **Alliance** : fédération **privée** de COG d'une organisation, sur un réseau sécurisé, administrée par un ou plusieurs COG-CC
- **COG-CC** (centre de commandement) : groupe d'au moins 6 COG qui forme le **quorum administratif** d'une Alliance et fait tourner les services `Alliance-*`
- **COG poste** : COG utilisé par un ou plusieurs utilisateurs de l'Alliance ; porte le client **COG-Admin**
- **HECTOR** : framework IA qui pilote à moitié un COG-CC ; remplace MAIA sur les COG du CC
- **Alliance-Admin** : service d'annuaire, d'authentification et de politiques de l'Alliance (côté COG-CC)
- **COG-Admin** : client d'Alliance-Admin sur chaque COG poste
- **COG-Gateway** : **pare-feu de l'Alliance** ; permet d'en sortir sans compromission (navigation sur Internet, Fédération publique) , porte le télétravail et relie plusieurs Alliances en privé (VPN) ; polyvalente, au moins une par Alliance ; gouvernée par le CC, jamais membre du CC
- **Alliance liée** : autre Alliance reconnue à une cérémonie de liaison ; chacune est inscrite dans la base de l'autre, mise à jour automatique entre COG-CC
- **Compte d'alliance** : identité d'un utilisateur dans l'Alliance, indépendante du poste utilisé
- **Services `Alliance-*`** : services propres aux COG-CC (Alliance-Admin et suivants)
- **COG-Auth** : application d'authentification de l'utilisateur, à la manière de Google Authenticator ; approbations signées par la clé de l'appareil, codes TOTP pour la compatibilité
- **Alliance-MAIA** : service `Alliance-*` de MAIA au niveau de l'Alliance ; base de connaissances **métier** de l'organisation, en Fiches signées ; MAIA la lit sans y écrire ; aucune donnée personnelle ; distinct de la mémoire MAIA du poste (AL-D39)
- **Alliance-Files** : synchronisation des fichiers entre COG de l'Alliance, droits tirés d'Alliance-Admin ; le CC tient l'index, le contenu circule de COG à COG

## Conventions de ce dossier

- **Décidé (propriétaire)** : énoncé du propriétaire du projet, repris tel quel
- **Proposé** : proposition de conception, à valider ; chaque proposition renvoie au [registre](ALLIANCE-DECISIONS.md)
- **Question** `AL-Qnn` : point ouvert ; **décision** `AL-Dnn` : point tranché
- Les valeurs chiffrées marquées « proposé » sont des défauts de travail, pas des engagements

---

Date de mise a jour : 2026-10-25
