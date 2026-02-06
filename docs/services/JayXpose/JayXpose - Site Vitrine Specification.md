# JayXpose — Site Vitrine Specification

## Contexte

Ce document spécifie le **site vitrine** du service **JayXpose**. Chaque exposant peut publier un site vitrine complet avec une page d'accueil, un catalogue, une page de présentation et une page de contact. Le site est accessible via une **URL unique** personnalisable et bénéficie d'une **personnalisation visuelle** et d'un **référencement SEO basique**.

**Références** : [Document fondateur](./JayXpose%20-%20Document%20Fondateur.md), [Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md) (JXP-30 à JXP-38), [Catalogue Produits](./JayXpose%20-%20Catalogue%20Produits.md), [Ecrans et UI](./JayXpose%20-%20Ecrans%20et%20UI.md).

## Portée / Scope

- **Périmètre** : Architecture du site vitrine, pages, personnalisation, URL, SEO, responsive, statuts de publication, contenu riche.
- **Hors périmètre** : Boutique en ligne (achat/paiement) ; hébergement/domaine personnalisé ; blog intégré.

---

## 1. Principes directeurs

| Principe | Description |
|----------|-------------|
| **Simplicité** | Le site vitrine est auto-généré à partir des données JayXpose (profil + catalogue). L'exposant configure, il ne code pas. |
| **Multi-page** | 4 pages structurées : Accueil, Catalogue, Présentation, Contact. |
| **Personnalisable** | Couleurs, slug URL, contenu page présentation, SEO, choix des sections affichées. |
| **Responsive** | Adapté mobile, tablette, desktop. |
| **Gouvernance** | Rendu public = Public (0) à Standard (1). Configuration = propriétaire uniquement. |
| **Performance** | Temps de chargement < 3s au premier accès (NFR-JXP-01). |

---

## 2. Architecture du site vitrine

### 2.1 Pages

| Page | Type | Description | Source de données |
|------|------|-------------|-------------------|
| **Accueil** | Auto-générée | Bannière, slogan, produits vedettes, description courte, liens navigation. | `exposants` (banner_url, slogan, description_short) + `produits_catalogue` (is_featured). |
| **Catalogue** | Auto-générée | Liste des produits filtrable par catégorie, avec visuels et prix. | `produits_catalogue` + `categories_produits` + `produits_visuels`. |
| **Présentation** | Contenu libre | Histoire, savoir-faire, valeurs — contenu riche édité par l'exposant. | `vitrine_pages` (page_type = 'presentation'). |
| **Contact** | Semi-auto | Coordonnées (filtrées par confidentialité) + formulaire de contact. | `exposants` (contacts filtrés) + formulaire. |

### 2.2 Navigation

```
[Accueil] — [Catalogue] — [Présentation] — [Contact]
```

- Navigation en barre supérieure (desktop) ou menu hamburger (mobile).
- Chaque page peut être activée ou désactivée par l'exposant.
- Si une page est désactivée, elle n'apparaît pas dans la navigation.
- **Accueil** est toujours active (ne peut pas être désactivée).

### 2.3 URL

| Élément | Format | Exemple |
|---------|--------|---------|
| Base | `{domaine}/vitrine/{slug}` | `miyukini.app/vitrine/latelier-de-marie` |
| Accueil | `{base}/` | `miyukini.app/vitrine/latelier-de-marie/` |
| Catalogue | `{base}/catalogue` | `miyukini.app/vitrine/latelier-de-marie/catalogue` |
| Produit | `{base}/catalogue/{produit-id}` | `miyukini.app/vitrine/latelier-de-marie/catalogue/abc123` |
| Présentation | `{base}/presentation` | `miyukini.app/vitrine/latelier-de-marie/presentation` |
| Contact | `{base}/contact` | `miyukini.app/vitrine/latelier-de-marie/contact` |

