# Processus de signalement et de traitement des vulnérabilités (CRA)

<!-- @id securite.processus.cra -->
<!-- @role vulnerability-handling-process -->
<!-- @layer 0-9+zones -->
<!-- @human Processus interne : rôles, triage CVSS 4.0, traitement ISO/IEC 30111, chronologie CRA 24 h / 72 h / rapport final, plateforme ENISA, information des utilisateurs, registre -->
<!-- @do handle_vulnerability_report -->

- **Date** : 2026-10-26 · **Statut** : v1 de travail, **pas un avis juridique** ; les points « à vérifier » doivent être confirmés (juriste, ANSSI)
- **Politique publique** : [SECURITY.md](../../SECURITY.md) · `security.txt` : [docs/doc_for_website/.well-known/security.txt](../doc_for_website/.well-known/security.txt)
- **Base** : [synthèse conformité](../conformite/CONFORMITE-SYNTHESE.md) · [réglementaire UE/FR](../conformite/REGLEMENTAIRE-UE-FR.md) (NC-REG-02, NC-REG-03) · [normes](../conformite/NORMES-ET-STANDARDS.md) (NC-STD-10, NC-STD-11) · [modèle de menaces](MODELE-MENACES-COG.md)
- **Source juridique** : règlement (UE) 2024/2847 (« CRA »), JO L du 20/11/2024, ELI `http://data.europa.eu/eli/reg/2024/2847/oj`

## 1. Cadre et dates d'application

<!-- @id securite.processus.cra.cadre -->
<!-- @role legal-basis -->
<!-- @do cite_cra_obligations -->

- **Art. 71 §2** : le règlement s'applique à partir du **11/12/2027** ; l'**art. 14 s'applique depuis le 11/09/2026** (chapitre IV, organismes notifiés : 11/06/2026)
- **Art. 69 §3** : les obligations de l'art. 14 s'appliquent aussi aux produits **mis sur le marché avant le 11/12/2027** — donc aux versions de COG déjà diffusées (rédaction issue du rectificatif du 02/07/2025 : **à vérifier** sur le texte consolidé EUR-Lex)
- **Art. 13 §6** : vulnérabilité trouvée dans un composant (y compris libre) → la signaler à son mainteneur
- **Art. 13 §8** : période de support d'au moins 5 ans (sauf usage attendu plus court) et politique de divulgation coordonnée
- **Art. 13 §17** : point de contact unique permettant aux utilisateurs de signaler des vulnérabilités, non limité à des outils automatisés
- **Art. 16** : plateforme unique de signalement (SRP) opérée par l'ENISA
- **Annexe I, partie II** : exigences de traitement des vulnérabilités (politique de divulgation coordonnée, adresse de contact, correctifs diffusés sans délai et gratuitement, avis publics)
- **Statut Studio Miyukini** : fabricant (cf. [réglementaire UE/FR](../conformite/REGLEMENTAIRE-UE-FR.md)) ; établissement principal en France → CSIRT coordinateur français (**CERT-FR / ANSSI : désignation à vérifier**)

## 2. Rôles

<!-- @id securite.processus.cra.roles -->
<!-- @role roles -->
<!-- @do assign_security_roles -->

| Rôle | Responsabilité | Titulaire |
|---|---|---|
| Responsable sécurité produit | décide de la qualification CRA, signe les notifications ENISA, arbitre la divulgation | `<À DÉFINIR PAR LE PROPRIÉTAIRE>` |
| Suppléant | même pouvoir en cas d'absence (indispensable pour tenir 24 h) | `<À DÉFINIR PAR LE PROPRIÉTAIRE>` |
| Analyste de triage | accusé de réception, reproduction, score CVSS 4.0, tenue du registre | équipe de développement |
| Correcteur | correctif, tests de non-régression, revue | mainteneur du composant |
| Publication | paquet signé, canal officiel, avis de sécurité | responsable des versions |
| Communication | lien avec le signaleur, information des utilisateurs | responsable sécurité produit |

- Une **astreinte** couvre la boîte de sécurité 7 j / 7 : le délai de 24 h court même le week-end (**à organiser**)
- Compte sur la plateforme ENISA ouvert **avant** le premier incident, au nom du responsable et du suppléant

## 3. Réception (ISO/IEC 29147)

<!-- @id securite.processus.cra.reception -->
<!-- @role intake -->
<!-- @do receive_report -->

- **Entrées** :
  - contact de sécurité (SECURITY.md, `security.txt`)
  - détection interne : revue, fuzzing, `cargo audit` / `cargo deny`, WorrySentinel
  - CSIRT, ENISA, mainteneur d'une dépendance, avis public
