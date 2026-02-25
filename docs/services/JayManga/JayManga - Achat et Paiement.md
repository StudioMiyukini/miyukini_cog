# JayManga — Achat et Paiement

## Contexte

Ce document detaille le **module de paiement integre** de JayManga : achat d'oeuvres, gestion du panier, licences, remboursements, promotions et administration des ventes. Le module est **integre directement dans JayManga** (DS-02 du Document Fondateur) : pas d'adaptateur JayKonta en V1.

Le vendeur gere ses ventes depuis **Miyukini Central**. Le lecteur achete depuis le **Portail** (surface web du COG vendeur).

Ce document est un complement au [Document Fondateur JayManga](./JayManga%20-%20Document%20Fondateur.md).

---

## 1. Modeles de tarification

### 1.1 Types de prix

| Modele | Description |
|--------|-------------|
| **Gratuit** | L'oeuvre est entierement accessible sans achat. Toutes les pages sont lisibles. Aucune licence n'est generee. |
| **Payant par oeuvre** | Le lecteur achete l'acces complet a une oeuvre (tous les chapitres existants et futurs de ce volume). |
| **Payant par chapitre** | Le lecteur achete l'acces a un chapitre individuel. Permet la vente au chapitre pour les series en cours de publication. |
| **Payant par serie** | Le lecteur achete l'acces a tous les volumes d'une serie (existants et futurs). Tarif reduit par rapport a l'achat individuel. |

### 1.2 Regles de tarification

| Regle | Description |
|-------|-------------|
| **Devise** | Configurable par vendeur. Defaut : EUR. Les prix sont stockes en centimes (RM-05). |
| **Prix minimum** | 1 centime (0,01 €). Pas de prix negatifs. |
| **Gratuite partielle** | Le vendeur peut rendre certains chapitres gratuits au sein d'une oeuvre payante (ex. premier chapitre gratuit en plus des pages de demonstration). |
| **Modification de prix** | Le vendeur peut modifier le prix d'une oeuvre a tout moment. La modification ne s'applique pas retroactivement aux licences deja emises (RM-06). |

---

## 2. Parcours d'achat

### 2.1 Panier

| Fonctionnalite | Description |
|----------------|-------------|
| Ajout au panier | Depuis la fiche oeuvre ou l'ecran de fin de demonstration. Le lecteur peut ajouter des oeuvres, chapitres individuels ou series completes. |
| Contenu du panier | Liste des articles avec titre, type (oeuvre/chapitre/serie), prix unitaire, sous-total. |
| Modification | Suppression d'articles du panier. Le panier est persistant dans la session navigateur. |
| Detection de doublons | Si le lecteur possede deja une licence active pour un article, celui-ci est affiche comme « Deja achete » et ne peut pas etre ajoute. |
| Prix de serie ajuste | Si le lecteur possede deja certains volumes d'une serie et achete la serie complete, le prix est ajuste (deduction des volumes deja achetes). |

### 2.2 Checkout

```
┌─────────────────────────────────────────────┐
│  Recapitulatif de commande                  │
├─────────────────────────────────────────────┤
│  Titre Oeuvre A — Volume 3        4,99 €    │
│  Titre Oeuvre B — Chapitre 12     0,99 €    │
│  Remise « -20% Promo Ete »      -1,20 €    │
├─────────────────────────────────────────────┤
│  Total                            4,78 €    │
├─────────────────────────────────────────────┤
│  Mode de paiement :                         │
│  ○ Carte bancaire                           │
│  ○ Virement                                 │
│  ○ Autre                                    │
├─────────────────────────────────────────────┤
│         ┌─────────────────────┐             │
│         │   Confirmer l'achat  │             │
│         └─────────────────────┘             │
└─────────────────────────────────────────────┘
```

### 2.3 Flux de paiement

```
Lecteur → Panier → Checkout → Selection mode de paiement
  → [Carte bancaire] → Redirection passerelle externe → Callback → Licence generee
  → [Virement] → Instructions affichees → Confirmation manuelle par le vendeur → Licence generee
  → [Autre] → Instructions specifiques au vendeur → Confirmation manuelle → Licence generee
```

### 2.4 Confirmation

Apres validation du paiement :

1. Une **licence d'achat** est generee et stockee sur le COG vendeur.
2. Une **copie de la licence** est transmise au COG acheteur (si authentifie via MWS).
3. Un **recu** est affiche a l'ecran et envoyable par email (si l'email est fourni).
4. Le lecteur obtient immediatement **l'acces complet** a l'oeuvre.

