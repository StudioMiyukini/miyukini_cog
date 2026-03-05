---
id: mip.workflow.capability-negotiation
title: Capability Negotiation Protocol — AI s'annonce ses limites
---

# Capability Negotiation Protocol (CNP)

> **Principe** : Au lieu de MIP forçant des capacités non-existantes, le LLM s'annonce ("Je suis Mistral, voici mes limites") et proposent des workarounds proactifs.

---

## Quand se déclenche CNP?

### Contexte 1 : Démarrage du SETUP
```
User lance MIP / SETUP pour la 1re fois
→ Maria détecte l'environnement (IDE, outils)
→ Propose 5 profils pré-définis
→ Utilisateur sélectionne (ou auto-detect)
→ CNP commence
```

### Contexte 2 : Changement de profil
```
/mip_profile mistral-nemo  [switch from copilot-free]
→ MIP reload capabilities
→ Display transition warnings + new abilities
→ Annonces de vécus
```

### Contexte 3 : Tâche très difficile vs capacités
```
User : "T4 refactor 30 crates"
MIP (Mode 3 / Copilot Free) : 
  "❌ T4 impossible en Mode 3 (capabilities = 40%)
   💡 Suggestions :
      1. Découper T4 → 6 T2 (faisable)
      2. Upgrade vers Mistral (Mode 2)
      3. CLI pair = vous code, je review
   Votre choix ?"
```

---

## Protocole CNP détaillé (7 étapes)

### Étape 1 : Auto-announce des capacités

**Format standard** (chaque LLM commence par) :

```markdown
🤖 **[LLM Self-Announcement]**

Je suis **[Nom Profil]** (Mode [N])

**Capacités :**
  ✅ Code generation (native)
  ✅ Terminal execution (Sandbox)
  ✅ Parallel agents (via subagents)
  ✅ Fichier édition (multi_replace_string_in_file)
  ✅ TodoWrite (manage_todo_list)
  
**Limitations :**
  ❌ Web search (no MCP)
     → Fallback: Offline docs + SearchAPI
  ⚠️ Context window 128k
     → Read < 5 files / task
  
**Fallback strategy :**
  • PDF search → Local MIP Index JSON
  • Web query → DuckDuckGo shell script
  • Real-time → Log + retry

Je suis prêt pour T1-T3, T4 possible avec découpage.
```

**Ce que cela évite** :
- LLM tente terminal → fail silently
- LLM appelle MCP → crashes
- LLM promet parallélisme impossible → slow serialization
- LLM assume web access → reads stale docs

Instead: **Proactif transparency**.

---

### Étape 2 : Utilisateur accepte ou contredit

```
User : "Je préfère Copilot, pas d'autre option"

MIP :
  ✅ Profil accepté : github-copilot-free (Mode 3)
  
  Workflow adapté :
    P0 Framing        (Maria, text)
    P3 per-file       (Copilot 2 fichiers/iter)
    P4 vous run tests (Terminal manuel)
    P5 vous validez  (Human review)
    P6 Rapport texte (Copilot + Arianne memo)
    
  Budget :
    T2 : ~1h, gratuit ✅
    T3 : ~4-5h, gratuit ✅
    T4+ : Not recommended
    
  Ok? Let's go. [User hits Enter]
```

---

### Étape 3 : Assignation des rôles par capacité

**Base** : 10 roles (Maria orchestration, Lise, François, etc.)

**Constraint** : Assigner un role que si LLM peut l'exécuter.

Exemple Mode 3 (Copilot Free):
```yaml
Maria (Orchestration)     : ✅ Text-based P0, simple decisions
Francois (Backend spec)   : ✅ Code analysis, spec writing
Lise (Frontend)           : ✅ UI code, dioxus patterns
Victor (Security)         : ⚠️ Review only (pas terminal => can't test)
George (Compliance)       : ⚠️ Document-based only
Hugo (DevOps)             : ❌ Not possible (no terminal batch)
Jean (Efficiency)         : ✅ Token counting, estimation
Arianne (QA/Memory)       : ✅ Test-case writing, memo update

Unassignable : Hugo
Workaround    : "Hugo = Human DevOps person" or "Skip P4 auto-test"
```

---

### Étape 4 : Define fallback chains

Pour chaque capacité manquante, défenir :
```
1. Primary method (préféré)
2. Fallback 1 (workable)
3. Fallback 2 (degraded but ok)
4. Manual workaround (humain fait)
5. Skip entirely (pas critique)
```

**Exemple : Web search (Mode 2, pas MCP)**

```
Primary   : Mistral a pas MCP → skip web
Fallback1 : Use local markdown docs (Miyukini README)
Fallback2 : Shell DuckDuckGo query (curl)
           curl -s "https://duckduckgo.com/?q=dioxus+events" | grep ...
Fallback3 : Utilisateur tape la réponse manuellement
Skip      : Not critical, doc peut être stale
```

