# Miyukini Web Portal â€” Document Fondateur

## Contexte

**Miyukini Web Portal** (le **Portail**) est le **Service Fondamental** qui constitue le point d'entrÃ©e web pour les utilisateurs externes. Il est l'Ã©quivalent de Miyukini Central pour le monde extÃ©rieur : lÃ  oÃ¹ Central sert l'utilisateur du COG, le Portail sert les utilisateurs externes qui accÃ¨dent via un navigateur web.

Le Portail expose les **FaÃ§ades Publiques GouvernÃ©es** des Services de Type 2 (Ã  surface web externe) sous contrÃ´le strict de la gouvernance COG (BorderGuard, StrongFather, WorrySentinel).

**RÃ¨gle canonique :**

> **Central = COG, Portail = Web.**

## PortÃ©e / Scope

- **Applicable Ã  :** Conception, dÃ©veloppement et exploitation du Portail Miyukini Web Portal
- **Audience :** Architectes, dÃ©veloppeurs, designers UX/UI, responsables sÃ©curitÃ©
- **Statut :** Document de rÃ©fÃ©rence fondateur â€” Service Fondamental

### Couvert par ce document

- Positionnement et rÃ´le du Portail
- Relation avec Central et les Services
- Architecture et gouvernance
- Flux et sÃ©curitÃ©
- Identification et fichage des connexions

### Hors scope

