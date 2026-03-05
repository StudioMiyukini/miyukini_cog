<!-- @id mip.profiles.index
     @do route_mip_profile_selection
     @role config
     @layer config
     @human Index profils MIP — bascule outil/LLM avec contraintes -->

# MIP Profile System — Complete Index & Navigation

> **Basculer d'outil/LLM/contraintes sans changer le protocole.** MIP s'adapte aux capacités du profil actif.

---

## Quick Start (5 minutes)

**Vous venez d'arriver?**

1. **Créer un profil**: [TEMPLATES.md](./TEMPLATES.md) — 4 templates (corporate, freelance, healthcare, startup)
2. **Ajouter des contraintes**: [CONSTRAINTS.md](./CONSTRAINTS.md) — legal, confidential, regional, offline, etc.
3. **Comprendre CRUD**: [MANAGEMENT.md](./MANAGEMENT.md) — Create/Read/Update/Delete profiles

**Commandes rapides**:
```bash
mip_profile list                    # List all profiles
mip_profile show                    # Current active
mip_profile {name}                  # Switch to profile
mip_profile create-from-template corporate  # Interactive setup
mip_profile apply-constraint {profile} {constraint}  # Add constraint
```

---

## System Structure

### Core Files (Read These First)

| File | Purpose | Time |
|------|---------|------|
| **README.md** | Overview: 4 profile types + examples | 3 min |
| **MANAGEMENT.md** | CRUD protocol + all CLI commands | 10 min |
| **TEMPLATES.md** | 4 templates + how to create custom | 8 min |
| **CONSTRAINTS.md** | All constraint types + merging | 10 min |

### Builtin Profiles (Pre-configured)

```
builtin/
├── anthropic-opus.md        ← Claude 3 Opus (best, €1-3/task)
├── anthropic-sonnet.md      ← Claude 3 Sonnet (fast, good)
├── mistral-nemo.md          ← Mistral Nemo (balanced, cheap)
├── mistral-small-api.md     ← Mistral Small (minimal)
├── github-copilot-free.md   ← GitHub Copilot FREEplan, Mode 3)
├── ollama.md                ← Local Ollama (offline, free)
├── lm-studio.md             ← LM Studio GUI (offline, free)
└── cursor-composer.md       ← Cursor IDE integration
```

### Constraint Definitions

```
constraints/
├── legal-compliance.md      ← Audit trails + consent (SOX/GDPR)
├── confidential-data.md     ← No logging, no cache (secret code)
├── offline-only.md          ← Complete isolation (air-gap)
├── regional-eu.md           ← EU data residency (GDPR)
├── regional-us.md           ← US-only (CCPA)
├── regional-china.md        ← China-only (PIPL)
├── pii-strict.md            ← PII scanning + redaction
├── hipaa-compliant.md       ← Healthcare PHI handling
├── tool-locked-cursor.md    ← Requires Cursor IDE
├── tool-locked-vscode.md    ← Requires VS Code
└── custom-template.md       ← Create your own constraints
```

### User Profiles (Create Here)

```
custom/
├── my-work-setup.md         ← Your personal profile
├── client-acme.md           ← Client-specific
├── project-secret.md        ← Confidential project variant
└── [anything-you-create]
```

### Generated/Cache (Auto-managed)

```
cache/
├── merged-profiles/         ← Profile + stacked constraints
├── history/                 ← Profile switch history
└── validation-logs/         ← Last validation results
```

---

## Navigation by Use Case

### "I'm at Total Energy (Corporate)"

```
Template: corporate
Base: mistral-nemo
Constraints: legal-compliance, confidential-data, regional-eu
Cost: €0.10-0.15 per task
Ref: TEMPLATES.md#corporate
```

### "I'm a freelancer with clients"

```
Template: freelance
Base: mistral-nemo or ollama
Constraints: confidential-data (per client)
Cost: €0 or €0.10-0.15
Ref: TEMPLATES.md#freelance
```

### "I'm in healthcare"

```
Template: healthcare
Base: claude-sonnet (higher quality for medical)
Constraints: hipaa-compliant, pii-strict, legal-compliance
Cost: €0.50-1.00 per task
Ref: TEMPLATES.md#healthcare
```

