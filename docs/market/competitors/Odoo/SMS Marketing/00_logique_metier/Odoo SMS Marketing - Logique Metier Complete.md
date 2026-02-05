# Odoo SMS Marketing — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **SMS Marketing** d'Odoo (version 19.0), extraite de la documentation officielle et du cadre fonctionnel. Il identifie les modèles de données, règles métier, workflows et mécanismes pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 (applications/marketing/sms_marketing), module `sms`, `mass_mailing_sms` / `sms_marketing`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (mailing.mailing, mailing.list, mailing.contact, blacklist, sms.sms)
- Règles métier et contraintes (destinataires, opt-out, crédits)
- Workflows et états des envois (Draft, In Queue, Sending, Sent)
- Campagnes et A/B tests
- Listes de diffusion et blacklist
- Traçabilité et analyse (link tracker, reporting)

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `mailing.mailing` (Envoi SMS / Mailing)

**Rôle :** Représente un envoi SMS marketing (campagne ou mailing unique).

**Champs clés (déduits de la doc) :**
- `subject` : Sujet / libellé du mailing
- `recipients` : Type de destinataires (Mailing List, Contact, ou filtre personnalisé)
- `mailing_list_id` : Liste de diffusion (si Recipients = Mailing List)
- `body_plaintext` / contenu SMS : Texte du message (caractères, segments SMS)
- `state` : État (draft, in_queue, sending, sent)
- `schedule_date` : Date/heure d'envoi planifié
- `sent_date` : Date d'envoi effectif
- `opt_out_link` : Lien de désinscription (optionnel)
- `responsible_id` : Responsable (employé)
- `campaign_id` : Campagne marketing (si Mailing Campaigns activé)
- `ab_testing_*` : Champs A/B test (pourcentage, critère gagnant, date envoi final)

**Types de destinataires (Recipients) :**
- **Mailing List** : une ou plusieurs listes de diffusion
- **Contact** : tous les contacts (res.partner) avec filtres optionnels (pays, blacklist, etc.)
- Filtres domaine personnalisés (ex. Country = United States, Blacklist = not set)

**Règles métier :**
- Un mailing ne peut être envoyé sans crédits SMS Odoo (ou passerelle Twilio selon pays)
- Le nombre de segments SMS dépend du nombre de caractères (norme GSM / Unicode)
- Liens dans le message génèrent des link trackers pour l’analyse
- Option « Include opt-out link » pour désinscription

### 1.2 Modèle `mailing.list` (Liste de diffusion)

**Rôle :** Regroupe des contacts pour les envois ciblés.

**Champs clés :**
- `name` : Nom de la liste
- `is_public` : Accessible sur la page de gestion des abonnements (opt-in/opt-out)
- `contact_ids` : One2many vers mailing.contact
- Statistiques : nombre de contacts, mailings, destinataires (smart buttons)

**Règles métier :**
- Une liste peut être publique (gestion des préférences par le destinataire) ou privée
- Les contacts d’une liste peuvent être synchronisés avec des partenaires (res.partner)

### 1.3 Modèle `mailing.contact` (Contact liste de diffusion)

**Rôle :** Contact dans une ou plusieurs listes de diffusion (email, téléphone).

**Champs clés :**
- `list_ids` : Listes auxquelles le contact appartient
- `email` : Email (pour email marketing)
- `mobile` / `phone` : Numéro pour SMS
- `partner_id` : Lien optionnel vers res.partner
- Blacklist : exclusion des envois si numéro blacklisté

**Règles métier :**
- Par défaut, les vues excluent les numéros blacklistés (filtre « Exclude Blacklisted Phone »)
- Historique des mailings envoyés visible dans le Chatter du contact (app Contacts)

### 1.4 Blacklist (Numéros blacklistés)

**Rôle :** Liste des numéros ayant demandé à ne plus recevoir de SMS (opt-out).

**Caractéristiques :**
- Accès : SMS Marketing ‣ Configuration ‣ Blacklisted Phone Numbers
- Champs : numéro, actif (oui/non)
- Ajout manuel ou automatique (clic sur « Unsubscribe » depuis la page de gestion des abonnements)
- Import possible (migration depuis une autre plateforme)
- Action « Unblacklist » pour retirer un numéro

**Règles métier :**
- Aucun SMS ne doit être envoyé à un numéro blacklisté actif
- La désinscription depuis le lien opt-out ajoute automatiquement le numéro à la blacklist

### 1.5 Modèle `sms.sms` (SMS individuel — module sms)

**Rôle :** Représente un SMS technique envoyé ou à envoyer (couche bas niveau).

**Champs typiques (module sms) :**
- `number` : Numéro destinataire
- `body` : Corps du message
- `state` : état d’envoi (outgoing, sent, etc.)
- `partner_id` : Partenaire lié
- Lien avec mailing pour traçabilité

**Règles métier :**
- Envoi via IAP Odoo (crédits) ou Twilio selon configuration
- Validation du numéro (phone_validation) avant envoi

---

## 2. États et Workflows

