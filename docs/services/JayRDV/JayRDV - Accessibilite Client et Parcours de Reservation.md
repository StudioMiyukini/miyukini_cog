# JayRDV — Accessibilité Client et Parcours de Réservation

## Contexte

Ce document conceptualise **comment un client final peut réserver un créneau avec un professionnel utilisant JayRDV**, sans avoir lui-même à installer un COG. Il explore les différentes stratégies d'accessibilité et leurs implications architecturales.

## Portée / Scope

- **Applicable à :** Conception produit JayRDV, stratégie d'accessibilité Services COG
- **Couvre :** Parcours client, interfaces d'accès, architecture web/mobile, gouvernance
- **Ne couvre pas :** Implémentation technique détaillée

---

## 1. Le problème fondamental

### 1.1 Énoncé du problème

**Situation :**
- Marie est kinésithérapeute. Elle utilise JayRDV dans son COG Miyukini pour gérer son agenda.
- Paul est un patient. Il veut prendre rendez-vous avec Marie.

**Question critique :**
> **Comment Paul peut-il réserver un créneau sans installer un COG Miyukini ?**

### 1.2 Contraintes

| Contrainte | Description |
|------------|-------------|
| **Facilité client** | Paul doit pouvoir réserver en 2-3 clics, depuis son smartphone, sans inscription complexe |
| **Souveraineté du COG** | Le COG de Marie reste souverain (BorderGuard, StrongFather, KindMother) |
| **Pas de dépendance cloud** | Marie doit pouvoir fonctionner offline ; la réservation doit être possible même si le COG est temporairement hors ligne |
| **Sécurité** | Pas de spam, pas de fausses réservations, pas d'accès non autorisé |
| **Scalabilité** | Solution viable pour 1 professionnel comme pour 1000 professionnels |

---

## 2. Stratégies d'accessibilité — Vue d'ensemble

### 2.1 Les 4 stratégies possibles

| Stratégie | Client voit | Professionnel utilise | Complexité | Offline | Souveraineté |
|-----------|-------------|----------------------|------------|---------|--------------|
| **1. Interface Web publique** | Site web classique | COG Miyukini (JayRDV) | ⭐ Faible | ⚠️ Partielle | ✅ Forte |
| **2. App mobile légère (Progressive Web App)** | PWA smartphone | COG Miyukini (JayRDV) | ⭐⭐ Moyenne | ⚠️ Partielle | ✅ Forte |
| **3. COG Android/iOS léger (client invité)** | Mini COG sur téléphone | COG Miyukini (JayRDV) | ⭐⭐⭐ Élevée | ✅ Complète | ✅ Complète |
| **4. Fédération Inter-COG (client a son propre COG)** | COG complet | COG Miyukini (JayRDV) | ⭐⭐⭐⭐ Très élevée | ✅ Complète | ✅ Complète |

### 2.2 Stratégie recommandée par phase

**Phase 1 — MVP (Time-to-Market rapide) :**
- ✅ **Stratégie 1 : Interface Web publique**
- Pourquoi : Facilité client maximale, développement rapide, validation marché

**Phase 2 — Expérience mobile (6-12 mois après MVP) :**
- ✅ **Stratégie 2 : Progressive Web App**
- Pourquoi : Expérience mobile native, offline partiel, pas de store

**Phase 3 — Souveraineté client (18-24 mois) :**
- ✅ **Stratégie 3 : COG Android/iOS léger**
- Pourquoi : Offline complet, souveraineté client, gouvernance bout-en-bout

**Phase 4 — Fédération (long terme) :**
- ✅ **Stratégie 4 : Fédération Inter-COG**
- Pourquoi : Vision finale, réseau de COG fédérés

---

## 3. Stratégie 1 — Interface Web publique (MVP)

### 3.1 Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Client (Paul) — Navigateur web                              │
│  https://marie-kine.jaykoa.com                               │
└───────────────────┬─────────────────────────────────────────┘
                    │ HTTPS
                    ▼
