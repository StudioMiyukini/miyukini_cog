# Miyukini Conceptual References — Equivalents Réservation / RDV en Ligne

## Contexte

Ce document est une **analyse concurrentielle** des plateformes de prise de rendez-vous et de réservation en ligne, réalisée pour enrichir la documentation et la roadmap du service **JayRDV**. Il couvre les fonctionnalités, avantages, modèles tarifaires et parcours UX/UI des solutions de référence du marché (2025-2026).

**Objectif :** Identifier les fonctionnalités attendues, les standards UX/UI et les bonnes pratiques de la concurrence pour guider l'implémentation et le positionnement de JayRDV dans l'écosystème COG.

**Références croisées :** [JayRDV - Document Fondateur](../../services/JayRDV/JayRDV%20-%20Document%20Fondateur.md), [JayRDV - Fonctionnalités Solutions Réservation en Ligne](../../services/JayRDV/reference/JayRDV%20-%20Fonctionnalites%20Solutions%20Reservation%20en%20Ligne.md), [Equivalents Boutique CMS Réservation SaaS](./Miyukini%20Conceptual%20References%20-%20Equivalents%20Boutique%20CMS%20Reservation%20SaaS.md).

---

## Portée / Scope

- **Périmètre :** Planity, Doctolib, Treatwell, Booksy, Fresha, Calendly, Cal.com, SimplyBook.me — fonctionnalités, UX/UI, tarifs, avantages.
- **Hors périmètre :** Implémentation technique, contrats d'API, choix d'architecture (référencés dans la doc JayRDV et les crates).
- **Audience :** Équipes produit, architecture, développeurs du service JayRDV.

---

## 1. Planity (France — Beauté / Bien-être)

### 1.1 Positionnement

Leader français de la réservation en ligne pour les **professionnels de la beauté et du bien-être** (coiffeurs, barbiers, instituts, spas). 60 000+ établissements, 15 millions d'utilisateurs.

### 1.2 Fonctionnalités

| Domaine | Fonctionnalités |
|---------|----------------|
| **Réservation** | Réservation 24h/24 7j/7 (web + app mobile) ; agenda connecté synchronisé ; gestion automatique de la liste d'attente ; délais et fréquences configurables par prestation |
| **Rappels** | SMS de rappel automatiques (300/mois inclus) ; réduction des oublis de 75 % |
| **Vitrine** | Page d'établissement personnalisée (photos, infos, avis certifiés) ; référencement Google optimisé ; bouton réservation Instagram |
| **Ventes / Fidélisation** | Boutique en ligne, cartes cadeaux, cures ; campagnes marketing SMS personnalisées ; prépaiement et politique d'annulation configurable |
| **Caisse** | Logiciel de caisse certifié NF525 ; TPE WiFi ; transmission comptable simplifiée |
| **Equipe** | Gestion multi-collaborateurs avec plannings distincts ou partagés |
| **2025-2026** | RDV en visio (janvier 2026) ; photos et documents liés aux prestations ; délais/fréquences par prestation |

### 1.3 Avantages mesurés

| Indicateur | Valeur |
|-----------|--------|
| RDV pris hors horaires d'ouverture | 50 % |
| Fréquence de réservation en ligne vs téléphone | +50 % |
| RDV sauvés grâce aux rappels SMS | +75 % |
| Réduction des appels au salon | 50 % |
| Satisfaction (Trustpilot) | 4,7 / 5 |

### 1.4 Tarification

- **59-80 EUR HT/mois** (selon source), collaborateurs illimités, 0 % de commission sur les RDV.
- SMS rappels : 300/mois inclus, au-delà payant.

### 1.5 Parcours UX/UI (client)

