---
id: mip.workflow.capability-negotiation
title: Capability Negotiation Protocol â€” AI s'annonce ses limites
---

# Capability Negotiation Protocol (CNP)

> **Principe** : Au lieu de MIP forÃ§ant des capacitÃ©s non-existantes, le LLM s'annonce ("Je suis Mistral, voici mes limites") et proposent des workarounds proactifs.

---

## Quand se dÃ©clenche CNP?

### Contexte 1 : DÃ©marrage du SETUP
```
User lance MIP / SETUP pour la 1re fois
â†’ Maria dÃ©tecte l'environnement (IDE, outils)
â†’ Propose 5 profils prÃ©-dÃ©finis
â†’ Utilisateur sÃ©lectionne (ou auto-detect)
â†’ CNP commence
```

### Contexte 2 : Changement de profil
```
/mip_profile mistral-nemo  [switch from copilot-free]
â†’ MIP reload capabilities
â†’ Display transition warnings + new abilities
â†’ Annonces de vÃ©cus
```

### Contexte 3 : TÃ¢che trÃ¨s difficile vs capacitÃ©s
```
User : "T4 refactor 30 crates"
MIP (Mode 3 / Copilot Free) : 
  "âŒ T4 impossible en Mode 3 (capabilities = 40%)
   ðŸ’¡ Suggestions :
      1. DÃ©couper T4 â†’ 6 T2 (faisable)
      2. Upgrade vers Mistral (Mode 2)
      3. CLI pair = vous code, je review
   Votre choix ?"
```

---

## Protocole CNP dÃ©taillÃ© (7 Ã©tapes)

### Ã‰tape 1 : Auto-announce des capacitÃ©s

**Format standard** (chaque LLM commence par) :

```markdown
ðŸ¤– **[LLM Self-Announcement]**

Je suis **[Nom Profil]** (Mode [N])

**CapacitÃ©s :**
  âœ… Code generation (native)
  âœ… Terminal execution (Sandbox)
  âœ… Parallel agents (via subagents)
  âœ… Fichier Ã©dition (multi_replace_string_in_file)
  âœ… TodoWrite (manage_todo_list)
  
**Limitations :**
  âŒ Web search (no MCP)
     â†’ Fallback: Offline docs + SearchAPI
  âš ï¸ Context window 128k
     â†’ Read < 5 files / task
  
**Fallback strategy :**
  â€¢ PDF search â†’ Local MIP Index JSON
  â€¢ Web query â†’ DuckDuckGo shell script
  â€¢ Real-time â†’ Log + retry

Je suis prÃªt pour T1-T3, T4 possible avec dÃ©coupage.
```

**Ce que cela Ã©vite** :
- LLM tente terminal â†’ fail silently
- LLM appelle MCP â†’ crashes
- LLM promet parallÃ©lisme impossible â†’ slow serialization
- LLM assume web access â†’ reads stale docs

Instead: **Proactif transparency**.

---

### Ã‰tape 2 : Utilisateur accepte ou contredit

```
User : "Je prÃ©fÃ¨re Copilot, pas d'autre option"

MIP :
  âœ… Profil acceptÃ© : github-copilot-free (Mode 3)
  
  Workflow adaptÃ© :
    P0 Framing        (Maria, text)
    P3 per-file       (Copilot 2 fichiers/iter)
    P4 vous run tests (Terminal manuel)
    P5 vous validez  (Human review)
    P6 Rapport texte (Copilot + Arianne memo)
    
  Budget :
    T2 : ~1h, gratuit âœ…
    T3 : ~4-5h, gratuit âœ…
    T4+ : Not recommended
    
  Ok? Let's go. [User hits Enter]
```

---

### Ã‰tape 3 : Assignation des rÃ´les par capacitÃ©

**Base** : 10 roles (Maria orchestration, Lise, FranÃ§ois, etc.)

**Constraint** : Assigner un role que si LLM peut l'exÃ©cuter.

Exemple Mode 3 (Copilot Free):
```yaml
Maria (Orchestration)     : âœ… Text-based P0, simple decisions
Francois (Backend spec)   : âœ… Code analysis, spec writing
Lise (Frontend)           : âœ… UI code, dioxus patterns
Victor (Security)         : âš ï¸ Review only (pas terminal => can't test)
George (Compliance)       : âš ï¸ Document-based only
Hugo (DevOps)             : âŒ Not possible (no terminal batch)
Jean (Efficiency)         : âœ… Token counting, estimation
Arianne (QA/Memory)       : âœ… Test-case writing, memo update

Unassignable : Hugo
Workaround    : "Hugo = Human DevOps person" or "Skip P4 auto-test"
```

---

### Ã‰tape 4 : Define fallback chains

Pour chaque capacitÃ© manquante, dÃ©fenir :
```
1. Primary method (prÃ©fÃ©rÃ©)
2. Fallback 1 (workable)
3. Fallback 2 (degraded but ok)
4. Manual workaround (humain fait)
5. Skip entirely (pas critique)
```

**Exemple : Web search (Mode 2, pas MCP)**

