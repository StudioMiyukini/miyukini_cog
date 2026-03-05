<!-- @id mip.profiles.index
     @do route_mip_profile_selection
     @role config
     @layer config
     @human Index profils MIP â€” bascule outil/LLM avec contraintes -->

# MIP Profile System â€” Complete Index & Navigation

> **Basculer d'outil/LLM/contraintes sans changer le protocole.** MIP s'adapte aux capacitÃ©s du profil actif.

---

## Quick Start (5 minutes)

**Vous venez d'arriver?**

1. **CrÃ©er un profil**: [TEMPLATES.md](./TEMPLATES.md) â€” 4 templates (corporate, freelance, healthcare, startup)
2. **Ajouter des contraintes**: [CONSTRAINTS.md](./CONSTRAINTS.md) â€” legal, confidential, regional, offline, etc.
3. **Comprendre CRUD**: [MANAGEMENT.md](./MANAGEMENT.md) â€” Create/Read/Update/Delete profiles

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
â”œâ”€â”€ anthropic-opus.md        â† Claude 3 Opus (best, â‚¬1-3/task)
â”œâ”€â”€ anthropic-sonnet.md      â† Claude 3 Sonnet (fast, good)
â”œâ”€â”€ mistral-nemo.md          â† Mistral Nemo (balanced, cheap)
â”œâ”€â”€ mistral-small-api.md     â† Mistral Small (minimal)
â”œâ”€â”€ github-copilot-free.md   â† GitHub Copilot FREEplan, Mode 3)
â”œâ”€â”€ ollama.md                â† Local Ollama (offline, free)
â”œâ”€â”€ lm-studio.md             â† LM Studio GUI (offline, free)
â””â”€â”€ cursor-composer.md       â† Cursor IDE integration
```

### Constraint Definitions

```
constraints/
â”œâ”€â”€ legal-compliance.md      â† Audit trails + consent (SOX/GDPR)
â”œâ”€â”€ confidential-data.md     â† No logging, no cache (secret code)
â”œâ”€â”€ offline-only.md          â† Complete isolation (air-gap)
â”œâ”€â”€ regional-eu.md           â† EU data residency (GDPR)
â”œâ”€â”€ regional-us.md           â† US-only (CCPA)
â”œâ”€â”€ regional-china.md        â† China-only (PIPL)
â”œâ”€â”€ pii-strict.md            â† PII scanning + redaction
â”œâ”€â”€ hipaa-compliant.md       â† Healthcare PHI handling
â”œâ”€â”€ tool-locked-cursor.md    â† Requires Cursor IDE
â”œâ”€â”€ tool-locked-vscode.md    â† Requires VS Code
â””â”€â”€ custom-template.md       â† Create your own constraints
```

### User Profiles (Create Here)

```
custom/
â”œâ”€â”€ my-work-setup.md         â† Your personal profile
â”œâ”€â”€ client-acme.md           â† Client-specific
â”œâ”€â”€ project-secret.md        â† Confidential project variant
â””â”€â”€ [anything-you-create]
```

### Generated/Cache (Auto-managed)

```
cache/
â”œâ”€â”€ merged-profiles/         â† Profile + stacked constraints
â”œâ”€â”€ history/                 â† Profile switch history
â””â”€â”€ validation-logs/         â† Last validation results
```

---

## Navigation by Use Case

### "I'm at Total Energy (Corporate)"

```
Template: corporate
Base: mistral-nemo
Constraints: legal-compliance, confidential-data, regional-eu
Cost: â‚¬0.10-0.15 per task
Ref: TEMPLATES.md#corporate
```

### "I'm a freelancer with clients"

```
Template: freelance
Base: mistral-nemo or ollama
Constraints: confidential-data (per client)
Cost: â‚¬0 or â‚¬0.10-0.15
Ref: TEMPLATES.md#freelance
```

### "I'm in healthcare"

```
Template: healthcare
Base: claude-sonnet (higher quality for medical)
Constraints: hipaa-compliant, pii-strict, legal-compliance
Cost: â‚¬0.50-1.00 per task
Ref: TEMPLATES.md#healthcare
```

### "I'm a startup (MVP mode)"

```
Template: startup
Base: claude-sonnet or mistral-nemo
Constraints: None (speed first)
Cost: â‚¬1-3 per task
Ref: TEMPLATES.md#startup
```

### "I don't have internet"

```
Profile: ollama
Constraints: offline-only
Cost: â‚¬0 (hardware only)
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

