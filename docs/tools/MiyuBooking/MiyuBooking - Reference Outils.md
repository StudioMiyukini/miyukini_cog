# MiyuBooking — Reference Outils

## Contexte

Ce document liste les **Outils (Tools)** composant le Toolkit **MiyuBooking** (`toolkit.booking.reservations`). Chaque outil est une capacité atomique gouvernée ; décision = StrongFather ; persistance (réservations, créneaux, ressources) = WriteIntent KindMother.

**Référence :** [MiyuBooking - Documentation Fondatrice](./MiyuBooking%20-%20Documentation%20Fondatrice.md)

---

## Liste des outils

| ToolId | Action | Niveau sécurité | Note |
|--------|--------|------------------|------|
| `tool.booking.slots.list` | Lister les créneaux disponibles | 0–1 | Contexte ressource, date, durée fourni |
| `tool.booking.slots.resolve` | Résoudre un créneau par identifiant | 0–1 | Lecture |
| `tool.booking.create` | Créer une réservation | 1–2 | WriteIntent KindMother ; décision StrongFather |
| `tool.booking.update` | Mettre à jour une réservation | 1–2 | Déplacement, prolongation ; WriteIntent KindMother |
| `tool.booking.cancel` | Annuler une réservation | 1–2 | Décision politique StrongFather ; WriteIntent KindMother |
| `tool.booking.resource.resolve` | Résoudre une ressource (salle, équipement) | 0–1 | Contraintes fournies par KindMother |
| `tool.booking.resource.availability` | Retourner la disponibilité d'une ressource | 0–1 | Plage donnée ; lecture gouvernée |
| `tool.booking.price.compute` | Calculer le prix d'une réservation | 0–1 | Règles fournies dans le flux |
| `tool.booking.participants.compute` | Calculer places restantes / participants | 0–1 | Pour un créneau donné |

---

**Invariant :** Toute écriture (réservation) = **WriteIntent** vers KindMother. Toute décision (création, annulation) = StrongFather.
