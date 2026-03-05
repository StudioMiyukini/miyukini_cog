# JayRDV â€” AccessibilitÃ© Client et Parcours de RÃ©servation

## Contexte

Ce document conceptualise **comment un client final peut rÃ©server un crÃ©neau avec un professionnel utilisant JayRDV**, sans avoir lui-mÃªme Ã  installer un COG. Il explore les diffÃ©rentes stratÃ©gies d'accessibilitÃ© et leurs implications architecturales.

## PortÃ©e / Scope

- **Applicable Ã  :** Conception produit JayRDV, stratÃ©gie d'accessibilitÃ© Services COG
- **Couvre :** Parcours client, interfaces d'accÃ¨s, architecture web/mobile, gouvernance
- **Ne couvre pas :** ImplÃ©mentation technique dÃ©taillÃ©e

---

## 1. Le problÃ¨me fondamental

### 1.1 Ã‰noncÃ© du problÃ¨me

**Situation :**
- Marie est kinÃ©sithÃ©rapeute. Elle utilise JayRDV dans son COG Miyukini pour gÃ©rer son agenda.
- Paul est un patient. Il veut prendre rendez-vous avec Marie.

**Question critique :**
> **Comment Paul peut-il rÃ©server un crÃ©neau sans installer un COG Miyukini ?**

### 1.2 Contraintes

| Contrainte | Description |
|------------|-------------|
| **FacilitÃ© client** | Paul doit pouvoir rÃ©server en 2-3 clics, depuis son smartphone, sans inscription complexe |
| **SouverainetÃ© du COG** | Le COG de Marie reste souverain (BorderGuard, StrongFather, KindMother) |
| **Pas de dÃ©pendance cloud** | Marie doit pouvoir fonctionner offline ; la rÃ©servation doit Ãªtre possible mÃªme si le COG est temporairement hors ligne |
| **SÃ©curitÃ©** | Pas de spam, pas de fausses rÃ©servations, pas d'accÃ¨s non autorisÃ© |
| **ScalabilitÃ©** | Solution viable pour 1 professionnel comme pour 1000 professionnels |

---

## 2. StratÃ©gies d'accessibilitÃ© â€” Vue d'ensemble

### 2.1 Les 4 stratÃ©gies possibles

| StratÃ©gie | Client voit | Professionnel utilise | ComplexitÃ© | Offline | SouverainetÃ© |
|-----------|-------------|----------------------|------------|---------|--------------|
| **1. Interface Web publique** | Site web classique | COG Miyukini (JayRDV) | â­ Faible | âš ï¸ Partielle | âœ… Forte |
| **2. App mobile lÃ©gÃ¨re (Progressive Web App)** | PWA smartphone | COG Miyukini (JayRDV) | â­â­ Moyenne | âš ï¸ Partielle | âœ… Forte |
| **3. COG Android/iOS lÃ©ger (client invitÃ©)** | Mini COG sur tÃ©lÃ©phone | COG Miyukini (JayRDV) | â­â­â­ Ã‰levÃ©e | âœ… ComplÃ¨te | âœ… ComplÃ¨te |
| **4. FÃ©dÃ©ration Inter-COG (client a son propre COG)** | COG complet | COG Miyukini (JayRDV) | â­â­â­â­ TrÃ¨s Ã©levÃ©e | âœ… ComplÃ¨te | âœ… ComplÃ¨te |

### 2.2 StratÃ©gie recommandÃ©e par phase

**Phase 1 â€” MVP (Time-to-Market rapide) :**
- âœ… **StratÃ©gie 1 : Interface Web publique**
- Pourquoi : FacilitÃ© client maximale, dÃ©veloppement rapide, validation marchÃ©

**Phase 2 â€” ExpÃ©rience mobile (6-12 mois aprÃ¨s MVP) :**
- âœ… **StratÃ©gie 2 : Progressive Web App**
- Pourquoi : ExpÃ©rience mobile native, offline partiel, pas de store

**Phase 3 â€” SouverainetÃ© client (18-24 mois) :**
- âœ… **StratÃ©gie 3 : COG Android/iOS lÃ©ger**
- Pourquoi : Offline complet, souverainetÃ© client, gouvernance bout-en-bout

**Phase 4 â€” FÃ©dÃ©ration (long terme) :**
- âœ… **StratÃ©gie 4 : FÃ©dÃ©ration Inter-COG**
- Pourquoi : Vision finale, rÃ©seau de COG fÃ©dÃ©rÃ©s

---

## 3. StratÃ©gie 1 â€” Interface Web publique (MVP)

