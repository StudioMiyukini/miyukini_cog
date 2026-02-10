# Miyukini Web Portal — Surface Web Implementation et Gouvernance

## Contexte

Ce document définit **comment implémenter, guider, borner et normer** une surface web exposée via le Portail. Il s'adresse aux équipes qui conçoivent ou implémentent des Services de Type 2 (à surface web externe).

**Objectif :** Fournir un cadre clair pour que chaque surface web soit cohérente, sécurisée, gouvernée et conforme aux principes Miyukini.

## Portée / Scope

- **Applicable à :** Tout Service de Type 2 exposant une surface web via le Portail
- **Audience :** Architectes, développeurs, concepteurs produit, QA
- **Statut :** Guide de référence normatif

---

## 1. Comment Implémenter une Surface Web

### 1.1 Principe fondamental

> **Un Service de Type 2 ne sert pas HTTP directement. Il expose des capacités que le Portail consomme et rend accessibles via le web.**

Le Portail est l'unique point d'entrée HTTP pour les utilisateurs externes. Les Services fournissent des **capacités** (APIs internes, données, flux) que le Portail orchestre.

### 1.2 Contrat d'exposition

Chaque Service de Type 2 doit définir un **contrat d'exposition** :

| Élément | Description |
|---------|-------------|
| **Capacités exposées** | Liste des capacités accessibles via le Portail (lecture, actions) |
| **Données exposées** | Quelles données peuvent être lues par l'utilisateur externe |
| **Actions autorisées** | Quelles actions l'utilisateur externe peut effectuer (formulaire, réservation, achat) |
| **Niveau de sécurité** | Niveau WorrySentinel requis (0, 1, 2) |
| **Quotas / Limites** | Rate limiting, taille des requêtes, etc. |

**Exemple — JayXpose :**

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
    - Pages vitrine (publiées)
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

### 1.3 Architecture d'implémentation

```
┌─────────────────────────────────────────────────────────────────────┐
│  Portail (MiyuWeb)                                                   │
│  · Reçoit les requêtes HTTP                                         │
│  · Route vers le bon Service                                         │
│  · Applique le Mandat Public d'Accès                                │
└───────────────────────────┬─────────────────────────────────────────┘
                            │ Appel interne (BondingBrother)
                            ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Service (ex. JayXpose)                                              │
│  · Expose des capacités via API interne                             │
│  · Jamais d'accès HTTP direct depuis l'extérieur                    │
│  · Logique métier gouvernée                                         │
└───────────────────────────┬─────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────────┐
│  Cores (StrongFather, KindMother, BorderGuard)                       │
│  · Décision, persistance, gouvernance                               │
└─────────────────────────────────────────────────────────────────────┘
```

### 1.4 Stack technique recommandée

| Composant | Outil / Technologie |
|-----------|---------------------|
| **Rendu HTML** | MiyuWeb (Toolkit) |
| **Layout / Thème** | MiyuWeb layout, theme |
| **Formulaires** | MiyuWeb form, MiyuValidate |
| **Protection** | MiyuAntiSpam (rate limiting, captcha) |
| **Persistance** | KindMother (via le Service) |

### 1.5 Identification et fichage

**Obligation :** Toute requête entrante doit être identifiée et fichée **avant** d'atteindre la logique métier.

```rust
// Pseudo-code — identification au niveau Portail
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
    
    // 3. Vérification Mandat Public
    let mandat = borderguard.check_public_access(&req, &session)?;
    
    // 4. Routage vers le Service
    let response = route_to_service(&req, &session, &mandat)?;
    
    // 5. Fichage de la réponse
    kindmother.log_response(&session, &response);
    
    response
}
```

---

## 2. Comment Guider une Surface Web

### 2.1 Principe

> **Chaque surface web doit offrir un parcours utilisateur clair : découverte → action → confirmation.**

### 2.2 Documentation des parcours

Chaque Service de Type 2 doit documenter ses **parcours utilisateur externe** dans sa documentation (ex. `publics/UtilisateurNonConnecte/`).

