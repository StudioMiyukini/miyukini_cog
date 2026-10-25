# COG-Gateway — le pare-feu de l'Alliance

> **Conception v2 — hors gel, non normatif.** Voir l'[index](_index.md) pour les conventions (décidé / proposé / question).

<!-- @id arch.alliance.gateway -->
<!-- @role design-alliance-firewall -->
<!-- @layer 3-5+zones -->
<!-- @human COG qui permet de sortir de l'Alliance sans la compromettre : pare-feu de l'Alliance, navigation sur Internet, lien avec la Fédération publique -->
<!-- @do guard_alliance_egress -->

## Contexte

- Les COG d'une Alliance ne sont pas membres de la Fédération publique et n'ont pas de route directe vers Internet (AL-D09)
- Les utilisateurs doivent pourtant naviguer sur le web, recevoir les mises à jour du canal officiel et, le cas échéant, échanger avec la Fédération
- La COG-Gateway est le **seul point de sortie** : elle laisse sortir sans exposer l'Alliance

## Portée / Scope

- Rôle, place dans le modèle v1, flux, navigation, confinement d'une Gateway compromise
- Le réseau d'ensemble est dans la [vue globale §4](ALLIANCE-VUE-GLOBALE.md) ; l'autorité qui fixe les règles est le [COG-CC](COG-CC.md)

## 1. Ce qui est décidé

<!-- @id arch.alliance.gateway-decide -->
<!-- @role owner-decisions -->
<!-- @do record_gateway_decisions -->

- **Décidé (propriétaire)**
  - La **COG-Gateway** permet de **sortir de l'Alliance sans compromission**
  - Elle permet notamment la **navigation sur Internet** par un **navigateur classique** ou, de préférence, par un **navigateur maison** (à venir)
  - C'est le **pare-feu de l'Alliance**
  - Lien avec la Fédération publique assuré par la Gateway (AL-D09)
  - Elle permet aussi la **connexion privée entre plusieurs Alliances** (VPN)
  - **Polyvalente** : une même Gateway porte tous les usages ; **1 au minimum** par Alliance (AL-D12)
  - Le **télétravail** entre par la Gateway (AL-D13)
  - **Pas d'inspection TLS** par défaut pour le navigateur classique (AL-D14)
  - Navigation **journalisée**, dans le respect du droit du travail et du RGPD (AL-D15)
  - Liaisons inter-Alliances entre COG-Gateway seulement, **pas de partenaire non-COG pour le moment** (AL-D16)
  - Utilisateurs d'une autre Alliance : autorisés par le COG-CC selon ses informations ; à la cérémonie de liaison, les Alliances se reconnaissent et **s'inscrivent mutuellement dans leurs bases** ; **mise à jour automatique** entre COG-CC (AL-D17)

## 2. Place dans le modèle v1

<!-- @id arch.alliance.gateway-v1 -->
<!-- @role v1-mapping -->
<!-- @do map_gateway_to_v1 -->