1. **Recherche** : par type de service (coiffeur, barbier, manucure, institut) + localisation.
2. **Page établissement** : photos, prestations avec prix et durée, avis certifiés, horaires.
3. **Sélection prestation** : choix du service, du collaborateur (optionnel), du créneau disponible.
4. **Formulaire** : informations client (nom, téléphone, email) ; possibilité de créer un compte ou de réserver en tant qu'invité.
5. **Confirmation** : récapitulatif (prestation, date, heure, lieu) ; confirmation par email/SMS ; ajout au calendrier.
6. **Rappels** : SMS automatique la veille ou quelques heures avant.
7. **Annulation / modification** : via l'espace client ou le lien dans le SMS/email ; soumis à la politique d'annulation du pro.

### 1.6 Parcours UX/UI (professionnel)

1. **Inscription** : création du compte, saisie des infos établissement, prestations, horaires.
2. **Agenda** : vue jour/semaine/mois ; drag & drop ; filtrage par collaborateur ; blocage de créneaux.
3. **Gestion prestations** : CRUD services (nom, durée, prix, photo, document).
4. **Plannings** : horaires récurrents par collaborateur, exceptions (congés, pauses).
5. **Notifications** : configuration des rappels SMS/email, modèles personnalisables.
6. **Statistiques** : tableau de bord (RDV du jour, semaine, taux de remplissage, no-show).
7. **Intégrations** : bouton Instagram, widget site web, lien de réservation partageable.
8. **Caisse** : encaissement, historique, export comptable.

---

## 2. Doctolib (France — Santé)

### 2.1 Positionnement

Leader européen de la prise de rendez-vous médicaux. Modèle de référence UX pour la recherche de praticien + réservation instantanée. 80 millions de patients en Europe.

### 2.2 Fonctionnalités

| Domaine | Fonctionnalités |
|---------|----------------|
| **Réservation** | Recherche par spécialité + lieu + type de consultation (cabinet/vidéo) ; réservation de créneau verrouillé pendant 15 min avant validation ; agenda en ligne par praticien |
| **Rappels** | Confirmation email immédiate ; rappels programmés à J-7, J-1 et H-2 ; notifications personnalisables par le patient |
| **Annulation** | Annulation / déplacement en ligne en quelques clics ; délais imposés par le praticien |
| **Téléconsultation** | RDV en visio intégré (79 EUR TTC/mois/praticien) |
| **Fiche patient** | Historique des RDV, informations personnelles, suivi médical continu |
| **Praticien** | Agenda multi-praticien, gestion d'équipe, gestion de l'activité par spécialité |

### 2.3 Avantages

- UX de référence : parcours en 3 clics (chercher → choisir créneau → confirmer).
- Rappels multi-niveaux (J-7, J-1, H-2) : réduction majeure des no-show.
- Verrouillage de créneau pendant la réservation (hold 15 min) : évite les conflits.
- App mobile excellente (4,5/5, 10M+ téléchargements).

### 2.4 Tarification

- **79 EUR TTC/mois/praticien** (abonnement, 0 % de commission).

### 2.5 Parcours UX/UI (patient)

1. **Recherche** : barre de recherche (spécialité ou nom + ville) ; filtres (type de consultation, disponibilité prochaine, langues).
2. **Résultats** : carte + liste ; prochaine disponibilité affichée directement ; avis patients.
3. **Profil praticien** : photo, présentation, diplômes, tarifs, adresse, moyens de paiement.
4. **Sélection créneau** : calendrier avec créneaux disponibles (vert/gris) ; sélection du motif de consultation ; créneau verrouillé 15 min.
5. **Identification** : connexion ou création de compte (email/téléphone) ; complétion profil si nouveau.
6. **Confirmation** : récapitulatif ; validation ; email de confirmation immédiat.
7. **Rappels** : J-7 + J-1 + H-2.
8. **Annulation** : depuis l'espace « Mes RDV » ; possibilité de reprogrammer immédiatement.

---

## 3. Treatwell (Europe — Beauté / Bien-être)

### 3.1 Positionnement

Leader européen avec 150 000+ salons en Europe. Marketplace + logiciel de gestion intégré.

