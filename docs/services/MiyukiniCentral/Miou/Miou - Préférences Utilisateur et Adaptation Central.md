# Miou â€” PrÃ©fÃ©rences Utilisateur et Adaptation Central

Miou pose des questions sur les **goÃ»ts**, les **prÃ©fÃ©rences** et l'**humeur** de l'utilisateur, et utilise ces informations pour **adapter Miyukini Central** : thÃ¨me visuel, frÃ©quence des bulles, orientation des actions.

---

## 1. Contexte

L'adaptation de Central par Miou repose sur les donnÃ©es collectÃ©es via :
- Les **bulles** (questions du Registre)
- Le **chatbot** (discussion libre)
- Les **ParamÃ¨tres** Miou et Central

**Principe :** Miou ne modifie pas Central de faÃ§on arbitraire â€” elle propose des adaptations fondÃ©es sur ce que l'utilisateur a explicitement partagÃ©.

---

## 2. Dimensions d'adaptation

| Dimension | ClÃ©s sources | Effet sur Central |
|-----------|--------------|-------------------|
| **ThÃ¨me** | `theme_ambiance`, `theme_central`, `hobby` | ThÃ¨me visuel, ambiance Gaming / Zen / Productif |
| **FrÃ©quence** | `preference_ton`, `frequence_bulles` | Discret / Normal / Bavard |
| **Orientation** | `orientation_actions`, `style_accompagnement` | Pauses, rappels, curiositÃ©, soutien |

---

## 3. ThÃ¨me et ambiance

### 3.1 ClÃ©s impliquÃ©es