- ImplÃ©mentation technique dÃ©taillÃ©e (voir guide d'implÃ©mentation)
- UX/UI dÃ©taillÃ© des surfaces des Services (voir chaque Service)

---

## 1. DÃ©finition et Position dans la Pyramide

### 1.1 Nature du Miyukini Web Portal

| Aspect | DÃ©finition |
|--------|------------|
| **Type** | Service Fondamental â€” OpÃ©rateur d'Interface (Strate 7) |
| **RÃ´le** | Expose les FaÃ§ades Publiques des Services (Type 2) aux utilisateurs externes via le web |
| **Question fondamentale** | *Â« Comment les utilisateurs externes accÃ¨dent-ils aux surfaces web du COG ? Â»* |
| **Cible** | Utilisateurs externes (clients, visiteurs, prospects) â€” sans COG, via navigateur |
| **AutoritÃ©** | Aucune autoritÃ© propre â€” applique les dÃ©cisions des Cores (BorderGuard, StrongFather) |

**Phrase fondatrice :**

> **Le Portail est la porte d'entrÃ©e web du COG. L'utilisateur externe y accÃ¨de ; le COG sort vers lui, jamais l'inverse.**

### 1.2 Ce que le Portail N'EST PAS

| âŒ N'est pas | Pourquoi |
|--------------|----------|
| Un serveur central unique | Chaque COG expose **son** Portail ; pas de "super-serveur" qui affiche tous les COGs |
| Un remplacement de Central | Central reste le point d'entrÃ©e pour l'utilisateur du COG |
| Une porte ouverte | Tout accÃ¨s passe par BorderGuard, identification et Mandat Public d'AccÃ¨s |
| Une source de vÃ©ritÃ© | La vÃ©ritÃ© est dans les Cores ; le Portail relaie et affiche |

### 1.3 Position dans la Pyramide Miyukini

```
STRATE 7 â€” OpÃ©rateurs
â”œâ”€â”€ Miyukini Central (Service Fondamental) â† Point d'entrÃ©e COG
â”œâ”€â”€ Miyukini Web Portal (Service Fondamental) â† Point d'entrÃ©e Web
â”œâ”€â”€ JayXpose, JayFestival, JayRDV, JayKonta, ... (Services Type 1/2/3)
â””â”€â”€ â€¦

STRATE 9 â€” MiyukiniAdmin (exception, administration systÃ¨me)
```

---

## 2. Relation Central / Portail

### 2.1 DualitÃ© fondamentale

| Aspect | Miyukini Central | Miyukini Web Portal |
|--------|------------------|---------------------|
| **Cible** | Utilisateur du COG | Utilisateurs externes (web) |
| **AccÃ¨s** | Application desktop/web locale | Navigateur web distant |
| **Authentification** | IdentitÃ© COG souveraine | Sans identitÃ© COG (FaÃ§ade Publique) |
| **Gouvernance** | Mandat de Permission | Mandat Public d'AccÃ¨s |
| **RÃ´le** | Gestion, administration, crÃ©ation | Consultation, rÃ©servation, achat |

### 2.2 SchÃ©ma de relation

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                    Utilisateur du COG                                â”‚
â”‚  (propriÃ©taire, administrateur, gestionnaire)                       â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚
                            â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                    Miyukini Central                                  â”‚
â”‚  Â· Gestion, administration, crÃ©ation                                â”‚
â”‚  Â· AccÃ¨s complet aux Services (Type 1, 2, 3)                        â”‚
â”‚  Â· PrÃ©pare les contenus exposÃ©s au Portail                          â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚
                            â”‚ Les Services de Type 2 exposent
                            â”‚ leurs capacitÃ©s au Portail
                            â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                    Miyukini Web Portal                               â”‚
â”‚  Â· FaÃ§ades Publiques GouvernÃ©es                                     â”‚
â”‚  Â· Surfaces web des Services (vitrine, rÃ©servation, catalogue...)  â”‚
â”‚  Â· Identification et fichage des connexions entrantes               â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚
                            â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                    Utilisateurs externes                             â”‚
â”‚  (clients, visiteurs, prospects â€” sans COG)                         â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 2.3 RÃ¨gle de cohabitation

> **Central et Portail sont tous deux des Services Fondamentaux. Leur prÃ©sence fait partie de l'environnement versionnÃ© du COG.**

Un COG sans Central ne peut pas Ãªtre administrÃ©.  
Un COG sans Portail ne peut pas exposer de surfaces web (Services Type 2 limitÃ©s Ã  leur facette interne).

---

## 3. Architecture

### 3.1 Architecture logique

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚            Utilisateur externe (navigateur web)                      â”‚
â”‚  https://vitrine.mon-commerce.cog                                   â”‚
â”‚  https://rdv.kine-paris.cog                                         â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚ HTTPS
                            â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚              Miyukini Web Portal (Portail)                           â”‚
â”‚  Â· MiyuWeb (rendu HTML, layout, formulaires)                        â”‚
â”‚  Â· Routage vers les surfaces des Services                           â”‚
â”‚  Â· Identification et fichage (session, IP, scope)                   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚ BondingBrother
                            â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                    BorderGuard (Strate 4)                            â”‚
â”‚  Â· DÃ©finition des frontiÃ¨res et niveaux de confiance                â”‚
â”‚  Â· Mandat Public d'AccÃ¨s                                            â”‚
â”‚  Â· RÃ¨gles de franchissement                                         â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚
              â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
              â–¼             â–¼             â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â” â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ StrongFather     â”‚ â”‚ KindMother       â”‚ â”‚ WorrySentinel    â”‚
â”‚ DÃ©cision ALLOW/  â”‚ â”‚ DonnÃ©es          â”‚ â”‚ SÃ©curitÃ©         â”‚
â”‚ DENY             â”‚ â”‚ (lecture seule)  â”‚ â”‚ (Ã©tat, niveau)   â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜ â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚
                            â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Services (JayXpose, JayFestival, JayRDV, JayKonta)                  â”‚
â”‚  Â· Exposent leurs capacitÃ©s (lecture, formulaires, actions bornÃ©es) â”‚
â”‚  Â· Jamais d'accÃ¨s direct aux Cores depuis l'extÃ©rieur               â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 3.2 Flux typiques

| Flux | Description |
|------|-------------|
| **Consultation vitrine** | Utilisateur externe â†’ Portail â†’ BorderGuard â†’ JayXpose (lecture catalogue, pages) â†’ rendu HTML |
| **RÃ©servation RDV** | Utilisateur externe â†’ Portail â†’ BorderGuard â†’ JayRDV (crÃ©neaux, formulaire, crÃ©ation RDV) |
| **Achat e-shop** | Utilisateur externe â†’ Portail â†’ BorderGuard â†’ JayXpose (panier, commande) â†’ KindMother (persist) |
| **Consultation facture** | Client (lien email) â†’ Portail â†’ BorderGuard (token) â†’ JayKonta (lecture facture) |

---

## 4. Gouvernance et SÃ©curitÃ©

### 4.1 Principes fondamentaux

| Principe | RÃ¨gle |
|----------|-------|
| **FaÃ§ade Publique GouvernÃ©e** | Le Portail expose une zone tampon ; l'utilisateur externe n'entre jamais dans le COG |
| **Mandat Public d'AccÃ¨s** | Toute surface exposÃ©e est sous Mandat (quotas, rate limits, mÃ©thodes autorisÃ©es) |
| **Identification obligatoire** | Chaque connexion entrante est identifiÃ©e et fichÃ©e (session, IP, timestamp, scope) |
| **Pas d'accÃ¨s aux Cores** | L'utilisateur externe n'accÃ¨de jamais directement aux Cores ; uniquement via capacitÃ©s exposÃ©es |

### 4.2 Identification et fichage des connexions

**RÃ¨gle :**

> **Par sÃ©curitÃ©, chaque connexion entrante est identifiÃ©e et fichÃ©e.**

| DonnÃ©e collectÃ©e | Usage |
|------------------|-------|
| **Session ID** | Identification de la session (token opaque) |
| **IP / origine** | TraÃ§abilitÃ©, rate limiting, dÃ©tection d'abus |
| **Timestamp** | Horodatage de la connexion |
| **Scope / Visa** | Niveau d'accÃ¨s accordÃ© (S1/S2 selon Mandat Public) |
| **Actions effectuÃ©es** | Audit (pages consultÃ©es, formulaires soumis) |

**Stockage :** KindMother (logs de sÃ©curitÃ©), non exposÃ© Ã  l'utilisateur externe.

### 4.3 Niveaux de sÃ©curitÃ© (WorrySentinel)

| Niveau | Usage typique sur le Portail |
|--------|------------------------------|
| **0 â€” Public** | Lecture catalogue, pages vitrine |
| **1 â€” Standard** | Formulaire de contact, rÃ©servation simple |
| **2 â€” Sensitive** | Paiement, donnÃ©es personnelles |

Les niveaux 3+ (Critical, Highest) ne sont pas exposÃ©s via le Portail (rÃ©servÃ©s Ã  Central / Admin).

### 4.4 DÃ©gradation et protection

| Action | DÃ©clencheur | Effet |
|--------|-------------|-------|
| **Throttle** | RequÃªtes trop frÃ©quentes | Ralentissement |
| **Downgrade** | Comportement suspect | FonctionnalitÃ©s rÃ©duites |
| **Freeze** | Anomalie dÃ©tectÃ©e | Lecture seule |
| **Block** | Abus confirmÃ© | IP / session bloquÃ©e |
| **Blackhole** | Attaque | RÃ©ponse neutre, pas d'erreur exploitable |

---

## 5. Relation avec le Maillage Webway

### 5.1 RÃ´le du COG Tracker

Le **COG Tracker** (Maillage Webway) peut cataloguer les Portails des COGs participants :

- **DÃ©claration** : un COG dÃ©clare son Portail (adresse, services exposÃ©s)
- **DÃ©couverte** : d'autres COGs ou utilisateurs peuvent dÃ©couvrir les Portails disponibles
- **Pas de gouvernance** : le Tracker ne gouverne pas ; il rÃ©fÃ©rence et facilite la dÃ©couverte

### 5.2 Portail et Inter-COG

Le Portail sert les **utilisateurs externes sans COG**.

Pour les **utilisateurs d'autres COGs** (Visite gouvernÃ©e inter-COG), c'est le mÃ©canisme de Passeport / Visa qui s'applique, pas le Portail. Le Portail est rÃ©servÃ© aux utilisateurs non-COG.

---

## 6. Services exposÃ©s via le Portail

### 6.1 Services de Type 2 (Ã  surface web externe)

Seuls les Services de **Type 2** exposent une surface via le Portail.

| Service | Surface Portail | Parcours typiques |
|---------|-----------------|-------------------|
| **JayXpose** | Vitrine, e-shop, annuaire, blog | Consultation catalogue, achat, contact |
| **JayFestival** | Catalogue Ã©vÃ©nements, billets, espace visiteur | DÃ©couverte, inscription, rÃ©servation |
| **JayRDV** | Page rÃ©servation, parcours guest | Choix crÃ©neau, formulaire, confirmation |
| **JayKonta** | Portail client | Consultation factures, paiement en ligne |

### 6.2 Ce qui n'est PAS exposÃ©

| Service / Fonction | Exposition Portail |
|--------------------|--------------------|
| **JayKoa** (agenda personnel) | âŒ Interne uniquement |
| **Administration comptable** (JayKonta) | âŒ Central uniquement |
| **Gestion exposant** (JayXpose) | âŒ Central uniquement |
| **MiyukiniAdmin** | âŒ Jamais exposÃ© |

---

## 7. Type de Service et Espaces

| Aspect | Valeur |
|--------|--------|
| **Type** | Service Fondamental |
| **Description** | Point d'entrÃ©e web pour les utilisateurs externes |
| **Espace Central** | âœ… Administration du Portail (paramÃ¨tres, logs, monitoring) |
| **Espace Portail** | âœ… Le Portail lui-mÃªme (point d'entrÃ©e web) |
| **Espace Inter-COG** | âŒ Non concernÃ© (les visites inter-COG passent par Passeport/Visa, pas par le Portail) |

---

## 8. Outils et Kits d'Outils

Le Portail utilise les Outils suivants (Strate 6) :

| Outil / Kit | Usage |
|-------------|-------|
| **MiyuWeb** | Rendu HTML, layout, formulaires, Ã©vÃ©nements |
| **MiyuValidate** | Validation des entrÃ©es utilisateur |
| **MiyuAntiSpam** | Rate limiting, captcha, flood protection |
| **BorderGuard (capacitÃ©s)** | RÃ¨gles de franchissement, niveaux de confiance |

---

## 9. SynthÃ¨se

| Dimension | Miyukini Web Portal |
|-----------|---------------------|
| **Type** | Service Fondamental â€” OpÃ©rateur d'Interface (Strate 7) |
| **RÃ´le** | Point d'entrÃ©e web pour les utilisateurs externes |
| **Cible** | Clients, visiteurs, prospects â€” sans COG |
| **Gouvernance** | BorderGuard + Mandat Public d'AccÃ¨s + Identification/Fichage |
| **Services exposÃ©s** | Type 2 (JayXpose, JayFestival, JayRDV, JayKonta portail client) |
| **Outils** | MiyuWeb, MiyuValidate, MiyuAntiSpam |
| **Relation Central** | Central = COG, Portail = Web |

---

## 10. RÃ©fÃ©rences

| Document | Lien |
|----------|------|
| **Miyukini Central** | [Miyukini Central Hub Services](..//..//miyukini-webway-system//reference//_index.md) |
| **Types de Services** | [Types de Services et Espaces](..//..//miyukini-webway-system//reference//_index.md) |
| **Glossaire** | [Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| **FaÃ§ade Publique** | [Glossaire Â§ FaÃ§ade Publique GouvernÃ©e](..//..//miyukini-webway-system//reference//_index.md) |
| **BorderGuard** | [docs/core/BorderGuard](..//..//cores//BorderGuard//) |
| **SouverainetÃ©** | [SouverainetÃ© Environnement](..//..//miyukini-webway-system//reference//_index.md) |

---

**Date de crÃ©ation :** 2026-02-08  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur â€” Service Fondamental Miyukini Web Portal


