# MiyukiniAdmin — Dashboard & Metrics Display

## 1. Contexte

Ce document definit la specification du **Dashboard principal** et de l'**affichage des metriques** dans MiyukiniAdmin. Le dashboard est le point d'entree principal offrant une vue d'ensemble de l'etat du systeme.

## 2. Portee / Scope

Ce document definit :
- La structure du dashboard
- Les widgets de metriques
- Les visualisations temps reel
- Les alertes et notifications

Ce document **ne couvre pas** :
- Les autres interfaces (DB, Security)
- L'implementation technique
- Les contrats de donnees (voir Monitoring contracts)

---

## 3. Structure du Dashboard

### 3.1 Layout Principal

```
┌─────────────────────────────────────────────────────────────────────────┐
│  MiyukiniAdmin                    [Alerts: 2] [User] [Security: L2]     │
├────────────┬────────────────────────────────────────────────────────────┤
│            │  Dashboard                              [Refresh] [5s ▼]   │
│ Dashboard  │────────────────────────────────────────────────────────────│
│ Metriques  │                                                            │
│ Database   │  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐          │
│ Tests      │  │ Health: 92  │ │ Trust: T0   │ │ Security: 2 │          │
│ Securite   │  │ [Gauge]     │ │ [Badge]     │ │ [Badge]     │          │
│ Logs       │  └─────────────┘ └─────────────┘ └─────────────┘          │
│            │                                                            │
│            │  SYSTEM METRICS                                            │
│            │  ┌──────────────────────┐ ┌──────────────────────┐        │
│            │  │ CPU        45%       │ │ RAM        60%       │        │
│            │  │ [===========      ]  │ │ [============     ]  │        │
│            │  │ Load: 2.4 | 8 cores  │ │ 10GB / 16GB          │        │
│            │  └──────────────────────┘ └──────────────────────┘        │
│            │                                                            │
│            │  ┌──────────────────────┐ ┌──────────────────────┐        │
│            │  │ Disk       55%       │ │ Network              │        │
│            │  │ [==========       ]  │ │ ↓ 1.2 MB/s ↑ 0.5 MB/s│        │
│            │  │ 550GB / 1TB          │ │ 150 connections      │        │
│            │  └──────────────────────┘ └──────────────────────┘        │
│            │                                                            │
│            │  DATABASE METRICS                                          │
│            │  ┌──────────────────────────────────────────────────────┐ │
│            │  │ Queries/sec: 250 | Latency P95: 45ms | Pool: 15/50   │ │
│            │  └──────────────────────────────────────────────────────┘ │
│            │                                                            │
│            │  OPERATORS STATUS                                          │
│            │  ┌──────┬─────────────┬──────────┬───────────┬─────────┐ │
│            │  │ Name │ Status      │ Req/s    │ Errors    │ Latency │ │
│            │  ├──────┼─────────────┼──────────┼───────────┼─────────┤ │
│            │  │ CMS  │ ● Healthy   │ 150      │ 0.1%      │ 25ms    │ │
│            │  │ Auth │ ● Healthy   │ 80       │ 0%        │ 15ms    │ │
│            │  │ API  │ ◐ Degraded  │ 200      │ 2.5%      │ 120ms   │ │
│            │  └──────┴─────────────┴──────────┴───────────┴─────────┘ │
│            │                                                            │
│            │  RECENT ALERTS                                             │
│            │  ┌──────────────────────────────────────────────────────┐ │
│            │  │ ⚠ 12:05 - CPU usage above 85% for 2 minutes          │ │
│            │  │ ℹ 11:45 - Security level changed to 2                │ │
│            │  └──────────────────────────────────────────────────────┘ │
│            │                                                            │
├────────────┴────────────────────────────────────────────────────────────┤
│  v1.0.0 | Uptime: 15d 4h 32m | Last sync: 2s ago | Trust: T0            │
└─────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Sections du Dashboard

| Section | Contenu | Position |
|---------|---------|----------|
| **Header Statut** | Health, Trust, Security | Haut |
| **System Metrics** | CPU, RAM, Disk, Network | Centre-haut |
| **Database Metrics** | Queries, Latency, Pool | Centre |
| **Operators Status** | Liste Operateurs | Centre-bas |
| **Recent Alerts** | Alertes recentes | Bas |

### 3.3 Project Overview (cartes fonctionnalites, inspiration Supabase)

Objectif : afficher des **cartes fonctionnalites** type Supabase (Auth, Storage, Edge Functions, Realtime), adaptees au COG.

- **Contenu :** une carte par domaine pertinent (ex. KindMother, BondingBrother, StrongFather, CaringNanny, etc.) avec titre, description courte, et liens **Explore** (vers la page dediee) et **About** (vers la documentation).
- **Exemple :** "KindMother — Autorite persistance et donnees. Acces controle via BondingBrother." [Explore Database] [About KindMother]
- **Reference :** [Pages et Outils Reference Supabase](../reference/MiyukiniAdmin%20-%20Pages%20et%20Outils%20Reference%20Supabase.md).

### 3.4 PROJECT API (informations environnement)

Objectif : zone **PROJECT API** type Supabase affichant les informations de connexion et de securite de l'environnement.

- **Project URL :** URL de la console MiyukiniAdmin (ex. `https://admin.miyukini.local`) ou endpoint BondingBrother si pertinent. **Pas d'API publique MiyukiniAdmin** (INV-MA-3) : cette zone affiche l'URL de la console et les infos utiles pour l'administration, pas une "Publishable API Key" exposee au public.
- **Description :** "Votre console est securisee. Toute interaction avec les donnees passe par BondingBrother et KindMother. Aucune API publique n'est exposee par MiyukiniAdmin."
- **Reference :** [Invariants & Guarantees](../contracts/governance/MiyukiniAdmin%20-%20Invariants%20&%20Guarantees.md) (INV-MA-3 : aucune API publique).

