# Miou — Préférences Utilisateur et Adaptation Central

Miou pose des questions sur les **goûts**, les **préférences** et l'**humeur** de l'utilisateur, et utilise ces informations pour **adapter Miyukini Central** : thème visuel, fréquence des bulles, orientation des actions.

---

## 1. Contexte

L'adaptation de Central par Miou repose sur les données collectées via :
- Les **bulles** (questions du Registre)
- Le **chatbot** (discussion libre)
- Les **Paramètres** Miou et Central

**Principe :** Miou ne modifie pas Central de façon arbitraire — elle propose des adaptations fondées sur ce que l'utilisateur a explicitement partagé.

---

## 2. Dimensions d'adaptation

| Dimension | Clés sources | Effet sur Central |
|-----------|--------------|-------------------|
| **Thème** | `theme_ambiance`, `theme_central`, `hobby` | Thème visuel, ambiance Gaming / Zen / Productif |
| **Fréquence** | `preference_ton`, `frequence_bulles` | Discret / Normal / Bavard |
| **Orientation** | `orientation_actions`, `style_accompagnement` | Pauses, rappels, curiosité, soutien |

---

## 3. Thème et ambiance

### 3.1 Clés impliquées

| Clé | Description | Valeurs possibles |
|-----|-------------|-------------------|
| `theme_ambiance` | Orientation générale déclarée | Gaming, Zen, Productif, Social |
| `theme_central` | Thème visuel (Rite d'Entrée) | Minimal, Gaming, etc. |
| `hobby` | Loisir principal | Texte libre — peut influencer le thème suggéré |

### 3.2 Comportement

- Si l'utilisateur déclare `theme_ambiance = Gaming` → Miou propose ou confirme le thème Gaming pour Central.
- Si `hobby` contient « jeux vidéo », « gaming » → orientation Gaming possible.
- **Proposition, pas imposition :** Miou peut afficher une bulle « Tu préfères qu'on soit en mode Gaming ou Zen ? » — l'utilisateur choisit.

### 3.3 Cohérence

Le thème Central reste **modifiable par l'utilisateur** dans les Paramètres. Miou peut suggérer, mais la décision finale appartient à l'utilisateur.

---

## 4. Fréquence des bulles

### 4.1 Clés impliquées

| Clé | Description | Effet |
|-----|-------------|-------|
| `preference_ton` | Discrète / Bavarde / Comme maintenant | Contrôle la fréquence des bulles |
| `frequence_bulles` | Discret / Normal / Bavard | Override explicite si défini |

### 4.2 Mapping

| Valeur `preference_ton` | Fréquence appliquée |
|-------------------------|----------------------|
| Discrète | Bulles moins fréquentes, cooldown augmenté |
| Bavarde | Bulles plus fréquentes, curiosité plus présente |
| Comme maintenant | Aucun changement (défaut) |

### 4.3 Paramètres techniques

| Paramètre | Discret | Normal | Bavard |
|-----------|---------|--------|--------|
| Cooldown bulle accueil | 24 h | 12 h | 6 h |
| Cooldown bulle curiosité | 10 j | 7 j | 5 j |
| Cooldown bulle pause | 4 h | 2 h | 1 h 30 |

*(Valeurs indicatives — à caler avec le Moteur de Décision.)*

---

## 5. Orientation des actions

### 5.1 Types d'actions Miou

| Type | Description | Exemple |
|------|-------------|---------|
| **Pauses** | Rappels santé, déconnexion | « 2h — une pause ? » |
| **Rappels** | Événements, tâches | « Ton meeting dans 30 min » |
| **Curiosité** | Questions, taquinerie | « Tu préfères le matin ou le soir ? » |
| **Soutien** | Présence, encouragement | « Ton projet déco avance ? » |

### 5.2 Clés impliquées

| Clé | Description | Influence |
|-----|-------------|-----------|
| `orientation_actions` | Pauses / Rappels / Curiosité / Soutien | Pondération des catégories |
| `style_accompagnement` | Pousser / Mon rythme / Les deux | Ton des rappels et encouragements |
| `reconfort`, `activite_deconnexion` | Données de bien-être | Personnalisation des pauses |
| `projet_coeur`, `bonheur_quotidien` | Données personnelles | Personnalisation du soutien |

### 5.3 Comportement

- Si `orientation_actions = Pauses` → Miou privilégie les bulles pause santé, rappels déconnexion.
- Si `style_accompagnement = Mon rythme` → Moins de « pousse », plus de « je suis là quand tu veux ».
- Les données `reconfort`, `activite_deconnexion` sont **injectées** dans les bulles pause (voir [Miou - Roadmap et Améliorations](./Miou%20-%20Roadmap%20et%20Améliorations.md)).

---

## 6. Humeur

### 6.1 Clés impliquées

| Clé | Description | Usage |
|-----|-------------|-------|
| `humeur_actuelle` | État déclaré (optionnel) | Adapter le ton de la prochaine bulle |
| `humeur_preferee` | Humeur préférée | Référence pour les propositions |

### 6.2 Comportement

- Si l'utilisateur dit « Je suis fatigué » dans le chatbot → `humeur_actuelle` peut être mis à jour (avec confirmation).
- Miou adapte : moins de curiosité, plus de douceur, évite les taquineries si humeur basse.
- **Éphémère :** `humeur_actuelle` peut avoir une durée de vie courte (ex. 24 h) — à définir.

---

## 7. Flux de proposition d'adaptation

```
1. Miou détecte une opportunité (nouvelle donnée, changement de préférence)
2. Miou affiche une bulle ou message dans le chatbot : « Tu préfères qu'on soit plus discrète ou plus bavarde ? »
3. L'utilisateur répond
4. Miou met à jour la clé (preference_ton, theme_ambiance, etc.)
5. Le Moteur de Décision / Paramètres Central appliquent l'adaptation
```

**Règle :** Pas d'adaptation automatique sans donnée explicite. Les valeurs par défaut restent tant que l'utilisateur n'a pas répondu.

---

## 8. Références

- [Miou - Catalogue Exhaustif des Connaissances](./Miou%20-%20Catalogue%20Exhaustif%20des%20Connaissances.md)
- [Miou - Onglet Service Mode Chatbot](./Miou%20-%20Onglet%20Service%20Mode%20Chatbot.md)
- [Bot - Registre Questions et Paliers d'Attachement](./Bot/Bot%20-%20Registre%20Questions%20et%20Paliers%20d'Attachement.md)
- [Miou - Roadmap et Améliorations](./Miou%20-%20Roadmap%20et%20Améliorations.md)

---

**Version :** 1.0  
**Statut :** Spécification adaptation Central
