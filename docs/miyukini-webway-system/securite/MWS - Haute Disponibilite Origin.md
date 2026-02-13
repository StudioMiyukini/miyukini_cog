# MWS — Haute Disponibilité Origin

## Contexte

**Origin** est le point central de vérité unique du MWS. Une indisponibilité d'Origin impacte l'ensemble du réseau (vérifications, Registre de Services, politiques). Ce document définit les **exigences et mécanismes de haute disponibilité** pour éliminer le Single Point of Failure.

**Référence fondatrice :** [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)  
**Audit :** [MWS - Audit de Sécurité Complet](./MWS%20-%20Audit%20de%20Securite%20Complet.md) — R-001

---

## Portée / Scope

- Objectifs RTO/RPO
- Architecture actif-passif et actif-actif
- Réplication des données critiques
- Relays promotables (Origin temporaire)
- Intégration avec le reste du MWS

---

## 1. Objectifs de disponibilité

| Métrique | Objectif | Description |
|----------|----------|-------------|
| **RTO** (Recovery Time Objective) | < 5 minutes (Phase 1) ; < 30 secondes (Phase 2) | Délai maximum d'indisponibilité accepté |
| **RPO** (Recovery Point Objective) | 0 | Aucune perte de données ; réplication synchrone |
| **Disponibilité cible** | 99,9 % (Phase 1) ; 99,99 % (Phase 2) | Temps de disponibilité sur une année |

---

## 2. Données à répliquer

Les données suivantes, maintenues par Origin, doivent être répliquées de manière synchrone :

| Donnée | Criticité | Mode de réplication |
|--------|-----------|----------------------|
| Registre de Services | Critique | Synchrone |
| Versions des Cores et clés de conformité | Critique | Synchrone |
| Politiques de conformité (quarantaine, blacklist) | Critique | Synchrone |
| Whitelists / Blacklists / Quarantaines maîtres | Critique | Synchrone |
| Registre des Passeports spéciaux | Critique | Synchrone |
| Liste des relays et trackers officiels | Critique | Synchrone |
| Logs d'audit récents | Haute | Asynchrone (accepté) |

---

## 3. Architecture actif-passif (Phase 1)

### 3.1 Schéma

```
                    ┌─────────────────┐
                    │   Load Balancer │
                    │   (Health Check)│
                    └────────┬────────┘
                             │
              ┌──────────────┴──────────────┐
              │                             │
       ┌──────▼──────┐               ┌──────▼──────┐
       │   Origin    │◄─────────────►│   Origin    │
       │   Primaire  │  Réplication  │   Secondaire│
       │   (actif)   │   Synchrone   │   (standby) │
       └──────┬──────┘               └──────┬──────┘
              │                             │
              └──────────────┬──────────────┘
                             │
                    ┌────────▼────────┐
                    │  Stockage      │
                    │  partagé ou    │
                    │  réplication   │
                    │  (PostgreSQL   │
                    │   + Patroni)   │
                    └────────────────┘
```

### 3.2 Règles

| Règle | Description |
|-------|-------------|
| **Un seul actif** | Un seul nœud Origin traite les requêtes à la fois |
| **Bascule automatique** | En cas de défaillance du primaire, le secondaire prend le relais |
| **Health check** | Le load balancer sonde régulièrement (ex. toutes les 5 s) |
| **Réplication synchrone** | Aucun accusé d'écriture côté client tant que la réplication n'est pas confirmée |

### 3.3 Détection de défaillance

| Mécanisme | Seuil | Action |
|-----------|-------|--------|
| Health check HTTP/TCP | 3 échecs consécutifs | Marquer nœud indisponible |
| Timeout de réplication | > 5 secondes | Alerte ; éventuel basculement |
| Panne matérielle / OS | Surveillé par orchestrateur | Basculement automatique |

---

## 4. Architecture actif-actif (Phase 2 — optionnel)

Pour une disponibilité encore plus élevée :

| Aspect | Description |
|--------|-------------|
| **Plusieurs nœuds actifs** | Plusieurs Origin peuvent servir des requêtes en lecture |
| **Écritures** | Une seule autorité d'écriture (leader) ou consensus distribué |
| **Cohérence** | Lecture après écriture garantie pour tous les nœuds |

---

## 5. Relays promotables (Origin temporaire)

En cas d'indisponibilité prolongée d'Origin, des **relays désignés** peuvent être promus en **Origin temporaire** :

| Principe | Description |
|----------|-------------|
| **Désignation** | Origin maintient une liste de relays « promotables » (critères : conformité, capacité, audit) |
| **Rôle temporaire** | Le relay promu sert de source de vérité jusqu'au retour d'Origin |
| **Périmètre** | Lecture complète ; écritures limitées (ex. prolongation de Permis, pas de modification du Registre maître) |
| **Retour** | Dès qu'Origin est de nouveau disponible, synchronisation et retour à l'état relay |

La procédure détaillée est dans [MWS - Procédure de Failover](./MWS%20-%20Procedure%20de%20Failover.md).

---

## 6. Intégration avec les autres acteurs

| Acteur | Comportement en cas de failover |
|--------|---------------------------------|
| **Relays** | Continuent d'utiliser leur cache local ; pull vers le nouveau primaire |
| **Trackers** | Idem ; mise à jour des listes dès que le nouveau Origin répond |
| **COGs** | Aucun changement côté client ; la redirection (REDIRECT) peut pointer vers un relay si Origin est indisponible |

---

## 7. Checklist de déploiement

- [ ] Stockage répliqué (PostgreSQL + Patroni ou équivalent)
- [ ] Load balancer avec health checks
- [ ] Procédure de failover documentée et testée
- [ ] Liste des relays promotables à jour
- [ ] Surveillance (alertes RTO, RPO, réplication)
- [ ] Sauvegardes géo-distribuées du Registre

---

## Références

- [MWS - Document Fondateur](../MWS%20-%20Document%20Fondateur.md)
- [MWS - Origin](../acteurs/MWS%20-%20Origin.md)
- [MWS - Procédure de Failover](./MWS%20-%20Procedure%20de%20Failover.md)
- [MWS - Audit de Sécurité Complet](./MWS%20-%20Audit%20de%20Securite%20Complet.md)

---

**Version :** 1.0  
**Classification :** Documentation MWS — Sécurité
