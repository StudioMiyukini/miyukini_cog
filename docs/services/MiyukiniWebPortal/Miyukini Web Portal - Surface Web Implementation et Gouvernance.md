# Miyukini Web Portal â€” Surface Web Implementation et Gouvernance

## Contexte

Ce document dÃ©finit **comment implÃ©menter, guider, borner et normer** une surface web exposÃ©e via le Portail. Il s'adresse aux Ã©quipes qui conÃ§oivent ou implÃ©mentent des Services de Type 2 (Ã  surface web externe).

**Objectif :** Fournir un cadre clair pour que chaque surface web soit cohÃ©rente, sÃ©curisÃ©e, gouvernÃ©e et conforme aux principes Miyukini.

## PortÃ©e / Scope

- **Applicable Ã  :** Tout Service de Type 2 exposant une surface web via le Portail
- **Audience :** Architectes, dÃ©veloppeurs, concepteurs produit, QA
- **Statut :** Guide de rÃ©fÃ©rence normatif

---

## 1. Comment ImplÃ©menter une Surface Web

### 1.1 Principe fondamental

> **Un Service de Type 2 ne sert pas HTTP directement. Il expose des capacitÃ©s que le Portail consomme et rend accessibles via le web.**

Le Portail est l'unique point d'entrÃ©e HTTP pour les utilisateurs externes. Les Services fournissent des **capacitÃ©s** (APIs internes, donnÃ©es, flux) que le Portail orchestre.

### 1.2 Contrat d'exposition

Chaque Service de Type 2 doit dÃ©finir un **contrat d'exposition** :

| Ã‰lÃ©ment | Description |
|---------|-------------|
| **CapacitÃ©s exposÃ©es** | Liste des capacitÃ©s accessibles via le Portail (lecture, actions) |
| **DonnÃ©es exposÃ©es** | Quelles donnÃ©es peuvent Ãªtre lues par l'utilisateur externe |
| **Actions autorisÃ©es** | Quelles actions l'utilisateur externe peut effectuer (formulaire, rÃ©servation, achat) |
| **Niveau de sÃ©curitÃ©** | Niveau WorrySentinel requis (0, 1, 2) |
| **Quotas / Limites** | Rate limiting, taille des requÃªtes, etc. |

**Exemple â€” JayXpose :**

```yaml
service: JayXpose
surface_web:
  capacites_exposees:
    - catalogue.list.public
    - produit.get.public
    - page.get.public
    - contact.form.submit
  donnees_exposees:
    - Catalogue produits (public)
    - Pages vitrine (publiÃ©es)
    - Informations contact (publiques)
  actions_autorisees:
    - Consultation catalogue
    - Lecture pages
    - Soumission formulaire contact
    - Ajout panier (si e-shop)
    - Commande (avec paiement)
  niveau_securite:
    lecture: 0
    formulaire: 1
    paiement: 2
  quotas:
    requetes_par_minute: 60
    taille_max_requete: 1MB
```

### 1.3 Architecture d'implÃ©mentation

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Portail (MiyuWeb)                                                   â”‚
â”‚  Â· ReÃ§oit les requÃªtes HTTP                                         â”‚
â”‚  Â· Route vers le bon Service                                         â”‚
â”‚  Â· Applique le Mandat Public d'AccÃ¨s                                â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚ Appel interne (BondingBrother)
                            â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Service (ex. JayXpose)                                              â”‚
â”‚  Â· Expose des capacitÃ©s via API interne                             â”‚
â”‚  Â· Jamais d'accÃ¨s HTTP direct depuis l'extÃ©rieur                    â”‚
â”‚  Â· Logique mÃ©tier gouvernÃ©e                                         â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
                            â”‚
                            â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚  Cores (StrongFather, KindMother, BorderGuard)                       â”‚
â”‚  Â· DÃ©cision, persistance, gouvernance                               â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 1.4 Stack technique recommandÃ©e