```
Primary   : Mistral a pas MCP â†’ skip web
Fallback1 : Use local markdown docs (Miyukini README)
Fallback2 : Shell DuckDuckGo query (curl)
           curl -s "https://duckduckgo.com/?q=dioxus+events" | grep ...
Fallback3 : Utilisateur tape la rÃ©ponse manuellement
Skip      : Not critical, doc peut Ãªtre stale
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

### Ã‰tape 5 : Offer mode upgrades

Si tÃ¢che trop dure pour mode actuel :

```
User : "T4 refactor, please"
MIP (Mode 3) :
  
  âš ï¸ Mode 3 (Copilot gratuit) limitÃ© pour T4
  EstimÃ© : 1 semaine de travail itÃ©ratif
  
  ðŸ’¡ Mode upgrades suggÃ©rÃ©s :
    1. Mistral Nemo (local) â†’ Mode 2
       Temps : 2h (auto, parallÃ¨le)
       CoÃ»t : $0 (amortissement HW)
       
    2. Claude Code (API) â†’ Mode 1
       Temps : 1h (parallel agents)
       CoÃ»t : ~$2
       
    3. Rester Mode 3, dÃ©couper en 6 T2
       Temps : 1-2 semaines (itÃ©ratif)
       CoÃ»t : $0
       
  Votre choix ?
  - "1" (Mistral)
  - "2" (Claude)
  - "3" (DÃ©coupage)
  - "Cancel"
```

---

### Ã‰tape 6 : Lock et announce

AprÃ¨s choix utilisateur :

```
âœ… ConfirmÃ© : Mode 2 (Mistral Nemo)

 Profil actif : mistral-nemo
 Context window : 128k
 Parallelism : Agents simultanÃ©s âœ…
 Terminal : Sandbox âœ…
 MCP : âŒ (SearchAPI fallback)
 
 Workflow :
   P0 Framing (Maria, fast)
   P3 Auto implementation (parallel)
   P4 Testing (direct terminal)
   P5 Human review
   P6 Archive
   
 Estimated :
   - DurÃ©e : 1-2h (CPU bound)
   - Tokens : ~120k
   - CoÃ»ts : $0 (local)
   
 Let's go âš¡
```

---

### Ã‰tape 7 : Runtime renegotiation

Si pendant exec, capacitÃ© needed :

```
During P3 :

Mistral:  "Je dois consulter Tauri API docs"
         â†’ Vers MCP? Non (pas dispo)
         â†’ Fallback: Lire local README
         â†’ Pas trouvÃ© sur disque
         â†’ Fallback2: Web search via SearchAPI?
         â†’ (Too slow for this)
         
         ðŸ’¡ Renegotiate : "Jean, peut-on skipper
            cette recherche et assumer pattern?"
            
User/Jean : "Oui, use default Tauri pattern"

Mistral:  "Ok, procÃ©dant avec pattern"
```

---

## Formats de rÃ©ponse

### Format A : Acceptation directe

```markdown
âœ… **CapacitÃ©s acceptÃ©es**

Je suis Mode 2 (Mistral), prÃªt pour T2-T3.
CommenÃ§ons P0.
```

### Format B : Acceptation avec warnings

```markdown
âš ï¸ **Profil acceptÃ© avec limitations**

Mode 3 (Copilot gratuit)
- T2 idÃ©al (1-2h)
- T3 possible (3-5h itÃ©ratif)
- T4+ not recommended

Vous Ãªtes ok pour T2-T3 max ?
```

### Format C : Refus et contre-proposition

```markdown
âŒ **T4 impossible en Mode 3**

CapacitÃ©s insuffisantes (40% / 100%)

ðŸ’¡ Recommandation :
  Option 1: Upgrade Mistral (Mode 2) â†’ 2h
  Option 2: DÃ©couper â†’ 6 T2 (1 semaine)
  
Votre choix ?
[1/2/cancel]
```

---

## Integration avec Skills IA

Chaque SKILL.md dÃ©bute par :

```markdown
## Capacity negotiation

**Ce skill nÃ©cessite :**
- [ ] Terminal access (build, test)
- [ ] Multi-file edits (3+ files)
- [ ] Parallel agents (optionnel)
- [ ] Web search (optionnel)

**Modes supportÃ©s :**
- Mode 1 (Claude Code) : âœ… Plein support
- Mode 2 (Mistral) : âœ… Plein support
- Mode 3 (Copilot) : âš ï¸ Sans tests auto
- Mode 4 (GPT-mini) : âš ï¸ Code review only
- Mode 5 (Offline) : âš ï¸ Sans web search

**Fallbacks :**
- No web search? â†’ Use offline Markdown docs
- No parallel? â†’ Sequential approach
- No terminal? â†’ Manual user execution
```

---

## MÃ©triques de succÃ¨s

| MÃ©trique | Baseline | Target |
|----------|----------|--------|
| **User clarity** | 30% understand limitations | 90%+ |
| **Agent transparency** | 60% announce once | 95%+ proactive |
| **Fallback success** | 40% of failures recover | 80%+ auto-recovery |
| **Mode adoption** | All Mode 1 | 40% M1, 50% M2, 10% M3-5 |
| **Frustration rate** | 25% "Why doesn't this work?" | <5% |

---

## RÃ©fÃ©rences

- [ADAPTIVE-MODES.md](./ADAPTIVE-MODES.md) â€” 5 modes dÃ©tail
- [Capabilities Matrix](..//README.md) â€” Par outil
- [Profiles INDEX](..//README.md) â€” Basculer profils
- [MCP Fallback](..//README.md) â€” Miou bridge

