# Miyukini COG — Conformité réglementaire (UE + France)

<!-- @id conformite.reglementaire -->
<!-- @role compliance-research -->
<!-- @layer 0-9+zones -->
<!-- @human Obligations légales UE et France (CRA, NIS2, RGPD, AI Act, facturation électronique, chiffrement) appliquées à COG — pas un avis juridique -->
<!-- @do research_compliance -->

- **Recherche du 2026-10-24** — sources et dates dans le document ; points incertains marqués « à vérifier » ; synthèse : [CONFORMITE-SYNTHESE](CONFORMITE-SYNTHESE.md)

- **Date** : 2026-10-24 · **Périmètre** : Studio Miyukini (éditeur), Origin (serveur central opéré par l'éditeur), Market, COG (distribution auto-hébergée), MAIA, Alicia, JayKonta, MiyukiniSales, Jay Mail, messagerie, appairage mobile
- **Sources internes lues** : `docs/architecture/COG-VUE-GLOBALE.md`, `docs/securite/MODELE-MENACES-COG.md` (EX-01..63), `docs/implementation/CONTRAINTES-IMPLEMENTATION.md` (C-xx)
- **Avertissement** : **ce document n'est pas un conseil juridique.** C'est une cartographie technique faite à partir de sources publiques. Les points marqués « **à vérifier** » doivent être confirmés par un juriste (droit du numérique et fiscal) avant toute décision.
- **Légende applicabilité** : **obligatoire** · **probable** · **à vérifier** · **non applicable**
- **Nouvelles exigences proposées** : `NC-REG-nn` (récapitulatif en §14)

---

## 1. Cyber Resilience Act (CRA, règlement (UE) 2024/2847)

- **Résumé** : exigences de cybersécurité obligatoires pour tout « produit comportant des éléments numériques » (matériel ou logiciel) mis sur le marché de l'UE. Marquage CE, gestion des vulnérabilités pendant toute la période de support et signalement des incidents.
- **Applicabilité** : **obligatoire** pour Studio Miyukini, en tant que **fabricant**
  - COG est un logiciel mis à disposition dans le cadre d'une **activité commerciale** (éditeur, Market, Origin) : c'est un produit au sens de l'art. 3(1)
  - L'exclusion « logiciel libre hors activité commerciale » (considérants 18-19) ne s'applique pas, puisque le produit est monétisé ou accompagné de services payants. Le régime allégé de **gestionnaire de logiciels libres** (« open-source steward », art. 24) ne vise que les entités qui soutiennent durablement un logiciel libre **sans** le monétiser : il ne s'applique pas à l'éditeur, **à vérifier** si une fondation était créée
  - **Paquets du Market** : un paquet tiers est un produit distinct dont son éditeur est le fabricant. Le Market peut avoir les obligations d'un **distributeur** (art. 20) ou d'un **importateur** (art. 19) pour les éditeurs hors UE, **à vérifier**
  - **Origin en mode SaaS** : un service distant n'est pas un produit, sauf s'il assure le **traitement de données à distance** indispensable à une fonction du produit (art. 3(2)). La fédération, le Market et les mises à jour relèvent **probablement** de ce cas : Origin entre alors dans le périmètre du CRA de COG
- **Classification (annexe III, avec les descriptions techniques du règlement d'exécution adopté le 28 novembre 2025, numéro 2025/2392 à vérifier)**
  - **Principe** : la classe dépend de la **fonctionnalité principale** du produit, pas des composants qu'il intègre
  - **COG en tant que distribution de type système d'exploitation** : **important classe I** (« systèmes d'exploitation »), **probable**
  - **Identité, passeport et racine USB** : la classe I couvre la « gestion des identités / accès à privilèges » et les « gestionnaires de mots de passe ». Si ces briques sont vendues **séparément**, elles relèvent de la classe I. Intégrées à COG, elles ne changent pas la classe, **à vérifier**
  - **Relais Origin et accès distant (VPN-like)** : la classe I couvre les **VPN** si la fonction est vendue séparément
  - **Douane (pare-feu + IPS) et Vigies (IDS)** : la **classe II** couvre « pare-feu, systèmes de détection et de prévention d'intrusion ». Si COG est présenté comme un pare-feu ou une passerelle de sécurité, toute la distribution peut basculer en **classe II**, ce qui impose une **évaluation par un tiers**. **Point critique, à trancher par la communication produit**
  - **Sandbox par service (EX-21)** : la classe II couvre aussi les « hyperviseurs et environnements d'exécution de conteneurs ». C'est un composant, pas la fonction principale, **à vérifier**
  - **Alicia (domotique, voix)** : la classe I couvre les « assistants virtuels domestiques polyvalents ». Alicia relève de la **classe I si elle est distribuée comme produit séparé**
  - **Gestion réseau** : la classe I couvre les « systèmes de gestion de réseau »
  - **Critique (annexe IV)** : **non applicable** (boîtiers matériels de sécurité, passerelles de compteurs intelligents, cartes à puce)
- **Procédure d'évaluation de la conformité (art. 32)**
  - **Produit par défaut** : auto-évaluation (module A)
  - **Classe I** : module A **seulement si une norme harmonisée** (ou une certification européenne) est appliquée entièrement. Sinon, il faut un organisme notifié (module B+C ou H)
  - **Classe II** : organisme notifié obligatoire
  - Les normes harmonisées du CEN-CENELEC JTC 13 sont encore en cours. Les organismes notifiés ne peuvent être désignés que depuis le 11/06/2026 : risque de goulot d'étranglement
- **Obligations concrètes**
  - **Annexe I, partie I (exigences de sécurité)** :
    - aucune vulnérabilité exploitable connue
    - configuration sûre par défaut
    - mises à jour de sécurité, automatiques par défaut avec possibilité de les refuser
    - contrôle d'accès, confidentialité, intégrité
    - minimisation des données, disponibilité, surface d'attaque réduite
    - journalisation des événements de sécurité
    - effacement sûr des données
  - **Annexe I, partie II (gestion des vulnérabilités)** :
    - **SBOM** au moins pour les dépendances de premier niveau
    - politique de divulgation coordonnée (CVD) et point de contact
    - correctifs gratuits et publiés séparément des évolutions fonctionnelles quand c'est possible
    - avis de sécurité publiés
  - **Art. 13** :
    - analyse de risques documentée
    - diligence raisonnable sur les composants tiers, y compris les crates
    - **période de support d'au moins 5 ans** (sauf durée d'usage plus courte)
    - chaque mise à jour de sécurité reste disponible **10 ans**
    - documentation technique (annexe VII), déclaration UE de conformité (annexe V), **marquage CE**
  - **Art. 14 (signalement)** : vulnérabilité **activement exploitée** ou **incident grave** touchant la sécurité du produit, à notifier via la **plateforme unique de signalement de l'ENISA** (SRP, ouverte en 2026) :
    - **alerte précoce sous 24 h**
    - **notification sous 72 h**
    - **rapport final** sous 14 jours après le correctif (vulnérabilité) ou sous 1 mois (incident)
    - information des utilisateurs
  - **Rattachement** : l'art. 14 s'applique **aussi aux produits déjà sur le marché**