---

## 3. Modes de paiement

### 3.1 Modes supportes

| Mode | Identifiant | Confirmation | Description |
|------|-------------|--------------|-------------|
| **Carte bancaire** | `card` | Automatique | Redirection vers une passerelle de paiement externe configuree par le vendeur (Stripe, PayPal, Mollie, etc.). Le vendeur saisit ses identifiants de passerelle dans la configuration. |
| **Virement bancaire** | `transfer` | Manuelle | Le checkout affiche les coordonnees bancaires du vendeur. Le vendeur confirme manuellement la reception du virement dans le tableau de bord. |
| **Autre** | `other` | Manuelle | Mode personnalise. Le vendeur definit un libelle et des instructions (ex. « PayPal direct », « Especes au prochain salon », « Crypto »). Confirmation manuelle. |

### 3.2 Configuration des passerelles

| Champ | Description |
|-------|-------------|
| `payment_gateway` | Identifiant de la passerelle (`stripe`, `paypal`, `mollie`, `manual`). |
| `gateway_config` | JSON contenant les cles API / identifiants de la passerelle (stocke en niveau de securite 3 — Critical). |
| `webhook_url` | URL de callback pour les confirmations automatiques. Generee automatiquement par JayManga. |

### 3.3 Transactions en attente

Pour les paiements a confirmation manuelle (virement, autre) :

- La transaction est creee avec le statut `pending`.
- La licence n'est **pas** generee immediatement.
- Le vendeur voit la transaction en attente dans son tableau de bord.
- Le vendeur confirme ou rejette manuellement.
- A la confirmation, la licence est generee et le lecteur est notifie.
- Apres 30 jours sans confirmation, la transaction passe en `expired` (configurable).

---

## 4. Licences

### 4.1 Structure d'une licence

Chaque achat genere une licence qui fait office de **preuve d'achat** et de **cle d'acces** au contenu.

| Champ | Description |
|-------|-------------|
| `id` | Identifiant unique de la licence (UUID). |
| `buyer_cog_id` | Identifiant du COG acheteur (ou identite visiteur si non-COG). |
| `buyer_identity` | Niveau d'identite de l'acheteur (LSI, VID, WID). |
| `purchase_type` | `work` / `chapter` / `series`. |
| `target_id` | Identifiant de l'oeuvre, du chapitre ou de la serie achetee. |
| `amount_paid` | Montant paye (centimes). |
| `download_allowed` | Telechargement hors-ligne autorise au moment de l'achat. |
| `status` | `active` / `refunded` / `revoked`. |

### 4.2 Verification de licence

A chaque demande de page payante, la liseuse verifie que le lecteur possede une licence active couvrant le contenu demande :

1. Le lecteur presente son identite (cookie de session, token MWS, ou identite COG).
2. Le COG vendeur verifie l'existence d'une licence active pour cette identite et ce contenu.
3. Si valide → page servie. Si invalide → erreur 403 + ecran d'achat.

### 4.3 Stockage des licences

- **Sur le COG vendeur** : source de verite. Toutes les licences emises sont stockees.
- **Sur le COG acheteur** : copie locale pour affichage dans la bibliotheque et verification hors-ligne (si le contenu est telecharge).

---

## 5. Remboursements