---

## 4. Widgets de Metriques

### 4.1 Health Score Widget

```
┌───────────────────────────────┐
│         Health Score          │
│                               │
│            92                 │
│         ┌──────┐              │
│         │ ████ │              │
│         │ ████ │ ← Jauge      │
│         │ ████ │   circulaire │
│         │ ░░░░ │              │
│         └──────┘              │
│                               │
│   ● All systems operational   │
└───────────────────────────────┘
```

**Couleurs :**
- 80-100 : Vert
- 60-79 : Jaune
- 40-59 : Orange
- 0-39 : Rouge

### 4.2 CPU Widget

```
┌───────────────────────────────┐
│  CPU                    45%   │
│  ┌─────────────────────────┐  │
│  │█████████████░░░░░░░░░░░░│  │
│  └─────────────────────────┘  │
│                               │
│  Load avg: 2.4 / 2.1 / 1.8    │
│  Cores: 8 | User: 30% Sys: 15%│
└───────────────────────────────┘
```

### 4.3 RAM Widget

```
┌───────────────────────────────┐
│  Memory                 60%   │
│  ┌─────────────────────────┐  │
│  │████████████████░░░░░░░░░│  │
│  └─────────────────────────┘  │
│                               │
│  Used: 10 GB / 16 GB          │
│  Available: 6 GB | Cached: 2GB│
└───────────────────────────────┘
```

### 4.4 Disk Widget

```
┌───────────────────────────────┐
│  Disk                   55%   │
│  ┌─────────────────────────┐  │
│  │█████████████████░░░░░░░░│  │
│  └─────────────────────────┘  │
│                               │
│  550 GB / 1 TB                │
│  R: 1.2 MB/s | W: 0.5 MB/s    │
└───────────────────────────────┘
```

### 4.5 Network Widget

```
┌───────────────────────────────┐
│  Network                      │
│                               │
│  ↓ 1.2 MB/s    ↑ 0.5 MB/s     │
│                               │
│  ┌─────────────────────────┐  │
│  │  [Graph historique]     │  │
│  │  ~~~~~~~~~~~~~~~~~~~    │  │
│  └─────────────────────────┘  │
│                               │
│  Active: 150 | Established: 120│
└───────────────────────────────┘
```

### 4.6 Database Summary Widget