### 3.2 Fonctionnalités

| Domaine | Fonctionnalités |
|---------|----------------|
| **Réservation** | 24/7 sur tous les canaux (Google, site web, réseaux sociaux, app) ; agenda intelligent multi-salons |
| **Rappels** | Rappels illimités (SMS/email) pour réduire les no-show |
| **Marketing** | Campagnes email gratuites ; offres d'heures creuses et dernière minute |
| **Équipe** | Gestion des emplois du temps par collaborateur ; accès historique et préférences clients |
| **Paiement** | Paiement intégré ; gestion du stock |
| **Multi-établissement** | Gestion de plusieurs salons depuis un tableau de bord |

### 3.3 Tarification

- **20-60 EUR HT/mois** + 25 % de commission sur les premiers RDV de nouveaux clients (via marketplace Treatwell uniquement ; gratuit via widget/bouton propre du salon).

### 3.4 Avantage clé

- **Marketplace** : les clients trouvent des salons directement sur Treatwell → acquisition de nouveaux clients pour le pro (modèle commission sur acquisition).
- Offres « dernière minute » et « heures creuses » pour remplir les créneaux vides.

---

## 4. Booksy (Mondial — Beauté)

### 4.1 Positionnement

Leader mondial avec 200 000+ professionnels, moins visible en France mais en forte croissance. App mobile très bien notée.

### 4.2 Fonctionnalités

| Domaine | Fonctionnalités |
|---------|----------------|
| **Réservation** | App mobile fluide et intuitive ; paiement en ligne possible |
| **Fidélisation** | Programmes de fidélité intégrés ; promotions, cartes cadeaux, cartes de fidélité |
| **Marketing** | Campagnes email gratuites ; outil de publication sur réseaux sociaux |
| **Gestion** | Gestion des collaborateurs ; annulation/modification en 2 clics |

### 4.3 Avantage clé

- **UX mobile-first** : l'application est le point d'entrée principal, très fluide.
- Annulation/modification en 2 clics (benchmark pour la simplicité).
- Outils de publication sur réseaux sociaux (photos, stories) intégrés.

---

## 5. Fresha (Mondial — Beauté, modèle gratuit)

### 5.1 Positionnement

Plateforme **gratuite** (fonctionnalités de base) pour les salons de beauté. Monétisation sur les services optionnels (paiements, marketing avancé).

### 5.2 Fonctionnalités gratuites

| Domaine | Inclus gratuitement |
|---------|---------------------|
| **Calendrier** | Planification des rendez-vous |
| **Clients** | Gestion de la clientèle avec formulaires |
| **Notifications** | Email gratuites ; 20 SMS/mois gratuits |
| **Réservation sociale** | Facebook, Instagram, Google intégrés |
| **Marketplace** | Référencement sur la marketplace Fresha |
| **Analytics** | Google Analytics + pixel Meta |
| **App pro** | Application mobile professionnelle gratuite |

### 5.3 Fonctionnalités payantes

- Terminal de paiement par carte ; Tap to Pay (iPhone/Android) ; paiement par QR code.
- Paiements anticipés et politiques d'annulation (réduction des no-show de 89 %).
- Marketing avancé (campagnes, fidélisation).

### 5.4 Avantage clé

- **Gratuit** pour les fonctionnalités de base : zéro barrière à l'entrée.
- Modèle qui convient aux indépendants et petits salons.
- Réservation via réseaux sociaux intégrée nativement.

---

## 6. Calendly (Mondial — B2B, généraliste)

### 6.1 Positionnement

Référence mondiale pour la prise de rendez-vous B2B (freelances, consultants, équipes commerciales). 20M+ utilisateurs.

### 6.2 Fonctionnalités