### 3.1 Architecture

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Client (Paul) â€” Navigateur web                              â”‚
â”‚  https://marie-kine.jaykoa.com                               â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                    â”‚ HTTPS
                    â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  MiyuWeb (Toolkit HTTP/WebSocket) â€” Strate 6                â”‚
â”‚  Serveur HTTP/WebSocket dans le COG de Marie                â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                    â”‚ BondingBrother
                    â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  BorderGuard â€” Strate 4                                      â”‚
â”‚  Filtre les requÃªtes publiques, applique niveaux S1-S2      â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                    â”‚
                    â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  StrongFather â†’ KindMother â†’ JayRDV                          â”‚
â”‚  DÃ©cision, persistance, logique rÃ©servation                 â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 3.2 Parcours client

**Ã‰tape 1 : DÃ©couverte**
- Paul reÃ§oit un SMS/email de Marie : "Prenez RDV sur https://marie-kine.jaykoa.com"
- Ou Paul scanne un QR code dans la salle d'attente
- Ou Paul trouve le lien sur Google Maps / rÃ©seaux sociaux

**Ã‰tape 2 : AccÃ¨s au calendrier**
- Paul ouvre le lien dans son navigateur (mobile ou desktop)
- Pas de compte requis (pour consultation des crÃ©neaux disponibles)
- Interface web lÃ©gÃ¨re, responsive, rapide

**Ã‰tape 3 : SÃ©lection crÃ©neau**
- Paul voit les crÃ©neaux disponibles (vue semaine ou mois)
- CrÃ©neaux affichÃ©s selon disponibilitÃ©s rÃ©elles de JayRDV (via MiyuWeb â†’ JayRDV)
- Paul clique sur un crÃ©neau (ex : "Lundi 10/02 Ã  14h30")

**Ã‰tape 4 : Identification minimale**
- Formulaire simple :
  - Nom : "Paul Dupont"
  - TÃ©lÃ©phone : "06 12 34 56 78" (pour confirmation SMS)
  - Email : "paul@example.com" (optionnel, pour rappel email)
  - Raison consultation : "Douleur Ã©paule" (optionnel)

**Ã‰tape 5 : Confirmation**
- Paul clique "Confirmer le rendez-vous"
- RequÃªte envoyÃ©e Ã  MiyuWeb â†’ BorderGuard â†’ StrongFather â†’ JayRDV
- **StrongFather Ã©value** : crÃ©neau disponible ? spam ? doublon ?
- **KindMother persiste** : rÃ©servation enregistrÃ©e
- **MiyuNotify envoie SMS** : "RDV confirmÃ© le 10/02 Ã  14h30 avec Marie"

**Ã‰tape 6 : Rappel et gestion**
- J-1 : SMS/email de rappel automatique
- J-0 : Paul peut annuler via lien unique dans le SMS
- AprÃ¨s RDV : SMS de feedback (optionnel)

### 3.3 Fonctionnement offline du COG

**ProblÃ¨me :** Le COG de Marie est sur un mini PC dans son cabinet. Que se passe-t-il si Paul essaie de rÃ©server quand le COG est hors ligne ?

**Solutions :**

#### Option A : Buffer de rÃ©servations (recommandÃ©)

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  DNS dynamique + reverse proxy (Cloudflare Tunnel, Ngrok)   â”‚
â”‚  Redirige vers le COG si online, sinon vers buffer          â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                    â”‚
                    â–¼
         â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
         â”‚                     â”‚
   [COG Online]          [COG Offline]
         â”‚                     â”‚
         â–¼                     â–¼
  RÃ©servation          Buffer Redis/SQLite
  immÃ©diate            (CloudFlare Workers)
         â”‚                     â”‚
         â”‚                     â–¼
         â”‚              Sync quand COG revient
         â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                    â–¼
              KindMother persiste