```
┌────────────────────────────────────────────────────────────────┐
│  Database Performance                                          │
│  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐     │
│  │ Queries/sec    │ │ Latency P95    │ │ Pool Usage     │     │
│  │     250        │ │     45ms       │ │    15/50       │     │
│  │    ↑ 12%       │ │    ↓ 5ms       │ │    30%         │     │
│  └────────────────┘ └────────────────┘ └────────────────┘     │
│                                                                │
│  [View Details →]                                              │
└────────────────────────────────────────────────────────────────┘
```

---

## 5. Operateurs Status Table

### 5.1 Structure

```
┌────────────────────────────────────────────────────────────────────────┐
│  Operators Status                                    [Filter] [Refresh]│
├──────────┬─────────────┬──────────┬───────────┬─────────┬─────────────┤
│ Name     │ Status      │ Req/s    │ Error %   │ Latency │ Actions     │
├──────────┼─────────────┼──────────┼───────────┼─────────┼─────────────┤
│ MyCMS    │ ● Healthy   │ 150      │ 0.1%      │ 25ms    │ [...] │
│ AuthSvc  │ ● Healthy   │ 80       │ 0%        │ 15ms    │ [...] │
│ MainAPI  │ ◐ Degraded  │ 200      │ 2.5%      │ 120ms   │ [!] [...] │
│ Worker   │ ● Healthy   │ 50       │ 0%        │ N/A     │ [...] │
├──────────┴─────────────┴──────────┴───────────┴─────────┴─────────────┤
│ Showing 1-4 of 4 operators                                             │
└────────────────────────────────────────────────────────────────────────┘
```

### 5.2 Status Indicators

| Status | Icone | Couleur | Description |
|--------|-------|---------|-------------|
| Healthy | ● | Vert | Fonctionnement normal |
| Degraded | ◐ | Orange | Performance reduite |
| Unhealthy | ○ | Rouge | Problemes |
| Isolated | ⊘ | Gris | Isole par admin |
| Offline | ⊗ | Gris fonce | Non disponible |

### 5.3 Actions Row

| Action | Icone | Description |
|--------|-------|-------------|
| View | 👁 | Voir details |
| Isolate | 🔒 | Isoler l'Operateur |
| Logs | 📋 | Voir logs |

---

## 6. Alertes et Notifications

### 6.1 Panneau Alertes

```
┌────────────────────────────────────────────────────────────────────────┐
│  Recent Alerts                                           [View All →]   │
├────────────────────────────────────────────────────────────────────────┤
│  ⚠ 12:05:32  CPU usage exceeded 85% threshold                          │
│              Duration: 5 minutes | [Acknowledge] [Details]              │
├────────────────────────────────────────────────────────────────────────┤
│  ℹ 11:45:00  Security level changed from 1 to 2                        │
│              By: admin@miyukini | [Details]                             │
├────────────────────────────────────────────────────────────────────────┤
│  ✓ 11:30:15  Database migration completed successfully                  │
│              Duration: 45 seconds | [Details]                           │
└────────────────────────────────────────────────────────────────────────┘
```

### 6.2 Types d'Alertes

| Type | Icone | Couleur | Persistance |
|------|-------|---------|-------------|
| Critical | ⛔ | Rouge | Jusqu'a resolution |
| Warning | ⚠ | Orange | Jusqu'a acknowledge |
| Info | ℹ | Bleu | Auto-dismiss 10s |
| Success | ✓ | Vert | Auto-dismiss 5s |

### 6.3 Header Notification Badge

```
┌────────────┐
│ [Alerts: 2]│ ← Badge rouge si critical
└────────────┘
```

---

## 7. Controles de Rafraichissement

### 7.1 Options

```
┌─────────────────────────────────────┐
│  Auto-refresh: [5s ▼]               │
│                 1s                   │
│                 5s  ←               │
│                 15s                  │
│                 30s                  │
│                 Manual               │
└─────────────────────────────────────┘
```

### 7.2 Indicateur de Sync

```
Footer: ... | Last sync: 2s ago | ...
             ↑
             Indicateur temps reel
```

---

## 8. Graphiques Historiques

### 8.1 Mini-Graph dans Widget