| Domaine | Fonctionnalités |
|---------|----------------|
| **Types d'événements** | One-on-one, Group, Collective (multi-hôtes), Round Robin (distribution automatique) |
| **Workflows** | Automatisation pré/post-réunion : rappels, demandes de reprogrammation, suivis |
| **Administration** | Tableau de bord centralisé ; gestion d'utilisateurs et de groupes ; modèles standardisés et verrouillables |
| **Intégrations** | Zoom, Google Meet, Microsoft Teams, Salesforce, HubSpot, Stripe, webhooks |
| **Personnalisation** | Branding cohérent, liens de réservation par type d'événement, sondages de réunion |
| **Liens** | Liens directs, à usage unique, pages de réservation personnalisées |

### 6.3 Parcours UX/UI

1. **Lien partagé** : le pro envoie son lien Calendly (email, site, signature).
2. **Sélection** : le client choisit un type d'événement (ex. « Appel découverte 30 min »).
3. **Calendrier** : seuls les créneaux disponibles apparaissent (synchro Google/Outlook/iCal).
4. **Formulaire** : nom, email, questions personnalisées (champs configurables).
5. **Confirmation** : ajout automatique aux calendriers des deux parties ; email de confirmation.
6. **Rappels** : workflows personnalisables (rappel J-1, suivi post-réunion).
7. **Reprogrammation / annulation** : lien dans l'email de confirmation ; en 1 clic.

### 6.4 Avantage clé

- **Simplicité absolue** : un lien, un clic, un créneau.
- Workflows automatisés (pré/post-réunion) sans intervention manuelle.
- Round Robin pour les équipes (distribution automatique et équitable).

---

## 7. Cal.com (Open source — Self-hosted)

### 7.1 Positionnement

Alternative **open source** à Calendly. Infrastructure de planification ouverte, self-hostable, API-first. Pertinent pour JayRDV car le COG est un environnement souverain.

### 7.2 Fonctionnalités

| Domaine | Fonctionnalités |
|---------|----------------|
| **Événements** | Collectifs, récurrents, instantanés, liens de groupe dynamiques |
| **API** | API publique v2 ; **Cal Atoms** (composants de réservation pixel par pixel) ; webhooks |
| **Paiements** | Acceptation de paiements pour les réservations |
| **Gestion** | Congés (« out of office ») ; multi-calendrier ; multi-utilisateur |
| **Self-hosted** | Déploiement sur infrastructure propre ; données souveraines |
| **SDK** | Kit de démarrage pour créer sa propre app de planification |

### 7.3 Avantage clé

- **Souveraineté des données** : self-hosted, open source, pas de dépendance à un tiers.
- **API-first** : Cal Atoms permet de construire une expérience de réservation intégrée dans n'importe quelle interface.
- Philosophie très alignée avec le modèle COG (gouvernance locale, pas de cloud imposé).

---

## 8. SimplyBook.me (International — Multi-secteur)

### 8.1 Positionnement

Solution internationale multi-secteur (beauté, santé, coaching, services). Alternative économique à Planity.

### 8.2 Fonctionnalités

| Domaine | Fonctionnalités |
|---------|----------------|
| **Réservation** | Réservation en ligne 24/7 ; mini-site dédié personnalisable ; réservation vocale (IA) |
| **Calendrier** | Synchronisation bidirectionnelle (Google, Outlook, Apple iCal) ; multi-ressources |
| **Notifications** | Confirmation + rappels automatiques (email/SMS) |
| **Intégrations** | Widget intégrable ; API REST ; plugin WordPress |
| **Marketing** | Coupons, cartes cadeaux, programmes fidélité |
| **Multi-secteur** | Templates par secteur (beauté, médical, fitness, éducation) |

### 8.3 Tarification

- **Gratuit** (15 réservations/mois) ; plans à partir de **8,50 EUR/mois** pour 5 collaborateurs.
- Sync Google/iCal incluse dès le gratuit (à la différence de Planity).

### 8.4 Avantage clé

- **Prix** : très compétitif, version gratuite fonctionnelle.
- **Réservation vocale** (IA) : innovation UX différenciante.
- Sync bidirectionnelle calendriers incluse dès le gratuit.