### "I'm a startup (MVP mode)"

```
Template: startup
Base: claude-sonnet or mistral-nemo
Constraints: None (speed first)
Cost: €1-3 per task
Ref: TEMPLATES.md#startup
```

### "I don't have internet"

```
Profile: ollama
Constraints: offline-only
Cost: €0 (hardware only)
Ref: constraints/offline-only.md
```

### "I need GDPR compliance"

```
Base: mistral-nemo or claude
Constraints: regional-eu, legal-compliance
Cost: +5-10% (EU premium)
Ref: constraints/regional-eu.md
```

---

## Profil Actif

Le profil actif est stocké dans `.mip/profiles/active` (une ligne = slug du profil).

**Basculer**: `mip_profile {slug}` pour changer de profil.

---

## Tous les Profils Disponibles

| Slug | Nom | LLM | Mode | Coût | Use Case |
|------|-----|-----|------|------|----------|
| **anthropic-opus** | Claude 3 Opus | Claude Opus | 1 (100%) | €1-3 | Full autonomy |
| **mistral-nemo** | Mistral Nemo | Mistral 7B | 2 (90%) | €0-0.15 | Balanced choice |
| **github-copilot-free** | GitHub Copilot Free | Copilot | 3 (40%) | Free | Assisted only |
| **ollama** | Local Ollama | Llama/Mistral | 5 (50%) | Free | Offline |
| cursor-composer | Cursor Composer | Claude | 2 (90%) | Incl. | IDE native |
| codex | OpenAI Codex | GPT-4o | 1 (95%) | €2-5 | High quality |
| mistral-small | Mistral Small | Mistral | 2 (80%) | €0.01 | Budget |
| lm-studio | LM Studio | Any GGUF | 5 (60%) | Free | Offline GUI |

---

## Créer un Profil

### Méthode 1: Template Interactif (SETUP-5)

```bash
mip_profile create my-setup
# Maria pose des questions → profil auto-généré
```

### Méthode 2: Template pre-fait

```bash
mip_profile create-from-template corporate --name acme-setup
# Copie template, ouvre éditeur, remplace :PLACEHOLDER: values
```

### Méthode 3: YAML manuel

```bash
cp templates/freelance.md custom/my-project.md
# & modifier YAML manuellement
mip_profile validate my-project
```

**Voir**: [TEMPLATES.md](./TEMPLATES.md)

---

## Ajouter des Contraintes

### Étape 1: Identifier la contrainte

Besoin audit trail? → `legal-compliance`
Besoin secret? → `confidential-data`
Besoin offline? → `offline-only`
Besoin GDPR? → `regional-eu`

### Étape 2: Appliquer

```bash
mip_profile apply-constraint my-profile legal-compliance
mip_profile apply-constraint my-profile confidential-data
```

### Étape 3: Vérifier

```bash
mip_profile show my-profile
# Affiche: Constraints: legal-compliance, confidential-data
```

**Voir**: [CONSTRAINTS.md](./CONSTRAINTS.md)

---

## Arbres de Décision

### Quel profil?

```
Budget critique? → Mistral Nemo
Autonomie max? → Claude Opus
Gratuit + offline? → Ollama
Copilot only? → GitHub Copilot Free
```

### Quelle contrainte?

```
Audit trails? → legal-compliance
Secret/NDA? → confidential-data
Pas internet? → offline-only
GDPR? → regional-eu
Données sensibles? → pii-strict + confidential-data
```

