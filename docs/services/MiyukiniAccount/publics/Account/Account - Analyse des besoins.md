# Miyukini Account — Analyse des besoins (point d’entrée entreprise)

## Contexte

Ce document constitue l’**analyse des besoins** du point d’entrée **Miyukini Account** (entreprise) du service COG Miyukini Account. Il identifie l’ensemble des besoins fonctionnels et non fonctionnels, les parcours détaillés, les personas, l’intégration avec les services métier (MFS, JayRDV), ainsi que la priorisation et les dépendances. Il s’adresse aux équipes produit, conception et développement.

**Références** : [Document fondateur Miyukini Account](../../Miyukini%20Account%20-%20Document%20Fondateur.md), [Points d’entrée Purse et Account](../../reference/Miyukini%20Account%20-%20Points%20Entree%20Purse%20et%20Account.md), [Intégration avec les autres services](../../reference/Miyukini%20Account%20-%20Integration%20Services.md), [Niveaux de sécurité et protection](../../reference/Miyukini%20Account%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md).

## Portée / Scope

- **Public** : Professionnels, associations, TPE/PME, organisateurs (point d’entrée Miyukini Account).
- **Périmètre** : Tous les besoins identifiés pour ce point d’entrée (comptabilité, devis, facturation, rapports, intégration MFS/JayRDV).
- **Hors périmètre** : Budgets personnels type Purse (réservés au point d’entrée Miyukini Purse), spécifications techniques détaillées (API, schémas).

### Cadre de travail (protocole documentation conceptuelle)

Conformément au [Protocole d’écriture de la documentation conceptuelle](../../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) :

| Élément | Description |
|--------|-------------|
| **Documentation autorisée (liste fermée)** | Document fondateur Miyukini Account ; Points d’entrée Purse et Account ; Integration Services ; Niveaux sécurité et protection ; Document fondateur MFS ; Document fondateur JayRDV ; Glossaire Miyukini ; Politique de résidence des données sensibles. |
| **Ce document ne fusionne pas** | Avec le Document fondateur, les documents référence ou l’analyse des besoins Purse. |
| **Ce document n’anticipe pas** | Les parcours capacités / livrables détaillés, les contrats d’API MFS/JayRDV ni les spécifications d’Opérateurs/Kits Account. |

### Contraintes absolues

| Contrainte | Description |
|------------|-------------|
| ❌ **Ne pas anticiper** | Les écrans, flux détaillés ou contrats d’API ne sont pas rédigés dans ce document. |
| ❌ **Ne pas fusionner** | Ce document reste limité au point d’entrée Account ; pas de mélange avec Purse ni avec les documents MFS/JayRDV. |
| ❌ **Ne pas corriger hors périmètre** | Les besoins Purse, MFS ou JayRDV ne sont pas modifiés depuis ce document ; les contrats d’intégration restent dans Integration Services. |
| ✅ **Source de vérité** | Ce document est la **référence** pour l’analyse des besoins du point d’entrée Miyukini Account (entreprise). |

### Décisions structurantes (mini log)

| Id | Décision | Justification |
|----|----------|---------------|
| **DS-MAC-01** | Account : devis, facturation, grand livre, rapports légaux ; pas de budgets occasionnels type Purse (sauf réutilisation en contexte projet/édition). | Périmètre entreprise ; différenciation avec Purse ; réutilisation des capacités « budget par projet » pour MFS (budget par édition). |
| **DS-MAC-02** | Données Account niveau 2–3 (Sensitive à Critical) ; résidence centralisée recommandée ou obligatoire. | Conformité Politique de résidence ; données facturation et moyens de paiement critiques. |
| **DS-MAC-03** | MFS et JayRDV appellent les Opérateurs Miyukini Account (quote.create, invoice.emit, budget.movements.record) ; ils détiennent les données métier (exposant, professionnel), Miyukini Account détient les données comptables. | Réduction de la duplication ; responsabilités explicites (INT-1 à INT-4, voir Integration Services). |
| **Dépendance critique** | Ce document dépend du Document fondateur Miyukini Account, des documents Points d’entrée et Integration Services ; toute évolution des besoins Account (ex. nouveaux besoins MFS/JayRDV) doit être cohérente avec Integration Services. | — |

---


## 1. Profil du public et personas

### 1.1 Définition du public

