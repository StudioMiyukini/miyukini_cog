# Miou — Guide UI/UX

Guide exhaustif de conception et d’expérience utilisateur pour l’avatar Miou : principes, design system, comportements, accessibilité et patterns d’interaction.

---

## 1. Contexte

**Miou** est l’avatar et mascotte de l’écosystème Miyukini COG. Son interface se déploie à travers plusieurs surfaces :
- **Bulles** en bas à droite de Miyukini Central (canal principal)
- **Écrans Rite d’Entrée** et **Connexion** (voix + texte)
- **Gamification** (badges dans profil, bulles de félicitation)
- **Paramètres Miyukini > Miou** (personnalisation)

Ce guide définit les règles UI/UX pour garantir une expérience **cohérente**, **respectueuse** et **bienveillante** sur toutes ces surfaces.

---

## 2. Portée

| Inclus | Exclu |
|--------|-------|
| Bulles, anatomie, comportements, animations | Choix des templates (référence Bot) |
| Rite d’Entrée, Connexion, Salon | Logique du moteur de décision |
| Badges, progression visuelle | Contenu des phrases |
| Voix et audio UX | Implémentation audio (audio.rs) |
| Paramètres Miou | Autres écrans Central |

---

## 3. Principes UI/UX fondamentaux

### 3.1 Hiérarchie des principes

| Priorité | Principe | Application |
|----------|----------|--------------|
| **1** | **Respect** | L’utilisateur doit toujours se sentir libre. Fermer une bulle = choix respecté. Jamais de relance agressive. |
| **2** | **Discrétion** | Miou ne crie jamais. Position fixe bas-droite, non bloquante, pas de popup au centre. |
| **3** | **Cohérence** | Même personnalité visuelle et sonore du Rite d’Entrée à la 500e session. |
| **4** | **Bienveillance** | Chaque élément visuel reflète la chaleur de Miou : couleurs douces, espacement généreux, ton léger. |
| **5** | **Accessibilité** | Texte lisible, contraste suffisant, support clavier, réduction de mouvement possible. |

### 3.2 Philosophie anti-culpabilisation (UI)

| À éviter | À privilégier |
|----------|----------------|
| Badge grisé avec texte « Tu n’as pas encore… » | Badge grisé discret, condition au survol uniquement |
| Compteur « 12/30 jours » affiché en permanence | Progression consultable à la demande |
| Animation de « streak perdu » | Pas de perte visible — tout reste intact |
| Bulle qui revient après fermeture | Une bulle fermée ne revient pas dans la session |

---

## 4. Design system Miou

### 4.1 Palette de couleurs (recommandée)

| Usage | Couleur | Code (ex. hex) | Contraste |
|-------|---------|----------------|-----------|
| **Arrière-plan bulle** | Blanc cassé / gris très clair | `#F8F6F4` ou adapté au thème | Fond sur contenu |
| **Texte principal** | Gris foncé | `#2C2C2C` | Min. 4.5:1 |
| **Texte secondaire** | Gris moyen | `#5C5C5C` | Min. 4.5:1 |
| **Accent Miou** | Rose doux / saumon | `#E8A598` ou `#D4A5A5` | Optionnel, décoratif |
| **Icône avatar** | Accord avec accent | Cohérent avec identité visuelle |
| **Bordures** | Gris très léger | `#E8E4E0` | Subtil |

**Mode sombre :** Adapter avec fond `#2A2A2A`, texte `#E8E8E8`. Vérifier contraste WCAG AA.

### 4.2 Typographie

| Élément | Police | Taille | Poids | Usage |
|---------|--------|--------|-------|-------|
| **En-tête bulle** | Variable système | 14px | Semi-bold | « Miou » |
| **Corps bulle** | Variable système | 14px | Regular | Texte du message |
| **Boutons actions** | Variable système | 13px | Medium | « Ouvrir JayKoa », « C’est noté » |
| **Badge nom** | Variable système | 12px | Medium | Tooltip / profil |
| **Rite d’Entrée / Connexion** | Variable système | 16–18px | Regular | Phrases Miou |

