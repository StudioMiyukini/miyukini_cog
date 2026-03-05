---
id: freelance-template
name: "Freelance/Solo Developer Template"
description: |
  For solo developers, freelancers with:
  - Multiple clients
  - Cost-conscious
  - No strict compliance
  - Flexibility

placeholders:
  CLIENT_NAME: "e.g., ClientA, ClientB"
  PREFERRED_LLM: "mistral | claude | copilot"
  BUDGET_CONSTRAINT: "low | medium | high"
  TOOL: "Cursor | VS Code | Vim"

---

# Freelance Profile Template

**Client**: :CLIENT_NAME:
**Profile Name**: client-:CLIENT_NAME:-dev
**Base LLM**: :PREFERRED_LLM:

```yaml
---
id: client-:CLIENT_NAME:-dev
name: "Client :CLIENT_NAME: - Dev Profile"
tool: :PREFERRED_LLM:_tool
llm:
  provider: :PREFERRED_LLM:
  model: mistral-nemo | claude-sonnet
  context_window: 128000

capabilities:
  parallel_agents: true
  terminal: true
  web_search: true
  caching: true
  logging: true  # You control it

constraints:
  - "confidential-data"  # Client code is confidential
  # optional:
  # - "tool-locked-:TOOL:"

budget:
  per_task_limit: :BUDGET_CONSTRAINT:
  per_month_limit: "$500"
  fallback_to_local: "if over budget"  # Switch to free Mistral local

workflows:
  p3_autocode: true
  p4_auto_test: true
  p5_human_review: required
  p6_auto_report: true

client_notes:
  name: ":CLIENT_NAME:"
  contact: "your-email@example.com"
  project_url: "https://..."
  billing: "monthly | per-task"
  rate: "$50/hr : estimate"
```

**Usage**:

```bash
mip_profile create-from-template freelance --name client-acme-dev

# Fill:
#   :CLIENT_NAME: → Acme
#   :PREFERRED_LLM: → mistral
#   :BUDGET_CONSTRAINT: → medium
#   :TOOL: → VS Code

# Activate:
#   mip_profile client-acme-dev

# Now you can:
#   1. Work on Acme project (profile auto applies confidential-data)
#   2. Cost-conscious (tracks per-task budget)
#   3. Multiple clients (create per-client profile, switch as needed)
```