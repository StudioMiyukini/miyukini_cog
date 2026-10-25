# Alliance-Admin — annuaire et authentification de l'Alliance

> **Conception v2 — hors gel, non normatif.** Voir l'[index](_index.md) pour les conventions (décidé / proposé / question).

<!-- @id arch.alliance.admin -->
<!-- @role design-directory -->
<!-- @layer 5-7 -->
<!-- @human « Active Directory maison » en Rust : annuaire, authentification, propagation des politiques ; client COG-Admin ; compatibilité avec les postes Windows -->
<!-- @do administer_alliance_directory -->

## Contexte

- Une organisation attend d'un annuaire ce que fait Active Directory : comptes, groupes, unités, postes, authentification unique, politiques de groupe
- AD repose sur un empilement ancien (Kerberos, LDAP, NTLM, SMB, MS-RPC) dont les faiblesses sont connues (relais NTLM, Kerberoasting, golden ticket, délégation non contrainte)
- Alliance-Admin vise le même service, **mieux conçu**, sur le socle COG ; la compatibilité Windows est une **frontière**, pas le cœur

## Portée / Scope

- Modèle de l'annuaire, authentification native, politiques, client COG-Admin, compatibilité Windows par niveaux
- Le quorum et les classes d'actions sont dans [COG-CC](COG-CC.md)

## 1. Ce qui est décidé

<!-- @id arch.alliance.admin-decide -->
<!-- @role owner-decisions -->
<!-- @do record_admin_decisions -->

- **Décidé (propriétaire)**
  - Solution **maison en Rust**, nommée **Alliance-Admin**, service `Alliance-*` des COG-CC
  - Administration, gestion, **authentification** dans l'Alliance, **propagation des politiques** ; ce que fait AD « et plus encore, mieux conçu »
  - Fondée sur la sécurité COG en place et à venir
  - **Compatible** avec les machines Windows qui considèrent l'Alliance comme un **domaine AD** (reprise partielle de Kerberos et de LDAP envisagée)
  - Partie serveur dans les COG-CC ; partie **client « COG-Admin »** sur les COG postes

## 2. Modèle de l'annuaire

<!-- @id arch.alliance.admin-modele -->
<!-- @role directory-model -->
<!-- @do model_directory_objects -->

- **Proposé — objets**
  - **Compte d'alliance** : identifiant stable, nom, responsable, groupes, appareils enrôlés, drapeau **privilégié**, état (actif, verrouillé, suspendu, révoqué)
  - **Groupe** : membres, responsable, classe (ordinaire / privilégié)
  - **Unité** : arborescence de l'organisation, porte les politiques par héritage
  - **Poste** : identité du COG poste, unité, état d'enrôlement, santé
  - **Politique** : arrêté d'alliance (vue globale §5)
  - **Membre du CC** : COG du centre de commandement
  - **Espace synchronisé** : propriétaire, groupes autorisés (lecture, écriture, partage) ; consommé par [Alliance-Files](ALLIANCE-FILES.md) (AL-D28)
  - **COG-Gateway** : pare-feu de l'Alliance ([fiche](COG-GATEWAY.md))
  - **Alliance liée** (AL-D17) : identité du CC et des Gateways d'une autre Alliance, accord signé, **objets étrangers** reçus par mise à jour automatique (comptes, groupes désignés), en lecture seule, jamais confondus avec les comptes locaux ; chacun porte sa **provenance** et reste **révocable en interne**, l'Alliance d'origine étant informée (AL-D18, [COG-Gateway §5](COG-GATEWAY.md))
- **Proposé — règles**
  - Toute écriture = intention → quorum selon la classe → journal du CC (COG-CC §3, §5)
  - **Historique complet** : on peut relire l'annuaire tel qu'il était à toute tête du journal (EverBuddy : historique immuable)
  - Comptes **privilégiés séparés** des comptes de travail (un administrateur a deux comptes) ; jamais de privilège permanent : élévation **temporaire** accordée en classe 3 ([AL-Q16](ALLIANCE-DECISIONS.md))

## 3. Authentification native COG

<!-- @id arch.alliance.admin-auth-native -->
<!-- @role native-auth -->
<!-- @do authenticate_alliance_account -->

- **Proposé**
  - Un compte d'alliance a une **clé par appareil enrôlé** ; l'appareil reçoit un **passeport d'alliance** (Sceau `0x01`, émis par le CC au quorum ou par délégation, durée courte)
  - Connexion sur un COG poste : COG-Admin demande un **visa** au CC pour ce poste, avec **preuve de possession** (lot 2 §7)
  - **Décidé (AL-D46)** : le visa **n'est pas une écriture ordonnée** du journal ; tout membre sain l'émet, court (≤ 8 h), après vérification du passeport et des révocations, et verse la trace au journal ensuite ; un membre qui émet pour un compte révoqué est détecté par l'intégrité collective et suspendu ; CC en lecture seule ⇒ connexions possibles, aucune écriture d'annuaire hors bris de glace (AL-D45)
  - Second facteur : [COG-Auth](COG-AUTH.md) (AL-D32), approbation signée par défaut, TOTP en compatibilité
  - Changement de poste : le compte s'authentifie sur le nouveau poste avec un appareil enrôlé (téléphone avec COG-Auth, clé matérielle) ou un mot de passe **seulement** si la politique l'autorise
  - **Hors ligne** (LOI-2, isolement normal) : le poste garde un **vérificateur local** des derniers comptes connectés, durée et nombre bornés par politique ([AL-Q17](ALLIANCE-DECISIONS.md)) ; **décidé (AL-D47)** : vérificateur Argon2id effacé dès qu'une révocation est connue, 24 h au plus pour les comptes sensibles, 0 pour les privilégiés, poste chiffré intégralement (clé scellée par TPM, sinon phrase de démarrage)
  - Révocation d'un compte : inscrite au journal, fraîcheur des révocations comme en v1 (15 min), fail-closed