**Règles slug** :
- Alphanumérique + tirets uniquement.
- Min 3 caractères, max 60 caractères.
- Unique globalement.
- Modifiable (redirection 301 de l'ancien slug pendant 90 jours).
- Validation à la saisie (disponibilité en temps réel).

---

## 3. Personnalisation

### 3.1 Palette de couleurs

| Paramètre | Description | Défaut |
|-----------|-------------|--------|
| `color_primary` | Couleur principale (boutons, liens, accents). | `#2563eb` (bleu). |
| `color_secondary` | Couleur secondaire (fonds, badges). | `#f3f4f6` (gris clair). |
| `color_background` | Couleur de fond des pages. | `#ffffff` (blanc). |
| `color_text` | Couleur du texte principal. | `#1f2937` (gris foncé). |

### 3.2 Éléments configurables

| Élément | Options |
|---------|---------|
| Logo | Affiché en en-tête (issu du profil). |
| Bannière | Image de couverture page Accueil (issue du profil). |
| Pages activées | Cocher/décocher : Catalogue, Présentation, Contact. |
| Nombre de produits vedettes | 3, 4 ou 6 (affiché sur l'accueil). |
| Réseaux sociaux | Liens affichés dans le pied de page (issus du profil). |

### 3.3 Limites

| Contrainte | Valeur |
|------------|--------|
| Personnalisation CSS libre | Non (palette + options prédéfinies uniquement). |
| Thèmes | Non (un seul modèle, personnalisé par la palette). |
| JavaScript personnalisé | Non. |
| Widgets tiers | Non (alpha). |

---

## 4. Contenu de la page Présentation

### 4.1 Éditeur de contenu

L'exposant rédige le contenu de sa page Présentation via un **éditeur de contenu riche** simplifié :

| Fonctionnalité | Description |
|----------------|-------------|
| Texte formaté | Titres (H2, H3), paragraphes, gras, italique, listes. |
| Images | Insertion d'images (upload ou URL) avec alt text. |
| Vidéos intégrées | Embed YouTube / Vimeo (via URL). |
| Liens | Liens hypertextes. |
| Séparateurs | Lignes de séparation visuelle. |

### 4.2 Limites contenu

| Contrainte | Valeur |
|------------|--------|
| Taille max contenu | 50 000 caractères. |
| Images par page | 10 max. |
| Taille image | 5 Mo max. |
| HTML brut | Non autorisé (sanitized). |

### 4.3 Stockage

Le contenu est stocké en **JSON structuré** (blocs de contenu) dans `vitrine_pages.content` :

```json
{
  "blocks": [
    { "type": "heading", "level": 2, "text": "Notre histoire" },
    { "type": "paragraph", "text": "Depuis 2015, nous créons..." },
    { "type": "image", "url": "https://...", "alt": "Atelier" },
    { "type": "heading", "level": 3, "text": "Nos valeurs" },
    { "type": "list", "items": ["Artisanat", "Local", "Qualité"] },
    { "type": "video", "provider": "youtube", "embed_id": "abc123" }
  ]
}
```

---

## 5. Page Contact

### 5.1 Coordonnées affichées

Les coordonnées affichées sur la page Contact respectent la **politique de confidentialité** définie par l'exposant dans sa fiche publique :

| Champ | Affiché si |
|-------|------------|
| Email de contact | Confidentialité = `public` ou `authentifie`. |
| Téléphone | Confidentialité = `public` ou `authentifie`. |
| Adresse | Confidentialité = `public`. |
| Site web externe | Toujours (si renseigné). |
| Réseaux sociaux | Toujours (si renseignés). |

### 5.2 Formulaire de contact

| Champ | Type | Obligatoire |
|-------|------|-------------|
| Nom | Input text | Oui |
| Email | Input email | Oui |
| Objet | Input text | Non |
| Message | Textarea | Oui |
| Captcha | Vérification anti-spam | Oui |

**Comportement** : Soumission → notification à l'exposant (in-app + email si configuré). Le message n'est pas stocké dans JayXpose (simple relais) sauf si l'exposant active l'option « Conserver les messages » (phase 2).

---

## 6. SEO

### 6.1 Balises configurables

| Balise | Source | Personnalisable |
|--------|--------|-----------------|
| `<title>` | Défaut : `{company_name} — Vitrine JayXpose`. Personnalisable via `seo_title`. | Oui. |
| `<meta name="description">` | Défaut : `description_short`. Personnalisable via `seo_description`. | Oui. |
| `<meta name="keywords">` | Défaut : `tags` du profil. Personnalisable via `seo_keywords`. | Oui. |
| `<link rel="canonical">` | URL canonique de la vitrine. | Auto. |

### 6.2 Données structurées (Schema.org)

```json
{
  "@context": "https://schema.org",
  "@type": "LocalBusiness",
  "name": "{company_name}",
  "description": "{description_short}",
  "url": "{vitrine_url}",
  "logo": "{logo_url}",
  "address": {
    "@type": "PostalAddress",
    "streetAddress": "{adresse}",
    "addressLocality": "{ville}",
    "addressCountry": "FR"
  },
  "sameAs": ["{social_facebook}", "{social_instagram}", "{social_linkedin}"]
}
```

### 6.3 Sitemap et robots

- Chaque vitrine publiée est incluse dans le sitemap global.
- Les vitrines en brouillon ou suspendue ne sont pas indexées (`noindex`).

---

## 7. Responsive

### 7.1 Points de rupture

| Breakpoint | Largeur | Comportement |
|------------|---------|-------------|
| Mobile | < 768px | Navigation hamburger ; grille produits 1 colonne ; bannière pleine largeur. |
| Tablette | 768px - 1023px | Navigation horizontale compacte ; grille produits 2 colonnes. |
| Desktop | >= 1024px | Navigation horizontale complète ; grille produits 3-4 colonnes. |

### 7.2 Images responsive

- Images servies en plusieurs tailles (thumbnail, medium, large) via Supabase Storage transforms ou génération au build.
- Format WebP si supporté par le navigateur, fallback JPG/PNG.

---

## 8. Statuts de la vitrine

| Statut | Description | Visibilité publique |
|--------|-------------|---------------------|
| `brouillon` | En cours de configuration. Accessible uniquement en prévisualisation par l'exposant. | Non. |
| `publiee` | Vitrine active, accessible publiquement via l'URL. | Oui. |
| `suspendue` | Vitrine temporairement retirée (choix exposant ou décision admin). | Non (page « Vitrine temporairement indisponible »). |

### 8.1 Transitions

| De | Vers | Déclencheur |
|----|------|-------------|
| — | `brouillon` | Création automatique à l'inscription. |
| `brouillon` | `publiee` | Clic « Publier » par l'exposant. Condition : au moins le profil complété (nom, description, logo). |
| `publiee` | `suspendue` | Choix exposant (« Suspendre ») ou action admin. |
| `publiee` | `brouillon` | Choix exposant (« Repasser en brouillon »). |
| `suspendue` | `publiee` | Choix exposant (« Réactiver »). |
| `suspendue` | `brouillon` | Choix exposant. |

---

## 9. Performance

| Objectif | Cible |
|----------|-------|
| Temps de chargement (première visite) | < 3 secondes (3G rapide). |
| Temps de chargement (navigation interne) | < 1 seconde. |
| Poids page accueil | < 500 Ko (sans images exposant). |
| Images | Lazy loading ; compression automatique. |

---

## 10. Références

- [JayXpose - Document Fondateur](./JayXpose%20-%20Document%20Fondateur.md)
- [JayXpose - Analyse des besoins](./JayXpose%20-%20Analyse%20des%20besoins.md)
- [JayXpose - Catalogue Produits](./JayXpose%20-%20Catalogue%20Produits.md)
- [JayXpose - Ecrans et UI](./JayXpose%20-%20Ecrans%20et%20UI.md)
- [JayXpose - Confidentialite et Partage Inter-Services](./JayXpose%20-%20Confidentialite%20et%20Partage%20Inter-Services.md)

---

**Document** : JayXpose — Site Vitrine Specification
**Version** : 1.0
**Date** : 2026-02-06
**Statut** : Référence produit