Le profil actif est stockÃ© dans `.mip/profiles/active` (une ligne = slug du profil).

**Basculer**: `mip_profile {slug}` pour changer de profil.

---

## Tous les Profils Disponibles

| Slug | Nom | LLM | Mode | CoÃ»t | Use Case |
|------|-----|-----|------|------|----------|
| **anthropic-opus** | Claude 3 Opus | Claude Opus | 1 (100%) | â‚¬1-3 | Full autonomy |
| **mistral-nemo** | Mistral Nemo | Mistral 7B | 2 (90%) | â‚¬0-0.15 | Balanced choice |
| **github-copilot-free** | GitHub Copilot Free | Copilot | 3 (40%) | Free | Assisted only |
| **ollama** | Local Ollama | Llama/Mistral | 5 (50%) | Free | Offline |
| cursor-composer | Cursor Composer | Claude | 2 (90%) | Incl. | IDE native |
| codex | OpenAI Codex | GPT-4o | 1 (95%) | â‚¬2-5 | High quality |
| mistral-small | Mistral Small | Mistral | 2 (80%) | â‚¬0.01 | Budget |
| lm-studio | LM Studio | Any GGUF | 5 (60%) | Free | Offline GUI |

---

## CrÃ©er un Profil

### MÃ©thode 1: Template Interactif (SETUP-5)

```bash
mip_profile create my-setup
# Maria pose des questions â†’ profil auto-gÃ©nÃ©rÃ©
```

### MÃ©thode 2: Template pre-fait

```bash
mip_profile create-from-template corporate --name acme-setup
# Copie template, ouvre Ã©diteur, remplace :PLACEHOLDER: values
```

### MÃ©thode 3: YAML manuel

```bash
cp templates/freelance.md custom/my-project.md
# & modifier YAML manuellement
mip_profile validate my-project
```

**Voir**: [TEMPLATES.md](./TEMPLATES.md)

---

## Ajouter des Contraintes

### Ã‰tape 1: Identifier la contrainte

Besoin audit trail? â†’ `legal-compliance`
Besoin secret? â†’ `confidential-data`
Besoin offline? â†’ `offline-only`
Besoin GDPR? â†’ `regional-eu`

### Ã‰tape 2: Appliquer

```bash
mip_profile apply-constraint my-profile legal-compliance
mip_profile apply-constraint my-profile confidential-data
```

### Ã‰tape 3: VÃ©rifier

```bash
mip_profile show my-profile
# Affiche: Constraints: legal-compliance, confidential-data
```

**Voir**: [CONSTRAINTS.md](./CONSTRAINTS.md)

---

## Arbres de DÃ©cision

### Quel profil?

```
Budget critique? â†’ Mistral Nemo
Autonomie max? â†’ Claude Opus
Gratuit + offline? â†’ Ollama
Copilot only? â†’ GitHub Copilot Free
```

### Quelle contrainte?

```
Audit trails? â†’ legal-compliance
Secret/NDA? â†’ confidential-data
Pas internet? â†’ offline-only
GDPR? â†’ regional-eu
DonnÃ©es sensibles? â†’ pii-strict + confidential-data
```