```

**Comportement :**
1. Paul fait une demande de rÃ©servation
2. Si COG online : rÃ©servation immÃ©diate, SMS confirmÃ©
3. Si COG offline : demande mise en buffer, SMS "demande reÃ§ue, confirmation dans quelques heures"
4. Quand COG revient online : synchronisation buffer â†’ JayRDV, SMS confirmation ou conflit

#### Option B : Calendrier en lecture seule offline

- Le COG publie pÃ©riodiquement (toutes les heures, via cron) un snapshot JSON des crÃ©neaux disponibles sur CDN
- Client voit les crÃ©neaux disponibles (lecture seule)
- RÃ©servation impossible si COG offline â†’ message "Marie n'est pas disponible, rÃ©essayez plus tard"
- Moins bon UX mais plus simple

### 3.4 Gouvernance et sÃ©curitÃ©

**BorderGuard applique des rÃ¨gles strictes :**

| Menace | Protection BorderGuard |
|--------|------------------------|
| **Spam** | Rate limiting : max 3 rÃ©servations/IP/jour |
| **Fausses rÃ©servations** | Validation tÃ©lÃ©phone (SMS OTP) avant confirmation |
| **Attaque DDoS** | Cloudflare devant le COG, filtrage IP |
| **Scraping** | CrÃ©neaux disponibles visibles uniquement aprÃ¨s identification |
| **Concurrence (double booking)** | StrongFather Ã©value + lock optimiste : si crÃ©neau pris entre temps, refus avec suggestion alternative |

**Niveaux de sÃ©curitÃ© appliquÃ©s :**
- **S1** (Observation) : Consultation crÃ©neaux disponibles (pas de compte)
- **S2** (Interaction contrÃ´lÃ©e) : RÃ©servation avec identification minimale (nom + tÃ©lÃ©phone)
- **S3+** : RÃ©servÃ© au professionnel (Marie) dans son COG

### 3.5 Stack technique

**Frontend (client web) :**
- HTML/CSS/JS vanilla ou framework lÃ©ger (Svelte, Preact)
- Responsive mobile-first
- Pas de dÃ©pendance lourde (React/Vue overkill pour ce cas)

**Backend (COG de Marie) :**
- **MiyuWeb** : Serveur HTTP/WebSocket (Rust + Axum ou Actix-web)
- **JayRDV** : Logique mÃ©tier rÃ©servation (crate Rust)
- **BorderGuard** : Filtre requÃªtes publiques
- **StrongFather** : Ã‰value intentions rÃ©servation
- **KindMother** : Persiste rÃ©servations (SQLite local)
- **MiyuNotify** : Envoie SMS/email (via Twilio/SMTP)

**Infrastructure :**
- COG de Marie sur mini PC (Windows/Linux) dans le cabinet
- DNS dynamique (No-IP, DuckDNS) ou Cloudflare Tunnel
- Certificat SSL (Let's Encrypt ou Cloudflare)

### 3.6 Avantages et inconvÃ©nients

**âœ… Avantages :**
- **Time-to-Market rapide** : Interface web classique, technos connues
- **FacilitÃ© client maximale** : Lien cliquable, pas d'installation
- **SEO possible** : RÃ©fÃ©rencement Google pour "kinÃ© Paris 15"
- **CoÃ»t faible** : Pas de store (Apple/Google), pas d'app native

**âŒ InconvÃ©nients :**
- **Offline partiel** : NÃ©cessite buffer ou snapshot si COG hors ligne
- **Pas d'expÃ©rience native** : Interface web responsive mais pas app native
- **Pas de notifications push** : SMS/email uniquement (pas de push mobile)

---

## 4. StratÃ©gie 2 â€” Progressive Web App (Phase 2)

### 4.1 Ã‰volution de la StratÃ©gie 1

**DiffÃ©rences clÃ©s :**
- Interface web â†’ **PWA installable** (icÃ´ne sur Ã©cran d'accueil)
- Pas de notifications push â†’ **Notifications push navigateur** (Web Push API)
- Pas de cache local â†’ **Service Worker** (cache crÃ©neaux, offline basique)

### 4.2 Parcours client amÃ©liorÃ©

**Ã‰tape 1 : Installation (optionnelle)**
- Paul ouvre https://marie-kine.jaykoa.com
- Navigateur propose "Ajouter Ã  l'Ã©cran d'accueil"
- Paul accepte â†’ icÃ´ne "Marie KinÃ©" apparaÃ®t sur son smartphone

**Ã‰tape 2 : RÃ©servation (identique Ã  StratÃ©gie 1)**
- MÃªmes Ã©tapes que StratÃ©gie 1

**Ã‰tape 3 : Notifications push**
- Paul autorise les notifications
- J-1 avant RDV : notification push "RDV demain Ã  14h30"
- Jour J : notification push "RDV dans 1 heure"

**Ã‰tape 4 : Offline basique**
- Paul consulte ses RDV passÃ©s/futurs hors ligne (cache Service Worker)
- RÃ©servation impossible offline (nÃ©cessite connexion au COG)

### 4.3 Stack technique

**Frontend :**
- PWA avec manifest.json
- Service Worker pour cache offline
- Web Push API pour notifications
- IndexedDB pour cache local

**Backend :**
- Identique Ã  StratÃ©gie 1
- + Endpoint Web Push (notifications)

### 4.4 Avantages et inconvÃ©nients

**âœ… Avantages (vs StratÃ©gie 1) :**
- **ExpÃ©rience mobile amÃ©liorÃ©e** : IcÃ´ne, splash screen, mode plein Ã©cran
- **Notifications push** : Rappels mÃªme si app fermÃ©e
- **Offline basique** : Consultation RDV passÃ©s hors ligne

**âŒ InconvÃ©nients :**
- **ComplexitÃ© accrue** : Service Worker, Web Push, IndexedDB
- **Limitations navigateur** : Safari iOS limitÃ© (pas de push jusqu'Ã  iOS 16.4)
- **Pas de gouvernance offline** : RÃ©servation impossible sans connexion COG

---

## 5. StratÃ©gie 3 â€” COG Android/iOS lÃ©ger (Phase 3)

### 5.1 Concept : Mini COG "Client InvitÃ©"

**IdÃ©e :** Paul installe une app mobile "Miyukini Client" qui contient un **mini COG lÃ©ger** (client invitÃ©).

**Architecture :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  App mobile "Miyukini Client" (Android/iOS)                 â”‚
â”‚  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”   â”‚
â”‚  â”‚  Mini COG lÃ©ger (client invitÃ©)                      â”‚   â”‚
â”‚  â”‚  - Kernel minimal                                    â”‚   â”‚
â”‚  â”‚  - BorderGuard (mode invitÃ©)                         â”‚   â”‚
â”‚  â”‚  - KindMother (cache local uniquement)              â”‚   â”‚
â”‚  â”‚  - MiyuWebwayParticipant (dÃ©couverte COG)           â”‚   â”‚
â”‚  â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                    â”‚ Protocoles Inter-COG (Passeport/Visa)
                    â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  COG de Marie (professionnel)                                â”‚
â”‚  JayRDV expose des Services publics avec Visa S1/S2          â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 5.2 Parcours client

**Ã‰tape 1 : Installation**
- Paul installe "Miyukini Client" depuis App Store / Play Store (gratuit)
- PremiÃ¨re ouverture : crÃ©ation **mini COG invitÃ©** (identifiant unique, aucune donnÃ©e sensible)

**Ã‰tape 2 : DÃ©couverte du COG de Marie**
- Paul scanne QR code dans le cabinet de Marie
- Ou Paul saisit code professionnel : "MARIE-KINE-75015"
- App dÃ©couvre le COG de Marie via **MiyuWebwayTracker** (rÃ©seau Webway)

**Ã‰tape 3 : Demande de Visa**
- Mini COG de Paul gÃ©nÃ¨re un **Passeport Utilisateur** (nom, tÃ©lÃ©phone, COG d'origine)
- Envoie **Demande de Visite** au COG de Marie (via protocole Inter-COG)
- COG de Marie Ã©value la demande :
  - **StrongFather** : ce visiteur est-il autorisÃ© Ã  consulter crÃ©neaux ?
  - **BorderGuard** : niveau de sÃ©curitÃ© S1 ou S2 ?
  - DÃ©cision : **Visa de Connexion** dÃ©livrÃ© (S2 : Interaction contrÃ´lÃ©e, 30 jours)

**Ã‰tape 4 : Consultation crÃ©neaux (offline possible)**
- Paul voit les crÃ©neaux disponibles (synchronisÃ©s dans son mini COG)
- **Offline** : Paul peut consulter les crÃ©neaux dÃ©jÃ  synchronisÃ©s
- **Online** : Mise Ã  jour en temps rÃ©el

**Ã‰tape 5 : RÃ©servation**
- Paul sÃ©lectionne un crÃ©neau
- Intention de rÃ©servation envoyÃ©e au COG de Marie (si online)
- StrongFather Ã©value, KindMother persiste
- Confirmation synchronisÃ©e dans le mini COG de Paul

**Ã‰tape 6 : Notifications gouvernÃ©es**
- J-1 : Mini COG de Paul affiche notification locale "RDV demain 14h30"
- Pas de dÃ©pendance Firebase/APNS (notifications locales uniquement)

### 5.3 Gouvernance Inter-COG

**RÃ¨gles strictes :**
- Le mini COG de Paul **ne peut jamais modifier** l'Ã©tat du COG de Marie
- Toute action passe par **Demande de Visite â†’ Visa â†’ Intention â†’ DÃ©cision**
- Le COG de Marie peut **rÃ©voquer le Visa** Ã  tout moment (spam, abus)

**Niveaux de Visa appliquÃ©s :**
- **S1** (Observation) : Consultation crÃ©neaux disponibles
- **S2** (Interaction contrÃ´lÃ©e) : RÃ©servation, annulation (avec validation SMS)
- **S3+** : RefusÃ© (rÃ©servÃ© au professionnel)

### 5.4 Stack technique

**App mobile :**
- Flutter ou React Native (cross-platform)
- Mini COG en Rust (compilÃ© pour Android/iOS via FFI)
- SQLite local pour cache
- Protocoles Inter-COG (Passeport/Visa/Webway)

**COG professionnel :**
- Identique aux stratÃ©gies prÃ©cÃ©dentes
- + Gestion Visas Inter-COG (BorderGuard)
- + MiyuWebwayParticipant (annonce dans le rÃ©seau)

### 5.5 Avantages et inconvÃ©nients

**âœ… Avantages :**
- **Offline complet** : Consultation crÃ©neaux hors ligne
- **SouverainetÃ© client** : Paul a son propre mini COG (donnÃ©es chez lui)
- **Gouvernance bout-en-bout** : Protocoles Inter-COG (Passeport/Visa)
- **Notifications locales** : Pas de dÃ©pendance Firebase/APNS
- **SÃ©curitÃ© structurelle** : BorderGuard, StrongFather des deux cÃ´tÃ©s

**âŒ InconvÃ©nients :**
- **ComplexitÃ© Ã©levÃ©e** : DÃ©veloppement app native, mini COG, protocoles Inter-COG
- **Taille app** : Mini COG + SQLite + Rust FFI (50-100 MB)
- **Time-to-Market long** : 12-18 mois de dÃ©veloppement
- **Adoption difficile** : Client doit installer app (friction)

---

## 6. StratÃ©gie 4 â€” FÃ©dÃ©ration Inter-COG (Phase 4, long terme)

### 6.1 Vision finale

**Paul a son propre COG complet** (pas un mini COG lÃ©ger, mais un vrai COG Miyukini).

**Architecture :**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  COG de Paul (citoyen)                                       â”‚
â”‚  - Kernel complet                                            â”‚
â”‚  - 9 Cores (StrongFather, KindMother, etc.)                 â”‚
â”‚  - JayKoa (son agenda personnel)                            â”‚
â”‚  - JayKonta (sa comptabilitÃ© perso)                         â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                    â”‚ Protocoles Inter-COG (FÃ©dÃ©ration)
                    â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  COG de Marie (professionnel)                                â”‚
â”‚  - JayRDV expose Services publics                            â”‚
â”‚  - Accepte Visas Inter-COG                                   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 6.2 Parcours client

**Ã‰tape 1 : Paul a dÃ©jÃ  un COG**
- Paul a installÃ© Miyukini Central (desktop ou mobile)
- Il utilise JayKoa pour son agenda personnel
- Il utilise JayKonta pour sa comptabilitÃ© perso

**Ã‰tape 2 : DÃ©couverte du COG de Marie**
- Paul cherche "kinÃ© Paris 15" dans MiyuWebwayTracker (moteur de dÃ©couverte COG)
- Trouve le COG de Marie dans le rÃ©seau Webway

**Ã‰tape 3 : Demande de Visite Inter-COG**
- Le COG de Paul gÃ©nÃ¨re un **Passeport Utilisateur** (certifiÃ© par son COG)
- Envoie **Demande de Visite** au COG de Marie
- COG de Marie Ã©value et dÃ©livre **Visa S2** (30 jours)

**Ã‰tape 4 : RÃ©servation Inter-COG**
- Paul rÃ©serve un crÃ©neau (intention envoyÃ©e via protocole Inter-COG)
- **Le RDV est enregistrÃ© dans les DEUX COG** :
  - COG de Marie : rÃ©servation dans JayRDV (agenda professionnel)
  - COG de Paul : Ã©vÃ©nement dans JayKoa (agenda personnel)
- **Synchronisation automatique** : si Marie dÃ©place le RDV, JayKoa de Paul est notifiÃ©

**Ã‰tape 5 : Paiement Inter-COG (si applicable)**
- COG de Marie envoie facture (JayKonta)
- COG de Paul reÃ§oit et enregistre facture (JayKonta)
- Paiement via protocole sÃ©curisÃ© Inter-COG

### 6.3 Avantages et inconvÃ©nients

**âœ… Avantages :**
- **Vision finale** : RÃ©seau de COG fÃ©dÃ©rÃ©s, souverainetÃ© totale
- **Synchronisation bi-directionnelle** : RDV dans les deux agendas
- **Ã‰cosystÃ¨me complet** : Paul peut utiliser Miyukini pour tout (agenda, compta, etc.)
- **SÃ©curitÃ© maximale** : Gouvernance bout-en-bout, protocoles cryptographiques

**âŒ InconvÃ©nients :**
- **Adoption trÃ¨s difficile** : Paul doit installer un COG complet (desktop ou mobile lourd)
- **ComplexitÃ© extrÃªme** : FÃ©dÃ©ration, synchronisation, conflits, etc.
- **Horizon long terme** : 3-5 ans de dÃ©veloppement

---

## 7. Recommandation stratÃ©gique â€” Feuille de route

### 7.1 Phase 1 (MVP â€” 6 mois) : Interface Web publique

**PrioritÃ© : Validation marchÃ© et Time-to-Market**

**Livrables :**
- Site web responsive : https://[prenom-profession-ville].jaykoa.com
- RÃ©servation en 2-3 clics (nom, tÃ©lÃ©phone, crÃ©neau)
- SMS confirmation/rappel (via Twilio)
- Buffer offline (Cloudflare Workers + Redis)

**Cibles :**
- 10 professionnels pilotes (kinÃ©s, artisans, restaurateurs)
- 100 clients finaux (rÃ©servations)

**MÃ©triques de succÃ¨s :**
- Taux de conversion : >60% (visiteur â†’ rÃ©servation)
- Taux d'annulation : <15%
- Satisfaction client : >4/5

---

### 7.2 Phase 2 (12 mois) : Progressive Web App

**PrioritÃ© : ExpÃ©rience mobile et notifications**

**Livrables :**
- PWA installable (manifest.json)
- Notifications push (Web Push API)
- Service Worker (cache crÃ©neaux offline)
- IntÃ©gration calendrier systÃ¨me (iCal, Google Calendar)

**Cibles :**
- 100 professionnels
- 1000 clients finaux

**MÃ©triques de succÃ¨s :**
- Taux d'installation PWA : >30%
- Taux d'activation notifications : >50%
- RÃ©duction no-show : -20% (grÃ¢ce aux notifications)

---

### 7.3 Phase 3 (24 mois) : COG Android/iOS lÃ©ger

**PrioritÃ© : SouverainetÃ© client et offline complet**

**Livrables :**
- App "Miyukini Client" (App Store / Play Store)
- Mini COG lÃ©ger (client invitÃ©)
- Protocoles Inter-COG (Passeport/Visa)
- DÃ©couverte Webway (rÃ©seau de COG)

**Cibles :**
- 500 professionnels
- 10 000 clients finaux

**MÃ©triques de succÃ¨s :**
- Taux d'installation app : >40%
- Offline utilisÃ© : >20% des consultations
- NPS (Net Promoter Score) : >60

---

### 7.4 Phase 4 (36-48 mois) : FÃ©dÃ©ration Inter-COG

**PrioritÃ© : Vision finale et Ã©cosystÃ¨me complet**

**Livrables :**
- COG complet mobile (Android/iOS)
- FÃ©dÃ©ration Inter-COG opÃ©rationnelle
- Synchronisation bi-directionnelle (JayKoa, JayKonta)
- Paiements Inter-COG sÃ©curisÃ©s

**Cibles :**
- 5 000 professionnels
- 100 000 citoyens avec COG complet

**MÃ©triques de succÃ¨s :**
- Taux d'adoption COG complet : >10% des clients
- RÃ©seau Webway : >1000 COG fÃ©dÃ©rÃ©s
- Transactions Inter-COG : >10 000/mois

---

## 8. Matrice de dÃ©cision par profil client

### 8.1 Quel parcours pour quel client ?

| Profil client | StratÃ©gie recommandÃ©e | Justification |
|---------------|----------------------|---------------|
| **Client occasionnel** (1-2 RDV/an) | **StratÃ©gie 1 : Web** | Pas de friction, lien direct, pas d'installation |
| **Client rÃ©gulier** (1 RDV/mois) | **StratÃ©gie 2 : PWA** | Notifications utiles, installation lÃ©gÃ¨re |
| **Client fidÃ¨le** (plusieurs pro : kinÃ© + dentiste + coiffeur) | **StratÃ©gie 3 : Mini COG** | Centralisation RDV, offline, souverainetÃ© |
| **Citoyen COG** (utilise dÃ©jÃ  Miyukini pour autres services) | **StratÃ©gie 4 : FÃ©dÃ©ration** | IntÃ©gration JayKoa/JayKonta, Ã©cosystÃ¨me complet |

### 8.2 Parcours de migration naturel

**ScÃ©nario rÃ©aliste :**

1. **Paul dÃ©couvre JayRDV via lien web** (StratÃ©gie 1)
   - Premier RDV avec Marie la kinÃ©
   - ExpÃ©rience fluide, pas de friction

2. **Paul devient client rÃ©gulier** â†’ Installe PWA (StratÃ©gie 2)
   - Marie propose "Installez l'app pour recevoir des rappels"
   - Paul accepte, installe PWA

3. **Paul consulte plusieurs professionnels** â†’ Installe Mini COG (StratÃ©gie 3)
   - Paul a maintenant 3 pros : kinÃ© + dentiste + coiffeur
   - App web suggÃ¨re "Centralisez tous vos RDV avec Miyukini Client"
   - Paul installe app native avec mini COG

4. **Paul adopte Miyukini pour d'autres besoins** â†’ COG complet (StratÃ©gie 4)
   - Paul dÃ©couvre JayKonta pour sa comptabilitÃ© perso
   - Paul installe Miyukini Central (desktop ou mobile complet)
   - Synchronisation automatique avec ses professionnels

---

## 9. Aspects techniques critiques

### 9.1 Gestion des conflits de rÃ©servation

**ProblÃ¨me :** Deux clients rÃ©servent le mÃªme crÃ©neau en mÃªme temps.

**Solution : Lock optimiste avec rÃ©solution automatique**

```rust
// Dans JayRDV (Strate 7)
pub fn reserve_slot(slot_id: SlotId, client: ClientInfo) -> Result<Booking, ConflictError> {
    // 1. StrongFather Ã©value l'intention
    let intent = Intent::ReserveSlot { slot_id, client };
    let decision = strongfather::evaluate(intent)?;

    // 2. KindMother vÃ©rifie disponibilitÃ© (lock optimiste)
    let slot = kindmother::read_slot(slot_id)?;
    if slot.status != SlotStatus::Available {
        // CrÃ©neau dÃ©jÃ  pris, proposer alternatives
        let alternatives = find_nearby_slots(slot_id, 3)?;
        return Err(ConflictError::SlotTaken { alternatives });
    }

    // 3. KindMother persiste (transaction SQLite)
    let booking = kindmother::write_booking(slot_id, client)?;

    // 4. MiyuNotify envoie confirmation
    miyunotify::send_sms(client.phone, "RDV confirmÃ© le {}", slot.datetime)?;

    Ok(booking)
}
```

**Comportement :**
- Si conflit : SMS "DÃ©solÃ©, crÃ©neau dÃ©jÃ  pris. Propositions : 14h45, 15h15, 16h00"
- Client choisit alternative en 1 clic

### 9.2 Gestion de l'identitÃ© sans compte

**ProblÃ¨me :** Comment identifier Paul sans lui imposer de crÃ©er un compte ?

**Solution : Identification par tÃ©lÃ©phone + token temporaire**

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  PremiÃ¨re rÃ©servation                                        â”‚
â”‚  - Paul saisit : nom + tÃ©lÃ©phone                             â”‚
â”‚  - SystÃ¨me envoie SMS OTP (One-Time Password)                â”‚
â”‚  - Paul valide OTP â†’ Token temporaire (JWT 90 jours)         â”‚
â”‚  - Token stockÃ© dans cookie/localStorage                     â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  RÃ©servations suivantes                                      â”‚
â”‚  - Paul a dÃ©jÃ  le token (cookie)                             â”‚
â”‚  - Formulaire prÃ©-rempli : "Paul Dupont - 06 12 34 56 78"   â”‚
â”‚  - RÃ©servation en 1 clic                                     â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**Avantages :**
- Pas de mot de passe Ã  retenir
- Validation SMS = anti-spam
- Token rÃ©vocable si abus

### 9.3 Gestion des annulations et modifications

**RÃ¨gles mÃ©tier :**
- Client peut annuler jusqu'Ã  24h avant (lien unique dans SMS)
- Client peut modifier jusqu'Ã  48h avant (mÃªme process que rÃ©servation)
- AprÃ¨s deadline : annulation impossible (ou avec frais si JayKonta intÃ©grÃ©)

**Architecture :**
```
Paul clique "Annuler RDV" (lien SMS)
    â†“