┌─────────────────────────────────────────────────────────────┐
│  MiyuWeb (Toolkit HTTP/WebSocket) — Strate 6                │
│  Serveur HTTP/WebSocket dans le COG de Marie                │
└───────────────────┬─────────────────────────────────────────┘
                    │ BondingBrother
                    ▼
┌─────────────────────────────────────────────────────────────┐
│  BorderGuard — Strate 4                                      │
│  Filtre les requêtes publiques, applique niveaux S1-S2      │
└───────────────────┬─────────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────────────────────────┐
│  StrongFather → KindMother → JayRDV                          │
│  Décision, persistance, logique réservation                 │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 Parcours client

**Étape 1 : Découverte**
- Paul reçoit un SMS/email de Marie : "Prenez RDV sur https://marie-kine.jaykoa.com"
- Ou Paul scanne un QR code dans la salle d'attente
- Ou Paul trouve le lien sur Google Maps / réseaux sociaux

**Étape 2 : Accès au calendrier**
- Paul ouvre le lien dans son navigateur (mobile ou desktop)
- Pas de compte requis (pour consultation des créneaux disponibles)
- Interface web légère, responsive, rapide

**Étape 3 : Sélection créneau**
- Paul voit les créneaux disponibles (vue semaine ou mois)
- Créneaux affichés selon disponibilités réelles de JayRDV (via MiyuWeb → JayRDV)
- Paul clique sur un créneau (ex : "Lundi 10/02 à 14h30")

**Étape 4 : Identification minimale**
- Formulaire simple :
  - Nom : "Paul Dupont"
  - Téléphone : "06 12 34 56 78" (pour confirmation SMS)
  - Email : "paul@example.com" (optionnel, pour rappel email)
  - Raison consultation : "Douleur épaule" (optionnel)

**Étape 5 : Confirmation**
- Paul clique "Confirmer le rendez-vous"
- Requête envoyée à MiyuWeb → BorderGuard → StrongFather → JayRDV
- **StrongFather évalue** : créneau disponible ? spam ? doublon ?
- **KindMother persiste** : réservation enregistrée
- **MiyuNotify envoie SMS** : "RDV confirmé le 10/02 à 14h30 avec Marie"

**Étape 6 : Rappel et gestion**
- J-1 : SMS/email de rappel automatique
- J-0 : Paul peut annuler via lien unique dans le SMS
- Après RDV : SMS de feedback (optionnel)

### 3.3 Fonctionnement offline du COG

**Problème :** Le COG de Marie est sur un mini PC dans son cabinet. Que se passe-t-il si Paul essaie de réserver quand le COG est hors ligne ?

**Solutions :**

#### Option A : Buffer de réservations (recommandé)

```
┌─────────────────────────────────────────────────────────────┐
│  DNS dynamique + reverse proxy (Cloudflare Tunnel, Ngrok)   │
│  Redirige vers le COG si online, sinon vers buffer          │
└───────────────────┬─────────────────────────────────────────┘
                    │
                    ▼
         ┌──────────┴──────────┐
         │                     │
   [COG Online]          [COG Offline]
         │                     │
         ▼                     ▼
  Réservation          Buffer Redis/SQLite
  immédiate            (CloudFlare Workers)
         │                     │
         │                     ▼
         │              Sync quand COG revient
         └──────────┬──────────┘
                    ▼
              KindMother persiste
```

**Comportement :**
1. Paul fait une demande de réservation
2. Si COG online : réservation immédiate, SMS confirmé
3. Si COG offline : demande mise en buffer, SMS "demande reçue, confirmation dans quelques heures"
4. Quand COG revient online : synchronisation buffer → JayRDV, SMS confirmation ou conflit

#### Option B : Calendrier en lecture seule offline

- Le COG publie périodiquement (toutes les heures, via cron) un snapshot JSON des créneaux disponibles sur CDN
- Client voit les créneaux disponibles (lecture seule)
- Réservation impossible si COG offline → message "Marie n'est pas disponible, réessayez plus tard"
- Moins bon UX mais plus simple

### 3.4 Gouvernance et sécurité

**BorderGuard applique des règles strictes :**