- Ouvrir une entrée au **registre** (§9) dès réception, horodatée en UTC
- **Accusé de réception** sous 3 jours ouvrés, avec identifiant interne `MSV-AAAA-NNN`
- Le signalement reste confidentiel : dépôt privé ou espace chiffré, jamais de ticket public avant l'avis

## 4. Triage (CVSS 4.0)

<!-- @id securite.processus.cra.triage -->
<!-- @role triage -->
<!-- @do triage_vulnerability -->

- **Reproduire**, identifier les composants et versions touchés, y compris les versions déjà diffusées
- **Score** : CVSS 4.0 (métriques de base + menace ; vecteur conservé au registre)

| Gravité | Score CVSS 4.0 | Correctif visé |
|---|---|---|
| Critique | 9,0 – 10,0 | 30 jours (au plus vite) |
| Haute | 7,0 – 8,9 | 30 jours |
| Moyenne | 4,0 – 6,9 | 90 jours |
| Faible | 0,1 – 3,9 | version suivante |

- **Question CRA systématique**, à chaque triage et à chaque nouvel élément : y a-t-il **exploitation active** ? Y a-t-il **incident grave** (§5) ?
  - si **oui** → le délai de 24 h court **depuis le moment où l'éditeur en a connaissance** ; prévenir immédiatement le responsable sécurité produit
  - noter au registre l'heure de prise de connaissance et les éléments qui fondent la décision
- Indices d'exploitation active : rapport d'un CSIRT ou d'un utilisateur, journaux, catalogue public des vulnérabilités exploitées, preuve d'attaque — en cas de doute, **traiter comme exploitée** et documenter

## 5. Définitions CRA utiles

<!-- @id securite.processus.cra.definitions -->
<!-- @role definitions -->
<!-- @do qualify_cra_event -->

