# Les 8 Lois d'Autonomie

## Principes Fondamentaux Non Négociables

Les **8 Lois d'Autonomie** constituent le socle inviolable du système Miyukini. Ces lois ne sont **jamais** négociables, modifiables ou contournables. Elles garantissent l'indépendance et la souveraineté de chaque COG.

---

## LOI-1 : Aucune Dépendance Externe Critique

> *Aucune dépendance externe critique à l'exécution*

### Signification

Le système doit pouvoir fonctionner **intégralement** sans connexion à des services externes. Aucune fonctionnalité critique ne peut dépendre d'un serveur tiers, d'une API cloud, ou d'un service distant.

### Implications

- Les bases de données sont locales
- L'authentification fonctionne offline
- Les mises à jour ne sont jamais forcées
- Aucun "phone home" obligatoire

---

## LOI-2 : L'Isolement comme État Normal

> *Le système accepte l'isolement comme état normal*

### Signification

L'isolation n'est pas une dégradation, c'est un **mode de fonctionnement légitime**. Un COG doit considérer l'absence de réseau comme une situation normale, pas comme une erreur.

### Implications

- Mode offline-first par conception
- Aucune fonctionnalité dégradée en isolation
- Synchronisation opportuniste, jamais obligatoire
- Interface utilisateur identique online/offline

---

## LOI-3 : Souveraineté de l'État Local

> *L'état local est souverain*

### Signification

Les données locales sont la **source de vérité absolue**. Aucune entité externe ne peut imposer un état différent. En cas de conflit, l'état local prime.

### Implications

- Pas de "sync forcée" depuis le cloud
- Résolution de conflits sous contrôle utilisateur
- Données jamais écrasées sans consentement
- Backups locaux prioritaires

---

## LOI-4 : Indépendance Temporelle

> *Pas de temps global requis*

### Signification

Le système ne dépend pas d'une horloge externe ou d'un serveur NTP. Chaque COG gère son propre temps, et les protocoles acceptent les dérives temporelles.

### Implications

- Horloges vectorielles pour la synchronisation
- Tolérance aux décalages temporels
- Pas de certificats dépendant d'une heure précise
- Fonctionnement sur hardware sans RTC fiable

---

## LOI-5 : Coût Proportionnel au Hardware

> *Le coût doit être proportionnel au hardware*

### Signification

Un COG doit pouvoir fonctionner sur du matériel modeste. Les ressources consommées doivent être **proportionnelles** aux capacités de la machine, pas aux fonctionnalités désirées.

### Implications

- Pas de minimum hardware prohibitif
- Fonctionnalités adaptatives selon les ressources
- Optimisation pour machines anciennes
- Pas de bloat artificiel

---

## LOI-6 : Fédération Optionnelle

> *L'autonomie n'empêche pas la fédération*

### Signification

La capacité de fonctionner seul n'exclut pas la possibilité de **collaborer**. Un COG peut rejoindre le réseau Webway sans compromettre son autonomie.

### Implications

- Participation réseau = choix, jamais obligation
- Fédération révocable à tout moment
- Données partagées uniquement sur consentement
- Interopérabilité sans dépendance

---

## LOI-7 : Immutabilité des Cores

> *La strate Cores est immuable — évolution par environnement*

### Signification

Les 8 Cores sont **figés** une fois déployés. Aucune mise à jour, aucun patch. L'évolution du système passe par la création d'un **nouveau COG** avec une nouvelle version des Cores.

### Implications

- Pas de "mise à jour système" traditionnelle
- Versions des Cores = contrat immuable
- Nouvelles fonctionnalités = nouvel environnement
- Stabilité absolue garantie

---

## LOI-8 : Migration Diplomatique

> *Migration = diplomatie entre environnements*

### Signification

Le transfert de données entre COGs est un acte **formel et négocié**. Jamais une simple copie. Les deux environnements doivent s'accorder sur les modalités du transfert.

### Implications

- Protocoles de migration explicites
- Validation des invariants à chaque étape
- Consentement mutuel requis
- Traçabilité complète du processus

---

## Hiérarchie des Lois

En cas de conflit apparent entre lois, l'ordre de priorité est :

1. **LOI-1** (autonomie) > tout
2. **LOI-3** (souveraineté locale) > synchronisation
3. **LOI-7** (immutabilité) > évolution
4. **LOI-2** (isolement) > connectivité

## Application Pratique

Ces lois influencent chaque décision architecturale :

| Décision | Loi Appliquée |
|----------|---------------|
| Base de données SQLite locale | LOI-1, LOI-3 |
| Mode offline complet | LOI-2 |
| Pas de serveur de temps | LOI-4 |
| Support Raspberry Pi | LOI-5 |
| Webway optionnel | LOI-6 |
| Versions figées des Cores | LOI-7 |
| Processus de migration formel | LOI-8 |

## En pratique : vérification rapide

Avant toute décision produit ou stratégique, on peut vérifier :

| Question | Réponse attendue |
|----------|------------------|
| Le système fonctionne-t-il offline ? | Oui (LOI-1, LOI-2) |
| Dépend-il d'un service externe pour fonctionner ? | Non (LOI-1) |
| Un non-développeur peut-il utiliser des Services ? | Oui (via les Opérateurs exposés) |
| La dégradation en cas de problème est-elle contrôlée ? | Oui (WorrySentinel, états T0–T4) |
| Peut-on faire évoluer sans tout casser ? | Oui (LOI-7, LOI-8 : nouveaux environnements, composition) |