**Lisibilité :** Interligne ≥ 1.4 pour les paragraphes. Largeur de ligne max ~50 caractères.

### 4.3 Espacement

| Token | Valeur | Usage |
|-------|--------|-------|
| `space-xs` | 4px | Entre icône et texte |
| `space-sm` | 8px | Padding interne petit |
| `space-md` | 16px | Marge bulle / bord écran |
| `space-lg` | 24px | Entre sections (corps / boutons) |
| `radius-sm` | 8px | Coins boutons |
| `radius-md` | 12px | Coins bulle |
| `radius-lg` | 16px | Bulle principale |

### 4.4 Icônes

| Élément | Symbole | Taille suggérée |
|---------|--------|-----------------|
| Avatar Miou | 🌸 ou mascotte illustrée | 32×32px |
| Fermer | ✕ ou × | 24×24px, zone cliquable 44×44px |
| Badge | Emoji ou icône dédiée | 24×24px dans bulle, 32×32px en profil |

---

## 5. Anatomie des bulles

### 5.1 Structure

```
┌─────────────────────────────────────────────────────┐
│  [Avatar]  Miou                              [✕]   │  ← En-tête
│  ─────────────────────────────────────────────────  │
│                                                      │
│  « Bonsoir Kaito ! Tu as passé un bon moment        │  ← Corps
│    sur JayXpose ce soir. Pense à faire une pause. »│
│                                                      │
│  ┌─────────────────┐  ┌─────────────────────────┐   │  ← Actions (0–2)
│  │ 🕐 Bonne idée    │  │ C'est noté              │   │
│  └─────────────────┘  └─────────────────────────┘   │
└─────────────────────────────────────────────────────┘
```

### 5.2 Éléments obligatoires

| Élément | Présence | Règle |
|---------|----------|-------|
| Avatar | Toujours | Identité visuelle de Miou |
| Nom « Miou » | Toujours | Source du message |
| Bouton fermer | Toujours | Sortie explicite |
| Corps texte | Toujours | 1–3 phrases max |

### 5.3 Éléments optionnels

| Élément | Condition |
|---------|-----------|
| Icône badge | Bulle de type Félicitation (badge débloqué) |
| 1 ou 2 boutons d’action | Selon type (Suggestion, Rappel, Notification) |

### 5.4 Types de bulles — Indices visuels (optionnel)

Pour une différenciation subtile sans surcharge :

| Type | Indice visuel suggéré |
|------|------------------------|
| Accueil | Bord gauche accent couleur douce |
| Suggestion | Icône légère (💡) ou aucun |
| Rappel | Icône horloge ou événement |
| Félicitation | Icône badge + léger glow optionnel |
| Notification | Icône cloche ou personne |

---

## 6. Comportements et interactions

### 6.1 Apparition

| Règle | Spec |
|-------|------|
| **Animation** | Fade-in + slide-up 200ms ease-out. Pas de pop brutal. |
| **Délai initial** | 2–3 secondes après arrivée dans Central (laisser le Salon se charger). |
| **Modales** | Si modale ouverte (profil, formulaire), attendre la fermeture. |
| **Réduction mouvement** | Si `prefers-reduced-motion`, fade-in seul sans slide. |

### 6.2 Fermeture

| Action | Comportement |
|--------|--------------|
| Clic ✕ | Fermeture immédiate, fade-out 150ms. |
| Clic sur action | Exécution action (ouvrir service, etc.) puis fermeture. |
| Auto-dismiss | Accueil / Félicitation : ~15 s. Rappels : restent jusqu’à action ou dismiss. |
| Clic hors bulle | Ne ferme pas (éviter fermeture accidentelle). |

### 6.3 File d’attente et débit

| Règle | Valeur |
|-------|--------|
| Bulles simultanées | 1 maximum |
| Espacement minimum | 30 secondes entre deux bulles (configurable) |
| Maximum par session | 5 (Normale), 2 (Discrète), 10 (Bavarde) |