- **Échéances**
  - Entrée en vigueur : 10/12/2024
  - Organismes notifiés : 11/06/2026
  - **Signalement (art. 14) : 11/09/2026, déjà applicable**
  - **Application complète : 11/12/2027**
- **Correspondance avec l'existant**
  - mises à jour signées et sûres : EX-01, EX-02, EX-26, EX-28
  - builds reproductibles : EX-04, C-06
  - composants vulnérables : EX-15, C-04 (`cargo audit` / `cargo deny`)
  - sûr par défaut : C-21, C-12 (fail-closed)
  - contrôle d'accès : C-20, C-22
  - cryptographie : C-24, C-25
  - intégrité : EX-09, C-23
  - surface d'attaque : C-33, EX-21, EX-41
  - journalisation et incidents : EX-12, EX-17, flux « Incident » de la vue globale
  - fuzzing : EX-10, C-26
  - **Manques** : SBOM, politique CVD, procédure ENISA 24 h/72 h, période de support déclarée, dossier technique et marquage CE, décision de classification → **NC-REG-01 à 06**
  - **Point d'attention** : la quarantaine de 48 h des mises à jour (EX-02) est compatible avec le CRA, à condition que les **correctifs critiques co-signés** soient bien publiés sans délai, ce que prévoit EX-02
- **Sanctions (art. 64)**
  - jusqu'à **15 M€ ou 2,5 % du CA mondial** (annexe I, art. 13-14)
  - 10 M€ ou 2 % (autres obligations)
  - 5 M€ ou 1 % (informations inexactes)
  - pas d'amende pour une micro ou petite entreprise qui manque le délai de l'alerte précoce de 24 h (art. 64(10)), **à vérifier**
  - retrait du marché possible
- **Sources**
  - Texte : https://eur-lex.europa.eu/eli/reg/2024/2847/oj (consulté le 2026-10-24)
  - ENISA SRP : https://www.enisa.europa.eu/news/the-cra-single-reporting-platform-is-launched (2026)
  - Signalement au 11/09/2026 : https://www.mccannfitzgerald.com/knowledge/data-privacy-and-cyber-risk/cyber-resilience-act-reporting-obligations-apply-from-11-september-2026
  - Catégories : https://www.hsfkramer.com/zh_HK/notes/cybersecurity/2026-posts/cyber-resilience-act-commission-clarifies-important-and-critical-product-categories (2026)

## 2. NIS2 (directive (UE) 2022/2555) et transposition française

- **Résumé** : obligations de gestion des risques cyber et de notification d'incidents (24 h/72 h/1 mois) pour les entités « essentielles » et « importantes » de 18 secteurs.
- **Statut en France** : **non transposée** au 2026-10-24
  - Le projet de loi « résilience des infrastructures critiques et renforcement de la cybersécurité » (NIS2 + DORA + REC) a été adopté par le Sénat le 12/03/2025
  - Le 06/10/2026, il a été **retiré de l'ordre du jour de l'Assemblée** des 7 et 9 octobre, sans nouvelle date
  - Point bloquant : l'art. 16 bis, qui interdit les portes dérobées dans les services de chiffrement
  - L'ANSSI a publié son référentiel **ReCyf**
- **Applicabilité**
  - **Éditeur** : **non applicable (probable)** tant que Studio Miyukini reste sous le seuil de la moyenne entreprise (< 50 salariés et ≤ 10 M€ de CA ou de bilan). Il deviendrait concerné au-delà, au titre des « fournisseurs de services gérés » ou « numériques », **à vérifier** selon la loi finale
  - **Origin** : **à vérifier**. S'il relaie des communications entre COG ou appareils, il peut être qualifié de **service de communications électroniques interpersonnelles accessible au public** (NI-ICS). Ce cas est **visé quelle que soit la taille** de l'entité (art. 2(2)(a)). Une fédération sur invitation n'est probablement pas « accessible au public »
  - **Utilisateurs** : **probable** pour les organisations clientes qui sont elles-mêmes des entités NIS2 (collectivités, santé…). COG fait alors partie de leur **chaîne d'approvisionnement** (art. 21(2)(d)), ce qui entraîne des exigences contractuelles envers l'éditeur