**Voir la matrice complète**: [CONSTRAINTS.md](./CONSTRAINTS.md#constraint-matrix)

---

## Commandes Rapides

```bash
# Profils
mip_profile list                                    # Tous les profils
mip_profile show                                    # Profil actif
mip_profile {nom}                                   # Activer
mip_profile create {nom} --base mistral-nemo      # Créer
mip_profile delete {nom}                           # Supprimer

# Contraintes
mip_profile apply-constraint {profil} {contrainte}     # Ajouter
mip_profile remove-constraint {profil} {contrainte}    # Retirer
mip_profile show-constraints {profil}                  # Lister

# Validation
mip_profile validate {profil}                      # Vérifier
mip_profile check-capabilities {profil}            # Capacités
mip_profile history                                # Historique bascules

# Export/Import
mip_profile export {profil}                        # Sauvegarder
mip_profile import {fichier.yaml}                 # Charger

# Contraintes spécifiques
mip_profile show-constraint regional-eu            # Détails contrainte
mip_profile test-constraint offline-only           # Tester
```

**Référence complète**: [MANAGEMENT.md#cli-commands](./MANAGEMENT.md#cli-commands)

---

## Abonnements & Quotas

Configurer les quotas dans `.mip/config/subscriptions.md` (token limits par fournisseur).

MIP estime la consommation vs quota et alerte si >80%.

---

## Exemples Concrets

### Exemple 1: Freelancer avec 2 clients

```
Profil par défaut: mistral-nemo-budget
  Constraints: None

Profil client Acme (NDA):
  Base: mistral-nemo
  Constraints: confidential-data
  
Profil client BankCorp (GDPR):
  Base: mistral-nemo
  Constraints: legal-compliance, regional-eu, confidential-data

Workflow: Basculer entre profils selon client
```

### Exemple 2: Startup (vitesse)

```
Profil: claude-opus-startup
  Constraints: None (vitesse avant tout)
  
Workflow: Autonomie max, parallelism, coût acceptable
```

### Exemple 3: Top Secret (Government)

```
Profil: offline-vault
  Base: ollama-mistral
  Constraints: offline-only, confidential-data, legal-compliance
  
Workflow: Machine air-gappée, pas internet, traitement 100% local
```

---

## Contraintes Courantes

| Contrainte | Cas d'usage | Overhead | Ref |
|-----------|-----------|----------|-----|
| **legal-compliance** | Audit required (SOX, compliance) | +15% | [lire](./constraints/legal-compliance.md) |
| **confidential-data** | Client code / trade secrets | +30% | [lire](./constraints/confidential-data.md) |
| **offline-only** | No internet (air-gap, security) | -50% perf | [lire](./constraints/offline-only.md) |
| **regional-eu** | GDPR / EU data residency | +5% cost | [lire](./constraints/regional-eu.md) |
| **pii-strict** | Healthcare / customer data | +10% | [lire](./constraints/pii-strict.md) |
| **hipaa-compliant** | HIPAA (healthcare) | +20% | [lire](./constraints/hipaa-compliant.md) |

---

## Matrice: Profile × Constraint

| Profile | legal | confid | offline | regional-eu | pii |
|---------|-------|--------|---------|-------------|-----|
| Claude | ✅ | ✅ | ❌ | ⚠️ | ✅ |
| Mistral | ✅ | ✅ | ❌ | ✅ | ✅ |
| Copilot | ✅ | ✅ | ❌ | ❌ | ✅ |
| Ollama | ✅ | ✅ | ✅ | ✅ | ✅ |

✅ = Fully supported | ⚠️ = Partial | ❌ = Not supported

---

## Références

| Document | | |
|----------|------|---|
| **README.md** | Overview | 5 min |
| **MANAGEMENT.md** | CRUD protocol | 10 min |
| **TEMPLATES.md** | Create custom | 8 min |
| **CONSTRAINTS.md** | All constraints | 10 min |
| **Individual profiles** | builtin/*.md | Skim |
| **Individual constraints** | constraints/*.md | As needed |

---

## Pour Aller Plus Loin

- **Modes adaptatifs** : `.mip/profiles/ADAPTIVE-MODES.md` (5 modes d'exécution)
- **Négociation capacités** : `.mip/profiles/CAPABILITY-NEGOTIATION.md` (transparence LLM)
- **Scenarios industriels** : `.mip/usecases/INDUSTRIAL-SCENARIOS.md` (Total/freelance/startup)
- **Schema** : `.mip/profiles/SCHEMA.md` (structure YAML)
- **Matrice capacités** : `.mip/profiles/capabilities-matrix.md` (tool × capability)
- **SETUP** : `.mip/modules/setup.md` (intégration SETUP-5)
- **Config** : `.mip/config/subscriptions.md` (quotas API)

---

*Last updated: 2025-01*  
*Part of MIP v2 — Adaptive Profile System with Constraints*