| Composant | Outil / Technologie |
|-----------|---------------------|
| **Rendu HTML** | MiyuWeb (Toolkit) |
| **Layout / ThÃ¨me** | MiyuWeb layout, theme |
| **Formulaires** | MiyuWeb form, MiyuValidate |
| **Protection** | MiyuAntiSpam (rate limiting, captcha) |
| **Persistance** | KindMother (via le Service) |

### 1.5 Identification et fichage

**Obligation :** Toute requÃªte entrante doit Ãªtre identifiÃ©e et fichÃ©e **avant** d'atteindre la logique mÃ©tier.

```rust
// Pseudo-code â€” identification au niveau Portail
fn handle_request(req: Request) -> Response {
    // 1. Identification
    let session = identify_or_create_session(&req);
    let connection_log = ConnectionLog {
        session_id: session.id,
        ip: req.remote_addr(),
        timestamp: now(),
        path: req.path(),
        user_agent: req.user_agent(),
    };
    
    // 2. Fichage (persistance)
    kindmother.log_connection(connection_log);
    
    // 3. VÃ©rification Mandat Public
    let mandat = borderguard.check_public_access(&req, &session)?;
    
    // 4. Routage vers le Service
    let response = route_to_service(&req, &session, &mandat)?;
    
    // 5. Fichage de la rÃ©ponse
    kindmother.log_response(&session, &response);
    
    response
}
```

---

## 2. Comment Guider une Surface Web

### 2.1 Principe

> **Chaque surface web doit offrir un parcours utilisateur clair : dÃ©couverte â†’ action â†’ confirmation.**

### 2.2 Documentation des parcours

Chaque Service de Type 2 doit documenter ses **parcours utilisateur externe** dans sa documentation (ex. `publics/UtilisateurNonConnecte/`).

| Ã‰lÃ©ment | Description |
|---------|-------------|
| **Point d'entrÃ©e** | Comment l'utilisateur arrive (lien, QR code, recherche) |
| **Parcours principal** | Ã‰tapes du parcours (ex. choix produit â†’ panier â†’ commande) |
| **Points de sortie** | Confirmation, redirection, erreur |
| **Passerelles** | Liens vers inscription/connexion si applicable |

### 2.3 CohÃ©rence avec Central

Le contenu exposÃ© sur le Portail doit Ãªtre **prÃ©parÃ©** dans Central :

| Central (gestion) | Portail (exposition) |
|-------------------|----------------------|
| CrÃ©er une page vitrine | Page affichÃ©e publiquement |
| Publier un produit | Produit visible dans le catalogue |
| Configurer les crÃ©neaux RDV | CrÃ©neaux affichÃ©s pour rÃ©servation |
| RÃ©diger un article | Article publiÃ© sur le blog |

### 2.4 Liens et routage

Le Portail doit fournir un **routage clair** vers les surfaces des Services :

```
https://moncommerce.cog/                â†’ Page d'accueil (JayXpose)
https://moncommerce.cog/catalogue       â†’ Catalogue (JayXpose)
https://moncommerce.cog/produit/123     â†’ Fiche produit (JayXpose)
https://moncommerce.cog/contact         â†’ Formulaire contact (JayXpose)
https://rdv.kine.cog/                   â†’ Page rÃ©servation (JayRDV)
https://festival.event.cog/             â†’ Catalogue Ã©vÃ©nement (JayFestival)
```

---

## 3. Comment Borner une Surface Web

### 3.1 Principe

> **Toute surface web doit avoir des frontiÃ¨res strictes : ce qui est exposÃ© (liste blanche) vs ce qui reste interne.**

### 3.2 RÃ¨gles de bornage

| RÃ¨gle | Description |
|-------|-------------|
| **Liste blanche** | Seules les capacitÃ©s explicitement dÃ©clarÃ©es sont exposÃ©es |
| **Pas d'accÃ¨s aux Cores** | L'utilisateur externe n'accÃ¨de jamais aux Cores directement |
| **Pas de donnÃ©es d'autres utilisateurs** | Un utilisateur externe ne voit que les donnÃ©es publiques ou les siennes |
| **Pas d'actions administratives** | CrÃ©ation, modification, suppression = Central uniquement |

