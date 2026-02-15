# Jay1Tribu — Service Governance Compliance

## Contexte

Ce document atteste la conformité du **Service Jay1Tribu** aux obligations de gouvernance Miyukini (Cores, Lois d'Autonomie, contraintes du service). Référence : [Jay1Tribu - Contraintes et Invariants](../../Jay1Tribu%20-%20Contraintes%20et%20Invariants.md).

**ServiceId :** `service.jay1tribu`  
**Type :** Service Inter-COG (Type 3).

---

## Obligations de gouvernance

- **Décision** (envoi message, création tribu, invitation, attribution rôles) = **StrongFather** via BondingBrother.
- **Persistance locale** (messages, fichiers, tribus, amis) = **WriteIntent** vers **KindMother** exclusivement.
- **Capacités et permissions** (créer salon, inviter, être Chef de tribu, envoyer fichiers) = **Master Butler**.
- **Sécurité et chiffrement** (transit, au repos) = **WorrySentinel**.
- **Frontières Inter-COG** (qui peut communiquer avec qui) = **Border Guard**.
- **Présence** = fournie par le **MWS** ; Jay1Tribu ne duplique pas la logique.
- **Évolution et compatibilité** = **Ever Buddy**.
- **Modération et intervention humaine** = **TAMR**.

---

## Contraintes non négociables (rappel)

| # | Contrainte |
|---|------------|
| C-1 | Pas d'archives centrales de contenu. |
| C-2 | Transit crypté. |
| C-3 | Hébergement utilisateur. |
| C-4 | Persistance locale via KindMother. |
| C-5 | Service Inter-COG (Type 3). |
| C-6 | Livraison différée tribu conditionnée (reconnexion, émetteur connecté). |
| C-7 | Rôles gouvernés (Chef de tribu, Master Butler, StrongFather). |
| C-8 | Liste d'amis et présence via MWS. |

---

**Date de création :** 2026-02-15  
**Version :** 1.0  
**Statut :** Contrat de conformité gouvernance Service
