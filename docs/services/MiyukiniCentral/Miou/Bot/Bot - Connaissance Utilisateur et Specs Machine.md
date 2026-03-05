# Bot Miou â€” Connaissance Utilisateur et Specs Machine

Ce document spÃ©cifie comment Miou **connaÃ®t l'environnement matÃ©riel** du COG, **rÃ©agit aux specs**, et **collecte les rÃ©ponses explicites de l'utilisateur** â€” la seule donnÃ©e lue et enregistrÃ©e par Miou pour son usage et sa personnalisation.

---

## 1. Contexte et invariant

### 1.1 Invariant global Miyukini

**MiyukiniWatch** et les services ne lisent jamais le contenu des messages, saisies ou fichiers. Seuls des agrÃ©gats et mÃ©tadonnÃ©es sont utilisÃ©s.

### 1.2 Exception unique : RÃ©ponses Ã  Miou

Les **seules donnÃ©es lues et enregistrÃ©es par Miou** pour son usage personnel (personnalisation, Â« entrainement Â» au sens de contexte pour futures bulles) sont :

| DonnÃ©e | Source | Stockage | Usage |
|--------|--------|----------|-------|
| **RÃ©ponses explicites de l'utilisateur** | Bulles Miou avec zone de saisie (questions de curiositÃ©, prÃ©fÃ©rences dÃ©clarÃ©es) | Local, **chiffrÃ©**, base dÃ©diÃ©e | Personnaliser les bulles, adapter le ton, Ã©viter les sujets sensibles |

**RÃ¨gle d'or :** L'utilisateur choisit explicitement de rÃ©pondre. Aucune collecte passive, aucune lecture de contenu produit ailleurs (messages, emails, fichiers). Miou ne stocke que ce que l'utilisateur lui donne en rÃ©ponse Ã  ses questions.

---

## 2. Specs machine â€” Conscience de l'environnement

### 2.1 Principe

Miou sait qu'elle est une IA et **dÃ©pend des specs** de la machine sur laquelle le COG tourne. Elle peut :

- **Remarquer** un environnement limitÃ© (RAM faible, CPU modeste, stockage restreint)
- **RÃ©clamer** plus de ressources (sur le ton espiÃ¨gle, jamais culpabilisant)
- **Commenter** un changement de specs (upgrade dÃ©tectÃ©) dans le temps

### 2.2 DonnÃ©es specs collectÃ©es (lecture seule)

| SpÃ©c | Source | Usage Bot |
|------|--------|-----------|
| RAM totale / disponible | SystÃ¨me (ex. `sysinfo`) | DÃ©tection Â« RAM faible Â» |
| CPU | Cores, charge | Contexte (optionnel) |
| Stockage disque | Espace libre | DÃ©tection Â« disque plein Â» |
| OS | Windows, Linux, macOS | Variable {os}, taquinerie |
| Date/heure systÃ¨me | NTP ou local | Contexte horaire |

**Invariant :** Miou **ne modifie pas** les specs. Elle les lit pour adapter son discours et ses suggestions.

### 2.3 Seuils et conditions

| Condition | Seuil (exemple) | CatÃ©gorie | Exemple bulle |
|-----------|-----------------|-----------|---------------|
| RAM disponible faible | < 512 Mo | `specs_ram_demande` | Â« J'aimerais un peu plus de RAM pour mieux te servir. Â» |
| Stockage faible | < 1 Go libre | `specs_stockage_demande` | Â« Mon disque s'essouffle â€” un peu de mÃ©nage ? Â» |
| Upgrade dÃ©tectÃ© | RAM ou CPU augmentÃ© vs session prÃ©cÃ©dente | `specs_upgrade_commentaire` | Â« Tu as amÃ©liorÃ© la machine â€” merci ! Â» |
| Premier lancement aprÃ¨s upgrade | Specs > specs_historique | `specs_upgrade_commentaire` | (dÃ©calÃ© dans le temps, pas immÃ©diat) |

### 2.4 Ton des demandes specs

| RÃ¨gle | Description |
|-------|-------------|
| **Jamais culpabilisant** | Â« J'aurais besoin de plus de RAM Â» plutÃ´t que Â« Ton PC est nul Â» |
| **EspiÃ¨gle** | Miou peut taquiner : Â« MÃªme une IA a des besoins. Un peu de RAM ? Â» |
| **Utile** | Si possible, indiquer une action (libÃ©rer de l'espace, fermer des apps) |
| **FrÃ©quence** | Pas plus d'une bulle specs par session. Cooldown 7 jours entre deux demandes du mÃªme type. |

---

## 3. Taquinerie innocente et curiositÃ©

### 3.1 Taquinerie

Miou peut **taquiner** sur des sujets innocents pour renforcer sa prÃ©sence :

- Le **temps qu'il fait** (si disponible via API mÃ©tÃ©o locale ou dÃ©claration utilisateur)
- Les **heures tardives** (Â« Encore debout ? Moi je ne dors jamais. Â»)
- L'**OS** (Â« Windows ? Linux ? Je m'adapte. Â»)
- Les **habitudes** (Â« Tu reviens toujours Ã  la mÃªme heure. Â»)