### 3.3 Checklist de bornage

Pour chaque parcours web, vÃ©rifier :

| # | Question | âœ… / âŒ |
|---|----------|--------|
| 1 | Les capacitÃ©s exposÃ©es sont-elles explicitement listÃ©es ? | |
| 2 | Les donnÃ©es sensibles sont-elles masquÃ©es ? | |
| 3 | Les actions autorisÃ©es sont-elles limitÃ©es (lecture, formulaire, achat) ? | |
| 4 | Le rate limiting est-il configurÃ© ? | |
| 5 | L'identification des connexions est-elle active ? | |
| 6 | Les erreurs ne rÃ©vÃ¨lent-elles pas d'information sensible ? | |

### 3.4 BorderGuard et Mandat Public d'AccÃ¨s

Chaque surface web est gouvernÃ©e par un **Mandat Public d'AccÃ¨s** :

```yaml
mandat_public:
  service: JayXpose
  surface: vitrine
  allowed_methods:
    - GET (lecture)
    - POST (formulaires)
  quotas:
    requests_per_minute: 60
    max_request_size: 1MB
  security_level: 0-2
  expected_behavior:
    - Consultation catalogue
    - Soumission formulaire
    - Commande (niveau 2)
```

### 3.5 Ce qui n'est JAMAIS exposÃ©

| Ã‰lÃ©ment | Raison |
|---------|--------|
| **DonnÃ©es d'autres clients** | Protection vie privÃ©e |
| **Identifiants internes** | SÃ©curitÃ© |
| **Logs systÃ¨me** | SÃ©curitÃ© |
| **Configuration** | SÃ©curitÃ© |
| **Actions admin** | RÃ©servÃ© Ã  Central |
| **Cores** | Architecture fondamentale |

---

## 4. Comment Normer une Surface Web

### 4.1 Normes techniques

#### 4.1.1 Formats d'API

| Aspect | Norme |
|--------|-------|
| **Format de donnÃ©es** | JSON (UTF-8) |
| **Codes HTTP** | Standards (200, 400, 401, 403, 404, 429, 500) |
| **Identifiants** | UUID v4 ou slug (publics uniquement) |
| **Pagination** | `?page=1&limit=20` |
| **Erreurs** | `{ "error": "code", "message": "description" }` â€” jamais de stacktrace |

#### 4.1.2 Sessions et tokens

| Aspect | Norme |
|--------|-------|
| **Session ID** | Token opaque (UUID), cookie HttpOnly Secure |
| **DurÃ©e de session** | Configurable (dÃ©faut : 1h consultation, 30min action sensible) |
| **Token d'action** | Token unique, temporaire, non devinable (ex. lien annulation RDV) |

### 4.2 Normes UX

#### 4.2.1 AccessibilitÃ©

| Aspect | Norme |
|--------|-------|
| **Contraste** | WCAG AA minimum (4.5:1 texte, 3:1 grands textes) |
| **Navigation clavier** | Tab, Enter, Escape fonctionnels |
| **Lecteur d'Ã©cran** | Alt text, ARIA labels |
| **Responsive** | Mobile-first, breakpoints standards |

#### 4.2.2 LibellÃ©s et messages

| Ã‰lÃ©ment | Norme |
|---------|-------|
| **Boutons** | Verbes d'action clairs (Â« RÃ©server Â», Â« Envoyer Â», Â« Acheter Â») |
| **Erreurs** | Messages explicites, sans jargon technique |
| **Confirmations** | RÃ©capitulatif clair avant action dÃ©finitive |
| **Ã‰tats vides** | Message explicatif (Â« Aucun crÃ©neau disponible Â») |

### 4.3 Normes de sÃ©curitÃ©

