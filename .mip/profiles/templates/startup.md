---
id: startup-template
name: "Startup/Rapid Development"
description: |
  For startups, rapid prototyping:
  - Maximum speed
  - Minimal overhead
  - Cost-optimal
  - Parallel agents

---

# Startup Profile Template

```yaml
---
id: startup-mvp
name: "Startup - MVP Mode"
tool: claude-code
llm:
  provider: anthropic
  model: claude-sonnet
  context_window: 200000

capabilities:
  parallel_agents: true     # FULL parallelism
  terminal: true
  web_search: true
  multi_file_edit: true
  todo_write: true
  background_tasks: true

constraints: []             # No constraints, speed first

budget:
  monthly: "$2000"
  alerts: "at 80%"
  fallback: "swap to mistral-nemo"

workflows:
  p0_framing: "minimal"
  p3_autocode: "aggressive parallel"
  p4_testing: "auto"
  p5_human_review: "light"
  p6_archive: "quick"

optimization:
  agent_parallelism: 4      # 4 agents simultaneous
  context_reuse: true
  model_cache: true
  cost_mode: "speed>cost"
```