| Aspect | Description |
|--------|-------------|
| **Declenchement** | Le vendeur initie un remboursement depuis le tableau de bord. Le lecteur peut demander un remboursement (la demande est soumise au vendeur). |
| **Remboursement total** | Le montant integral est rembourse. La licence passe en statut `refunded`. L'acces en ligne est revoque. Les fichiers deja telecharges restent sur le COG lecteur (LOI-3, RM-04). |
| **Remboursement partiel** | Un montant partiel est rembourse (ex. remboursement d'un chapitre sur une oeuvre complete). La licence reste active pour le contenu non rembourse. |
| **Delai** | Le vendeur configure un delai maximum de remboursement (defaut : 14 jours apres l'achat). Au-dela, les demandes sont rejetees automatiquement. |
| **Execution** | Pour les paiements par carte, le remboursement est execute via la passerelle. Pour les paiements manuels, le vendeur gere le remboursement en dehors du systeme et marque la licence comme remboursee. |

---

## 6. Promotions et remises

### 6.1 Types de promotions

| Type | Description |
|------|-------------|
| **Remise pourcentage** | Reduction en pourcentage sur le prix d'une oeuvre, d'un chapitre ou d'une serie (ex. -30%). |
| **Remise montant fixe** | Reduction d'un montant fixe (ex. -2,00 €). |
| **Gratuite temporaire** | L'oeuvre est rendue gratuite pour une periode definie. |

### 6.2 Configuration

| Champ | Description |
|-------|-------------|
| `name` | Nom de la promotion (affiche au lecteur). |
| `discount_type` | `percent` / `fixed_amount` / `free`. |
| `discount_value` | Valeur de la remise (pourcentage ou montant en centimes). |
| `target_scope` | `work` / `chapter` / `series` / `catalog` (toutes les oeuvres). |
| `target_ids` | Liste des IDs concernes (si scope = work/chapter/series). |
| `start_date` | Debut de la promotion (ISO 8601). |
| `end_date` | Fin de la promotion (ISO 8601). |
| `active` | Active/desactivee manuellement. |

### 6.3 Application

- Les promotions actives sont affichees sur la fiche oeuvre (prix barre + prix reduit).
- Dans le panier, la remise est appliquee automatiquement.
- Une oeuvre ne peut beneficier que d'une seule promotion a la fois (la plus avantageuse pour le lecteur est appliquee).

---

## 7. Administration des ventes (Central)

### 7.1 Tableau de bord

| Indicateur | Description |
|------------|-------------|
| Revenus du jour / semaine / mois | Montant total des ventes confirmees sur la periode. |
| Nombre de ventes | Nombre de licences generees sur la periode. |
| Panier moyen | Montant moyen par transaction. |
| Top oeuvres vendues | Classement des oeuvres par nombre de ventes. |
| Ventes en attente | Nombre de transactions a confirmer manuellement. |
| Remboursements | Nombre et montant des remboursements sur la periode. |
| Revenus nets | Revenus bruts - remboursements. |

### 7.2 Historique des transactions

| Champ affiche | Description |
|---------------|-------------|
| Date/heure | Horodatage de la transaction. |
| Acheteur | Identite de l'acheteur (COG ID ou identite visiteur). |
| Oeuvre/Chapitre/Serie | Article achete. |
| Montant | Montant paye. |
| Mode de paiement | Carte, virement, autre. |
| Statut | Confirme, en attente, rembourse, expire. |

Filtres disponibles : periode, statut, mode de paiement, oeuvre.

### 7.3 Gestion des licences

| Action | Description |
|--------|-------------|
| Voir les licences | Liste de toutes les licences emises avec statut. |
| Revoquer une licence | En cas de fraude ou de litige. Le lecteur perd l'acces en ligne. |
| Generer manuellement | Le vendeur peut generer une licence gratuite (cadeau, echange, partenariat). |

### 7.4 Export

| Format | Contenu |
|--------|---------|
| CSV | Historique des transactions (date, acheteur, oeuvre, montant, mode, statut). |
| PDF | Rapport de ventes synthetique par periode. |

---

## 8. Securite du module de paiement

| Mesure | Description |
|--------|-------------|
| **Niveau de securite 3 (Critical)** | Les donnees de paiement (montants, modes, cles API) sont classees Critical. |
| **Pas de stockage de donnees carte** | JayManga ne stocke jamais les numeros de carte. Le paiement est delegue a la passerelle externe. |
| **Cles API chiffrees** | Les identifiants de passerelle sont stockes chiffres via KindMother (db-encryption). |
| **Webhook securise** | Les callbacks de passerelle sont verifies par signature (HMAC ou equivalent selon la passerelle). |
| **Anti-fraude basique** | Detection de doublons (meme licence demandee deux fois), limitation du nombre de transactions par minute par acheteur. |

---

## 9. References

| Document | Role |
|----------|------|
| [JayManga - Document Fondateur](./JayManga%20-%20Document%20Fondateur.md) | Document de reference du service. |
| [JayManga - Favoris et Bibliotheque](./JayManga%20-%20Favoris%20et%20Bibliotheque.md) | Gestion des oeuvres achetees cote lecteur. |
| [JayShop - Document Fondateur](../JayShop/JayShop%20-%20Document%20Fondateur.md) | Reference pour les patterns de vente (modele de paiement, tickets). |

---

**Document** : JayManga — Achat et Paiement
**Version** : 1.0
**Date** : 2026-02-24
**Statut** : Specification fonctionnelle detaillee.