- **Vulnérabilité activement exploitée** : preuve fiable qu'un acteur malveillant l'a exploitée dans un système sans autorisation du propriétaire (définition de l'art. 3 : **libellé exact à vérifier**)
- **Incident grave** (art. 14 §5) : incident qui
  - affecte ou peut affecter la capacité du produit à protéger la disponibilité, l'authenticité, l'intégrité ou la confidentialité de données ou de fonctions sensibles ou importantes, **ou**
  - a conduit ou peut conduire à l'introduction ou à l'exécution de code malveillant dans le produit ou dans les réseaux et systèmes de l'utilisateur
- Exemples COG : compromission de la clé du canal officiel, paquet de mise à jour falsifié, contournement de BorderGuard exploité, fuite de la base secondaire

## 6. Traitement (ISO/IEC 30111)

<!-- @id securite.processus.cra.traitement -->
<!-- @role remediation -->
<!-- @do remediate_vulnerability -->

1. **Analyse** : cause racine, variantes, autres composants touchés (recherche du même motif)
2. **Contournement** temporaire si le correctif tarde (configuration, désactivation d'un Core)
3. **Correctif** : branche privée, test de non-régression qui reproduit la faille, revue par une seconde personne
4. **Dépendance** en cause : signalement au mainteneur, avec le correctif s'il existe (art. 13 §6)
5. **Publication** : paquet signé et reproductible, canal officiel ; correctif de sécurité séparable des évolutions fonctionnelles quand c'est possible
6. **Avis de sécurité** signé : versions touchées, CVSS 4.0, correctif ou contournement, CVE si attribuée, remerciements
7. **Clôture** : retour d'expérience, mise à jour du modèle de menaces et des tests

## 7. Chronologie CRA (art. 14)

<!-- @id securite.processus.cra.chronologie -->
<!-- @role cra-reporting-timeline -->
<!-- @do notify_enisa -->

- **T0** = moment où l'éditeur **prend connaissance** de l'exploitation active ou de l'incident grave
- Notification **simultanée** au CSIRT coordinateur et à l'ENISA, via la plateforme unique (art. 14 §1, §3 et §7)

### 7.1 Vulnérabilité activement exploitée (art. 14 §2)

| Échéance | Envoi | Contenu minimal |
|---|---|---|
| **T0 + 24 h** | alerte précoce (art. 14 §2 a) | existence de la vulnérabilité exploitée ; États membres où le produit est disponible, le cas échéant |
| **T0 + 72 h** | notification (art. 14 §2 b) | informations générales sur le produit, nature de l'exploitation et de la vulnérabilité, mesures correctives ou d'atténuation, y compris celles que les utilisateurs peuvent prendre ; sensibilité de l'information |
| **au plus tard 14 jours après la mise à disposition d'une mesure corrective ou d'atténuation** | rapport final (art. 14 §2 c) | description, gravité et impact ; acteur malveillant s'il est connu ; détails de la mise à jour de sécurité ou des mesures |

### 7.2 Incident grave (art. 14 §4)

| Échéance | Envoi | Contenu minimal |
|---|---|---|
| **T0 + 24 h** | alerte précoce (art. 14 §4 a) | soupçon d'acte illicite ou malveillant ; États membres concernés, le cas échéant |
| **T0 + 72 h** | notification (art. 14 §4 b) | nature de l'incident, évaluation initiale, mesures correctives ou d'atténuation, y compris celles des utilisateurs ; sensibilité de l'information |
| **dans le mois suivant la notification (72 h)** | rapport final (art. 14 §4 c) | description détaillée, gravité et impact ; type de menace ou cause racine probable ; mesures appliquées et en cours |

- Délais vérifiés sur l'art. 14 ; colonnes « contenu minimal » résumées (libellé exact **à vérifier** au JO, notamment « acteur malveillant » et « sensibilité »)
- **Rapport intermédiaire** : à fournir si le CSIRT coordinateur le demande (art. 14 §6)
- Une information déjà transmise n'a pas à être répétée à l'étape suivante
- Format et procédures : actes d'exécution prévus (art. 14 §10) — **à vérifier** s'ils sont adoptés, et suivre les gabarits de la plateforme
- Règle interne : préparer la notification de 72 h **en même temps** que l'alerte de 24 h ; ne jamais attendre le correctif pour notifier

## 8. Plateforme unique ENISA

<!-- @id securite.processus.cra.plateforme -->
<!-- @role reporting-platform -->
<!-- @do use_enisa_srp -->

- **Plateforme** : Single Reporting Platform (SRP), capacité initiale déployée le **11/09/2026** (communiqué ENISA)
- **Portail** : `https://portal.cra-srp.enisa.europa.eu/` — modalités d'inscription du fabricant **à vérifier** (guide utilisateur et FAQ ENISA)
- **Routage** : point d'accès électronique du CSIRT coordinateur de l'État membre de l'établissement principal (France), accès simultané de l'ENISA (art. 14 §7)
- Conserver au registre : accusé de la plateforme, horodatage, contenu exact envoyé
- Plateforme indisponible : **à vérifier** la voie de repli auprès du CERT-FR ; consigner la tentative horodatée

## 9. Information des utilisateurs (art. 14 §8)

<!-- @id securite.processus.cra.utilisateurs -->
<!-- @role user-notification -->
<!-- @do inform_users -->

- Après prise de connaissance, informer les **utilisateurs touchés** (et, si utile, tous) : impact, mesures d'atténuation et correctifs qu'ils peuvent appliquer
- Format structuré et lisible par machine quand c'est possible : avis signé sur le canal officiel, notification dans Central, page d'avis publique (CSAF : **à étudier**)
- À défaut d'information en temps utile par l'éditeur, le CSIRT peut informer lui-même les utilisateurs
- Coordonner le calendrier avec le CSIRT et le signaleur (divulgation coordonnée, [SECURITY.md](../../SECURITY.md))

## 10. Registre des signalements

<!-- @id securite.processus.cra.registre -->
<!-- @role report-register -->
<!-- @do keep_report_register -->

- **Emplacement** : dépôt privé ou espace chiffré, accès limité aux rôles du §2 (`<À DÉFINIR PAR LE PROPRIÉTAIRE>`)
- **Champs** :
  - identifiant `MSV-AAAA-NNN`, source, date de réception (UTC)
  - composants et versions touchés, vecteur CVSS 4.0, gravité
  - exploitation active / incident grave : oui-non, **T0**, justification
  - envois ENISA : date, heure, référence de la plateforme, pour 24 h, 72 h, intermédiaire, final
  - correctif : version, date de mise à disposition (point de départ des 14 jours)
  - avis : date, CVE, information des utilisateurs, remerciements
  - décisions et responsables
- Conservation : au moins **10 ans** (alignée sur la conservation de la documentation technique CRA : **à vérifier**)
- Revue trimestrielle ; **exercice annuel** de bout en bout (signalement fictif jusqu'au brouillon ENISA) — NC-REG-03

## 11. Reste à fournir

<!-- @id securite.processus.cra.reste -->
<!-- @role open-items -->
<!-- @do list_open_items -->

- Contact de sécurité et, si souhaité, clé de chiffrement publiée (SECURITY.md, `security.txt`)
- Titulaires des rôles, astreinte, comptes sur la plateforme ENISA
- Période de support déclarée (≥ 5 ans)
- Route `/.well-known/security.txt` sur `miyukini.com` (nginx) et signature du fichier
- Vérifications juridiques marquées « à vérifier » ci-dessus

---

Date de mise a jour : 2026-10-26