| Menace | Protection BorderGuard |
|--------|------------------------|
| **Spam** | Rate limiting : max 3 réservations/IP/jour |
| **Fausses réservations** | Validation téléphone (SMS OTP) avant confirmation |
| **Attaque DDoS** | Cloudflare devant le COG, filtrage IP |
| **Scraping** | Créneaux disponibles visibles uniquement après identification |
| **Concurrence (double booking)** | StrongFather évalue + lock optimiste : si créneau pris entre temps, refus avec suggestion alternative |

**Niveaux de sécurité appliqués :**
- **S1** (Observation) : Consultation créneaux disponibles (pas de compte)
- **S2** (Interaction contrôlée) : Réservation avec identification minimale (nom + téléphone)
- **S3+** : Réservé au professionnel (Marie) dans son COG

### 3.5 Stack technique

**Frontend (client web) :**
- HTML/CSS/JS vanilla ou framework léger (Svelte, Preact)
- Responsive mobile-first
- Pas de dépendance lourde (React/Vue overkill pour ce cas)

**Backend (COG de Marie) :**
- **MiyuWeb** : Serveur HTTP/WebSocket (Rust + Axum ou Actix-web)
- **JayRDV** : Logique métier réservation (crate Rust)
- **BorderGuard** : Filtre requêtes publiques
- **StrongFather** : Évalue intentions réservation
- **KindMother** : Persiste réservations (SQLite local)
- **MiyuNotify** : Envoie SMS/email (via Twilio/SMTP)