Les **utilisateurs Miyukini Account** (marque) sont des **professionnels**, **associations** ou **entreprises** qui souhaitent **tenir une comptabilité** au sens large : grand livre, devis, facturation, suivi des encaissements, rapports légaux. Ils accèdent au service COG Miyukini Account via le point d’entrée **Miyukini Account**. Ce point d’entrée est également **consommé** par les services métier (Miyukini Festival Service pour facturation exposants, JayRDV pour facturation professionnels).

### 1.2 Personas

| Persona | Profil | Objectifs principaux | Frustrations typiques |
|---------|--------|----------------------|------------------------|
| **TPE / auto-entrepreneur** | Peu de factures, besoin de simplicité et de conformité (TVA, numérotation). | Émettre des devis et factures, suivre les encaissements, rapports simples. | Outils trop lourds ou trop limités, conformité floue. |
| **Association** | Budget par projet ou par édition (événement), facturation occasionnelle (stands, prestations). | Budget par édition, devis/factures exposants ou prestataires, rapports pour l’AG. | Mélange budget perso et asso, pas de vue par projet. |
| **Organisateur (MFS)** | Plusieurs éditions (festivals), nombreux exposants, budget par édition. | Budget par édition, devis et factures exposants, ventilation revenus/dépenses, rapports. | Duplication des outils, pas d’intégration avec le service événementiel. |
| **Professionnel (JayRDV)** | Praticien ou cabinet ; facturation des RDV, abonnements. | Facturation des prestations (RDV, abonnements), relances, suivi des encaissements. | Double saisie entre agenda et facturation. |
| **Comptable / gestionnaire** | Gestion de la comptabilité, conformité, rapports légaux. | Grand livre, rapports (bilan, compte de résultat), export pour expert-comptable, conformité TVA. | Données éparpillées, pas d’audit traçable. |

### 1.3 Contexte d’usage

- **Fréquence** : connexion régulière (saisie des mouvements, émission de factures, relances) ou ponctuelle (rapports, clôture).
- **Appareils** : desktop prioritaire (comptabilité, rapports) ; mobile pour consultation et validation.
- **Intégration** : consommation par MFS (budget édition, facturation exposants), JayRDV (facturation professionnels) ; données métier (exposant, professionnel) détenues par le service consommateur, données comptables par Miyukini Account.

---

## 2. Besoins fonctionnels

### 2.1 Compte et accès

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| MAC-01 | Compte entreprise (Account) | Créer ou utiliser un compte Miyukini Account (entreprise, association) avec identité légale (SIRET, etc.) si exigé. | Formulaire dédié Account ; validation selon politique ; Mandat et permissions (Master Butler) pour devis, facturation, rapports. |
| MAC-02 | Rôles et permissions | Gérer les rôles (admin, comptable, lecture seule) et les permissions (qui peut émettre une facture, qui peut consulter les rapports). | Rôles définis par Contrat d’équipe ; permissions (Master Butler) ; audit des actions sensibles. |
| MAC-03 | Données niveau 2–3 | Les données Account (devis, factures, mouvements, moyens de paiement) sont niveau 2–3 (Sensitive à Critical) ; résidence centralisée recommandée ou obligatoire. | Niveau WorrySentinel 2–3 ; résidence sur COG de référence ; chiffrement et audit. |

### 2.2 Grand livre et mouvements

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| MAC-04 | Enregistrement des mouvements | Enregistrer des revenus et dépenses (date, montant, libellé, catégorie, client/fournisseur, pièce justificative). | Saisie manuelle ou import (CSV) ; catégories et comptes ; liaison client/fournisseur ; correction et annulation selon règles. |
| MAC-05 | Ventilation par catégorie / projet | Ventiler les mouvements par catégorie comptable, projet ou édition (ex. budget par édition MFS). | Catégories et projets configurables ; filtres par projet/édition ; rapports par projet. |
| MAC-06 | Grand livre et journal | Consulter le grand livre et le journal (liste des mouvements, tri, filtres). | Vue grand livre et journal ; filtres (date, catégorie, projet, client/fournisseur) ; export pour expert-comptable. |