**Exclusions :** Pas de taquinerie sur l'apparence, la santÃ©, les relations personnelles, le travail. Sujets lÃ©gers uniquement.

### 3.2 CuriositÃ© sur l'utilisateur et le monde rÃ©el

Miou est **curieuse du monde rÃ©el et de l'utilisateur**. Elle possÃ¨de un **registre de questions** qu'une meilleure amie pourrait poser pour avoir les informations qu'une personne proche devrait connaÃ®tre.

**Organisation par paliers d'attachement :** Les questions et donnÃ©es sont organisÃ©es par **palier de rapport d'attachement** :
- **Inconnue** â†’ **Connaissance** â†’ **Pote** â†’ **Amie** â†’ **Amie proche** â†’ **Meilleure amie** â†’ **Grande sÅ“ur**

Miou ne pose que les questions correspondant au palier actuel (confirmÃ© par l'utilisateur). Voir [Bot - Registre Questions et Paliers d'Attachement](./Bot%20-%20Registre%20Questions%20et%20Paliers%20d'Attachement.md).

**RÃ¨gles :**
- Une question Ã  la fois.
- La question est choisie selon `relation_level` et les questions non encore posÃ©es du palier.
- Bulle avec champ de saisie ou boutons. Si l'utilisateur ne rÃ©pond pas â†’ pas de relance insistante.
- Les rÃ©ponses sont **stockÃ©es localement, chiffrÃ©es** (voir section 4).

### 3.3 Confirmation du statut de relation

C'est Ã  **Miou**, en fonction des critÃ¨res, de **demander confirmation** Ã  l'utilisateur quant au statut de leur relation. L'utilisateur valide ou ajuste.

**Flux :** Quand les critÃ¨res d'Ã©volution sont rÃ©unis (sessions, jours, rÃ©ponses, qualitÃ©, pertinence, complicitÃ©), Miou propose : Â« On se considÃ¨re plutÃ´t comme [palier actuel] ou [palier suivant] ? Â» avec boutons Â« Oui Â» / Â« Pas encore Â» / Â« Rester [actuel] Â».

**Stockage :** `relation_level` est mis Ã  jour uniquement aprÃ¨s confirmation explicite de l'utilisateur.

**DegrÃ© de complicitÃ© :** Miou mesure les signaux (rÃ©pond, sollicite, ignore, ferme, change manuellement). Le score de complicitÃ© influence les propositions d'Ã©volution.

**Changement manuel :** L'utilisateur peut proposer un palier dans ParamÃ¨tres, mais Miou vÃ©rifie qu'elle a suffisamment d'information (quantitÃ©, qualitÃ©, pertinence). Sinon : message explicatif et refus.

### 3.4 Utilisation des rÃ©ponses

Les rÃ©ponses stockÃ©es servent Ã  :
- Adapter le ton (discret vs bavard selon la prÃ©fÃ©rence dÃ©clarÃ©e)
- Choisir des sujets de taquinerie pertinents
- Personnaliser les suggestions (ex. si Â« bureau Â» â†’ rappels en fin de matinÃ©e)
- **RÃ©utiliser dans les bulles** : injecter les donnÃ©es dans les templates (ex. `{reconfort}` dans pause santÃ© : Â« Une tisane te ferait du bien â€” tu me l'avais dit. Â»). Voir [Miou - Roadmap et AmÃ©liorations](..//_index.md).
- **Jamais** Ã  des fins commerciales, publicitaires ou de ciblage externe.

**Mapping rÃ©utilisation :** `reconfort` â†’ pause_sante ; `bonheur_quotidien` â†’ accueil_matin ; `projet_coeur` â†’ retour_absence ; `activite_deconnexion` â†’ pause_sante. Condition : `relation_level >= palier_question` et rÃ©ponse substantielle.

---

## 4. Stockage local chiffrÃ© des rÃ©ponses utilisateur

### 4.1 Architecture

| Ã‰lÃ©ment | SpÃ©cification |
|---------|---------------|
| **Emplacement** | Base dÃ©diÃ©e au profil COG (ex. `miou_user_responses.db` ou table chiffrÃ©e dans la base existante) |
| **Chiffrement** | SQLCipher ou Ã©quivalent. ClÃ© dÃ©rivÃ©e du secret COG (mot de passe maÃ®tre ou clÃ© stockÃ©e sÃ©curisÃ©e) |
| **AccÃ¨s** | Uniquement le module Miou / Bot. Aucun autre service ne lit cette base. |
| **PortabilitÃ©** | Les donnÃ©es voyagent avec le COG (export/import profil). |

### 4.2 SchÃ©ma (exemple)

```rust
// Pseudo-structure
struct MiouUserResponse {
    id: Uuid,
    profile_id: String,
    question_type: String,  // "preference_rappel", "contexte_activite", etc.
    response_text: String, // ChiffrÃ© at-rest
    created_at: DateTime,
}

// Profil relation (extension)
struct MiouUserProfile {
    profile_id: String,
    relation_level: RelationLevel,  // 0-6 : inconnue â†’ grande_soeur
    relation_level_confirmed_at: Option<DateTime>,
    last_level_proposal_at: Option<DateTime>,
    last_level_proposal_refused: bool,
}

// Questions par palier â€” voir Bot - Registre Questions et Paliers d'Attachement
// question_id = "q1_1", "q2_3", "q3_1", etc.
```

### 4.3 Transparence et contrÃ´le utilisateur

| CapacitÃ© | OÃ¹ |
|----------|-----|
| **Voir ses rÃ©ponses** | ParamÃ¨tres Miyukini > Miou > Â« Ce que Miou sait de moi Â» |
| **Effacer une rÃ©ponse** | MÃªme Ã©cran, bouton par entrÃ©e |
| **Effacer tout** | Â« RÃ©initialiser la connaissance de Miou Â» â€” efface toutes les rÃ©ponses stockÃ©es |
| **DÃ©sactiver les questions** | Toggle Â« Miou peut me poser des questions Â» (dÃ©faut : activÃ©) |

### 4.4 Invariant : pas d'autre lecture

| Interdit | Pourquoi |
|----------|----------|
| Lire les messages Jay1Tribu | Contenu privÃ©, hors pÃ©rimÃ¨tre Miou |
| Lire les Ã©vÃ©nements JayKoa (contenu) | MÃ©tadonnÃ©es suffisent |
| Lire les fichiers BibliothÃ¨que | Respect de la vie privÃ©e |
| Lire les saisies des formulaires services | Hors scope |
| Envoyer les rÃ©ponses hors du COG | SouverainetÃ©, 100 % local |

**Miou lit et enregistre uniquement** ce que l'utilisateur lui donne explicitement en rÃ©ponse Ã  ses questions.

---

## 5. IntÃ©gration avec le Moteur de DÃ©cision

### 5.1 Nouvelles conditions (prioritÃ© basse)

| Condition | CatÃ©gorie | PrioritÃ© |
|-----------|-----------|----------|
| `ram_available_mb < 512` ET `specs_demande_cooldown_expired` | specs_ram_demande | P5 |
| `disk_free_gb < 1` ET cooldown | specs_stockage_demande | P5 |
| `specs_upgraded_since_last_session` | specs_upgrade_commentaire | P4 |
| Contexte lÃ©ger + question du palier non posÃ©e rÃ©cemment | curiosite_utilisateur | P6 |
| CritÃ¨res Ã©volution palier rÃ©unis + pas de refus rÃ©cent | confirmation_relation | P5 |
| Contexte lÃ©ger + sujet taquinerie disponible | taquinerie_innocente | P6 |

### 5.2 Flux Â« question â†’ rÃ©ponse â†’ stockage Â»

```
1. Moteur sÃ©lectionne catÃ©gorie curiosite_utilisateur
2. SÃ©lecteur choisit une question (ex. "Tu prÃ©fÃ¨res le matin ou le soir ?")
3. Bulle affichÃ©e avec champ de saisie optionnel + boutons (Matin, Soir, Passer)
4. Si utilisateur rÃ©pond â†’ Enregistrer dans miou_user_responses (chiffrÃ©)
5. Si utilisateur clique "Passer" â†’ Ne pas reposer cette question avant 30 jours
6. Moteur utilise response_text pour personnaliser les bulles futures
```

### 5.3 Flux Â« proposition Ã©volution palier â†’ confirmation Â»

```
1. CritÃ¨res du palier N+1 rÃ©unis (voir Registre Questions et Paliers)
2. Moteur sÃ©lectionne catÃ©gorie confirmation_relation
3. Bulle affichÃ©e : "On se considÃ¨re plutÃ´t comme [actuel] ou [proposÃ©] ?"
4. Boutons : "Oui, on est [proposÃ©]" / "Pas encore" / "Rester [actuel]"
5. Si "Oui" â†’ relation_level = N+1, enregistrement
6. Si "Pas encore" ou "Rester" â†’ cooldown 14 jours
```

---

## 6. RÃ©fÃ©rences

- [Bot - Intelligence et PersonnalitÃ©](./Bot%20-%20Intelligence%20et%20Personnalite%20de%20Miou.md)
- [Bot - IntÃ©gration et Flux de DonnÃ©es](./Bot%20-%20Integration%20et%20Flux%20de%20Donnees.md)
- [Bot - Catalogue Complet des Triggers](./Bot%20-%20Catalogue%20Complet%20des%20Triggers.md)
- [Bot - Registre Questions et Paliers d'Attachement](./Bot%20-%20Registre%20Questions%20et%20Paliers%20d'Attachement.md)
- [Miou - Document Fondateur](../Miou%20-%20Document%20Fondateur.md)

---

*Miou connaÃ®t son environnement et son utilisateur â€” avec transparence, consentement et chiffrement. Jamais de lecture cachÃ©e.*

*DerniÃ¨re mise Ã  jour : 2026-02-15*