- **Proposé — la Gateway est à l'Alliance ce que la Douane est à un COG**
  - Le **COG-CC définit** les règles (rôle de BorderGuard à l'échelle de l'Alliance) : **arrêtés de frontière d'alliance**, scellés au quorum (classe 2 ; classe 3 s'ils ouvrent un flux)
  - La **Gateway applique** (opérateur sous autorité, D1) ; elle ne décide rien, ne modifie pas ses propres règles
  - Ses Vigies remontent au WorrySentinel de la Gateway **et** à HECTOR (procédure d'alerte P-005)
- La Gateway reste un **COG complet** : sa propre Douane, son Portail, sa zone COG ; les flux de l'Alliance la traversent dans des **sandboxes** par usage, jamais par sa zone COG
- **Jamais membre du CC**, aucun droit sur l'annuaire : c'est la machine la plus exposée de l'Alliance

## 3. Flux

<!-- @id arch.alliance.gateway-flux -->
<!-- @role flows -->
<!-- @do list_gateway_flows -->

| Flux | Sens | Traitement proposé |
|---|---|---|
| Navigation web des utilisateurs | sortant | §4 |
| Téléchargements | entrant (réponse) | **CDR** ; non pris en charge ⇒ **quarantaine** (décision A-004 n° 2), jamais ouvert par un composant |
| Canal officiel (Origin, mises à jour, modèles) | sortant (relevé) | relevé seulement, la vérification reste dans chaque COG (2 sur 2, 48 h) |
| Fédération publique | les deux | sandbox dédiée, titres fédéraux vérifiés par la Gateway **et** par le destinataire |
| Autre Alliance | les deux | tunnel privé Gateway ⇄ Gateway, §5 |
| Mail, services H3 | sortant | selon le contrat du service |
| Télétravail (poste nomade → Alliance) | entrant | **par la Gateway** (AL-D13), sandbox séparée de la navigation ; la Gateway **transporte** seulement : le poste nomade est authentifié de bout en bout par le CC (passeport, visa, preuve de possession) |

- Tout flux non déclaré par un arrêté de frontière d'alliance est **refusé** (`policy drop`, comme la Douane)
- Contenu reçu par la Gateway = **International (UNKNOWN)** pour le poste qui le reçoit : la Gateway ne « blanchit » rien, seule sa propre identité est VERIFIED

## 4. Navigation sur Internet

<!-- @id arch.alliance.gateway-navigation -->
<!-- @role web-browsing -->
<!-- @do browse_through_gateway -->

- **Navigateur classique** (décidé : admis)
  - Le poste n'a aucune route vers Internet ; le navigateur passe par un **mandataire explicite** de la Gateway
  - Filtrage : DNS résolu par la Gateway seulement, listes de domaines par politique, débits, types de contenu
  - **Pas d'inspection TLS** par défaut (AL-D14) : aucune autorité de certification de l'Alliance sur les postes ; filtrage par domaine et DNS ; le navigateur maison remplace l'inspection par la rupture protocolaire
- **Navigateur maison** (décidé : préféré, à venir)
  - Lien proposé avec le service H3 « explorateur web » de la v1 ([services hybrides §1](../SERVICES-HYBRIDES-ET-PORTAIL.md))
  - **Proposé** : **rupture protocolaire** (modèle v1, zones §8) — la page s'exécute dans une sandbox de la Gateway ; le poste reçoit une **représentation inerte** (rendu ou document reconstruit), jamais le code de la page
  - Effet : un code malveillant d'un site reste sur la Gateway, dans une sandbox jetable par session
- **Journalisation de la navigation** (AL-D15) : domaines et volumes, **pas les contenus** ; durée de conservation fixée par politique ; consultation des journaux en **classe 3** (quorum et TAMR) ; information des utilisateurs (droit du travail, RGPD — voir [conformité](../../conformite/REGLEMENTAIRE-UE-FR.md))

## 5. Liaisons entre Alliances

<!-- @id arch.alliance.gateway-inter-alliance -->
<!-- @role inter-alliance-link -->
<!-- @do link_alliances -->

- **Décidé (propriétaire)**
  - La Gateway assure la **connexion privée entre plusieurs Alliances** (VPN)
  - Entre COG-Gateway seulement, **pas de partenaire non-COG pour le moment** (AL-D16)
  - **Cérémonie de liaison** : les Alliances **se reconnaissent** et **s'inscrivent mutuellement dans leurs bases** (AL-D17)
  - **Mise à jour automatique** entre COG-CC ensuite (AL-D17)
  - Accès d'un utilisateur de l'autre Alliance : **autorisé par notre COG-CC**, selon les informations qu'il détient (AL-D17)
  - Les données venues d'un autre COG-CC sont **identifiables** et **révocables en interne** ; le COG-CC concerné en est **informé** (AL-D18)
- **Proposé**
  - **Protocole** : **WireGuard** de Gateway à Gateway (cohérent avec la Fédération v1) ; IPsec IKEv2 réservé à un futur partenaire non-COG, hors périmètre aujourd'hui
  - **Tunnel** de Gateway à Gateway seulement, jamais de poste à poste ; aucune route vers l'autre Alliance hors du tunnel
  - **Cérémonie de liaison** (classe 3 des deux côtés : quorum et TAMR)
    1. Chaque CC présente son identité : ensemble des membres, clés des Gateways, tête de journal
    2. Les deux CC signent au quorum un **accord inter-Alliances** : flux autorisés, services exposés, **catégories d'informations partagées**, durée
    3. Chacun inscrit l'autre dans sa base : objet **Alliance liée** (identité du CC, membres, Gateways, accord)
  - **Mise à jour automatique**
    - L'Alliance B envoie à A des **extraits** de son annuaire, scellés **au quorum de B**, limités aux **comptes et groupes désignés par l'accord**, jamais les secrets ni les comptes privilégiés (AL-D20, minimisation RGPD)
    - A vérifie le quorum de B, la fraîcheur et le périmètre ; il les range comme **objets étrangers en lecture seule**, jamais comme comptes locaux
    - **Décidé (AL-D19)** : une mise à jour automatique ne peut que **restreindre** ou **ajouter sans ouvrir de droit** — une **révocation** venue de B s'applique immédiatement ; un **ajout** ou un **élargissement** n'ouvre aucun droit tant qu'une politique de A (classe 2) ne l'accorde pas
  - **Provenance et révocation interne** (AL-D18)
    - Chaque objet étranger porte sa **provenance** : identifiant de l'Alliance liée, empreinte de l'extrait scellé qui l'a apporté, date de réception ; elle reste visible dans l'annuaire, les journaux et les visas émis
    - Le CC de A peut **révoquer en interne** n'importe quel objet de B : un objet = **classe 1** (une révocation ne fait que restreindre), tous les objets de B = coupure de la liaison (**classe 2**)
    - La révocation tient face aux mises à jour suivantes de B : seule une décision du CC de A la lève
    - **Avis de révocation** scellé par le CC de A, envoyé au CC de B par le tunnel, inscrit au journal des deux côtés ; il donne l'objet et la catégorie du motif, pas le détail interne de A
  - **Accès** d'un utilisateur de B chez A : visa émis par le **CC de A**, d'après ses politiques et l'extrait de B ; jamais de droit d'administration
  - **Confiance** : B est **VERIFIED restreint**, périmètre `inter-alliance:<id>` ; le CC de B n'a **aucune autorité** sur A
  - **Rupture** : un seul des deux CC peut **couper** la liaison (classe 2), et les objets de l'autre Alliance sont gelés ; la rétablir exige les deux (comme la coupure de voie v1)
  - Risque : une Alliance liée compromise **au niveau de son quorum** pourrait déclarer de faux utilisateurs ; borné par AL-D18 (révocation interne) et AL-D19 (aucun droit ouvert automatiquement)

## 6. Gateway compromise : ce qui reste protégé

<!-- @id arch.alliance.gateway-compromission -->
<!-- @role containment -->
<!-- @do contain_gateway_compromise -->

- **Décidé (propriétaire, AL-D12)** : Gateway **polyvalente**, **1 au minimum**
- **Constat** : avec une seule Gateway, sa perte coupe toute sortie, le télétravail et les liaisons inter-Alliances ; sa compromission expose tous les usages à la fois
- **Proposé**
  - En production, une **deuxième** Gateway recommandée (sans changer le minimum décidé) ; les postes répartissent et basculent
  - Usages cloisonnés **dans** la Gateway : une sandbox par usage (navigation, Fédération, télétravail, chaque liaison inter-Alliances), comme les services d'un COG
  - Une Gateway compromise peut **voir et altérer** le trafic qu'elle porte ; elle **ne peut pas** : écrire dans l'annuaire, émettre une politique, atteindre la zone COG d'un poste, se faire passer pour le CC, signer une mise à jour (le canal officiel est vérifié par chaque COG), authentifier un poste nomade (fait de bout en bout par le CC)
  - Intégrité : la Gateway publie la même attestation périodique que les membres du CC (COG-CC §4) ; écart ⇒ retirée de la répartition par le CC (classe 2)
  - Une Gateway compromise qui porte une liaison inter-Alliances peut voir ce trafic ; elle n'obtient aucun droit dans l'autre Alliance (accord signé par les CC, pas par les Gateways)
  - Toutes les Gateways muettes ⇒ **plus de sortie** (fail-closed) ; l'Alliance continue de fonctionner en interne

## 7. Points à trancher

- Aucun point ouvert propre à la Gateway ; restent le navigateur maison (spécification à écrire) et IPsec (au premier partenaire non-COG, AL-D16)

---

Date de mise a jour : 2026-10-25