**Voir la matrice complÃ¨te**: [CONSTRAINTS.md](./CONSTRAINTS.md#constraint-matrix)

---

## Commandes Rapides

```bash
# Profils
mip_profile list                                    # Tous les profils
mip_profile show                                    # Profil actif
mip_profile {nom}                                   # Activer
mip_profile create {nom} --base mistral-nemo      # CrÃ©er
mip_profile delete {nom}                           # Supprimer

# Contraintes
mip_profile apply-constraint {profil} {contrainte}     # Ajouter
mip_profile remove-constraint {profil} {contrainte}    # Retirer
mip_profile show-constraints {profil}                  # Lister

# Validation
mip_profile validate {profil}                      # VÃ©rifier
mip_profile check-capabilities {profil}            # CapacitÃ©s
mip_profile history                                # Historique bascules

# Export/Import
mip_profile export {profil}                        # Sauvegarder
mip_profile import {fichier.yaml}                 # Charger

# Contraintes spÃ©cifiques
mip_profile show-constraint regional-eu            # DÃ©tails contrainte
mip_profile test-constraint offline-only           # Tester
```

**RÃ©fÃ©rence complÃ¨te**: [MANAGEMENT.md#cli-commands](./MANAGEMENT.md#cli-commands)

---

## Abonnements & Quotas

Configurer les quotas dans `.mip/config/subscriptions.md` (token limits par fournisseur).

MIP estime la consommation vs quota et alerte si >80%.

---

## Exemples Concrets

### Exemple 1: Freelancer avec 2 clients

```
Profil par dÃ©faut: mistral-nemo-budget
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
  
Workflow: Autonomie max, parallelism, coÃ»t acceptable
```

### Exemple 3: Top Secret (Government)

```
Profil: offline-vault
  Base: ollama-mistral
  Constraints: offline-only, confidential-data, legal-compliance
  
Workflow: Machine air-gappÃ©e, pas internet, traitement 100% local
```

---

## Contraintes Courantes

| Contrainte | Cas d'usage | Overhead | Ref |
|-----------|-----------|----------|-----|
| **legal-compliance** | Audit required (SOX, compliance) | +15% | [lire](README.md) |
| **confidential-data** | Client code / trade secrets | +30% | [lire](README.md) |
| **offline-only** | No internet (air-gap, security) | -50% perf | [lire](README.md) |
| **regional-eu** | GDPR / EU data residency | +5% cost | [lire](README.md) |
| **pii-strict** | Healthcare / customer data | +10% | [lire](README.md) |
| **hipaa-compliant** | HIPAA (healthcare) | +20% | [lire](README.md) |

---

## Matrice: Profile Ã— Constraint

| Profile | legal | confid | offline | regional-eu | pii |
|---------|-------|--------|---------|-------------|-----|
| Claude | âœ… | âœ… | âŒ | âš ï¸ | âœ… |
| Mistral | âœ… | âœ… | âŒ | âœ… | âœ… |
| Copilot | âœ… | âœ… | âŒ | âŒ | âœ… |
| Ollama | âœ… | âœ… | âœ… | âœ… | âœ… |

âœ… = Fully supported | âš ï¸ = Partial | âŒ = Not supported

---

## RÃ©fÃ©rences

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

- **Modes adaptatifs** : `.mip/profiles/ADAPTIVE-MODES.md` (5 modes d'exÃ©cution)
- **NÃ©gociation capacitÃ©s** : `.mip/profiles/CAPABILITY-NEGOTIATION.md` (transparence LLM)
- **Scenarios industriels** : `.mip/usecases/INDUSTRIAL-SCENARIOS.md` (Total/freelance/startup)
- **Schema** : `.mip/profiles/SCHEMA.md` (structure YAML)
- **Matrice capacitÃ©s** : `.mip/profiles/capabilities-matrix.md` (tool Ã— capability)
- **SETUP** : `.mip/modules/setup.md` (intÃ©gration SETUP-5)
- **Config** : `.mip/config/subscriptions.md` (quotas API)

---

*Last updated: 2025-01*  
*Part of MIP v2 â€” Adaptive Profile System with Constraints*