MiyuWeb vÃ©rifie token unique
    â†“
BorderGuard vÃ©rifie dÃ©lai (>24h ?)
    â†“
StrongFather Ã©value intention annulation
    â†“
KindMother libÃ¨re crÃ©neau (SlotStatus::Available)
    â†“
MiyuNotify SMS confirmation annulation + notification Marie
```

---

## 10. Comparaison avec solutions existantes

### 10.1 Benchmark concurrentiel

| Solution | Client voit | Pro utilise | Offline | SouverainetÃ© | CoÃ»t pro |
|----------|-------------|-------------|---------|--------------|----------|
| **Calendly** | Site web | Calendly SaaS | âŒ | âŒ Cloud US | â‚¬8-12/mois |
| **Doctolib** | Site + app | Doctolib SaaS | âŒ | âŒ Cloud FR | â‚¬129/mois |
| **SimplyBook** | Site web | SimplyBook SaaS | âŒ | âŒ Cloud | â‚¬8-50/mois |
| **JayRDV Phase 1** | Site web | COG Miyukini | âš ï¸ Buffer | âœ… Chez le pro | â‚¬10-30/mois |
| **JayRDV Phase 2** | PWA | COG Miyukini | âš ï¸ Partiel | âœ… Chez le pro | â‚¬10-30/mois |
| **JayRDV Phase 3** | Mini COG | COG Miyukini | âœ… Complet | âœ… Les deux | â‚¬10-30/mois |

### 10.2 Proposition de valeur unique

**Ce que JayRDV apporte vs concurrents :**
- âœ… **SouverainetÃ© des donnÃ©es** : Pro garde ses donnÃ©es chez lui (mini PC, NAS)
- âœ… **Offline-first** : Fonctionne sans rÃ©seau (Ã©vÃ©nements, zones isolÃ©es)
- âœ… **CoÃ»t maÃ®trisÃ©** : Pas d'abonnement cloud obligatoire (licence perpÃ©tuelle possible)
- âœ… **InterpolaritÃ©** : S'intÃ¨gre avec JayKoa (agenda), JayKonta (compta), JayXpose (vitrine)
- âœ… **Gouvernance** : BorderGuard, StrongFather, anti-spam structurel

**Cibles privilÃ©giÃ©es :**
- Professionnels sensibles Ã  la souverainetÃ© (mÃ©decins, avocats, collectivitÃ©s)
- Professionnels en zones isolÃ©es (rural, montagne)
- Professionnels Ã©vÃ©nementiels (festivals, marchÃ©s)

---

## 11. Prochaines Ã©tapes

### 11.1 Validation marchÃ©

**Actions immÃ©diates :**
1. **Interviews** : 20 professionnels cibles (kinÃ©s, artisans, restaurateurs)
   - Quel systÃ¨me utilisent-ils ? (Calendly, papier, Google Calendar ?)
   - Quels pain points ? (no-show, oublis, double booking ?)
   - Accepteraient-ils une solution Miyukini ?

2. **Prototype Figma** : Parcours web (5 Ã©crans)
   - DÃ©couverte (lien/QR code)
   - Calendrier crÃ©neaux
   - Formulaire rÃ©servation
   - Confirmation
   - Gestion (annulation/modification)

3. **MVP technique** : Site web de dÃ©mo (2 semaines)
   - MiyuWeb + JayRDV mockÃ©s
   - CrÃ©neaux disponibles (hard-coded)
   - Formulaire rÃ©servation (sans persistance)
   - SMS simulation (console log)

### 11.2 DÃ©cisions architecturales Ã  valider

**Questions ouvertes :**
1. **Buffer offline** : Redis/SQLite ou abandon si COG offline ?
2. **DNS dynamique** : Cloudflare Tunnel ou No-IP ?
3. **SMS provider** : Twilio, Vonage ou OVH Telecom ?
4. **Domaines** : Sous-domaines gÃ©nÃ©riques (marie-kine.jaykoa.com) ou domaines perso (marie-dupont.fr) ?
5. **Multilingue** : FR uniquement en MVP ou EN/FR dÃ¨s le dÃ©but ?

---

## 12. Conclusion

### 12.1 StratÃ©gie recommandÃ©e

**Phase 1 (MVP) : Interface Web publique**
- PrioritÃ© absolue : validation marchÃ©
- Time-to-Market : 6 mois
- Cible : 10 professionnels pilotes

**Phase 2 (12 mois) : Progressive Web App**
- AmÃ©lioration UX mobile
- Notifications push
- Cible : 100 professionnels

**Phase 3 (24 mois) : COG Android/iOS lÃ©ger**
- SouverainetÃ© client
- Offline complet
- Cible : 500 professionnels

**Phase 4 (36+ mois) : FÃ©dÃ©ration Inter-COG**
- Vision finale
- Ã‰cosystÃ¨me complet
- Cible : 5000 professionnels

### 12.2 Principe directeur

> **"Le client ne doit jamais Ãªtre contraint d'installer un COG pour bÃ©nÃ©ficier d'un Service. Mais s'il le fait, il en tire une valeur supplÃ©mentaire (souverainetÃ©, offline, interpolaritÃ©)."**

---

## 13. RÃ©fÃ©rences

| ThÃ¨me | Document |
|-------|----------|
| **Protocoles Inter-COG** | [Connexion Inter-COG](..//..//_index.md) |
| **Webway** | [Tools et Toolkits](..//..//_index.md) (MiyuWebwayParticipant/Tracker) |
| **BorderGuard** | [Pyramide Architecture](..//..//_index.md) |
| **Lois d'autonomie** | [Lois Autonomie Systeme](..//..//_index.md) |

---

**Date de crÃ©ation :** 2026-02-07
**Version :** 1.0
**Statut :** Document de conception â€” AccessibilitÃ© client et parcours de rÃ©servation