### 6.4 Priorité d’affichage

1. Rappel (pause, événement imminent)
2. Notification (ami, message)
3. Accueil
4. Suggestion
5. Félicitation

---

## 7. États et feedback

### 7.1 États de la bulle

| État | Visuel | Durée |
|------|--------|-------|
| **Apparition** | Animation d’entrée | 200ms |
| **Affichée** | Opacité 100 %, focus possible | Variable |
| **Survol bouton** | Légère surbrillance, cursor pointer | — |
| **Clic bouton** | Micro-feedback (scale 0.98 ou ombre) | 100ms |
| **Fermeture** | Fade-out | 150ms |

### 7.2 États émotionnels (référence visuelle)

Pour les designers : Miou peut adapter son avatar ou une bordure selon le contexte. Optionnel, à garder subtil.

| État | Suggestion visuelle |
|------|---------------------|
| Accueillant | Avatar standard, bordure neutre |
| Célébrant | Léger émoji ou icône joyeuse |
| Nocturne | Tonalité un peu plus douce (opacité réduite ?) |
| Bienveillant | Bordure douce ou icône cœur |

---

## 8. Responsive et adaptation

### 8.1 Breakpoints bulles

| Largeur fenêtre | Comportement |
|-----------------|--------------|
| > 800px | Bulle standard 360px max, marges 16px |
| 480–800px | Bulle 280px max, marges 12px |
| < 480px | Pleine largeur en bas, marges 8px, type notification |

### 8.2 Zone de sécurité

La bulle ne doit jamais :
- Recouvrir un champ de saisie actif
- Masquer un bouton critique (ex. Valider)
- Dépasser la zone visible (scroll si nécessaire sur petit écran)

---

## 9. Accessibilité

### 9.1 Règles obligatoires

| Critère | Application |
|---------|-------------|
| **Contraste** | Texte / fond ≥ 4.5:1 (WCAG AA) |
| **Taille cliquable** | Zone de clic min 44×44px pour ✕ et boutons |
| **Focus clavier** | Tab jusqu’à la bulle ; Enter/Space pour actions |
| **Announce** | Si lecteur d’écran : annoncer « Bulle de Miou : [contenu] » |
| **Réduction mouvement** | Respecter `prefers-reduced-motion` |

### 9.2 Bonnes pratiques

- Fournir un `aria-label` sur le bouton fermer : « Fermer le message de Miou »
- Les boutons d’action : « Ouvrir JayKoa », « Prendre une pause » (descriptifs)
- Éviter le texte en images ; privilégier le texte natif

---

## 10. Gamification UI

### 10.1 Badges dans le profil

| Règle | Description |
|-------|-------------|
| **Débloqué** | Icône en couleur, tooltip au survol (nom + date) |
| **Verrouillé** | Icône grisée, tooltip avec condition (discret) |
| **Disposition** | Grille ou rangée, espacement régulier |
| **Pas de compteur proéminent** | Pas de « 12/30 badges » en gros |

### 10.2 Bulle de félicitation (badge)

| Élément | Règle |
|---------|-------|
| Icône badge | Visible à gauche du texte ou au-dessus |
| Texte | Phrase personnalisée (voir Banque Templates) |
| Bouton | « Voir mes badges » optionnel |
| Priorité | Basse — ne pas couper un rappel important |

### 10.3 Ton visuel gamification

- Pas de fanfare, pas de confetti excessif
- Petit carillon sonore optionnel (désactivé par défaut)
- Couleurs douces, jamais agressives

---

## 11. Écrans Rite d’Entrée et Connexion

### 11.1 Hiérarchie visuelle

| Élément | Priorité |
|---------|----------|
| Phrase Miou (texte) | Élément principal, centré ou proéminent |
| Champ de saisie | Immédiatement accessible |
| Bouton validation | Visible, contraste suffisant |
| Voix | Déclenchée automatiquement si activée |

### 11.2 Cohérence avec les bulles