| Élément | Description |
|---------|-------------|
| **Point d'entrée** | Comment l'utilisateur arrive (lien, QR code, recherche) |
| **Parcours principal** | Étapes du parcours (ex. choix produit → panier → commande) |
| **Points de sortie** | Confirmation, redirection, erreur |
| **Passerelles** | Liens vers inscription/connexion si applicable |

### 2.3 Cohérence avec Central

Le contenu exposé sur le Portail doit être **préparé** dans Central :

| Central (gestion) | Portail (exposition) |
|-------------------|----------------------|
| Créer une page vitrine | Page affichée publiquement |
| Publier un produit | Produit visible dans le catalogue |
| Configurer les créneaux RDV | Créneaux affichés pour réservation |
| Rédiger un article | Article publié sur le blog |

### 2.4 Liens et routage

Le Portail doit fournir un **routage clair** vers les surfaces des Services :

```
https://moncommerce.cog/                → Page d'accueil (JayXpose)
https://moncommerce.cog/catalogue       → Catalogue (JayXpose)
https://moncommerce.cog/produit/123     → Fiche produit (JayXpose)
https://moncommerce.cog/contact         → Formulaire contact (JayXpose)
https://rdv.kine.cog/                   → Page réservation (JayRDV)
https://festival.event.cog/             → Catalogue événement (JayFestival)
```

---

## 3. Comment Borner une Surface Web

### 3.1 Principe

> **Toute surface web doit avoir des frontières strictes : ce qui est exposé (liste blanche) vs ce qui reste interne.**

### 3.2 Règles de bornage

| Règle | Description |
|-------|-------------|
| **Liste blanche** | Seules les capacités explicitement déclarées sont exposées |
| **Pas d'accès aux Cores** | L'utilisateur externe n'accède jamais aux Cores directement |
| **Pas de données d'autres utilisateurs** | Un utilisateur externe ne voit que les données publiques ou les siennes |
| **Pas d'actions administratives** | Création, modification, suppression = Central uniquement |

### 3.3 Checklist de bornage

Pour chaque parcours web, vérifier :

| # | Question | ✅ / ❌ |
|---|----------|--------|
| 1 | Les capacités exposées sont-elles explicitement listées ? | |
| 2 | Les données sensibles sont-elles masquées ? | |
| 3 | Les actions autorisées sont-elles limitées (lecture, formulaire, achat) ? | |
| 4 | Le rate limiting est-il configuré ? | |
| 5 | L'identification des connexions est-elle active ? | |
| 6 | Les erreurs ne révèlent-elles pas d'information sensible ? | |

### 3.4 BorderGuard et Mandat Public d'Accès

Chaque surface web est gouvernée par un **Mandat Public d'Accès** :

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

### 3.5 Ce qui n'est JAMAIS exposé

| Élément | Raison |
|---------|--------|
| **Données d'autres clients** | Protection vie privée |
| **Identifiants internes** | Sécurité |
| **Logs système** | Sécurité |
| **Configuration** | Sécurité |
| **Actions admin** | Réservé à Central |
| **Cores** | Architecture fondamentale |

---

## 4. Comment Normer une Surface Web

### 4.1 Normes techniques

#### 4.1.1 Formats d'API

| Aspect | Norme |
|--------|-------|
| **Format de données** | JSON (UTF-8) |
| **Codes HTTP** | Standards (200, 400, 401, 403, 404, 429, 500) |
| **Identifiants** | UUID v4 ou slug (publics uniquement) |
| **Pagination** | `?page=1&limit=20` |
| **Erreurs** | `{ "error": "code", "message": "description" }` — jamais de stacktrace |

#### 4.1.2 Sessions et tokens

| Aspect | Norme |
|--------|-------|
| **Session ID** | Token opaque (UUID), cookie HttpOnly Secure |
| **Durée de session** | Configurable (défaut : 1h consultation, 30min action sensible) |
| **Token d'action** | Token unique, temporaire, non devinable (ex. lien annulation RDV) |

### 4.2 Normes UX

#### 4.2.1 Accessibilité

