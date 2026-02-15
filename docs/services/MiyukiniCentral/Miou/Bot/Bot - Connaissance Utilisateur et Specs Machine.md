# Bot Miou — Connaissance Utilisateur et Specs Machine

Ce document spécifie comment Miou **connaît l'environnement matériel** du COG, **réagit aux specs**, et **collecte les réponses explicites de l'utilisateur** — la seule donnée lue et enregistrée par Miou pour son usage et sa personnalisation.

---

## 1. Contexte et invariant

### 1.1 Invariant global Miyukini

**MiyukiniWatch** et les services ne lisent jamais le contenu des messages, saisies ou fichiers. Seuls des agrégats et métadonnées sont utilisés.

### 1.2 Exception unique : Réponses à Miou

Les **seules données lues et enregistrées par Miou** pour son usage personnel (personnalisation, « entrainement » au sens de contexte pour futures bulles) sont :

| Donnée | Source | Stockage | Usage |
|--------|--------|----------|-------|
| **Réponses explicites de l'utilisateur** | Bulles Miou avec zone de saisie (questions de curiosité, préférences déclarées) | Local, **chiffré**, base dédiée | Personnaliser les bulles, adapter le ton, éviter les sujets sensibles |

**Règle d'or :** L'utilisateur choisit explicitement de répondre. Aucune collecte passive, aucune lecture de contenu produit ailleurs (messages, emails, fichiers). Miou ne stocke que ce que l'utilisateur lui donne en réponse à ses questions.

---

## 2. Specs machine — Conscience de l'environnement

### 2.1 Principe

Miou sait qu'elle est une IA et **dépend des specs** de la machine sur laquelle le COG tourne. Elle peut :

- **Remarquer** un environnement limité (RAM faible, CPU modeste, stockage restreint)
- **Réclamer** plus de ressources (sur le ton espiègle, jamais culpabilisant)
- **Commenter** un changement de specs (upgrade détecté) dans le temps

### 2.2 Données specs collectées (lecture seule)

| Spéc | Source | Usage Bot |
|------|--------|-----------|
| RAM totale / disponible | Système (ex. `sysinfo`) | Détection « RAM faible » |
| CPU | Cores, charge | Contexte (optionnel) |
| Stockage disque | Espace libre | Détection « disque plein » |
| OS | Windows, Linux, macOS | Variable {os}, taquinerie |
| Date/heure système | NTP ou local | Contexte horaire |

**Invariant :** Miou **ne modifie pas** les specs. Elle les lit pour adapter son discours et ses suggestions.

### 2.3 Seuils et conditions

| Condition | Seuil (exemple) | Catégorie | Exemple bulle |
|-----------|-----------------|-----------|---------------|
| RAM disponible faible | < 512 Mo | `specs_ram_demande` | « J'aimerais un peu plus de RAM pour mieux te servir. » |
| Stockage faible | < 1 Go libre | `specs_stockage_demande` | « Mon disque s'essouffle — un peu de ménage ? » |
| Upgrade détecté | RAM ou CPU augmenté vs session précédente | `specs_upgrade_commentaire` | « Tu as amélioré la machine — merci ! » |
| Premier lancement après upgrade | Specs > specs_historique | `specs_upgrade_commentaire` | (décalé dans le temps, pas immédiat) |

### 2.4 Ton des demandes specs

| Règle | Description |
|-------|-------------|
| **Jamais culpabilisant** | « J'aurais besoin de plus de RAM » plutôt que « Ton PC est nul » |
| **Espiègle** | Miou peut taquiner : « Même une IA a des besoins. Un peu de RAM ? » |
| **Utile** | Si possible, indiquer une action (libérer de l'espace, fermer des apps) |
| **Fréquence** | Pas plus d'une bulle specs par session. Cooldown 7 jours entre deux demandes du même type. |

---

## 3. Taquinerie innocente et curiosité

### 3.1 Taquinerie

Miou peut **taquiner** sur des sujets innocents pour renforcer sa présence :

- Le **temps qu'il fait** (si disponible via API météo locale ou déclaration utilisateur)
- Les **heures tardives** (« Encore debout ? Moi je ne dors jamais. »)
- L'**OS** (« Windows ? Linux ? Je m'adapte. »)
- Les **habitudes** (« Tu reviens toujours à la même heure. »)