---

## 9. Synthèse comparative

### 9.1 Tableau fonctionnel

| Fonctionnalité | Planity | Doctolib | Treatwell | Booksy | Fresha | Calendly | Cal.com | SimplyBook |
|----------------|:-------:|:--------:|:---------:|:------:|:------:|:--------:|:-------:|:----------:|
| Réservation 24/7 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Sync Google/Outlook/iCal | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Rappels SMS | ✅ | ✅ | ✅ | ✅ | ✅* | ✅ | ✅ | ✅ |
| Hold / verrouillage créneau | ? | ✅ (15 min) | ? | ? | ? | ❌ | ❌ | ❌ |
| Paiement intégré | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Marketplace (acquisition) | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ |
| Widget / embed | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Multi-collaborateur | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Multi-établissement | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ |
| Logiciel de caisse | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ✅ |
| Open source / self-hosted | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ |
| Téléconsultation / visio | ✅ | ✅ | ❌ | ❌ | ❌ | ✅ | ✅ | ✅ |
| Round Robin | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ✅ | ❌ |
| Réservation vocale (IA) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| Gratuit (plan de base) | ❌ | ❌ | ❌ | ❌ | ✅ | ✅* | ✅ | ✅ |

\* Fresha : 20 SMS/mois gratuits. Calendly : plan gratuit limité (1 type d'événement).

### 9.2 Tarifs résumés

| Plateforme | Tarif de base | Commission |
|-----------|--------------|-----------|
| **Planity** | 59-80 EUR/mois | 0 % |
| **Doctolib** | 79 EUR/mois/praticien | 0 % |
| **Treatwell** | 20-60 EUR/mois | 25 % (nouveaux clients marketplace) |
| **Booksy** | 30-50 EUR/mois | 0 % |
| **Fresha** | Gratuit (base) | Options payantes |
| **Calendly** | Gratuit (1 event) — 10-16 USD/mois | 0 % |
| **Cal.com** | Gratuit (self-hosted) | 0 % |
| **SimplyBook** | Gratuit (15 RDV/mois) — 8,50 EUR/mois | 0 % |

---

## 10. Patterns UX/UI de référence pour JayRDV

### 10.1 Parcours client universel (best practices)

```
1. Recherche / Lien direct
   └── Recherche par service + lieu (Planity, Doctolib, Treatwell)
   └── Lien direct partagé par le pro (Calendly, Cal.com)

2. Page vitrine du professionnel
   └── Photos, services, prix, durée, avis (Planity, Doctolib)
   └── Description, diplômes, tarifs, adresse (Doctolib)

3. Sélection du service / prestation
   └── Liste catégorisée (Planity : coupe, couleur, soin…)
   └── Type d'événement (Calendly : 30 min, 60 min…)

4. Sélection du créneau
   └── Calendrier avec seules dispos (toutes plateformes)
   └── Choix du collaborateur (Planity, Treatwell, Booksy)
   └── Hold temporaire (Doctolib : 15 min) ← recommandé

5. Formulaire client
   └── Minimal : nom, email, téléphone (toutes)
   └── Optionnel : infos supplémentaires, motif (Doctolib, SimplyBook)
   └── Guest possible (Planity, Fresha) ← recommandé

6. Prépaiement / Politique annulation
   └── Optionnel : acompte, empreinte CB (Planity, Fresha)
   └── Politique d'annulation configurable par le pro

7. Confirmation
   └── Récapitulatif complet (prestation, date, heure, lieu, pro)
   └── Email + SMS de confirmation
   └── Ajout automatique au calendrier (Google, Outlook, iCal)

8. Rappels
   └── J-7 + J-1 + H-2 (Doctolib) — référence
   └── J-1 ou J même (Planity, Booksy)
   └── Workflows personnalisables (Calendly)

9. Modification / Annulation
   └── Depuis l'espace client ou lien SMS/email
   └── En 2 clics max (Booksy ← benchmark)
   └── Possibilité de reprogrammer immédiatement (Doctolib)
```

### 10.2 Parcours professionnel universel (best practices)

```
1. Onboarding
   └── Inscription, infos établissement, services, horaires
   └── Import éventuel de clients existants (Treatwell, Fresha)

2. Agenda
   └── Vue jour / semaine / mois (toutes)
   └── Drag & drop (Planity) ← UX valorisée
   └── Filtrage par collaborateur
   └── Code couleur par type de prestation

3. Gestion des services
   └── CRUD (nom, durée, prix, description, photo)
   └── Catégories / regroupements

4. Plannings
   └── Horaires récurrents par collaborateur
   └── Exceptions (congés, pauses, jours fériés)
   └── Temps tampon entre RDV (Calendly, SmartAgenda)

5. Notifications
   └── Configuration canaux (SMS, email) et timing
   └── Modèles personnalisables (Planity, Calendly)

6. Lien de réservation / Widget
   └── Génération URL unique (Calendly, Cal.com)
   └── Widget intégrable (Planity, Treatwell, SimplyBook)
   └── Bouton réseaux sociaux (Instagram, Facebook)

7. Statistiques
   └── RDV par période, taux de remplissage, no-show (Planity, Treatwell)
   └── Export CSV/PDF (Calendly, Cal.com)

8. Équipe
   └── Invitation, rôles, établissements (Planity, Calendly)
   └── Round Robin (Calendly, Cal.com) ← pour distribution équitable
```

---

## 11. Enseignements pour JayRDV

### 11.1 Fonctionnalités prioritaires (P0)

| Fonctionnalité | Justification | Référence concurrence |
|----------------|--------------|----------------------|
| **Réservation 24/7** | Standard marché, attendu par 75 % des consommateurs | Toutes |
| **Rappels multi-niveaux** (J-7, J-1, H-2) | Réduction no-show de 75-89 % | Doctolib, Planity, Fresha |
| **Verrouillage temporaire de créneau** (hold) | Évite les conflits de réservation | Doctolib (15 min) |
| **Réservation guest** (sans compte) | Barrière à l'entrée minimale | Planity, Fresha, Calendly |
| **Annulation en 2 clics** | UX de référence | Booksy |
| **Confirmation + ajout calendrier** | Standard attendu | Toutes |

### 11.2 Fonctionnalités différenciantes (P1-P2)

| Fonctionnalité | Valeur | Référence |
|----------------|--------|-----------|
| **Souveraineté des données** (COG, self-hosted) | Pas de cloud imposé, gouvernance locale | Cal.com (approche) ; unique à Miyukini (Cores) |
| **Round Robin** (distribution équitable) | Productivité d'équipe | Calendly, Cal.com |
| **Offres heures creuses / dernière minute** | Remplissage des créneaux vides | Treatwell |
| **Réservation vocale (IA)** | Innovation UX | SimplyBook |
| **Widget pixel par pixel** (composants) | Intégration totale dans le site du pro | Cal.com (Cal Atoms) |

### 11.3 Lien avec l'écosystème JayRDV / JayXpose / JayKoa

- **JayXpose** (vitrine) remplace la « page établissement » de Planity/Doctolib → alimente JayRDV avec les infos du pro et les services proposés.
- **JayKoa** (agenda) remplace la sync Google/Outlook → agenda unifié COG avec reflets des RDV confirmés.
- **JayRDV** orchestre le flux de réservation, le CRUD des créneaux, les annulations et rappels (MiyuNotify).
- Le modèle COG (StrongFather, KindMother, WorrySentinel) apporte la **gouvernance souveraine** que seul Cal.com approche partiellement (open source, self-hosted).

---

*Document de référence concurrentielle — enrichissement JayRDV. Dernière mise à jour : 2026-02-16.*