### 2.3 Devis

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| MAC-07 | Création de devis | Créer un devis (client, lignes, montants, TVA, conditions, validité). | Formulaire devis ; lignes (description, quantité, prix, TVA) ; numérotation conforme ; enregistrement (KindMother). |
| MAC-08 | Envoi et suivi des devis | Envoyer un devis au client (email, lien) et suivre le statut (envoyé, accepté, refusé). | Envoi par email ou lien ; statut mis à jour ; notification (Miyunotify) optionnelle. |
| MAC-09 | Conversion devis → facture | Convertir un devis accepté en facture (sans ressaisie). | Action « Convertir en facture » ; reprise des lignes et montants ; numérotation facture distincte. |
| MAC-10 | Intégration MFS / JayRDV | Les services MFS et JayRDV appellent les Opérateurs Miyukini Account pour créer des devis (ex. devis exposant, devis professionnel). | Appel `quote.create` avec référence métier (exposant, édition, professionnel) ; identifiant retourné ; audit et niveau de sécurité déclaré. |

### 2.4 Facturation

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| MAC-11 | Émission de factures | Émettre une facture (client, lignes, montants, TVA, numérotation, conditions de paiement). | Formulaire facture ; conformité (TVA, numérotation selon juridiction) ; enregistrement et PDF. |
| MAC-12 | Relances | Gérer les relances (factures impayées, rappels, escalade). | Liste des factures impayées ; envoi de relance (email, modèle) ; suivi des relances ; configuration des seuils. |
| MAC-13 | Suivi des encaissements | Enregistrer les encaissements et associer à une facture (statut payé / partiel / impayé). | Saisie encaissement (montant, date, moyen) ; liaison facture ; pas de stockage RIB/carte en clair (token ou référence opaque). |
| MAC-14 | Intégration MFS / JayRDV | MFS et JayRDV appellent les Opérateurs Miyukini Account pour émettre des factures (ex. facture exposant, facture professionnel). | Appel `invoice.emit` avec référence métier ; facture enregistrée ; suivi relances/encaissements ; audit. |

### 2.5 Rapports et export

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| MAC-15 | Tableaux de bord | Consulter des tableaux de bord (CA, encaissements, factures en attente, répartition par catégorie/projet). | Tableau de bord configurable ; indicateurs clés ; filtres par période, projet. |
| MAC-16 | Rapports légaux | Produire des rapports (bilan, compte de résultat, journal, grand livre) pour conformité et expert-comptable. | Rapports prédéfinis selon juridiction ; export PDF/CSV ; pas d’export de données de paiement brutes. |
| MAC-17 | Export pour tiers | Exporter des données pour expert-comptable ou logiciel tiers (format standard, périmètre contrôlé). | Export CSV/Excel ou format standard ; périmètre et niveau de sécurité contrôlés ; audit de l’export. |

### 2.6 Budget par projet / édition (intégration MFS)

| Id | Besoin | Description | Critères d’acceptation |
|----|--------|-------------|-------------------------|
| MAC-18 | Budget par édition (MFS) | Miyukini Festival Service enregistre les revenus et dépenses par édition via Miyukini Account. | Appel `budget.movements.record` avec référence édition ; ventilation par édition ; rapports budget par édition dans MFS. |
| MAC-19 | Vue budget organisateur | L’organisateur MFS consulte le budget de ses éditions (revenus/dépenses, ventilation) depuis son espace MFS. | Données comptables fournies par Miyukini Account (rapports) ; affichage dans MFS selon Mandat et permissions. |

---

## 3. Besoins non fonctionnels

### 3.1 Sécurité et confidentialité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-MAC-01 | Données niveau 2–3 (Sensitive à Critical) | Devis, factures, mouvements : niveau 2 ; données de paiement, pièces comptables : niveau 3 ; flux chiffrés, résidence centralisée. |
| NFR-MAC-02 | Résidence centralisée | COG de référence désigné par contrat ; pas de copie non gouvernée sur terminaux ou COG tiers. |
| NFR-MAC-03 | Pas de stockage des données de paiement en clair | RIB, cartes, tokens sensibles : référencement par token ou identifiant opaque ; conformité PCI-DSS / réglementation. |
| NFR-MAC-04 | Audit complet | Traçabilité de toutes les lectures et écritures (devis, factures, mouvements, export) ; révocation possible (StrongFather, WorrySentinel). |

### 3.2 Conformité légale

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-MAC-05 | Conformité facturation | Numérotation, TVA, mentions légales selon juridiction (France, UE, etc.). |
| NFR-MAC-06 | Conformité rapports | Rapports (bilan, compte de résultat) conformes aux normes en vigueur selon juridiction. |

### 3.3 Performance et disponibilité