**Infrastructure :**
- COG de Marie sur mini PC (Windows/Linux) dans le cabinet
- DNS dynamique (No-IP, DuckDNS) ou Cloudflare Tunnel
- Certificat SSL (Let's Encrypt ou Cloudflare)

### 3.6 Avantages et inconvénients

**✅ Avantages :**
- **Time-to-Market rapide** : Interface web classique, technos connues
- **Facilité client maximale** : Lien cliquable, pas d'installation
- **SEO possible** : Référencement Google pour "kiné Paris 15"
- **Coût faible** : Pas de store (Apple/Google), pas d'app native

**❌ Inconvénients :**
- **Offline partiel** : Nécessite buffer ou snapshot si COG hors ligne
- **Pas d'expérience native** : Interface web responsive mais pas app native
- **Pas de notifications push** : SMS/email uniquement (pas de push mobile)

---

## 4. Stratégie 2 — Progressive Web App (Phase 2)

### 4.1 Évolution de la Stratégie 1

**Différences clés :**
- Interface web → **PWA installable** (icône sur écran d'accueil)
- Pas de notifications push → **Notifications push navigateur** (Web Push API)
- Pas de cache local → **Service Worker** (cache créneaux, offline basique)

### 4.2 Parcours client amélioré

**Étape 1 : Installation (optionnelle)**
- Paul ouvre https://marie-kine.jaykoa.com
- Navigateur propose "Ajouter à l'écran d'accueil"
- Paul accepte → icône "Marie Kiné" apparaît sur son smartphone

**Étape 2 : Réservation (identique à Stratégie 1)**
- Mêmes étapes que Stratégie 1

**Étape 3 : Notifications push**
- Paul autorise les notifications
- J-1 avant RDV : notification push "RDV demain à 14h30"
- Jour J : notification push "RDV dans 1 heure"

**Étape 4 : Offline basique**
- Paul consulte ses RDV passés/futurs hors ligne (cache Service Worker)
- Réservation impossible offline (nécessite connexion au COG)

### 4.3 Stack technique

**Frontend :**
- PWA avec manifest.json
- Service Worker pour cache offline
- Web Push API pour notifications
- IndexedDB pour cache local

**Backend :**
- Identique à Stratégie 1
- + Endpoint Web Push (notifications)

### 4.4 Avantages et inconvénients

**✅ Avantages (vs Stratégie 1) :**
- **Expérience mobile améliorée** : Icône, splash screen, mode plein écran
- **Notifications push** : Rappels même si app fermée
- **Offline basique** : Consultation RDV passés hors ligne

**❌ Inconvénients :**
- **Complexité accrue** : Service Worker, Web Push, IndexedDB
- **Limitations navigateur** : Safari iOS limité (pas de push jusqu'à iOS 16.4)
- **Pas de gouvernance offline** : Réservation impossible sans connexion COG

---

## 5. Stratégie 3 — COG Android/iOS léger (Phase 3)

### 5.1 Concept : Mini COG "Client Invité"

**Idée :** Paul installe une app mobile "Miyukini Client" qui contient un **mini COG léger** (client invité).

**Architecture :**

```
┌─────────────────────────────────────────────────────────────┐
│  App mobile "Miyukini Client" (Android/iOS)                 │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Mini COG léger (client invité)                      │   │
│  │  - Kernel minimal                                    │   │
│  │  - BorderGuard (mode invité)                         │   │
│  │  - KindMother (cache local uniquement)              │   │
│  │  - MiyuWebwayParticipant (découverte COG)           │   │
│  └──────────────────────────────────────────────────────┘   │
└───────────────────┬─────────────────────────────────────────┘
                    │ Protocoles Inter-COG (Passeport/Visa)
                    ▼
┌─────────────────────────────────────────────────────────────┐
│  COG de Marie (professionnel)                                │
│  JayRDV expose des Services publics avec Visa S1/S2          │
└─────────────────────────────────────────────────────────────┘
```

### 5.2 Parcours client

**Étape 1 : Installation**
- Paul installe "Miyukini Client" depuis App Store / Play Store (gratuit)
- Première ouverture : création **mini COG invité** (identifiant unique, aucune donnée sensible)

**Étape 2 : Découverte du COG de Marie**
- Paul scanne QR code dans le cabinet de Marie
- Ou Paul saisit code professionnel : "MARIE-KINE-75015"
- App découvre le COG de Marie via **MiyuWebwayTracker** (réseau Webway)

**Étape 3 : Demande de Visa**
- Mini COG de Paul génère un **Passeport Utilisateur** (nom, téléphone, COG d'origine)
- Envoie **Demande de Visite** au COG de Marie (via protocole Inter-COG)
- COG de Marie évalue la demande :
  - **StrongFather** : ce visiteur est-il autorisé à consulter créneaux ?
  - **BorderGuard** : niveau de sécurité S1 ou S2 ?
  - Décision : **Visa de Connexion** délivré (S2 : Interaction contrôlée, 30 jours)

**Étape 4 : Consultation créneaux (offline possible)**
- Paul voit les créneaux disponibles (synchronisés dans son mini COG)
- **Offline** : Paul peut consulter les créneaux déjà synchronisés
- **Online** : Mise à jour en temps réel

**Étape 5 : Réservation**
- Paul sélectionne un créneau
- Intention de réservation envoyée au COG de Marie (si online)
- StrongFather évalue, KindMother persiste
- Confirmation synchronisée dans le mini COG de Paul

**Étape 6 : Notifications gouvernées**
- J-1 : Mini COG de Paul affiche notification locale "RDV demain 14h30"
- Pas de dépendance Firebase/APNS (notifications locales uniquement)

### 5.3 Gouvernance Inter-COG

**Règles strictes :**
- Le mini COG de Paul **ne peut jamais modifier** l'état du COG de Marie
- Toute action passe par **Demande de Visite → Visa → Intention → Décision**
- Le COG de Marie peut **révoquer le Visa** à tout moment (spam, abus)

**Niveaux de Visa appliqués :**
- **S1** (Observation) : Consultation créneaux disponibles
- **S2** (Interaction contrôlée) : Réservation, annulation (avec validation SMS)
- **S3+** : Refusé (réservé au professionnel)

### 5.4 Stack technique

**App mobile :**
- Flutter ou React Native (cross-platform)
- Mini COG en Rust (compilé pour Android/iOS via FFI)
- SQLite local pour cache
- Protocoles Inter-COG (Passeport/Visa/Webway)

**COG professionnel :**
- Identique aux stratégies précédentes
- + Gestion Visas Inter-COG (BorderGuard)
- + MiyuWebwayParticipant (annonce dans le réseau)

### 5.5 Avantages et inconvénients

**✅ Avantages :**
- **Offline complet** : Consultation créneaux hors ligne
- **Souveraineté client** : Paul a son propre mini COG (données chez lui)
- **Gouvernance bout-en-bout** : Protocoles Inter-COG (Passeport/Visa)
- **Notifications locales** : Pas de dépendance Firebase/APNS
- **Sécurité structurelle** : BorderGuard, StrongFather des deux côtés

**❌ Inconvénients :**
- **Complexité élevée** : Développement app native, mini COG, protocoles Inter-COG
- **Taille app** : Mini COG + SQLite + Rust FFI (50-100 MB)
- **Time-to-Market long** : 12-18 mois de développement
- **Adoption difficile** : Client doit installer app (friction)

---

## 6. Stratégie 4 — Fédération Inter-COG (Phase 4, long terme)

### 6.1 Vision finale

**Paul a son propre COG complet** (pas un mini COG léger, mais un vrai COG Miyukini).

**Architecture :**

```
┌─────────────────────────────────────────────────────────────┐
│  COG de Paul (citoyen)                                       │
│  - Kernel complet                                            │
│  - 9 Cores (StrongFather, KindMother, etc.)                 │
│  - JayKoa (son agenda personnel)                            │
│  - JayKonta (sa comptabilité perso)                         │
└───────────────────┬─────────────────────────────────────────┘
                    │ Protocoles Inter-COG (Fédération)
                    ▼
┌─────────────────────────────────────────────────────────────┐
│  COG de Marie (professionnel)                                │
│  - JayRDV expose Services publics                            │
│  - Accepte Visas Inter-COG                                   │
└─────────────────────────────────────────────────────────────┘
```

### 6.2 Parcours client

**Étape 1 : Paul a déjà un COG**
- Paul a installé Miyukini Central (desktop ou mobile)
- Il utilise JayKoa pour son agenda personnel
- Il utilise JayKonta pour sa comptabilité perso

**Étape 2 : Découverte du COG de Marie**
- Paul cherche "kiné Paris 15" dans MiyuWebwayTracker (moteur de découverte COG)
- Trouve le COG de Marie dans le réseau Webway

**Étape 3 : Demande de Visite Inter-COG**
- Le COG de Paul génère un **Passeport Utilisateur** (certifié par son COG)
- Envoie **Demande de Visite** au COG de Marie
- COG de Marie évalue et délivre **Visa S2** (30 jours)

**Étape 4 : Réservation Inter-COG**
- Paul réserve un créneau (intention envoyée via protocole Inter-COG)
- **Le RDV est enregistré dans les DEUX COG** :
  - COG de Marie : réservation dans JayRDV (agenda professionnel)
  - COG de Paul : événement dans JayKoa (agenda personnel)
- **Synchronisation automatique** : si Marie déplace le RDV, JayKoa de Paul est notifié

**Étape 5 : Paiement Inter-COG (si applicable)**
- COG de Marie envoie facture (JayKonta)
- COG de Paul reçoit et enregistre facture (JayKonta)
- Paiement via protocole sécurisé Inter-COG

### 6.3 Avantages et inconvénients

**✅ Avantages :**
- **Vision finale** : Réseau de COG fédérés, souveraineté totale
- **Synchronisation bi-directionnelle** : RDV dans les deux agendas
- **Écosystème complet** : Paul peut utiliser Miyukini pour tout (agenda, compta, etc.)
- **Sécurité maximale** : Gouvernance bout-en-bout, protocoles cryptographiques

**❌ Inconvénients :**
- **Adoption très difficile** : Paul doit installer un COG complet (desktop ou mobile lourd)
- **Complexité extrême** : Fédération, synchronisation, conflits, etc.
- **Horizon long terme** : 3-5 ans de développement

---

## 7. Recommandation stratégique — Feuille de route

### 7.1 Phase 1 (MVP — 6 mois) : Interface Web publique

**Priorité : Validation marché et Time-to-Market**

**Livrables :**
- Site web responsive : https://[prenom-profession-ville].jaykoa.com
- Réservation en 2-3 clics (nom, téléphone, créneau)
- SMS confirmation/rappel (via Twilio)
- Buffer offline (Cloudflare Workers + Redis)

**Cibles :**
- 10 professionnels pilotes (kinés, artisans, restaurateurs)
- 100 clients finaux (réservations)

**Métriques de succès :**
- Taux de conversion : >60% (visiteur → réservation)
- Taux d'annulation : <15%
- Satisfaction client : >4/5

---

### 7.2 Phase 2 (12 mois) : Progressive Web App

**Priorité : Expérience mobile et notifications**

**Livrables :**
- PWA installable (manifest.json)
- Notifications push (Web Push API)
- Service Worker (cache créneaux offline)
- Intégration calendrier système (iCal, Google Calendar)

**Cibles :**
- 100 professionnels
- 1000 clients finaux

**Métriques de succès :**
- Taux d'installation PWA : >30%
- Taux d'activation notifications : >50%
- Réduction no-show : -20% (grâce aux notifications)

---

### 7.3 Phase 3 (24 mois) : COG Android/iOS léger

**Priorité : Souveraineté client et offline complet**

**Livrables :**
- App "Miyukini Client" (App Store / Play Store)
- Mini COG léger (client invité)
- Protocoles Inter-COG (Passeport/Visa)
- Découverte Webway (réseau de COG)

**Cibles :**
- 500 professionnels
- 10 000 clients finaux

**Métriques de succès :**
- Taux d'installation app : >40%
- Offline utilisé : >20% des consultations
- NPS (Net Promoter Score) : >60

---

### 7.4 Phase 4 (36-48 mois) : Fédération Inter-COG

**Priorité : Vision finale et écosystème complet**

**Livrables :**
- COG complet mobile (Android/iOS)
- Fédération Inter-COG opérationnelle
- Synchronisation bi-directionnelle (JayKoa, JayKonta)
- Paiements Inter-COG sécurisés

**Cibles :**
- 5 000 professionnels
- 100 000 citoyens avec COG complet

**Métriques de succès :**
- Taux d'adoption COG complet : >10% des clients
- Réseau Webway : >1000 COG fédérés
- Transactions Inter-COG : >10 000/mois

---

## 8. Matrice de décision par profil client

### 8.1 Quel parcours pour quel client ?

| Profil client | Stratégie recommandée | Justification |
|---------------|----------------------|---------------|
| **Client occasionnel** (1-2 RDV/an) | **Stratégie 1 : Web** | Pas de friction, lien direct, pas d'installation |
| **Client régulier** (1 RDV/mois) | **Stratégie 2 : PWA** | Notifications utiles, installation légère |
| **Client fidèle** (plusieurs pro : kiné + dentiste + coiffeur) | **Stratégie 3 : Mini COG** | Centralisation RDV, offline, souveraineté |
| **Citoyen COG** (utilise déjà Miyukini pour autres services) | **Stratégie 4 : Fédération** | Intégration JayKoa/JayKonta, écosystème complet |

### 8.2 Parcours de migration naturel

**Scénario réaliste :**

1. **Paul découvre JayRDV via lien web** (Stratégie 1)
   - Premier RDV avec Marie la kiné
   - Expérience fluide, pas de friction

2. **Paul devient client régulier** → Installe PWA (Stratégie 2)
   - Marie propose "Installez l'app pour recevoir des rappels"
   - Paul accepte, installe PWA

3. **Paul consulte plusieurs professionnels** → Installe Mini COG (Stratégie 3)
   - Paul a maintenant 3 pros : kiné + dentiste + coiffeur
   - App web suggère "Centralisez tous vos RDV avec Miyukini Client"
   - Paul installe app native avec mini COG

4. **Paul adopte Miyukini pour d'autres besoins** → COG complet (Stratégie 4)
   - Paul découvre JayKonta pour sa comptabilité perso
   - Paul installe Miyukini Central (desktop ou mobile complet)
   - Synchronisation automatique avec ses professionnels

---

## 9. Aspects techniques critiques

### 9.1 Gestion des conflits de réservation

**Problème :** Deux clients réservent le même créneau en même temps.

**Solution : Lock optimiste avec résolution automatique**

```rust
// Dans JayRDV (Strate 7)
pub fn reserve_slot(slot_id: SlotId, client: ClientInfo) -> Result<Booking, ConflictError> {
    // 1. StrongFather évalue l'intention
    let intent = Intent::ReserveSlot { slot_id, client };
    let decision = strongfather::evaluate(intent)?;

    // 2. KindMother vérifie disponibilité (lock optimiste)
    let slot = kindmother::read_slot(slot_id)?;
    if slot.status != SlotStatus::Available {
        // Créneau déjà pris, proposer alternatives
        let alternatives = find_nearby_slots(slot_id, 3)?;
        return Err(ConflictError::SlotTaken { alternatives });
    }

    // 3. KindMother persiste (transaction SQLite)
    let booking = kindmother::write_booking(slot_id, client)?;

    // 4. MiyuNotify envoie confirmation
    miyunotify::send_sms(client.phone, "RDV confirmé le {}", slot.datetime)?;

    Ok(booking)
}
```

**Comportement :**
- Si conflit : SMS "Désolé, créneau déjà pris. Propositions : 14h45, 15h15, 16h00"
- Client choisit alternative en 1 clic

### 9.2 Gestion de l'identité sans compte

**Problème :** Comment identifier Paul sans lui imposer de créer un compte ?

**Solution : Identification par téléphone + token temporaire**

```
┌─────────────────────────────────────────────────────────────┐
│  Première réservation                                        │
│  - Paul saisit : nom + téléphone                             │
│  - Système envoie SMS OTP (One-Time Password)                │
│  - Paul valide OTP → Token temporaire (JWT 90 jours)         │
│  - Token stocké dans cookie/localStorage                     │
└─────────────────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────────┐
│  Réservations suivantes                                      │
│  - Paul a déjà le token (cookie)                             │
│  - Formulaire pré-rempli : "Paul Dupont - 06 12 34 56 78"   │
│  - Réservation en 1 clic                                     │
└─────────────────────────────────────────────────────────────┘
```

**Avantages :**
- Pas de mot de passe à retenir
- Validation SMS = anti-spam
- Token révocable si abus

### 9.3 Gestion des annulations et modifications

**Règles métier :**
- Client peut annuler jusqu'à 24h avant (lien unique dans SMS)
- Client peut modifier jusqu'à 48h avant (même process que réservation)
- Après deadline : annulation impossible (ou avec frais si JayKonta intégré)

**Architecture :**
```
Paul clique "Annuler RDV" (lien SMS)
    ↓
MiyuWeb vérifie token unique
    ↓
BorderGuard vérifie délai (>24h ?)
    ↓
StrongFather évalue intention annulation
    ↓
KindMother libère créneau (SlotStatus::Available)
    ↓
MiyuNotify SMS confirmation annulation + notification Marie
```

---

## 10. Comparaison avec solutions existantes

### 10.1 Benchmark concurrentiel

| Solution | Client voit | Pro utilise | Offline | Souveraineté | Coût pro |
|----------|-------------|-------------|---------|--------------|----------|
| **Calendly** | Site web | Calendly SaaS | ❌ | ❌ Cloud US | €8-12/mois |
| **Doctolib** | Site + app | Doctolib SaaS | ❌ | ❌ Cloud FR | €129/mois |
| **SimplyBook** | Site web | SimplyBook SaaS | ❌ | ❌ Cloud | €8-50/mois |
| **JayRDV Phase 1** | Site web | COG Miyukini | ⚠️ Buffer | ✅ Chez le pro | €10-30/mois |
| **JayRDV Phase 2** | PWA | COG Miyukini | ⚠️ Partiel | ✅ Chez le pro | €10-30/mois |
| **JayRDV Phase 3** | Mini COG | COG Miyukini | ✅ Complet | ✅ Les deux | €10-30/mois |

### 10.2 Proposition de valeur unique

**Ce que JayRDV apporte vs concurrents :**
- ✅ **Souveraineté des données** : Pro garde ses données chez lui (mini PC, NAS)
- ✅ **Offline-first** : Fonctionne sans réseau (événements, zones isolées)
- ✅ **Coût maîtrisé** : Pas d'abonnement cloud obligatoire (licence perpétuelle possible)
- ✅ **Interpolarité** : S'intègre avec JayKoa (agenda), JayKonta (compta), JayXpose (vitrine)
- ✅ **Gouvernance** : BorderGuard, StrongFather, anti-spam structurel

**Cibles privilégiées :**
- Professionnels sensibles à la souveraineté (médecins, avocats, collectivités)
- Professionnels en zones isolées (rural, montagne)
- Professionnels événementiels (festivals, marchés)

---

## 11. Prochaines étapes

### 11.1 Validation marché

**Actions immédiates :**
1. **Interviews** : 20 professionnels cibles (kinés, artisans, restaurateurs)
   - Quel système utilisent-ils ? (Calendly, papier, Google Calendar ?)
   - Quels pain points ? (no-show, oublis, double booking ?)
   - Accepteraient-ils une solution Miyukini ?

2. **Prototype Figma** : Parcours web (5 écrans)
   - Découverte (lien/QR code)
   - Calendrier créneaux
   - Formulaire réservation
   - Confirmation
   - Gestion (annulation/modification)

3. **MVP technique** : Site web de démo (2 semaines)
   - MiyuWeb + JayRDV mockés
   - Créneaux disponibles (hard-coded)
   - Formulaire réservation (sans persistance)
   - SMS simulation (console log)

### 11.2 Décisions architecturales à valider

**Questions ouvertes :**
1. **Buffer offline** : Redis/SQLite ou abandon si COG offline ?
2. **DNS dynamique** : Cloudflare Tunnel ou No-IP ?
3. **SMS provider** : Twilio, Vonage ou OVH Telecom ?
4. **Domaines** : Sous-domaines génériques (marie-kine.jaykoa.com) ou domaines perso (marie-dupont.fr) ?
5. **Multilingue** : FR uniquement en MVP ou EN/FR dès le début ?

---

## 12. Conclusion

### 12.1 Stratégie recommandée

**Phase 1 (MVP) : Interface Web publique**
- Priorité absolue : validation marché
- Time-to-Market : 6 mois
- Cible : 10 professionnels pilotes

**Phase 2 (12 mois) : Progressive Web App**
- Amélioration UX mobile
- Notifications push
- Cible : 100 professionnels

**Phase 3 (24 mois) : COG Android/iOS léger**
- Souveraineté client
- Offline complet
- Cible : 500 professionnels

**Phase 4 (36+ mois) : Fédération Inter-COG**
- Vision finale
- Écosystème complet
- Cible : 5000 professionnels

### 12.2 Principe directeur

> **"Le client ne doit jamais être contraint d'installer un COG pour bénéficier d'un Service. Mais s'il le fait, il en tire une valeur supplémentaire (souveraineté, offline, interpolarité)."**

---

## 13. Références

| Thème | Document |
|-------|----------|
| **Protocoles Inter-COG** | [Connexion Inter-COG](../public/Miyukini%20-%20Connexion%20Inter-COG.md) |
| **Webway** | [Tools et Toolkits](../public/Miyukini%20-%20Tools%20et%20Toolkits.md) (MiyuWebwayParticipant/Tracker) |
| **BorderGuard** | [Pyramide Architecture](../public/Miyukini%20-%20Pyramide%20Architecture%20Complete.md) |
| **Lois d'autonomie** | [Lois Autonomie Systeme](../public/Miyukini%20-%20Lois%20Autonomie%20Systeme.md) |

---

**Date de création :** 2026-02-07
**Version :** 1.0
**Statut :** Document de conception — Accessibilité client et parcours de réservation
