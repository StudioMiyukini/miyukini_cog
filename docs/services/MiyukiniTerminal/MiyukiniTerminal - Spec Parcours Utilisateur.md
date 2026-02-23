# MiyukiniTerminal — Spécification Parcours Utilisateur

## Contexte

Ce document décrit les **parcours utilisateur** principaux (liaison, première sync, consultation services, action différée, erreur réseau) et les **edge cases** (token expiré, parent déconnecté, limite 5 atteinte) avec messages d'erreur.

**Références :**

- [Spec Flux Liaison Parent](./MiyukiniTerminal%20-%20Spec%20Flux%20Liaison%20Parent.md)
- [Spec Mode Offline](./MiyukiniTerminal%20-%20Spec%20Mode%20Offline.md)
- [Spec Ecrans et Navigation](./MiyukiniTerminal%20-%20Spec%20Ecrans%20et%20Navigation.md)

---

## Portée / Scope

- Parcours : liaison, sync, consultation, action différée, erreur
- Edge cases
- Messages d'erreur

---

## 1. Parcours liaison

| Étape | Action utilisateur | Système |
|-------|-------------------|---------|
| 1 | Ouvrir app (première fois) | Afficher écran Liaison |
| 2 | Choisir "Scanner QR" ou "Saisir code" | — |
| 3a | Scanner QR affiché sur Central | Décoder token |
| 3b | Saisir code XXXX-XXXX | Récupérer token via API |
| 4 | Clic "Lier" | Valider token, REGISTER Relay |
| 5 | Attente | Indicateur chargement |
| 6 | Succès | Passage au Salon |
| 6' | Échec | Message erreur ; reste sur Liaison |

---

## 2. Parcours première sync

| Étape | Action | Système |
|-------|--------|---------|
| 1 | Après liaison | Demander sync initiale |
| 2 | Envoi requête au parent | — |
| 3 | Réception données | Stocker cache |
| 4 | Affichage Salon | Liste services peuplée |
| 5 | Erreur | Message "Impossible de charger. Réessayez." |

---

## 3. Parcours consultation services

| Étape | Action | Système |
|-------|--------|---------|
| 1 | Clic sur JayKonta | Ouvrir écran Service |
| 2 | Affichage soldes, mouvements | Données depuis cache ou sync |
| 3 | Pull-to-refresh | Lancer sync |
| 4 | Données à jour | Rafraîchir affichage |

---

## 4. Parcours action différée (offline)

| Étape | Action | Système |
|-------|--------|---------|
| 1 | Utilisateur hors ligne | Indicateur Offline |
| 2 | Saisir dépense | Enregistrer en queue |
| 3 | Message | "Enregistré. Sera synchronisé à la reconnexion." |
| 4 | Reconnexion | Rejouer queue |
| 5 | Succès | "Synchronisé" ; vider badge |

---

## 5. Parcours erreur réseau

| Étape | Action | Système |
|-------|--------|---------|
| 1 | Perte connexion | Passage Offline |
| 2 | Indicateur | Icône Offline, bannière optionnelle |
| 3 | Lecture | Cache uniquement |
| 4 | Écriture | Queue |
| 5 | Reconnexion | Sync auto ; notification "Connexion rétablie" |

---

## 6. Edge cases

### 6.1 Token expiré

| Contexte | Message |
|----------|---------|
| Scan/saisie token > 15 min | "Le lien a expiré. Générez-en un nouveau depuis Central." |
| Action | Bouton "Réessayer" ; afficher instructions pour Central |

### 6.2 Parent déconnecté

| Contexte | Message |
|----------|---------|
| REGISTER_ERR (parent invalide) | "Votre COG parent n'est pas connecté. Démarrez Central sur votre ordinateur et réessayez." |
| Action | Proposer "Réessayer" |

### 6.3 Limite 5 atteinte

| Contexte | Message (côté Central) |
|----------|------------------------|
| Clic "Ajouter" avec 5 terminaux | "Vous avez atteint la limite de 5 terminaux. Révoquez-en un pour en ajouter." |

### 6.4 Permis révoqué

| Contexte | Message |
|----------|---------|
| CLOSE ou PERMIT_REVOKE | "Votre accès a été révoqué. Relancez la liaison depuis Central." |
| Action | Retour écran Liaison ; effacer identité locale |

### 6.5 Données corrompues

| Contexte | Message |
|----------|---------|
| Cache illisible | "Données endommagées. Reconnectez-vous pour recharger." |
| Action | Proposer relancer liaison ou réinstaller |

---

## 7. Messages d'erreur (référentiel)

| Code / Situation | Message utilisateur |
|------------------|---------------------|
| Token invalide | "Lien invalide. Vérifiez le code ou scannez à nouveau." |
| Token expiré | "Le lien a expiré. Générez-en un nouveau depuis Central." |
| parent_invalid | "Votre COG parent n'est pas accessible. Vérifiez qu'il est démarré." |
| blacklisted | "Accès refusé. Contactez le support." |
| Pas de réseau | "Pas de connexion. Vérifiez votre réseau." |
| Timeout | "La connexion a expiré. Réessayez." |
| Erreur inconnue | "Une erreur s'est produite. Réessayez plus tard." |

---

## 8. Références

- [Spec Flux Liaison Parent](./MiyukiniTerminal%20-%20Spec%20Flux%20Liaison%20Parent.md)
- [Spec Mode Offline](./MiyukiniTerminal%20-%20Spec%20Mode%20Offline.md)
- [Spec Ecrans et Navigation](./MiyukiniTerminal%20-%20Spec%20Ecrans%20et%20Navigation.md)