| ClÃ© | Description | Valeurs possibles |
|-----|-------------|-------------------|
| `theme_ambiance` | Orientation gÃ©nÃ©rale dÃ©clarÃ©e | Gaming, Zen, Productif, Social |
| `theme_central` | ThÃ¨me visuel (Rite d'EntrÃ©e) | Minimal, Gaming, etc. |
| `hobby` | Loisir principal | Texte libre â€” peut influencer le thÃ¨me suggÃ©rÃ© |

### 3.2 Comportement

- Si l'utilisateur dÃ©clare `theme_ambiance = Gaming` â†’ Miou propose ou confirme le thÃ¨me Gaming pour Central.
- Si `hobby` contient Â« jeux vidÃ©o Â», Â« gaming Â» â†’ orientation Gaming possible.
- **Proposition, pas imposition :** Miou peut afficher une bulle Â« Tu prÃ©fÃ¨res qu'on soit en mode Gaming ou Zen ? Â» â€” l'utilisateur choisit.

### 3.3 CohÃ©rence

Le thÃ¨me Central reste **modifiable par l'utilisateur** dans les ParamÃ¨tres. Miou peut suggÃ©rer, mais la dÃ©cision finale appartient Ã  l'utilisateur.

---

## 4. FrÃ©quence des bulles

### 4.1 ClÃ©s impliquÃ©es

| ClÃ© | Description | Effet |
|-----|-------------|-------|
| `preference_ton` | DiscrÃ¨te / Bavarde / Comme maintenant | ContrÃ´le la frÃ©quence des bulles |
| `frequence_bulles` | Discret / Normal / Bavard | Override explicite si dÃ©fini |

### 4.2 Mapping

| Valeur `preference_ton` | FrÃ©quence appliquÃ©e |
|-------------------------|----------------------|
| DiscrÃ¨te | Bulles moins frÃ©quentes, cooldown augmentÃ© |
| Bavarde | Bulles plus frÃ©quentes, curiositÃ© plus prÃ©sente |
| Comme maintenant | Aucun changement (dÃ©faut) |

### 4.3 ParamÃ¨tres techniques

| ParamÃ¨tre | Discret | Normal | Bavard |
|-----------|---------|--------|--------|
| Cooldown bulle accueil | 24 h | 12 h | 6 h |
| Cooldown bulle curiositÃ© | 10 j | 7 j | 5 j |
| Cooldown bulle pause | 4 h | 2 h | 1 h 30 |

*(Valeurs indicatives â€” Ã  caler avec le Moteur de DÃ©cision.)*

---

## 5. Orientation des actions

### 5.1 Types d'actions Miou

| Type | Description | Exemple |
|------|-------------|---------|
| **Pauses** | Rappels santÃ©, dÃ©connexion | Â« 2h â€” une pause ? Â» |
| **Rappels** | Ã‰vÃ©nements, tÃ¢ches | Â« Ton meeting dans 30 min Â» |
| **CuriositÃ©** | Questions, taquinerie | Â« Tu prÃ©fÃ¨res le matin ou le soir ? Â» |
| **Soutien** | PrÃ©sence, encouragement | Â« Ton projet dÃ©co avance ? Â» |

### 5.2 ClÃ©s impliquÃ©es

| ClÃ© | Description | Influence |
|-----|-------------|-----------|
| `orientation_actions` | Pauses / Rappels / CuriositÃ© / Soutien | PondÃ©ration des catÃ©gories |
| `style_accompagnement` | Pousser / Mon rythme / Les deux | Ton des rappels et encouragements |
| `reconfort`, `activite_deconnexion` | DonnÃ©es de bien-Ãªtre | Personnalisation des pauses |
| `projet_coeur`, `bonheur_quotidien` | DonnÃ©es personnelles | Personnalisation du soutien |

### 5.3 Comportement

- Si `orientation_actions = Pauses` â†’ Miou privilÃ©gie les bulles pause santÃ©, rappels dÃ©connexion.
- Si `style_accompagnement = Mon rythme` â†’ Moins de Â« pousse Â», plus de Â« je suis lÃ  quand tu veux Â».
- Les donnÃ©es `reconfort`, `activite_deconnexion` sont **injectÃ©es** dans les bulles pause (voir [Miou - Roadmap et AmÃ©liorations](_index.md)).

---

## 6. Humeur

### 6.1 ClÃ©s impliquÃ©es

| ClÃ© | Description | Usage |
|-----|-------------|-------|
| `humeur_actuelle` | Ã‰tat dÃ©clarÃ© (optionnel) | Adapter le ton de la prochaine bulle |
| `humeur_preferee` | Humeur prÃ©fÃ©rÃ©e | RÃ©fÃ©rence pour les propositions |

### 6.2 Comportement

- Si l'utilisateur dit Â« Je suis fatiguÃ© Â» dans le chatbot â†’ `humeur_actuelle` peut Ãªtre mis Ã  jour (avec confirmation).
- Miou adapte : moins de curiositÃ©, plus de douceur, Ã©vite les taquineries si humeur basse.
- **Ã‰phÃ©mÃ¨re :** `humeur_actuelle` peut avoir une durÃ©e de vie courte (ex. 24 h) â€” Ã  dÃ©finir.

---

## 7. Flux de proposition d'adaptation

```
1. Miou dÃ©tecte une opportunitÃ© (nouvelle donnÃ©e, changement de prÃ©fÃ©rence)
2. Miou affiche une bulle ou message dans le chatbot : Â« Tu prÃ©fÃ¨res qu'on soit plus discrÃ¨te ou plus bavarde ? Â»
3. L'utilisateur rÃ©pond
4. Miou met Ã  jour la clÃ© (preference_ton, theme_ambiance, etc.)
5. Le Moteur de DÃ©cision / ParamÃ¨tres Central appliquent l'adaptation
```

**RÃ¨gle :** Pas d'adaptation automatique sans donnÃ©e explicite. Les valeurs par dÃ©faut restent tant que l'utilisateur n'a pas rÃ©pondu.

---

## 8. RÃ©fÃ©rences

- [Miou - Catalogue Exhaustif des Connaissances](./Miou%20-%20Catalogue%20Exhaustif%20des%20Connaissances.md)
- [Miou - Onglet Service Mode Chatbot](./Miou%20-%20Onglet%20Service%20Mode%20Chatbot.md)
- [Bot - Registre Questions et Paliers d'Attachement](./Bot/Bot%20-%20Registre%20Questions%20et%20Paliers%20d'Attachement.md)
- [Miou - Roadmap et AmÃ©liorations](_index.md)

---

**Version :** 1.0  
**Statut :** SpÃ©cification adaptation Central