- Les mots de passe ne servent que là où un protocole hérité l'exige (§4)

## 4. Compatibilité Windows

<!-- @id arch.alliance.admin-windows -->
<!-- @role legacy-compat -->
<!-- @do serve_windows_domain_members -->

- **Constat** — pour qu'un poste Windows rejoigne et utilise un domaine AD, le contrôleur doit au moins servir :
  - **DNS** avec les enregistrements SRV du domaine (`_ldap._tcp.dc._msdcs.<domaine>`, `_kerberos._tcp.<domaine>`)
  - **CLDAP** (« LDAP ping », UDP 389) pour la localisation du contrôleur
  - **LDAP / LDAPS** avec le **schéma AD** et le rootDSE attendus
  - **Kerberos** (88) et changement de mot de passe (464), avec **PAC** signé (MS-PAC)
  - **MS-RPC** : Netlogon (MS-NRPC, canal sécurisé de la machine), SAMR, LSARPC
  - **SMB** (445) pour SYSVOL et NETLOGON : les stratégies de groupe sont des objets LDAP **et** des fichiers dans SYSVOL
  - Heure synchronisée (Kerberos tolère 5 min par défaut)
- **Constat** : c'est le travail de Samba AD DC, mûri sur plus de dix ans ; une réimplémentation complète en Rust est un projet du même ordre
- **Décidé (propriétaire, AL-D07)** : cible **W3 complet**, stratégies de groupe comprises ; la compatibilité Windows ne commence qu'**après** une réussite effective d'une Alliance **entièrement COG** (critère : [AL-Q21](ALLIANCE-DECISIONS.md))
- **Proposé** : livrer W3 par paliers testables W1 → W2 → W3
- **Niveaux de compatibilité**

| Niveau | Windows peut | À implémenter |
|---|---|---|
| **W1** | applications web et métier en SSO (SPNEGO), annuaire en lecture | Kerberos (AES), LDAPS lecture, DNS SRV |
| **W2** | ouvrir une session Windows avec un compte d'alliance sur un poste **joint** | + CLDAP, Netlogon, SAMR, LSARPC, PAC, changement de mot de passe |
| **W3** | recevoir les **stratégies de groupe** | + SMB (SYSVOL), objets GPO, traduction arrêté d'alliance → GPO |

- **Proposé — architecture**
  - Service séparé **`Alliance-CompatAD`**, dans un **sandbox** du COG-CC (comme une façade), derrière une **frontière dédiée** aux protocoles hérités
  - Il **traduit** vers Alliance-Admin ; il **ne décide rien** et n'écrit rien dans l'annuaire sans passer par les intentions et le quorum
  - Durcissement imposé, non configurable à la baisse :
    - **NTLM refusé** (pas de relais, pas de pass-the-hash)
    - Kerberos **AES** seulement (RC4 et DES refusés), pré-authentification obligatoire
    - **Signature LDAP** et liaison de canal (channel binding) exigées ; LDAP simple sans TLS refusé
    - **Signature SMB** exigée
    - Aucune délégation Kerberos **non contrainte**
    - Comptes de service : mots de passe longs aléatoires, rotation automatique (contre le Kerberoasting)
- **Secrets Kerberos — point dur**
  - Kerberos exige que le contrôleur détienne des **clés dérivées des mots de passe** (pas un hachage lent type Argon2) et la clé **krbtgt**, qui signe tous les tickets : la voler permet de forger n'importe quel ticket (« golden ticket »)
  - Proposé : clés Kerberos chiffrées sous une clé détenue **à seuil** par les membres du CC ; krbtgt en rotation régulière ; Alliance-CompatAD ne voit que ce que sa requête exige ([AL-Q18](ALLIANCE-DECISIONS.md))
- **Alternative écartée** par AL-D07 : un fournisseur d'identifiants (Credential Provider) sans jonction de domaine ; Windows ne verrait pas l'Alliance comme un domaine AD

## 5. Client COG-Admin

<!-- @id arch.alliance.admin-client -->
<!-- @role client -->
<!-- @do run_cog_admin_client -->

- **Proposé**
  - Service de la **zone COG** du poste ; parle au CC par une **voie déclarée** du Portail (la zone COG tire, D4)
  - Fonctions : ouverture de session des comptes d'alliance, tirage et vérification des politiques, remontée de santé (CaringNanny → résumé signé), écran « mot de passe perdu » (HECTOR P-001)
  - **N'exécute rien lui-même** : il présente les politiques vérifiées à StrongFather du poste, qui décide ; KindMother persiste
  - Relation avec **MiyukiniAdmin** local ([AL-Q06](ALLIANCE-DECISIONS.md)) : proposé que MiyukiniAdmin garde la main sur la constitution du poste et la sortie de l'Alliance, et que COG-Admin gère tout le reste

## 6. Points à trancher

- [AL-Q21](ALLIANCE-DECISIONS.md) critère de réussite avant Windows · [AL-Q06](ALLIANCE-DECISIONS.md) MiyukiniAdmin et COG-Admin · [AL-Q16](ALLIANCE-DECISIONS.md) privilèges temporaires · [AL-Q17](ALLIANCE-DECISIONS.md) connexion hors ligne · [AL-Q18](ALLIANCE-DECISIONS.md) garde des clés Kerberos

---

Date de mise a jour : 2026-10-25