#### 4.3.1 RÃ¨gles COG-ADAPT

| RÃ¨gle | Description |
|-------|-------------|
| **COG-ADAPT-01** | Tout accÃ¨s externe passe par Visa et BorderGuard â€” jamais d'accÃ¨s direct |
| **COG-ADAPT-06** | La complexitÃ© COG est cachÃ©e Ã  l'utilisateur final â€” l'UX reste simple |

#### 4.3.2 Protection des donnÃ©es

| Aspect | Norme |
|--------|-------|
| **DonnÃ©es personnelles** | Minimisation (ne collecter que le nÃ©cessaire) |
| **Stockage** | KindMother uniquement, chiffrÃ© si niveau 2+ |
| **Transmission** | HTTPS obligatoire |
| **Consentement** | Explicite pour collecte de donnÃ©es |

#### 4.3.3 Protection contre les abus

| Protection | ImplÃ©mentation |
|------------|----------------|
| **Rate limiting** | MiyuAntiSpam (par IP, par session) |
| **Captcha** | Sur formulaires sensibles (contact, inscription) |
| **Validation** | MiyuValidate (entrÃ©es utilisateur) |
| **CSRF** | Token par formulaire |
| **XSS** | Ã‰chappement systÃ©matique des sorties |

---

## 5. Checklist ComplÃ¨te â€” Nouvelle Surface Web

Avant de mettre en production une nouvelle surface web :

### 5.1 Documentation

| # | Ã‰lÃ©ment | âœ… |
|---|---------|---|
| 1 | Contrat d'exposition rÃ©digÃ© (capacitÃ©s, donnÃ©es, actions, niveau) | |
| 2 | Parcours utilisateur documentÃ© | |
| 3 | Checklist de bornage validÃ©e | |

### 5.2 ImplÃ©mentation

| # | Ã‰lÃ©ment | âœ… |
|---|---------|---|
| 4 | Routage via Portail (pas d'accÃ¨s direct au Service) | |
| 5 | Identification et fichage des connexions | |
| 6 | Mandat Public d'AccÃ¨s configurÃ© | |
| 7 | Rate limiting actif | |
| 8 | Validation des entrÃ©es (MiyuValidate) | |

### 5.3 SÃ©curitÃ©

| # | Ã‰lÃ©ment | âœ… |
|---|---------|---|
| 9 | HTTPS obligatoire | |
| 10 | Erreurs sans fuite d'information | |
| 11 | DonnÃ©es sensibles masquÃ©es | |
| 12 | Protection CSRF/XSS | |

### 5.4 UX

| # | Ã‰lÃ©ment | âœ… |
|---|---------|---|
| 13 | Responsive (mobile, tablette, desktop) | |
| 14 | AccessibilitÃ© WCAG AA | |
| 15 | LibellÃ©s clairs et cohÃ©rents | |
| 16 | Confirmations avant actions dÃ©finitives | |

---

## 6. RÃ©fÃ©rences

| Document | Lien |
|----------|------|
| **Document Fondateur Portail** | [Miyukini Web Portal - Document Fondateur](./Miyukini%20Web%20Portal%20-%20Document%20Fondateur.md) |
| **Types de Services** | [Types de Services et Espaces](..//..//miyukini-webway-system//reference//_index.md) |
| **Glossaire** | [Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| **FaÃ§ade Publique** | [Glossaire Â§ FaÃ§ade Publique GouvernÃ©e](..//..//miyukini-webway-system//reference//_index.md) |
| **MiyuWeb** | [docs/tools/MiyuWeb](../../tools/MiyuWeb/) |
| **MiyuAntiSpam** | [docs/tools/MiyuAntiSpam](../../tools/MiyuAntiSpam/) |

---

**Date de crÃ©ation :** 2026-02-08  
**Version :** 1.0  
**Statut :** Guide de rÃ©fÃ©rence normatif â€” ImplÃ©mentation et gouvernance des surfaces web