| Id | Besoin | Critères d’acceptation |
|----|--------|-------------------------|
| NFR-MAC-07 | Temps de chargement des rapports | Les rapports (tableau de bord, grand livre) se chargent en moins de 5 secondes (réseau standard). |
| NFR-MAC-08 | Émission de facture | L’émission d’une facture (création + PDF) s’effectue en moins de 3 secondes. |

---

## 4. Priorisation et dépendances

### 4.1 Priorisation (exemple)

| Priorité | Besoins | Justification |
|----------|---------|---------------|
| **P0** | MAC-01 à MAC-06, MAC-11 à MAC-13 (compte, mouvements, facturation, encaissements) | Fondamentaux comptabilité entreprise. |
| **P1** | MAC-07 à MAC-10 (devis, intégration MFS/JayRDV devis) | Parcours devis → facture ; intégration services. |
| **P2** | MAC-14 à MAC-17 (intégration facturation MFS/JayRDV, rapports, export) | Intégration complète, rapports légaux. |
| **P3** | MAC-18 à MAC-19 (budget par édition MFS) | Besoin MFS déjà cité dans Document fondateur MFS. |

### 4.2 Dépendances

| Besoin | Dépendance |
|--------|-------------|
| Compte Account | Miyauth, Master Butler (permissions, rôles), WorrySentinel (niveau 2–3). |
| Devis, factures, mouvements, rapports | Opérateurs et Kits Miyukini Account (COG) : `quote.create`, `invoice.emit`, `budget.movements.record`, `report.balance`, `report.export`. |
| Intégration MFS | Miyukini Festival Service (données métier exposants, éditions) ; Miyukini Account (données comptables). |
| Intégration JayRDV | JayRDV (données métier professionnels, RDV) ; Miyukini Account (facturation, encaissements). |
| Notifications | Miyunotify (relances, rappels). |

### 4.3 Dépendances explicites (ordre de lecture recommandé)

Pour cohérence inter-documents, l’ordre suivant est recommandé :

| Ordre | Document | Rôle |
|-------|----------|------|
| 1 | [Miyukini Account - Document Fondateur](../../Miyukini%20Account%20-%20Document%20Fondateur.md) | Contexte service COG, points d’entrée, sécurité. |
| 2 | [Points d’entrée Purse et Account](../../reference/Miyukini%20Account%20-%20Points%20Entree%20Purse%20et%20Account.md) | Périmètre Account, capacités exposées. |
| 3 | [Integration Services](../../reference/Miyukini%20Account%20-%20Integration%20Services.md) | Flux MFS, JayRDV, responsabilités. |
| 4 | Ce document (Account - Analyse des besoins) | Besoins fonctionnels et non fonctionnels Account. |

---

## 5. Références

| Document | Rôle |
|----------|------|
| [Miyukini Account - Document Fondateur](../../Miyukini%20Account%20-%20Document%20Fondateur.md) | Contexte, besoins stratégiques, positionnement Account. |
| [Points d’entrée Purse et Account](../../reference/Miyukini%20Account%20-%20Points%20Entree%20Purse%20et%20Account.md) | Périmètre Account, données, résidence. |
| [Intégration avec les autres services](../../reference/Miyukini%20Account%20-%20Integration%20Services.md) | MFS, JayRDV, flux de données, responsabilités. |
| [Niveaux de sécurité et protection](../../reference/Miyukini%20Account%20-%20Niveaux%20Securite%20et%20Protection%20Donnees.md) | Niveaux WorrySentinel, mesures de protection. |
| [Miyukini Festival Service - Document Fondateur](../../../MiyukiniFestivalService/Miyukini%20Festival%20Service%20-%20Document%20Fondateur.md) | Service consommateur (budget édition, facturation exposants). |
| [JayRDV - Document Fondateur](../../../JayRDV/JayRDV%20-%20Document%20Fondateur.md) | Service consommateur (facturation professionnels). |
| [Miyukini Prompt Protocol — Écriture documentation conceptuelle](../../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md) | Protocole d’écriture de la documentation conceptuelle (cadre de travail, contraintes, décisions structurantes). |

---

**Document** : Miyukini Account — Analyse des besoins (point d’entrée entreprise)  
**Version** : 1.1  
**Date** : 2026-01-31  
**Statut** : Document d’analyse (point d’entrée Account). Enrichi selon [Protocole d’écriture documentation conceptuelle](../../../../protocols/Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).