**Exclusions :** Pas de taquinerie sur l'apparence, la santé, les relations personnelles, le travail. Sujets légers uniquement.

### 3.2 Curiosité sur l'utilisateur et le monde réel

Miou est **curieuse du monde réel et de l'utilisateur**. Elle possède un **registre de questions** qu'une meilleure amie pourrait poser pour avoir les informations qu'une personne proche devrait connaître.

**Organisation par paliers d'attachement :** Les questions et données sont organisées par **palier de rapport d'attachement** :
- **Inconnue** → **Connaissance** → **Pote** → **Amie** → **Amie proche** → **Meilleure amie** → **Grande sœur**

Miou ne pose que les questions correspondant au palier actuel (confirmé par l'utilisateur). Voir [Bot - Registre Questions et Paliers d'Attachement](./Bot%20-%20Registre%20Questions%20et%20Paliers%20d'Attachement.md).

**Règles :**
- Une question à la fois.
- La question est choisie selon `relation_level` et les questions non encore posées du palier.
- Bulle avec champ de saisie ou boutons. Si l'utilisateur ne répond pas → pas de relance insistante.
- Les réponses sont **stockées localement, chiffrées** (voir section 4).

### 3.3 Confirmation du statut de relation

C'est à **Miou**, en fonction des critères, de **demander confirmation** à l'utilisateur quant au statut de leur relation. L'utilisateur valide ou ajuste.

**Flux :** Quand les critères d'évolution sont réunis (sessions, jours, réponses, qualité, pertinence, complicité), Miou propose : « On se considère plutôt comme [palier actuel] ou [palier suivant] ? » avec boutons « Oui » / « Pas encore » / « Rester [actuel] ».

**Stockage :** `relation_level` est mis à jour uniquement après confirmation explicite de l'utilisateur.

**Degré de complicité :** Miou mesure les signaux (répond, sollicite, ignore, ferme, change manuellement). Le score de complicité influence les propositions d'évolution.

**Changement manuel :** L'utilisateur peut proposer un palier dans Paramètres, mais Miou vérifie qu'elle a suffisamment d'information (quantité, qualité, pertinence). Sinon : message explicatif et refus.

### 3.4 Utilisation des réponses

Les réponses stockées servent à :
- Adapter le ton (discret vs bavard selon la préférence déclarée)
- Choisir des sujets de taquinerie pertinents
- Personnaliser les suggestions (ex. si « bureau » → rappels en fin de matinée)
- **Réutiliser dans les bulles** : injecter les données dans les templates (ex. `{reconfort}` dans pause santé : « Une tisane te ferait du bien — tu me l'avais dit. »). Voir [Miou - Roadmap et Améliorations](../Miou%20-%20Roadmap%20et%20Améliorations.md).
- **Jamais** à des fins commerciales, publicitaires ou de ciblage externe.

**Mapping réutilisation :** `reconfort` → pause_sante ; `bonheur_quotidien` → accueil_matin ; `projet_coeur` → retour_absence ; `activite_deconnexion` → pause_sante. Condition : `relation_level >= palier_question` et réponse substantielle.

---

## 4. Stockage local chiffré des réponses utilisateur

### 4.1 Architecture

| Élément | Spécification |
|---------|---------------|
| **Emplacement** | Base dédiée au profil COG (ex. `miou_user_responses.db` ou table chiffrée dans la base existante) |
| **Chiffrement** | SQLCipher ou équivalent. Clé dérivée du secret COG (mot de passe maître ou clé stockée sécurisée) |
| **Accès** | Uniquement le module Miou / Bot. Aucun autre service ne lit cette base. |
| **Portabilité** | Les données voyagent avec le COG (export/import profil). |

### 4.2 Schéma (exemple)

```rust
// Pseudo-structure
struct MiouUserResponse {
    id: Uuid,
    profile_id: String,
    question_type: String,  // "preference_rappel", "contexte_activite", etc.
    response_text: String, // Chiffré at-rest
    created_at: DateTime,
}

// Profil relation (extension)
struct MiouUserProfile {
    profile_id: String,
    relation_level: RelationLevel,  // 0-6 : inconnue → grande_soeur
    relation_level_confirmed_at: Option<DateTime>,
    last_level_proposal_at: Option<DateTime>,
    last_level_proposal_refused: bool,
}

// Questions par palier — voir Bot - Registre Questions et Paliers d'Attachement
// question_id = "q1_1", "q2_3", "q3_1", etc.
```

### 4.3 Transparence et contrôle utilisateur

| Capacité | Où |
|----------|-----|
| **Voir ses réponses** | Paramètres Miyukini > Miou > « Ce que Miou sait de moi » |
| **Effacer une réponse** | Même écran, bouton par entrée |
| **Effacer tout** | « Réinitialiser la connaissance de Miou » — efface toutes les réponses stockées |
| **Désactiver les questions** | Toggle « Miou peut me poser des questions » (défaut : activé) |

### 4.4 Invariant : pas d'autre lecture

| Interdit | Pourquoi |
|----------|----------|
| Lire les messages Jay1Tribu | Contenu privé, hors périmètre Miou |
| Lire les événements JayKoa (contenu) | Métadonnées suffisent |
| Lire les fichiers Bibliothèque | Respect de la vie privée |
| Lire les saisies des formulaires services | Hors scope |
| Envoyer les réponses hors du COG | Souveraineté, 100 % local |

**Miou lit et enregistre uniquement** ce que l'utilisateur lui donne explicitement en réponse à ses questions.

---

## 5. Intégration avec le Moteur de Décision

### 5.1 Nouvelles conditions (priorité basse)

| Condition | Catégorie | Priorité |
|-----------|-----------|----------|
| `ram_available_mb < 512` ET `specs_demande_cooldown_expired` | specs_ram_demande | P5 |
| `disk_free_gb < 1` ET cooldown | specs_stockage_demande | P5 |
| `specs_upgraded_since_last_session` | specs_upgrade_commentaire | P4 |
| Contexte léger + question du palier non posée récemment | curiosite_utilisateur | P6 |
| Critères évolution palier réunis + pas de refus récent | confirmation_relation | P5 |
| Contexte léger + sujet taquinerie disponible | taquinerie_innocente | P6 |

### 5.2 Flux « question → réponse → stockage »

```
1. Moteur sélectionne catégorie curiosite_utilisateur
2. Sélecteur choisit une question (ex. "Tu préfères le matin ou le soir ?")
3. Bulle affichée avec champ de saisie optionnel + boutons (Matin, Soir, Passer)
4. Si utilisateur répond → Enregistrer dans miou_user_responses (chiffré)
5. Si utilisateur clique "Passer" → Ne pas reposer cette question avant 30 jours
6. Moteur utilise response_text pour personnaliser les bulles futures
```

### 5.3 Flux « proposition évolution palier → confirmation »

```
1. Critères du palier N+1 réunis (voir Registre Questions et Paliers)
2. Moteur sélectionne catégorie confirmation_relation
3. Bulle affichée : "On se considère plutôt comme [actuel] ou [proposé] ?"
4. Boutons : "Oui, on est [proposé]" / "Pas encore" / "Rester [actuel]"
5. Si "Oui" → relation_level = N+1, enregistrement
6. Si "Pas encore" ou "Rester" → cooldown 14 jours
```

---

## 6. Références

- [Bot - Intelligence et Personnalité](./Bot%20-%20Intelligence%20et%20Personnalite%20de%20Miou.md)
- [Bot - Intégration et Flux de Données](./Bot%20-%20Integration%20et%20Flux%20de%20Donnees.md)
- [Bot - Catalogue Complet des Triggers](./Bot%20-%20Catalogue%20Complet%20des%20Triggers.md)
- [Bot - Registre Questions et Paliers d'Attachement](./Bot%20-%20Registre%20Questions%20et%20Paliers%20d'Attachement.md)
- [Miou - Document Fondateur](../Miou%20-%20Document%20Fondateur.md)

---

*Miou connaît son environnement et son utilisateur — avec transparence, consentement et chiffrement. Jamais de lecture cachée.*

*Dernière mise à jour : 2026-02-15*