- Même palette, même typographie
- Miou (avatar/texte) présent visuellement
- Transition fluide Connexion → Salon → première bulle

### 11.3 UX voix

- Le son ne se déclenche qu’après un délai court (500ms) pour éviter coupure
- Texte visible en parallèle (toujours)
- Pas de blocage : l’utilisateur peut avancer même si le son n’est pas fini

---

## 12. Paramètres Miou

### 12.1 Structure recommandée

```
Paramètres Miyukini
└── Miou
    ├── Bulles activées         [Toggle]
    ├── Ne pas déranger (DND)   [Toggle] — Aucune bulle sauf exceptions
    ├── Fréquence               [Discrète | Normale | Bavarde]
    ├── Son des bulles          [Toggle]
    ├── Rappels de pause        [1h | 2h | 3h | Désactivé]
    ├── Voix Miou               [Toggle] — Master : aucun son si désactivé
    ├── TTS eSpeak              [Toggle] — Synthèse vocale pour textes dynamiques
    ├── Voix Salon (futur)      [Toggle]
    ├── Mode LLM (Intelligence) [Toggle] — Non implémenté en 0.1.x, présent pour préparation
    └── Ce que Miou sait de moi  [Lien]
```

### 12.2 Labels et aide

- Labels clairs : « Bulles activées » avec aide « Afficher les messages de Miou en bas à droite »
- Valeurs par défaut explicites
- Changement appliqué immédiatement (pas de « Sauvegarder » séparé si possible)

---

## 13. Do’s et Don’ts

### 13.1 À faire

| Do | Exemple |
|----|---------|
| Garder les bulles courtes | 1–3 phrases max |
| Une idée par bulle | Ne pas mélanger accueil + rappel + suggestion |
| Boutons d’action explicites | « Ouvrir JayKoa » plutôt que « OK » |
| Respecter le dismiss | Fermer = accepté, pas de reproche |
| Adapter au thème | Clair / sombre selon préférence utilisateur |

### 13.2 À éviter

| Don’t | Pourquoi |
|-------|----------|
| Popup au centre | Bloque l’activité, frustrant |
| Plusieurs bulles simultanées | Surcharge visuelle |
| Texte long (paragraphe) | Miou est légère, pas un cours |
| Son non désactivable | Respect de l’utilisateur |
| Couleurs criardes | Cohérence bienveillante |

---

## 14. Edge cases et contraintes

### 14.1 Cas limites

| Cas | Comportement |
|-----|--------------|
| Fenêtre très petite (< 360px) | Bulle pleine largeur, texte tronqué avec « … » si besoin (éviter) |
| Changement d’onglet pendant affichage | Bulle reste visible (Miou est transversale) |
| Déconnexion pendant bulle | Fermer immédiatement |
| Premier lancement (Rite) | Pas de bulles ; Miou est dans l’écran principal |

### 14.2 Contraintes techniques

- Z-index bulle : au-dessus du contenu, sous les modales système
- Pas de drag des bulles (position fixe)
- État persistant : `miou_session_count`, `miou_last_shown` pour éviter répétitions

---

## 15. Références

| Document | Lien |
|----------|------|
| Miou - Document Fondateur | [Document Fondateur](./Miou%20-%20Document%20Fondateur.md) |
| Miou - Système de Bulles et UI | [Bulles et UI](./Miou%20-%20Systeme%20de%20Bulles%20et%20UI.md) |
| Miou - Gamification et Progression | [Gamification](./Miou%20-%20Gamification%20et%20Progression.md) |
| Miou - Voix et Audio | [Voix et Audio](./Miou%20-%20Voix%20et%20Audio.md) |
| Bot - Intelligence et Personnalité | [Intelligence Bot](./Bot/Bot%20-%20Intelligence%20et%20Personnalite%20de%20Miou.md) |

---

*Guide UI/UX Miou : cohérence, respect, bienveillance. Chaque pixel au service de la relation entre Miou et l’utilisateur.*

*Dernière mise à jour : 2026-02-15*