| Aspect | Norme |
|--------|-------|
| **Contraste** | WCAG AA minimum (4.5:1 texte, 3:1 grands textes) |
| **Navigation clavier** | Tab, Enter, Escape fonctionnels |
| **Lecteur d'écran** | Alt text, ARIA labels |
| **Responsive** | Mobile-first, breakpoints standards |

#### 4.2.2 Libellés et messages

| Élément | Norme |
|---------|-------|
| **Boutons** | Verbes d'action clairs (« Réserver », « Envoyer », « Acheter ») |
| **Erreurs** | Messages explicites, sans jargon technique |
| **Confirmations** | Récapitulatif clair avant action définitive |
| **États vides** | Message explicatif (« Aucun créneau disponible ») |

### 4.3 Normes de sécurité

#### 4.3.1 Règles COG-ADAPT

| Règle | Description |
|-------|-------------|
| **COG-ADAPT-01** | Tout accès externe passe par Visa et BorderGuard — jamais d'accès direct |
| **COG-ADAPT-06** | La complexité COG est cachée à l'utilisateur final — l'UX reste simple |

#### 4.3.2 Protection des données

| Aspect | Norme |
|--------|-------|
| **Données personnelles** | Minimisation (ne collecter que le nécessaire) |
| **Stockage** | KindMother uniquement, chiffré si niveau 2+ |
| **Transmission** | HTTPS obligatoire |
| **Consentement** | Explicite pour collecte de données |

#### 4.3.3 Protection contre les abus

| Protection | Implémentation |
|------------|----------------|
| **Rate limiting** | MiyuAntiSpam (par IP, par session) |
| **Captcha** | Sur formulaires sensibles (contact, inscription) |
| **Validation** | MiyuValidate (entrées utilisateur) |
| **CSRF** | Token par formulaire |
| **XSS** | Échappement systématique des sorties |

---

## 5. Checklist Complète — Nouvelle Surface Web

Avant de mettre en production une nouvelle surface web :

### 5.1 Documentation

| # | Élément | ✅ |
|---|---------|---|
| 1 | Contrat d'exposition rédigé (capacités, données, actions, niveau) | |
| 2 | Parcours utilisateur documenté | |
| 3 | Checklist de bornage validée | |

### 5.2 Implémentation

| # | Élément | ✅ |
|---|---------|---|
| 4 | Routage via Portail (pas d'accès direct au Service) | |
| 5 | Identification et fichage des connexions | |
| 6 | Mandat Public d'Accès configuré | |
| 7 | Rate limiting actif | |
| 8 | Validation des entrées (MiyuValidate) | |

### 5.3 Sécurité

| # | Élément | ✅ |
|---|---------|---|
| 9 | HTTPS obligatoire | |
| 10 | Erreurs sans fuite d'information | |
| 11 | Données sensibles masquées | |
| 12 | Protection CSRF/XSS | |

### 5.4 UX

| # | Élément | ✅ |
|---|---------|---|
| 13 | Responsive (mobile, tablette, desktop) | |
| 14 | Accessibilité WCAG AA | |
| 15 | Libellés clairs et cohérents | |
| 16 | Confirmations avant actions définitives | |

---

## 6. Références

| Document | Lien |
|----------|------|
| **Document Fondateur Portail** | [Miyukini Web Portal - Document Fondateur](./Miyukini%20Web%20Portal%20-%20Document%20Fondateur.md) |
| **Types de Services** | [Types de Services et Espaces](../../reference/Miyukini%20Conceptual%20References%20-%20Types%20de%20Services%20et%20Espaces.md) |
| **Glossaire** | [Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| **Façade Publique** | [Glossaire § Façade Publique Gouvernée](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| **MiyuWeb** | [docs/tools/MiyuWeb](../../tools/MiyuWeb/) |
| **MiyuAntiSpam** | [docs/tools/MiyuAntiSpam](../../tools/MiyuAntiSpam/) |

---

**Date de création :** 2026-02-08  
**Version :** 1.0  
**Statut :** Guide de référence normatif — Implémentation et gouvernance des surfaces web