```
┌───────────────────────────────┐
│  CPU (last 5 min)             │
│  ┌─────────────────────────┐  │
│  │    ╱╲    ╱╲             │  │
│  │   ╱  ╲  ╱  ╲    ╱       │  │
│  │  ╱    ╲╱    ╲  ╱        │  │
│  │ ╱            ╲╱         │  │
│  └─────────────────────────┘  │
│  Min: 35% | Max: 72% | Now:45%│
└───────────────────────────────┘
```

### 8.2 Graph Expanded (Click pour agrandir)

```
┌────────────────────────────────────────────────────────────────────────┐
│  CPU History                                    [1h] [6h] [24h] [7d]   │
│  ┌──────────────────────────────────────────────────────────────────┐ │
│  │                                                                  │ │
│  │   100% ─┼──────────────────────────────────────────────────────  │ │
│  │         │                ╱╲                                      │ │
│  │    75% ─┼───────────────╱──╲──────────────────────────────────   │ │
│  │         │     ╱╲    ╱╲ ╱    ╲    ╱╲                              │ │
│  │    50% ─┼────╱──╲──╱──╳──────╲──╱──╲─────────────────────────   │ │
│  │         │   ╱    ╲╱          ╲╱    ╲  ╱╲                         │ │
│  │    25% ─┼──╱──────────────────────────╲╱─────────────────────   │ │
│  │         │                                                        │ │
│  │     0% ─┼──────────────────────────────────────────────────────  │ │
│  │         └────────────────────────────────────────────────────    │ │
│  │          10:00    10:15    10:30    10:45    11:00               │ │
│  └──────────────────────────────────────────────────────────────────┘ │
│                                                                        │
│  Current: 45% | Average: 52% | Peak: 87% at 10:32                      │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 9. Trust Level et Security Level Display

### 9.1 Trust Level Badge

```
┌─────────────────┐
│  Trust: T0      │ ← Fond vert
│  Normal         │
└─────────────────┘

┌─────────────────┐
│  Trust: T3      │ ← Fond rouge, clignote
│  CRITIQUE       │
└─────────────────┘
```

### 9.2 Security Level Badge

```
┌─────────────────┐
│  Security: 2    │ ← Fond jaune
│  SENSITIVE      │
│  [Change]       │
└─────────────────┘
```

---

## 10. Responsive Behavior

### 10.1 Desktop (> 1200px)

Layout complet avec tous les widgets visibles.

### 10.2 Tablet (768-1200px)

- Sidebar collapse par defaut
- Widgets en 2 colonnes au lieu de 4
- Table scrollable horizontalement

### 10.3 Mobile (< 768px)

- Message recommandant desktop
- Affichage basic des metriques critiques seulement

---

## 11. Implications Securite — Session et XSS

### 11.1 Securite de Session

Le Dashboard MiyukiniAdmin manipule des donnees sensibles. La securite de session est critique.

| Controle | Implementation |
|----------|----------------|
| **Session Token** | JWT signe, valide par StrongFather |
| **Expiration** | 30 minutes d'inactivite, 8 heures maximum |
| **Refresh Token** | Rotation a chaque rafraichissement |
| **Invalidation** | Immediate sur logout ou changement de niveau securite |
| **Binding** | Session liee a IP + User-Agent |

### 11.2 Gestion des Sessions UI

```
┌────────────────────────────────────────────────────────────────────────┐
│  SESSION STATUS                                                        │
├────────────────────────────────────────────────────────────────────────┤
│  User: admin@miyukini                                                  │
│  Session expires in: 25:30                                             │
│  Last activity: 4m ago                                                 │
│  IP: 192.168.1.xxx                                                     │
│                                                                        │
│  ⚠ Session sera rafraichie automatiquement                            │
│  [Extend Session] [Logout]                                             │
└────────────────────────────────────────────────────────────────────────┘
```

### 11.3 Prevention XSS (Cross-Site Scripting)

> **INV-UI-SEC-1 : Toute donnee affichee est echappee avant rendu.**

| Type de Donnee | Traitement | Exemple |
|----------------|------------|---------|
| **Texte utilisateur** | Echappement HTML | `<` → `&lt;` |
| **Noms d'operateurs** | Echappement + Whitelist caracteres | Alphanumerique + `-_` |
| **Valeurs metriques** | Validation numerique stricte | Nombre ou erreur |
| **Messages d'alerte** | Echappement + Sanitization | Pas de balises HTML |
| **URLs** | Validation scheme + Echappement | `https://` uniquement |

