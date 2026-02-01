# Miyukini Account — Parcours, capacités et livrables (point d’entrée entreprise)

## Contexte

Ce document détaille le **parcours**, les **capacités** et les **livrables** du point d’entrée **Miyukini Account** (entreprise) du service COG Miyukini Account. Il complète le [Document fondateur Miyukini Account](../../Miyukini%20Account%20-%20Document%20Fondateur.md) et s’appuie sur l’[analyse des besoins](./Account%20-%20Analyse%20des%20besoins.md) et le document [Operateurs et Toolkits](./Account%20-%20Operateurs%20et%20Toolkits.md).

## Portée / Scope

- **Public** : Professionnels, associations, TPE/PME, organisateurs (point d’entrée Miyukini Account).
- **Périmètre** : Parcours (onboarding, tableau de bord, grand livre, devis, facturation, relances, encaissements, rapports, export, intégration MFS/JayRDV), capacités et livrables associés.
- **Hors périmètre** : Budgets personnels type Purse (réservés au point d’entrée Miyukini Purse) ; spécifications techniques (API, schémas).

---

## 1. Profil du public

| Critère | Description |
|---------|-------------|
| **Qui** | Professionnels, associations, TPE/PME, organisateurs qui souhaitent tenir une comptabilité au sens large : grand livre, devis, facturation, suivi des encaissements, rapports légaux. |
| **Compte** | Compte Miyukini Account (entreprise, association) avec identité légale (SIRET, etc.) si exigé ; rôles (admin, comptable, lecture seule). |
| **Accès** | Authentification (Miyauth) ; session gouvernée par Mandat ; point d’entrée Miyukini Account ; permissions (Master Butler). |
| **Espace** | Tableau de bord Account (CA, encaissements, factures en attente), grand livre, journal, devis, facturation, relances, encaissements, rapports, export. |
| **Intégration** | MFS (budget par édition, devis/factures exposants) et JayRDV (facturation professionnels) consomment les Opérateurs Miyukini Account. |

---

## 2. Parcours utilisateur

### 2.1 Parcours onboarding (compte Account)

1. **Accès** : L’utilisateur accède à la page d’inscription ou de connexion Miyukini Account (entreprise).
2. **Formulaire** : Saisie identité légale (SIRET, etc.) si exigé ; validation selon politique.
3. **Rôles** : Attribution des rôles (admin, comptable, lecture seule) et permissions (qui peut émettre une facture, qui peut consulter les rapports) selon Contrat d’équipe.
4. **Résultat** : Compte Account créé ; redirection vers le tableau de bord.

**Livrables sollicités** : Formulaire dédié Account ; validation politique ; Mandat et permissions (MAC-01, MAC-02).

### 2.2 Parcours grand livre et mouvements

1. **Saisie** : Enregistrement des revenus et dépenses (date, montant, libellé, catégorie, client/fournisseur, pièce justificative) ; saisie manuelle ou import CSV.
2. **Ventilation** : Ventilation par catégorie comptable, projet ou édition (ex. budget par édition MFS).
3. **Grand livre et journal** : Consultation du grand livre et du journal (liste des mouvements, tri, filtres : date, catégorie, projet, client/fournisseur) ; export pour expert-comptable.
4. **Correction** : Correction et annulation selon règles.

**Livrables sollicités** : Formulaire saisie mouvement ; ventilation catégorie/projet ; vue grand livre et journal ; export (MAC-04, MAC-05, MAC-06).

### 2.3 Parcours devis

1. **Création** : Création d’un devis (client, lignes : description, quantité, prix, TVA, conditions, validité) ; numérotation conforme ; enregistrement (KindMother).
2. **Envoi** : Envoi du devis au client (email, lien) ; statut mis à jour (envoyé, accepté, refusé) ; notification (Miyunotify) optionnelle.
3. **Conversion** : Conversion d’un devis accepté en facture (sans ressaisie) ; reprise des lignes et montants ; numérotation facture distincte.
4. **Intégration MFS/JayRDV** : Les services MFS et JayRDV appellent quote.create avec référence métier (exposant, édition, professionnel) ; identifiant retourné ; audit et niveau de sécurité déclaré.

**Livrables sollicités** : Formulaire devis ; envoi email/lien ; suivi statut ; action « Convertir en facture » ; intégration MFS/JayRDV (MAC-07 à MAC-10).

### 2.4 Parcours facturation

1. **Émission** : Émission d’une facture (client, lignes, montants, TVA, numérotation, conditions de paiement) ; conformité (TVA, numérotation selon juridiction) ; enregistrement et PDF.
2. **Relances** : Liste des factures impayées ; envoi de relance (email, modèle) ; suivi des relances ; configuration des seuils.
3. **Encaissements** : Saisie encaissement (montant, date, moyen) ; liaison facture ; statut payé / partiel / impayé ; pas de stockage RIB/carte en clair (token ou référence opaque).
4. **Intégration MFS/JayRDV** : MFS et JayRDV appellent invoice.emit avec référence métier ; facture enregistrée ; suivi relances/encaissements ; audit.