### 2.1 États d’un SMS Mailing

| État        | Description                          |
|------------|--------------------------------------|
| **Draft**  | Brouillon, modifiable, non envoyé    |
| **In Queue** | En file d’attente pour envoi         |
| **Sending** | En cours d’envoi                     |
| **Sent**   | Envoi terminé                        |

### 2.2 Workflow d’envoi

1. **Création** : Création du mailing (sujet, destinataires, contenu SMS).
2. **Configuration** : Choix des destinataires (liste, contacts, filtres), réglages (opt-out, responsable), éventuellement planification.
3. **Envoi** :
   - **Send** : envoi immédiat (file d’attente puis envoi).
   - **Schedule** : planification à une date/heure ; passage automatique en file puis envoi à l’heure dite.
   - **Test** : envoi à un ou plusieurs numéros de test (séparés par des virgules).
4. **Transition** : Draft → In Queue → Sending → Sent.
5. **Traçabilité** : Lien avec link tracker pour les clics, reporting dans Reporting.

### 2.3 Campagnes (Mailing Campaigns)

- Activées dans Email Marketing ‣ Configuration ‣ Paramètres (« Mailing Campaigns »).
- Menu **Campaigns** dans SMS Marketing : agrégation de mailings (SMS, emails, réseaux sociaux, push) par campagne.
- Chaque campagne a : nom, responsable, tags, smart buttons (Engagement, Opportunities, etc.).
- Onglets par type d’envoi (Send SMS, Send New Mailing, etc.) pour créer et suivre les mailings de la campagne.

### 2.4 A/B Tests

- Onglet **A/B Test** sur le formulaire de mailing (si Mailing Campaigns activé).
- Options : « Allow A/B Testing », pourcentage de destinataires pour le test, **Winner Selection** (Manual, Highest Click Rate, Leads, Quotations, Revenues), **Send Final On** (date/heure limite).
- Odoo envoie les variantes à un sous-ensemble, mesure selon le critère choisi, puis envoie la variante gagnante au reste des destinataires à la date « Send Final On ».

---

## 3. Règles Métier Critiques

### 3.1 Destinataires

- **Liste de diffusion** : envoi à tous les contacts des listes sélectionnés (hors blacklist).
- **Contact** : envoi à tous les contacts (res.partner) respectant le domaine (ex. pays, non blacklistés) ; filtres personnalisables.
- **Règle** : aucun envoi vers un numéro présent sur la blacklist (actif).

### 3.2 Contenu et Segments SMS

- Limite de caractères par segment (norme GSM vs Unicode) ; affichage du nombre de caractères et du nombre de segments dans l’interface.
- Liens : génération automatique de link trackers (Configuration ‣ Link Tracker) pour les métriques de clics.
- Emojis et liens autorisés dans le contenu.

### 3.3 Crédits et Passerelles

- Envoi via **IAP Odoo** (crédits achetés) ; sans crédits, les SMS ne partent pas.
- **Twilio** : alternative dans les pays à réglementation stricte ; configuration dédiée.
- Tarification : dépend du pays et du fournisseur (ex. doc : à partir d’environ 0,01 €/SMS).

### 3.4 Conformité et Opt-out

- **Opt-out** : lien de désinscription optionnel dans le message ; clic → ajout à la blacklist.
- **Listes publiques** : destinataires peuvent gérer leurs abonnements (Subscription Management page).
- **Blacklist** : respect obligatoire ; import de blacklists existantes possible (migration).

---

## 4. Analyse et Traçabilité

### 4.1 Link Tracker

- Liens insérés dans les SMS sont raccourcis et tracés (Configuration ‣ Link Tracker).
- Données collectées : clics par lien, par mailing, pour rapports et A/B test (Highest Click Rate).

### 4.2 Reporting (SMS Analysis)

- Menu **Reporting** : vues Graph, List, Cohort avec filtres et mesures configurables.
- Métriques typiques : nombre d’envois, taux d’ouverture/engagement, clics, conversions (leads, devis, revenus) selon configuration.

### 4.3 Historique dans le Chatter

- Chaque envoi (mailing) est enregistré dans le Chatter du contact (fiche partenaire / contact) pour historique des communications.

---

## 5. Points d’Attention pour Miyukini

- **Gouvernance** : envoi de SMS = décision (StrongFather) et persistance des consentements / blacklist (KindMother).
- **Sécurité** : données personnelles (numéros, préférences) → niveau de sécurité adapté (WorrySentinel), conformité RGPD/opt-out.
- **Opérateurs** : séparer Opérateur « Envoi SMS », « Listes / Contacts », « Blacklist », « Reporting », avec Mandats et Contrats d’équipe clairs.
- **Audit** : traçabilité des envois et désinscriptions pour preuve de conformité.
- **Capacités** : Master Butler pour déclarer les capacités (sms.send, list.manage, blacklist.manage, report.view).

---

**Document** : Odoo SMS Marketing — Logique Métier Complète  
**Version** : 1.0  
**Date** : 2026-02-01
