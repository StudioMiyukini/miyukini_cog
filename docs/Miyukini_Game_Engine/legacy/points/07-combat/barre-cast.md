# Barre de cast

**Catégorie :** 07. Combat (liaison avec 20. Interface)  
**Description :** Canalisation ; interruption ; feedback visuel.

## Contexte

La barre de cast affiche la progression d'un sort ou d'une compétence canalisée. Elle permet au joueur de voir le temps restant et d'annuler éventuellement. Les dégâts ou le déplacement peuvent interrompre le cast. Voir aussi [barre-cast](../20-interface/barre-cast.md) dans l'interface.

**Rôle :** Feedback visuel, risque tactique. Voir [MGE - Référence technique](../MGE%20-%20Miyukini%20Game%20Engine%20-%20Reference%20Technique.md).

---

## Spécifications techniques

### Interruption

- Dégâts reçus : annulent le cast
- Stun, knockdown : idem
- % de dégâts pour interrupt : option (ex. 10 % max HP)

### Annulation volontaire

- Touche dédiée (ex. Escape)
- Mouvement : selon le design

---

## Références

- [Index 07](_index.md)
- [Barre cast UI](../20-interface/barre-cast.md)
- [Action](action.md)