**Livrables sollicités** : Formulaire facture ; PDF ; liste factures impayées ; envoi relance ; saisie encaissement ; intégration MFS/JayRDV (MAC-11 à MAC-14).

### 2.5 Parcours rapports et export

1. **Tableaux de bord** : Consultation des tableaux de bord (CA, encaissements, factures en attente, répartition par catégorie/projet) ; indicateurs clés ; filtres par période, projet.
2. **Rapports légaux** : Production de rapports (bilan, compte de résultat, journal, grand livre) pour conformité et expert-comptable ; rapports prédéfinis selon juridiction ; export PDF/CSV ; pas d’export de données de paiement brutes.
3. **Export pour tiers** : Export des données pour expert-comptable ou logiciel tiers (format standard, périmètre contrôlé) ; audit de l’export.

**Livrables sollicités** : Tableau de bord configurable ; rapports légaux ; export PDF/CSV ; export expert-comptable (MAC-15, MAC-16, MAC-17).

### 2.6 Parcours budget par édition (intégration MFS)

1. **Enregistrement** : Miyukini Festival Service enregistre les revenus et dépenses par édition via Miyukini Account (appel budget.movements.record avec référence édition) ; ventilation par édition.
2. **Vue organisateur** : L’organisateur MFS consulte le budget de ses éditions (revenus/dépenses, ventilation) depuis son espace MFS ; données comptables fournies par Miyukini Account (rapports) ; affichage dans MFS selon Mandat et permissions.

**Livrables sollicités** : Appel budget.movements.record (MFS) ; rapports budget par édition ; vue budget dans MFS (MAC-18, MAC-19).

---

## 3. Capacités et livrables (synthèse)

| Capacité | Description | Livrable | Besoin(s) couvert(s) |
|----------|-------------|----------|----------------------|
| **Compte Account** | Inscription/compte entreprise, identité légale (SIRET), rôles et permissions. | Formulaire Account ; rôles (admin, comptable, lecture seule) ; permissions. | MAC-01, MAC-02 |
| **Données niveau 2–3** | Données Account niveau 2–3 ; résidence centralisée ; chiffrement et audit. | Gouvernance WorrySentinel, KindMother ; pas de copie non gouvernée. | MAC-03 |
| **Grand livre et mouvements** | Enregistrement mouvements, ventilation catégorie/projet, grand livre, journal, export. | Formulaire saisie ; vue grand livre et journal ; export expert-comptable. | MAC-04, MAC-05, MAC-06 |
| **Devis** | Création, envoi, suivi, conversion en facture ; intégration MFS/JayRDV. | Formulaire devis ; envoi email/lien ; conversion devis → facture ; quote.create. | MAC-07 à MAC-10 |
| **Facturation** | Émission factures, relances, encaissements ; intégration MFS/JayRDV. | Formulaire facture ; PDF ; relances ; saisie encaissement ; invoice.emit. | MAC-11 à MAC-14 |
| **Rapports et export** | Tableaux de bord, rapports légaux, export PDF/CSV/expert-comptable. | Tableau de bord ; rapports (bilan, compte de résultat) ; export. | MAC-15, MAC-16, MAC-17 |
| **Budget par édition (MFS)** | Enregistrement mouvements par édition ; vue budget organisateur. | budget.movements.record (MFS) ; rapports par édition ; affichage MFS. | MAC-18, MAC-19 |

---

## 4. Références

| Document | Rôle |
|----------|------|
| [Account - Analyse des besoins](./Account%20-%20Analyse%20des%20besoins.md) | Liste exhaustive des besoins MAC-01 à MAC-19, NFR-MAC-01 à NFR-MAC-08. |
| [Account - Operateurs et Toolkits](./Account%20-%20Operateurs%20et%20Toolkits.md) | Matrice Besoin → Service / Opérateur / Toolkit. |
| [Miyukini Account - Document Fondateur](../../Miyukini%20Account%20-%20Document%20Fondateur.md) | Contexte service COG, points d’entrée Purse/Account. |
| [Integration Services](../../reference/Miyukini%20Account%20-%20Integration%20Services.md) | Flux MFS, JayRDV, responsabilités. |
| [Points d’entrée Purse et Account](../../reference/Miyukini%20Account%20-%20Points%20Entree%20Purse%20et%20Account.md) | Périmètre Account, capacités exposées. |

---

**Document** : Miyukini Account — Parcours, capacités et livrables (point d’entrée entreprise)  
**Version** : 1.0  
**Date** : 2026-01-31  
**Statut** : Document de référence (parcours, capacités, livrables)