- **Obligations concrètes (indirectes)** : fournir aux clients un dossier de sécurité aligné sur ReCyf (mesures, notification, support) → **NC-REG-18**
- **Échéance** : loi française **en attente** (la transposition était due le 17/10/2024). Il y aura probablement des délais de mise en conformité après l'adoption
- **Correspondance** : EX-12, EX-30, flux « Incident », TAMR, plus NC-REG-03 (même chaîne d'alerte que le CRA)
- **Sanctions** :
  - entités essentielles : 10 M€ ou 2 %
  - entités importantes : 7 M€ ou 1,4 %
  - plafonds français **à vérifier**
- **Sources**
  - https://www.leto.legal/news/assemblee-nationale-report-projet-loi-resilience-nis2-octobre-2026 (oct. 2026)
  - https://next.ink/briefs/le-brief-du-23-septembre-2026/ (23/09/2026)
  - https://next.ink/229605/nis-2-lanssi-publie-son-recyf-la-csnp-simpatiente-et-le-fait-savoir/
  - https://eur-lex.europa.eu/eli/dir/2022/2555/oj

## 3. RGPD (règlement (UE) 2016/679), loi Informatique et Libertés et recommandations CNIL

- **Résumé** : tout traitement de données personnelles doit avoir une base légale, respecter la minimisation, la sécurité (art. 32) et la protection dès la conception (art. 25), et garantir les droits des personnes. Les violations de données sont notifiées sous 72 h.
- **Applicabilité** : **obligatoire**. Le rôle de chaque acteur doit être précisé :
  - **Éditeur sur COG auto-hébergé** : il n'est pas responsable de traitement des données locales (contacts, agenda, compta, voix) s'il **n'y a pas accès**, ce qui est cohérent avec le guide CNIL développeurs. Il doit cependant fournir un produit qui **permet** la conformité de ses clients (art. 25 et considérant 78)
  - **Utilisateurs** :
    - un particulier relève de **l'exemption domestique** (art. 2(2)(c))
    - une organisation (TPE, association, cabinet) est **responsable de traitement**, y compris pour les données de tiers : clients JayKonta, rendez-vous JayRDV, courriels
  - **Origin** :
    - **responsable de traitement** pour ses propres données : comptes, identités fédérales, registre Merkle, métadonnées de relais, journaux, comptes Market, vitrine
    - **sous-traitant (art. 28)** des organisations clientes pour les données relayées ou sauvegardées pour leur compte (sessions relayées, sauvegarde hors site EX-27 si elle transite par Origin), **à vérifier** selon les flux
    - le considérant 18 maintient le RGPD pour un prestataire qui sert des utilisateurs relevant de l'exemption domestique
  - **Registre Merkle et blacklist (EX-32 à 35)** : ce sont des données personnelles si un membre est une personne physique. Il faut concilier l'immuabilité avec le droit d'effacement (stocker des empreintes, données hors chaîne)
- **AIPD (art. 35)**, **probable** pour :
  - Origin : surveillance des pairs, « ban score » (EX-37), quorum de sanction (EX-32), assimilables à du profilage et à une décision affectant l'accès
  - les organisations clientes utilisant la voix (Alicia) ou la mémoire MAIA (EX-50 à 56)
  - l'éditeur devrait fournir un **modèle d'AIPD** (pratique recommandée par la CNIL)
- **Voix et biométrie**
  - Une voix identifiant le locuteur (reconnaissance du locuteur) est une **donnée biométrique (art. 9)** : il faut un consentement explicite ou une autre exception
  - Le simple traitement de commandes vocales n'est pas biométrique
  - Livre blanc CNIL « À votre écoute » (2020) : traitement local, pas de stockage du tampon du mot d'éveil, indicateur d'écoute
  - **Interdiction (AI Act, art. 5)** : reconnaissance des émotions au travail ou à l'école
- **Journaux** : recommandation CNIL journalisation (2021) : durée de conservation de **6 mois à 1 an** en général
- **Recommandations CNIL** :
  - guide RGPD du développeur
  - fiches IA (2024-2025) : base légale, information, droits sur modèles et mémoire, sécurité
  - recommandation sur les mots de passe et l'authentification
- **Correspondance avec l'existant** :
  - EX-21, C-40, EX-47 (segmentation, clé par service, étiquettes de sensibilité)
  - C-25 (secrets)
  - EX-36 (identifiant de passeport différent par COG visité : bonne pratique de non-chaînabilité)
  - EX-33 (pas de destruction à distance)
  - EX-50 à 56 (mémoire MAIA : à relier au droit d'effacement)
  - C-52 (pas d'apprentissage sur données étrangères)
  - **Manques** → **NC-REG-07 à 10**
- **Sanctions** :
  - jusqu'à **20 M€ ou 4 % du CA mondial**
  - procédure simplifiée CNIL : jusqu'à 20 k€
  - injonctions sous astreinte
- **Sources**
  - https://eur-lex.europa.eu/eli/reg/2016/679/oj
  - guide développeur : https://www.cnil.fr/fr/guide-rgpd-du-developpeur
  - IA : https://www.cnil.fr/fr/les-fiches-pratiques-ia
  - assistants vocaux : https://www.cnil.fr/fr/votre-ecoute-la-cnil-publie-son-livre-blanc-sur-les-assistants-vocaux (2020)
  - journalisation : https://www.cnil.fr/fr/la-cnil-publie-une-recommandation-relative-aux-mesures-de-journalisation (2021)
  - liens consultés le 2026-10-24, **à vérifier** pour les mises à jour 2026

## 4. AI Act (règlement (UE) 2024/1689), modifié par l'« omnibus IA » (règlement (UE) 2026/1744)

- **Résumé** : approche par niveaux de risque : pratiques interdites, systèmes à haut risque, transparence (art. 50), modèles d'IA à usage général (GPAI). L'omnibus IA (en vigueur fin juillet 2026, **à vérifier**) reporte les obligations « haut risque ».
- **Applicabilité**
  - **Studio Miyukini** est **fournisseur** des systèmes d'IA MAIA et Alicia, mis sur le marché sous son nom : **obligatoire** pour l'art. 50
  - Les **organisations utilisatrices** sont des **déployeurs**
  - **Classe de risque** : assistants généralistes, donc **non à haut risque**, **probable**. Ils le deviendraient en cas d'usage RH (tri de CV), d'évaluation de crédit ou d'éducation (JayEcole : notation, orientation, art. 6 et annexe III-3). Il faut donc **interdire ou encadrer** ces usages dans les conditions d'utilisation
  - **GPAI (Gemma, etc.)** : Google reste le fournisseur du modèle. Studio Miyukini est un **fournisseur en aval**, sans obligations GPAI propres, **sauf modification substantielle** du modèle (lignes directrices de la Commission de juillet 2025 : seuil indicatif d'un tiers du calcul d'entraînement initial), **à vérifier** en cas d'ajustement fin
  - **Conflit interne** : C-72 n'autorise que les modèles sous Apache-2.0 ou MIT, alors que **Gemma est sous « Gemma Terms of Use »**, une licence propriétaire avec une politique d'usage interdit, **à vérifier selon la version**. C-72 doit être arbitré
- **Obligations concrètes**
  - **Art. 50(1)** : informer clairement les personnes qu'elles interagissent avec une IA (MAIA, Alicia, et aussi les tiers qui appellent ou écrivent à un service animé par MAIA)
  - **Art. 50(2)** : **marquage lisible par machine** des contenus synthétiques (texte, audio TTS, images), sauf fonction d'assistance à l'édition
  - **Art. 50(4)** : obligation du déployeur pour les hypertrucages et les textes d'intérêt public. Prévoir des outils d'étiquetage
  - **Art. 5** : pratiques interdites (en vigueur depuis le 02/02/2025), notamment la reconnaissance des émotions au travail ou à l'école et la catégorisation biométrique sensible. À exclure d'Alicia et de MAIA
  - **Art. 4 (maîtrise de l'IA)** : depuis le 02/02/2025. Portée possiblement allégée par l'omnibus, **à vérifier**
  - **Notice et instructions** pour les déployeurs : limites, langues, supervision humaine (cohérent avec TAMR et EX-60)
- **Échéances**
  - 02/02/2025 : art. 4 et 5
  - 02/08/2025 : obligations GPAI
  - **02/08/2026 : art. 50**. Les sources divergent sur un délai de grâce jusqu'au **02/12/2026** pour le marquage de l'art. 50(2), **à vérifier** dans le règlement 2026/1744
  - Haut risque de l'annexe III reporté au 02/12/2027 ; annexe I reporté au 02/08/2028
- **Correspondance avec l'existant** :
  - EX-13 (contenu relayé jamais interprété comme une instruction)
  - EX-57 à 63 et C-51 (marquage `etranger`, validation humaine)
  - C-50 (outils appelés par intentions)
  - C-53, EX-26 (modèles signés)
  - C-72
  - **Manques** → **NC-REG-11, 12**
- **Sanctions** :
  - art. 5 : 35 M€ ou 7 %
  - art. 50 et autres obligations : 15 M€ ou 3 %
  - informations inexactes : 7,5 M€ ou 1 %
  - pour les PME, c'est le **montant le plus bas** qui s'applique
- **Sources**
  - https://eur-lex.europa.eu/eli/reg/2024/1689/oj
  - https://natlawreview.com/article/eu-digital-omnibus-ai-enters-force (2026)
  - https://www.addleshawgoddard.com/en/insights/insights-briefings/2026/technology/eu-ai-act-ai-omnibus-formally-adopted/ (07/2026)
  - https://secureprivacy.ai/blog/eu-ai-act-digital-omnibus-the-new-high-risk-ai-deadlines-after-council-approval

## 5. eIDAS 2 (règlement (UE) 2024/1183) et portefeuille d'identité EUDI

- **Résumé** : chaque État membre doit fournir un portefeuille d'identité numérique européen (fin 2026, 24 mois après les actes d'exécution de fin 2024). Les parties utilisatrices privées des secteurs réglementés et les très grandes plateformes devront l'accepter (horizon fin 2027).
- **Applicabilité** :
  - **non applicable** comme obligation : COG n'est ni un prestataire de confiance ni une partie utilisatrice réglementée
  - **opportunité** : accepter le portefeuille EUDI comme justificatif à l'enrôlement d'une personne (passeport, C-20) ou pour une signature avancée dans JayKonta et la Suite Jay Bureau
  - Une signature ou un horodatage « qualifié » exigerait un prestataire de confiance qualifié (QTSP) : les horodatages signés par Origin (EX-22) **ne sont pas qualifiés** au sens d'eIDAS
- **Obligation concrète** : aucune. **Ne pas présenter** les signatures et horodatages Origin comme qualifiés ou ayant une valeur probante eIDAS
- **Échéances** : portefeuilles fin 2026 ; acceptation obligatoire par les parties utilisatrices réglementées vers fin 2027, **à vérifier**
- **Sources** : https://eur-lex.europa.eu/eli/reg/2024/1183/oj · https://ec.europa.eu/digital-building-blocks/sites/display/EUDIGITALIDENTITYWALLET

## 6. Facturation électronique, comptabilité et logiciels de caisse (France)

- **Résumé** : facturation électronique B2B obligatoire entre assujettis à la TVA en France, via une **plateforme agréée (PA, ex-PDP)**, avec e-reporting des opérations B2C et internationales. La comptabilité informatisée doit produire un **FEC** et garantir l'irréversibilité des écritures. Les logiciels de caisse doivent garantir **inaltérabilité, sécurisation, conservation et archivage** (art. 286 I 3° bis du CGI).
- **Applicabilité**
  - **JayKonta (facturation et compta)** : **obligatoire** pour les clients assujettis. JayKonta **n'a pas à devenir une PA** : il doit **émettre et recevoir via une PA**, dans les formats Factur-X, UBL ou CII (socle EN 16931), avec les mentions obligatoires (SIREN, nature de l'opération, adresse de livraison, option TVA sur les débits)
  - **FEC** (art. L47 A et A47 A-1 du LPF) : **obligatoire** dès qu'une entreprise tient sa comptabilité dans JayKonta. Il faut aussi une **procédure de clôture** rendant les écritures intangibles et un archivage de 6 ans (fiscal) et 10 ans (commercial)
  - **MiyukiniSales**
    - si la fonction se limite à l'envoi de devis : **non applicable** au régime caisse
    - s'il y a un **point de vente** encaissant des clients non assujettis : **obligatoire**. Il faut une **certification par un organisme accrédité** (NF525 par Infocert, ou LNE)
    - **à vérifier** : la loi de finances 2025 a supprimé l'auto-attestation de l'éditeur (date d'effet reportée à plusieurs reprises, puis 01/09/2026). Une source indique que la LF 2026 aurait **rétabli une attestation individuelle** de l'éditeur. Il faut vérifier le texte publié au JO et le BOFiP
- **Obligations concrètes** :
  - connecteur PA
  - formats normés
  - cycle de vie des statuts de facture
  - e-reporting
  - annuaire
  - FEC conforme
  - chaînage cryptographique des écritures et des tickets (journal inaltérable)
  - clôtures périodiques
  - archivage probant
  - certification pour la caisse
- **Échéances**
  - **01/09/2026 : réception** obligatoire pour **toutes** les entreprises, **émission** pour les grandes entreprises et les ETI (date passée)
  - **01/09/2027 : émission** pour les **PME et micro-entreprises**, c'est-à-dire la cible principale de JayKonta
  - caisse : régime en vigueur, certification à vérifier ci-dessus
- **Correspondance avec l'existant** :
  - EX-06 (opérations signées par leur auteur)
  - EX-08, C-41 (DB versionnée)
  - EX-22 (horodatage)
  - C-61 (vecteurs figés)
  - **Manques** → **NC-REG-13, 14**
- **Sanctions** :
  - facture électronique non émise : **15 € par facture**, plafonné à 15 000 €/an ; e-reporting : 250 € par manquement, plafonné à 15 000 €/an (LF 2024, **à vérifier**)
  - FEC absent ou non conforme : 5 000 € (art. 1729 D du CGI)
  - caisse non conforme : **7 500 € par système** (art. 1770 duodecies du CGI, **à vérifier**)
- **Sources**
  - https://www.impots.gouv.fr/professionnel/je-passe-la-facturation-electronique
  - https://www.weblex.fr/weblex-actualite/facturation-electronique-un-calendrier-confirme-mais-d-autres-doutes-subsistent
  - https://www.legifiscal.fr/actualites-fiscales/4098-fin-auto-certification-logiciels-caisse-report-1er-septembre-2025.html
  - https://www.fiducial.fr/Articles-et-dossiers/Conformite-des-logiciels-systemes-de-caisse
  - amendement PLF 2026 : https://www.senat.fr/enseance/2025-2026/138/Amdt_I-1191.html
  - consultés le 2026-10-24

## 7. Directive responsabilité du fait des produits (directive (UE) 2024/2853)

- **Résumé** : le **logiciel**, y compris le SaaS et les mises à jour, devient un « produit ». La responsabilité est **sans faute** pour les dommages causés par un défaut, y compris la **perte ou la corruption de données** à usage non professionnel. Un défaut de sécurité ou l'**absence de mise à jour de sécurité** peut constituer un défaut.
- **Applicabilité** : **obligatoire** pour l'éditeur, en tant que fabricant de COG et de ses services officiels
  - Les logiciels libres développés **hors activité commerciale** sont exclus
  - Un intégrateur peut être tenu responsable des composants qu'il intègre
  - Le Market peut être assimilé à un fournisseur ou un distributeur si l'éditeur d'un paquet est inconnu, **à vérifier**
- **Obligations concrètes** :
  - pas d'obligation de procédure en soi, mais un **risque**
  - la conformité CRA démontrée et le respect des obligations de mise à jour sont la meilleure défense
  - la clause de limitation de responsabilité est **inopposable** au consommateur
  - assurance responsabilité civile professionnelle et produits
  - documentation de ce qu'est un défaut et une mise à jour
  - pas d'exonération pour « risque de développement » sur les mises à jour qui restent sous le contrôle de l'éditeur
- **Échéance** : transposition au plus tard le **09/12/2026**. S'applique aux produits mis sur le marché après cette date ; loi française de transposition **à vérifier**
- **Correspondance** : NC-REG-04 (période de support), EX-01, EX-02 et EX-08 (sauvegardes réversibles, qui limitent le dommage « perte de données ») → **NC-REG-17**
- **Sanctions** : dommages et intérêts **sans plafond**. Présomption de défectuosité en cas de complexité technique ou de refus de communiquer des preuves
- **Source** : https://eur-lex.europa.eu/eli/dir/2024/2853/oj

## 8. Data Act (règlement (UE) 2023/2854)

- **Résumé** : droits d'accès et de partage des données générées par les **produits connectés** et les **services connexes**. Il prévoit aussi le changement de fournisseur cloud et l'interopérabilité.
- **Applicabilité** : **à vérifier, probablement limitée**
  - COG en logiciel seul n'est pas un « produit connecté » (il faut un objet physique qui génère des données)
  - Alicia pilotant des objets tiers : c'est le fabricant de ces objets qui a les obligations. Alicia pourrait être un « service connexe » si elle est fournie avec un objet du Studio
  - Si Studio Miyukini vend un **boîtier COG** ou des capteurs : **probable**
  - Chapitre VI (changement de fournisseur cloud) : MiyuCloud est auto-hébergé, donc **non applicable**. Origin propose-t-il un « service de traitement de données » (sauvegarde) ? **À vérifier**
- **Obligations concrètes (si applicable)** : accès aux données par conception (art. 3(1)), information précontractuelle (art. 3(2)), accès sans frais pour l'utilisateur. Les micro et petites entreprises ont des exemptions partielles (art. 7)
- **Échéances** : application le 12/09/2025 ; **art. 3(1) pour les produits mis sur le marché après le 12/09/2026**
- **Correspondance** : philosophie compatible avec LOI-3 (« état local souverain ») → **NC-REG-19**
- **Sanctions** : fixées par les États membres, **à vérifier** en France
- **Source** : https://eur-lex.europa.eu/eli/reg/2023/2854/oj

## 9. ePrivacy (directive 2002/58), CPCE et cookies

- **Résumé** : confidentialité des communications électroniques et règles sur les traceurs (cookies). Le projet de règlement ePrivacy a été **retiré** par la Commission en 2025 : c'est la directive transposée qui reste en vigueur (art. L32-3 du CPCE, art. 82 de la loi Informatique et Libertés).
- **Applicabilité**
  - **Jay Mail et la messagerie sur COG** : c'est l'organisation utilisatrice qui opère le service, l'éditeur n'est pas fournisseur. **Non applicable** à l'éditeur
  - **Origin en relais de messages ou de sessions entre membres** : **à vérifier**. Il pourrait s'agir d'un service de communications interpersonnelles (NI-ICS), soumis au **secret des correspondances** et à la sécurité
  - **Cookies et traceurs** sur la vitrine Origin, le Market et les pages publiques H1 : **obligatoire**, avec consentement préalable sauf traceurs strictement nécessaires (lignes directrices CNIL de 2020)
- **Obligations concrètes** :
  - aucun accès au contenu relayé (chiffrement de bout en bout, cohérent avec EX-05 et EX-13)
  - minimisation des métadonnées
  - bannière de consentement seulement si des traceurs non essentiels existent ; mieux, **ne pas en utiliser**
- **Sanctions** : CNIL (jusqu'à 2 % ou 4 %, voir les sanctions pour cookies de 2020 à 2025) ; atteinte au secret des correspondances (art. 226-15 du Code pénal)
- **Sources** : https://eur-lex.europa.eu/eli/dir/2002/58/oj · https://www.cnil.fr/fr/cookies-et-autres-traceurs

## 10. LCEN, DSA et obligations d'hébergeur ou d'intermédiaire (Origin, Market)

- **Résumé** : un hébergeur n'est responsable des contenus que s'il en a connaissance et ne les retire pas promptement. Il doit **conserver les données d'identification** et répondre aux réquisitions judiciaires. Le DSA (règlement (UE) 2022/2065) ajoute des obligations harmonisées.
- **Applicabilité**
  - **Origin** (vitrine, Market qui héberge des paquets tiers, relais, annuaire) : **probable**, au titre d'**hébergeur et intermédiaire** (art. 6 de la LCEN, DSA). Si le Market permet à des tiers de publier, c'est une **plateforme en ligne**, voire une place de marché
  - Les micro et petites entreprises sont **exemptées** des obligations propres aux plateformes (art. 19 du DSA), mais **pas** des obligations de base des hébergeurs
  - **Interception légale** : Origin n'est pas un opérateur de réseau ouvert au public. Les obligations d'interception du CPCE (art. L34-1 et suivants) visent les opérateurs, **à vérifier** si Origin est requalifié en NI-ICS
  - **Fourniture de prestations de cryptologie** :
    - l'art. L871-1 du Code de la sécurité intérieure impose aux fournisseurs de **prestations** de cryptologie de remettre les conventions de déchiffrement sur réquisition
    - l'art. 434-15-2 du Code pénal sanctionne le refus de remettre une convention secrète de déchiffrement connue
    - une architecture où l'éditeur **ne détient aucune clé** (EX-24, EX-25) limite ce qui peut être remis. Il faut le **documenter**
    - débat de l'art. 16 bis du projet de loi résilience (refus des portes dérobées), **à suivre**
- **Obligations concrètes**
  - **Conservation des données d'identification** (art. L34-1 du CPCE, art. 6 de la LCEN, décret n° 2021-1362) :
    - données d'identité civile : 5 ans après la fin du contrat
    - informations de compte et de paiement : 1 an
    - données de connexion techniques : 1 an
  - **DSA** :
    - point de contact unique (art. 11-12)
    - conditions générales (art. 14)
    - mécanisme de notification et d'action (art. 16)
    - motivation des retraits (art. 17)
    - signalement des infractions pénales graves (art. 18)
  - procédure de réquisition traçable (TAMR)
- **Échéances** : déjà en vigueur (DSA depuis le 17/02/2024 ; décret de 2021)
- **Correspondance** : EX-30 (gouvernance à seuil), EX-32 à 33 (sanction motivée, contestation : proche de l'art. 17 du DSA), EX-22 → **NC-REG-16**
- **Sanctions** :
  - LCEN : défaut de conservation jusqu'à 1 an d'emprisonnement et 75 000 € (personne physique), **à vérifier**
  - DSA : jusqu'à 6 % du CA (Arcom pour la France)
- **Sources**
  - https://www.legifrance.gouv.fr/loda/id/JORFTEXT000000801164 (LCEN)
  - https://www.legifrance.gouv.fr/jorf/id/JORFTEXT000044228074 (décret 2021-1362)
  - https://eur-lex.europa.eu/eli/reg/2022/2065/oj

## 11. Contrôle de la cryptographie (France et UE)

- **Résumé**
  - En France, l'**utilisation** de la cryptologie est libre (art. 30 de la LCEN)
  - La **fourniture**, l'importation et le transfert de moyens assurant la **confidentialité** sont soumis à **déclaration auprès de l'ANSSI**, sauf dispense (décret n° 2007-663, annexe 1)
  - Les fonctions limitées à l'authentification et à l'intégrité sont libres
  - L'**exportation hors UE** relève du règlement « double usage » (UE) 2021/821, annexe I, catégorie 5 partie 2 (5A002 / 5D002)
- **Applicabilité** : **obligatoire (probable)** pour l'éditeur. COG fournit du chiffrement de confidentialité (`miyusceau`, sauvegardes chiffrées EX-27, relais, clés par service)
  - Les dispenses de l'annexe 1 et le classement « grand public » (annexe 2, catégorie 3, comme l'attestation ANSSI accordée à Debian) sont **à vérifier**
  - Le statut « logiciel libre publié » ne vaut **pas** dispense automatique en France, **à vérifier**
  - **Export** :
    - la « General Software Note » exclut les logiciels « dans le domaine public » (code publié librement) ; la note « Cryptography Note 3 » (grand public) permet un classement allégé
    - les distributions commerciales hors UE et les **pays sous embargo** sont à contrôler
    - transferts intra-UE libres (sauf annexe IV)
- **Obligations concrètes** :
  - **déclaration ANSSI** (formulaire, délai d'un mois) **avant** la fourniture
  - analyse de classement export (5D002 ou note 3)
  - filtrage des destinations sous sanctions sur le Market et Origin
  - mise à jour de la déclaration en cas de changement d'algorithme majeur
- **Échéance** : **déjà applicable**, préalable à toute distribution
- **Correspondance** : C-24 (cryptographie centralisée dans `miyusceau` : inventaire facile), EX-24, EX-25 → **NC-REG-15**
- **Sanctions** : fourniture sans déclaration jusqu'à 1 an d'emprisonnement et 15 000 € (art. 35 de la LCEN, **à vérifier**). Export illicite de biens à double usage : sanctions pénales et douanières
- **Sources**
  - https://cyber.gouv.fr/controle-reglementaire-sur-la-cryptographie-demarches-accomplir
  - https://www.debian.org/legal/anssi.fr.html (exemple d'attestation)
  - https://eur-lex.europa.eu/eli/reg/2021/821/oj
  - consultés le 2026-10-24

## 12. Autres textes à surveiller (brièvement)

- **Directive RED, acte délégué (UE) 2022/30 (cybersécurité des équipements radio, normes EN 18031)** :
  - **à vérifier**, seulement si le Studio vend du **matériel radio** (boîtier COG Wi-Fi, capteurs Alicia)
  - applicable depuis le 01/08/2025, puis absorbé par le CRA en 2027
  - → **NC-REG-20**
- **European Accessibility Act (directive 2019/882, applicable depuis le 28/06/2025)** : **à vérifier** pour le Market s'il vend aux consommateurs (service de commerce électronique). Les micro-entreprises de services sont exemptées
- **Code de la consommation** : garantie légale de conformité des contenus et services numériques, avec **obligation de mises à jour** (art. L224-25-25 et suivants). **Obligatoire** pour une vente B2C de COG
- **NIS2 côté chiffrement (art. 16 bis du projet de loi)** : son issue conditionne le risque d'une obligation d'accès exceptionnel

## 13. Synthèse d'applicabilité

| Texte | COG (produit) | Origin | Market | MAIA / Alicia | JayKonta / Sales | Utilisateurs-organisations |
|---|---|---|---|---|---|---|
| CRA | **obligatoire** (classe I, peut-être II) | probable (traitement à distance) | distributeur, à vérifier | inclus (classe I si Alicia séparée) | inclus | — |
| NIS2 | — | à vérifier (NI-ICS) | non (taille) | — | — | probable (chaîne d'approvisionnement) |
| RGPD | conception (art. 25) | **obligatoire** (RT + ST) | obligatoire | obligatoire (voix, mémoire) | obligatoire (données tiers) | **obligatoire** (RT) |
| AI Act | — | — | — | **obligatoire** art. 50 / 5 / 4 | — | déployeurs |
| eIDAS 2 | opportunité | non qualifié | — | — | opportunité (signature) | — |
| Facturation électronique / FEC / caisse | — | — | — | — | **obligatoire** | obligatoire |
| Responsabilité produits | **obligatoire** (dès le 09/12/2026) | probable | à vérifier | inclus | inclus | — |
| Data Act | à vérifier (matériel) | à vérifier | — | à vérifier | — | — |
| ePrivacy / cookies | — | à vérifier / obligatoire (cookies) | obligatoire (cookies) | — | — | opérateurs de leur messagerie |
| LCEN / DSA | — | probable | probable | — | — | — |
| Cryptologie | **obligatoire** (déclaration) | inclus | inclus | — | — | utilisation libre |

## 14. Nouvelles exigences proposées (NC-REG)

- **NC-REG-01** SBOM (CycloneDX ou SPDX) généré en CI pour chaque version de Core, service officiel et paquet Market, signé avec la version (complète C-04, EX-15)
- **NC-REG-02** Politique de divulgation coordonnée publiée, `security.txt`, point de contact unique, avis de sécurité publics (CRA, annexe I, partie II)
- **NC-REG-03** Procédure de signalement CRA (et NIS2 éventuel) : un incident WorrySentinel de niveau critique ou une vulnérabilité exploitée déclenche une qualification TAMR, puis le dépôt sur la plateforme ENISA en **24 h / 72 h / 14 j ou 1 mois**, avec astreinte et exercice annuel
- **NC-REG-04** Période de support déclarée (**≥ 5 ans**) par version majeure, correctifs de sécurité gratuits et séparables, archives des mises à jour conservées **10 ans**
- **NC-REG-05** Dossier technique CRA (annexe VII) : modèle de menaces comme analyse de risques, matrice annexe I ↔ EX/C, déclaration UE de conformité, marquage CE, notice utilisateur (annexe II)
- **NC-REG-06** Fiche de **décision de classification** CRA par produit (COG, Alicia, briques vendues séparément), avec règles de communication produit (ne pas présenter COG comme « pare-feu » sans assumer la classe II) et obligations de distributeur et d'importateur pour le Market (vérification CE et SBOM des paquets tiers)
- **NC-REG-07** Registre des traitements d'Origin, politique de confidentialité, contrat de sous-traitance type (art. 28) pour les organisations, AIPD d'Origin (« ban score », sanctions) et **modèle d'AIPD** fourni aux clients
- **NC-REG-08** Confidentialité par défaut : télémétrie désactivée, métadonnées Origin minimales, outils intégrés d'export, de rectification et d'**effacement**, y compris pour la mémoire MAIA (EX-50 à 56) et les registres (empreintes hors chaîne)
- **NC-REG-09** Durées de conservation des journaux configurables, par défaut entre 6 et 12 mois. Journaux d'Origin alignés sur les obligations LCEN (§10), sans données de contenu
- **NC-REG-10** Voix : traitement 100 % local, pas de stockage du tampon du mot d'éveil, indicateur d'écoute, **pas d'identification biométrique du locuteur par défaut** (si activée : consentement explicite et gabarit chiffré local), aucune reconnaissance des émotions
- **NC-REG-11** Transparence IA : mention « IA » visible dans MAIA et Alicia (y compris envers les tiers), **marquage lisible par machine** des contenus générés (métadonnées ou C2PA), notice pour les déployeurs, usages à haut risque exclus dans les conditions d'utilisation
- **NC-REG-12** Registre des modèles IA (fournisseur, licence, version, documentation GPAI, empreinte). **Arbitrer C-72** face à la licence Gemma
- **NC-REG-13** JayKonta : formats Factur-X, UBL et CII, connecteur PA, e-reporting, export FEC, clôture irréversible (chaînage d'empreintes), archivage 6 et 10 ans
- **NC-REG-14** MiyukiniSales en point de vente : journal inaltérable et chaîné, clôtures, archivage, **certification NF525 ou LNE** (ou attestation selon la LF 2026, à vérifier) avant toute commercialisation
- **NC-REG-15** Déclaration ANSSI du moyen de cryptologie, fiche de classement export, filtrage des pays sous sanctions sur le Market et Origin
- **NC-REG-16** Origin et Market : conditions générales, point de contact DSA, notification et action, motivation des retraits, conservation des données d'identification, procédure de réquisition tracée par TAMR, documentation « l'éditeur ne détient aucune clé »
- **NC-REG-17** Responsabilité produits : assurance RC produits et pro, traçabilité des versions installées, sauvegardes réversibles activées par défaut
- **NC-REG-18** Dossier « fournisseur NIS2 » pour les clients (correspondance avec ReCyf, engagements de notification et de support)
- **NC-REG-19** Si le Studio vend du matériel : accès aux données par conception (Data Act, art. 3)
- **NC-REG-20** Si le Studio vend du matériel radio : conformité RED, normes EN 18031

## 15. Feuille de route, par échéance légale puis par risque

1. **Immédiat (obligations déjà en vigueur ou échéances dépassées)**
   - **Signalement CRA, applicable depuis le 11/09/2026** si COG est déjà distribué : NC-REG-02, NC-REG-03 (risque élevé, faible coût)
   - **Déclaration de cryptologie à l'ANSSI** avant toute distribution : NC-REG-15
   - **RGPD pour Origin** : NC-REG-07, 08, 09
   - **AI Act** : art. 5 et 4 (depuis le 02/02/2025), **art. 50 (depuis le 02/08/2026)** : NC-REG-10, 11, 12
   - **Facturation électronique, réception (01/09/2026)** : JayKonta doit recevoir via une PA (NC-REG-13, partie réception)
   - **Logiciel de caisse** : bloquer l'usage « point de vente » de MiyukiniSales tant que NC-REG-14 n'est pas fait
   - **LCEN / DSA** pour Origin et le Market : NC-REG-16
2. **Avant le 02/12/2026** : marquage des contenus IA, si un délai de grâce existe (à vérifier)
3. **Avant le 09/12/2026** : directive responsabilité produits (NC-REG-17) et période de support déclarée (NC-REG-04)
4. **Fin 2026 à 2027** : suivre la loi NIS2 française et l'art. 16 bis (NC-REG-18) ; portefeuille EUDI (opportunité)
5. **Avant le 01/09/2027** : **émission** de factures électroniques PME et micro (NC-REG-13 complet). C'est la cible principale de JayKonta : **priorité commerciale**
6. **Avant le 11/12/2027 : CRA complet**
   - SBOM (NC-REG-01)
   - classification (NC-REG-06), **à décider avant fin 2026** car un organisme notifié en classe II demande 6 à 12 mois
   - dossier technique et marquage CE (NC-REG-05)
7. **Selon l'offre** : matériel (NC-REG-19, 20), Market B2C (accessibilité, garantie de conformité)

> Rappel : cette analyse n'est pas un conseil juridique. Les points « à vérifier » (classification CRA, statut d'Origin au regard de NIS2 et du CPCE, régime de certification des caisses après la LF 2026, délai de grâce de l'art. 50, dispenses cryptologie) doivent être validés par un professionnel avant mise sur le marché.