### 11.4 Content Security Policy (CSP)

Le Dashboard applique une CSP stricte :

```
Content-Security-Policy:
  default-src 'self';
  script-src 'self';
  style-src 'self' 'unsafe-inline';
  img-src 'self' data:;
  connect-src 'self' wss://;
  frame-ancestors 'none';
  form-action 'self';
```

### 11.5 Protection des Donnees Affichees

| Donnee | Affichage | Protection |
|--------|-----------|------------|
| **Emails** | Partiellement masques | `j***@example.com` |
| **IPs internes** | Masquees en niveau securite >= 3 | `192.168.x.x` |
| **Tokens/Secrets** | Jamais affiches | `***` uniquement |
| **Query text** | Jamais affiche | Hash uniquement |
| **Stack traces** | Niveau Admin uniquement | Masque pour Operator |

### 11.6 Auto-Refresh et Securite

Le rafraichissement automatique (section 7) presente des risques :

| Risque | Mitigation |
|--------|------------|
| **Session hijacking** | Validation token a chaque refresh |
| **Stale session** | Verification expiration cote client ET serveur |
| **CSRF** | Token CSRF dans chaque requete |
| **Connection leak** | WebSocket avec heartbeat, cleanup automatique |

### 11.7 Notifications et Securite

Les alertes (section 6) doivent etre securisees :

| Controle | Implementation |
|----------|----------------|
| **Source verifiee** | Alertes provenant uniquement de CaringNanny via BondingBrother |
| **Contenu sanitize** | Echappement strict des messages |
| **Pas d'action directe** | Les alertes n'executent pas de code |
| **Tracabilite** | Acknowledge/Dismiss traces dans audit log |

### 11.8 Adaptation UI par Niveau de Securite (0-4)

| Element UI | Niveau 0-1 | Niveau 2 | Niveau 3 | Niveau 4 |
|------------|------------|----------|----------|----------|
| **Auto-refresh min** | 1s | 5s | 15s | 30s |
| **Details operateurs** | Complet | Complet | Limite | Minimal |
| **Historique** | 24h | 12h | 6h | 1h |
| **Export** | Tous formats | CSV/JSON | CSV | Desactive |
| **Session max** | 8h | 4h | 2h | 1h |

### 11.9 Indicateurs de Securite dans l'UI

Le Dashboard affiche toujours les indicateurs de securite :

```
┌────────────────────────────────────────────────────────────────────────┐
│  Header: ... [Security: L2] [Trust: T0] [Session: 25:30]               │
└────────────────────────────────────────────────────────────────────────┘
```

| Indicateur | Signification |
|------------|---------------|
| **Security: Lx** | Niveau de securite actuel (0-4) |
| **Trust: Tx** | Niveau de confiance systeme (T0-T4) |
| **Session** | Temps restant de session |

### 11.10 References Securite

- [Security - Core Integration Map](../../../security/architecture/Security%20-%20Core%20Integration%20Map.md)
- [Security - Documentation Fondatrice](../../../security/foundation/Security%20-%20Documentation%20Fondatrice.md)
- [MiyukiniAdmin - Threat Model Contract](../contracts/security/MiyukiniAdmin%20-%20Threat%20Model%20Contract.md)

---

## 12. Documents Associes

- [MiyukiniAdmin - UI Design Philosophy](./MiyukiniAdmin%20-%20UI%20Design%20Philosophy.md)
- [MiyukiniAdmin - Consumption Metrics Contract](../contracts/monitoring/MiyukiniAdmin%20-%20Consumption%20Metrics%20Contract.md)
- [MiyukiniAdmin - CaringNanny Integration Contract](../contracts/integration/MiyukiniAdmin%20-%20CaringNanny%20Integration%20Contract.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Document de reference