**Exemple : Terminal execution (Mode 3, pas terminal)**

```
Primary   : Use run_in_terminal (not available)
Fallback1 : Utilisateur execute `cargo build`, paste output
Fallback2 : Copilot reads output, suggests next command
Fallback3 : Code review manual (pas de tests auto)
Skip      : Tests must be manual
```

---

### Étape 5 : Offer mode upgrades

Si tâche trop dure pour mode actuel :

```
User : "T4 refactor, please"
MIP (Mode 3) :
  
  ⚠️ Mode 3 (Copilot gratuit) limité pour T4
  Estimé : 1 semaine de travail itératif
  
  💡 Mode upgrades suggérés :
    1. Mistral Nemo (local) → Mode 2
       Temps : 2h (auto, parallèle)
       Coût : $0 (amortissement HW)
       
    2. Claude Code (API) → Mode 1
       Temps : 1h (parallel agents)
       Coût : ~$2
       
    3. Rester Mode 3, découper en 6 T2
       Temps : 1-2 semaines (itératif)
       Coût : $0
       
  Votre choix ?
  - "1" (Mistral)
  - "2" (Claude)
  - "3" (Découpage)
  - "Cancel"
```

---

### Étape 6 : Lock et announce

Après choix utilisateur :

```
✅ Confirmé : Mode 2 (Mistral Nemo)

 Profil actif : mistral-nemo
 Context window : 128k
 Parallelism : Agents simultanés ✅
 Terminal : Sandbox ✅
 MCP : ❌ (SearchAPI fallback)
 
 Workflow :
   P0 Framing (Maria, fast)
   P3 Auto implementation (parallel)
   P4 Testing (direct terminal)
   P5 Human review
   P6 Archive
   
 Estimated :
   - Durée : 1-2h (CPU bound)
   - Tokens : ~120k
   - Coûts : $0 (local)
   
 Let's go ⚡
```

---

### Étape 7 : Runtime renegotiation

Si pendant exec, capacité needed :

```
During P3 :

Mistral:  "Je dois consulter Tauri API docs"
         → Vers MCP? Non (pas dispo)
         → Fallback: Lire local README
         → Pas trouvé sur disque
         → Fallback2: Web search via SearchAPI?
         → (Too slow for this)
         
         💡 Renegotiate : "Jean, peut-on skipper
            cette recherche et assumer pattern?"
            
User/Jean : "Oui, use default Tauri pattern"

Mistral:  "Ok, procédant avec pattern"
```

---

## Formats de réponse

### Format A : Acceptation directe

```markdown
✅ **Capacités acceptées**

Je suis Mode 2 (Mistral), prêt pour T2-T3.
Commençons P0.
```

### Format B : Acceptation avec warnings

```markdown
⚠️ **Profil accepté avec limitations**

Mode 3 (Copilot gratuit)
- T2 idéal (1-2h)
- T3 possible (3-5h itératif)
- T4+ not recommended

Vous êtes ok pour T2-T3 max ?
```

### Format C : Refus et contre-proposition

```markdown
❌ **T4 impossible en Mode 3**

Capacités insuffisantes (40% / 100%)

💡 Recommandation :
  Option 1: Upgrade Mistral (Mode 2) → 2h
  Option 2: Découper → 6 T2 (1 semaine)
  
Votre choix ?
[1/2/cancel]
```

---

## Integration avec Skills IA

Chaque SKILL.md débute par :

```markdown
## Capacity negotiation

**Ce skill nécessite :**
- [ ] Terminal access (build, test)
- [ ] Multi-file edits (3+ files)
- [ ] Parallel agents (optionnel)
- [ ] Web search (optionnel)

**Modes supportés :**
- Mode 1 (Claude Code) : ✅ Plein support
- Mode 2 (Mistral) : ✅ Plein support
- Mode 3 (Copilot) : ⚠️ Sans tests auto
- Mode 4 (GPT-mini) : ⚠️ Code review only
- Mode 5 (Offline) : ⚠️ Sans web search

**Fallbacks :**
- No web search? → Use offline Markdown docs
- No parallel? → Sequential approach
- No terminal? → Manual user execution
```

---

## Métriques de succès

| Métrique | Baseline | Target |
|----------|----------|--------|
| **User clarity** | 30% understand limitations | 90%+ |
| **Agent transparency** | 60% announce once | 95%+ proactive |
| **Fallback success** | 40% of failures recover | 80%+ auto-recovery |
| **Mode adoption** | All Mode 1 | 40% M1, 50% M2, 10% M3-5 |
| **Frustration rate** | 25% "Why doesn't this work?" | <5% |

---

## Références

- [ADAPTIVE-MODES.md](./ADAPTIVE-MODES.md) — 5 modes détail
- [Capabilities Matrix](./capabilities-matrix.md) — Par outil
- [Profiles INDEX](./INDEX.md) — Basculer profils
- [MCP Fallback](../modules/search-fallback.md) — Miou bridge
