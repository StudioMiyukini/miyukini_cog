# COG-Auth — authentificateur des COG et des Alliances

> **Conception v2 — hors gel, non normatif.** Voir l'[index](_index.md) pour les conventions (décidé / proposé / question).

<!-- @id arch.alliance.cog-auth -->
<!-- @role design-authenticator -->
<!-- @layer 5-6 -->
<!-- @human Application d'authentification à la manière de Google Authenticator : second facteur des comptes COG et d'alliance, approbations signées, codes TOTP de compatibilité -->
<!-- @do authenticate_second_factor -->

## Contexte

- Le socle v1 authentifie par clés (passeport, visa, preuve de possession) ; les mots de passe n'arrivent qu'avec les Alliances et la compatibilité Windows ([Alliance-Admin §4](ALLIANCE-ADMIN.md))
- Plusieurs procédures demandent un **second facteur que l'IA n'évalue pas** : mot de passe perdu (HECTOR P-001), MFA de MiyukiniAdmin, accès hors zone COG ([conformité ANSSI](../../conformite/ANSSI-ET-SOUVERAINETE.md) : MFA obligatoire)
- **Existant** : TOTP RFC 6238 dans `crates/miyucloud/src/auth/totp.rs` (`totp-rs`) ; selon la contrainte C-24, toléré **jusqu'à son remplacement par passeport et visa** ([contraintes](../../implementation/CONTRAINTES-IMPLEMENTATION.md)) ; l'[audit du 2026-10-23](../../securite/AUDIT-2026-10-23-auth-wan.md) l'a trouvé **inopérant** (jamais enrôlé, pas d'anti-rejeu)

## Portée / Scope

- Rôle de COG-Auth, modes d'authentification, enrôlement, perte de l'appareil
- Valable pour un COG seul **et** pour une Alliance

## 1. Ce qui est décidé

<!-- @id arch.alliance.cog-auth-decide -->
<!-- @role owner-decisions -->
<!-- @do record_cog_auth_decisions -->

- **Décidé (propriétaire, AL-D32)** : un service **COG-Auth**, authentificateur à la manière de **Google Authenticator**

## 2. Principe

<!-- @id arch.alliance.cog-auth-principe -->
<!-- @role design-principle -->
<!-- @do prefer_signed_approvals -->

- **Constat** : un code TOTP (6 chiffres toutes les 30 s) repose sur un **secret partagé** entre l'application et le serveur ; il se fait **hameçonner** (l'utilisateur le recopie sur une fausse page) et un serveur volé livre tous les secrets
- **Décidé (propriétaire, AL-D33) — deux modes, le natif d'abord**
  - **Mode natif « approbation signée »** (par défaut pour tout ce qui est COG)
    - la demande affiche **qui**, **quoi**, **depuis quel poste**, et un **nombre à recopier** sur l'écran demandeur (évite l'approbation par lassitude)
    - l'appareil signe la demande avec sa **clé d'appareil** (passeport, preuve de possession du lot 2 §7) : la signature est liée à cette demande, ce poste, ce défi ; rien à recopier, rien à hameçonner
    - aucun secret partagé : le serveur ne garde que des clés publiques
  - **Mode TOTP de compatibilité** (RFC 6238)
    - pour ce qui ne parle pas COG : MiyuCloud existant, applications tierces, plus tard les postes Windows (AL-D07)
    - anti-rejeu obligatoire (un code accepté une seule fois), limitation des essais, secrets chiffrés au repos côté serveur
- Les deux modes vivent dans la **même application**

## 3. Où vit COG-Auth

<!-- @id arch.alliance.cog-auth-lieu -->
<!-- @role placement -->
<!-- @do place_authenticator -->

- **Décidé (AL-D34)**
  - Application sur l'**appareil personnel** de l'utilisateur (téléphone), qui est un **appareil pair** à passeport (D8), jamais un COG poste (AL-D06) ; à partir de `apps/central-mobile` ; **clé FIDO2** comme second appareil
  - Côté serveur : vérification par **Alliance-Admin** (comptes d'alliance) ou par le COG seul (comptes locaux, MiyukiniAdmin)
  - Clé d'appareil **non exportable** quand le matériel le permet (enclave sécurisée du téléphone)
  - Déverrouillage local (biométrie ou code) avant toute approbation

## 4. Enrôlement et perte

<!-- @id arch.alliance.cog-auth-enrolement -->
<!-- @role lifecycle -->
<!-- @do enroll_authenticator -->

- **Décidé (AL-D35)** : aucune synchronisation dans un nuage ; plusieurs appareils enrôlés ; codes de secours ; perte = révocation puis ré-enrôlement
- **Proposé** (précisions)
  - **Enrôlement** : depuis une session déjà authentifiée, par un QR code à usage unique affiché sur le poste ; l'appareil reçoit un passeport d'alliance ; inscription au journal du CC
  - **Pas de synchronisation dans un nuage** des clés ou des secrets TOTP (Google Authenticator le propose : c'est un point de vol unique)
  - **Plusieurs appareils** enrôlés recommandés (téléphone et clé matérielle FIDO2)
  - **Perte** : révocation de l'appareil (classe 1 si demandée depuis un autre appareil enrôlé, sinon parcours P-001 avec un autre facteur) ; ré-enrôlement d'un nouvel appareil
  - Codes de secours à usage unique remis à l'enrôlement, comme l'existant MiyuCloud (8 codes), stockés hachés

## 5. Usages dans l'Alliance

<!-- @id arch.alliance.cog-auth-usages -->
<!-- @role usages -->
<!-- @do map_authenticator_usages -->

| Usage | Mode |
|---|---|
| Ouverture de session sur un COG poste (si la politique exige un second facteur) | natif |
| Mot de passe perdu, HECTOR P-001 (« approbation sur un autre appareil enrôlé ») | natif |
| Approbations de classe 3 par un humain (TAMR) | natif |
| MiyukiniAdmin (MFA) | natif, TOTP en repli |
| Télétravail par la COG-Gateway | natif |
| MiyuCloud existant, services tiers | TOTP |
| Postes Windows (après AL-D07) | TOTP ou FIDO2 |

## 6. Points à trancher

- Aucun point ouvert de conception ; à l'implémentation : format de la demande d'approbation (type Sceau à attribuer par amendement), délai d'expiration d'une demande, plateformes mobiles visées

---

Date de mise a jour : 2026-10-25
