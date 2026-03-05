---
id: confidential-data
name: "Confidential Data Constraint"
category: confidentiality
description: |
  Prevents data leakage for sensitive client/company data:
  - No logging of operations
  - No caching of intermediate results
  - No external API calls (local-only processing)
  - No model output storage (memory files)
  - Manual file edits only (no auto-formatting)
  
applies_to: all profiles
risk_level: high
performance_impact: "+30% (manual mode, no parallel)"
cost_impact: "$0 (no external APIs)"

---

# Confidential Data Constraint

## What This Constraint Does

Treats all data as **client confidential** / **company secret**:
- Zero data leaves the machine
- Zero intermediate caching
- Zero logging of sensitive content
- Manual/explicit operations only
- No automation that involves data storage

```yaml
confidential_data:
  enabled: true
  
  # 1. No Logging
  logging:
    disabled: true           # No operation logs
    audit_trail: disabled    # No file tracking
    history: disabled        # No command history
    
  # 2. No Caching
  caching:
    model_cache: disabled
    response_cache: disabled
    intermediate_cache: disabled
    memory_files: disabled
    todo_results: cleared_after_use
    
  # 3. No External APIs
  network:
    external_api: blocked
    web_search: blocked
    model_upload: blocked
    cloud_storage: blocked
    
  # 4. Manual Mode
  automation:
    auto_formatting: disabled
    auto_testing: disabled
    auto_commit: disabled
    parallel_agents: disabled  # Only sequential
    
  # 5. Data Deletion
  cleanup:
    after_task: manual_confirmation
    temp_files: shred (wipe free space)
    memory_clear: explicit
    
  # 6. Monitoring
  monitoring:
    network_sniff: enabled   # Monitor what leaves machine
    clipboard_access: blocked
    screenshot_protection: enabled
    keystroke_logging: blocked
```

## Profile Effects

When `:confidential-data` is applied:

| Aspect | Normal Mode | Confidential Mode |
|--------|------------|------------------|
| **Logging** | ✅ Auto logging | ❌ No logs |
| **Caching** | ✅ Cache responses | ❌ No caching |
| **API Calls** | ✅ Stripe, Analytics | ❌ Blocked |
| **Parallelism** | ✅ 4 agents | ❌ 1 agent (sequential) |
| **Output Storage** | ✅ Memory files | ❌ Cleared after use |
| **Testing** | ✅ Auto test | ❌ Manual review |
| **Performance** | Fast | -30% (manual mode) |
| **Cost** | Variable | $0 (no APIs) |

## Mode Restrictions

```
✅ Can use:
  - Terminal (local commands only)
  - File editing (manual)
  - Code review (no storage)
  - Offline computation

❌ Cannot use:
  - Web search
  - External APIs (Stripe, Analytics, etc.)
  - Cloud storage
  - Model caching
  - Auto-testing
  - Parallel agents
  - Logging
```

## Compatibility Matrix

```
✅ Compatible with:
  - legal-compliance (legal + confidential = fort Knox)
  - pii-strict (overlaps, reinforces)
  - offline-only (confidential + offline = maximum isolation)
  - tool-locked-* (control access to tool)

⚠️  Conflicts with:
  - None strictly, but:
    - confidential + parallel_agents = impossible (will disable parallelism)
    - confidential + web_search = impossible (will block search)

❌ Incompatible with:
  - None (confidential data overrides lower-security constraints)
```

## Implementation Details

### Network Monitoring

When confidential-data is active, all network attempts are logged locally (without logging the data):

```
❌ Web request blocked: https://api.stripe.com/v1/charges
   Reason: external_api blocked by confidential-data
   
❌ Model upload blocked: /cache/model-response-123.json
   Reason: caching disabled by confidential-data
   
✅ Local file edit allowed: src/payment.rs
   Reason: local file (no network, no cache)
```

### Data Cleanup

```bash
# After each task, user must confirm cleanup:

Maria: "Task complete. Confidential data cleanup:
        - Delete temp files? (required)"
User: "Yes"

Maria: "Clearing:
        - /tmp/task-cache/
        - /tmp/model-output/
        - Browser cache
        - Clipboard
        
        ✅ Clean"
```

### File Operations

```
Manual (✅ allowed):
  - User edits file directly in editor
  - User selects what to change
  - User confirms before save

Automatic (❌ blocked):
  - Auto-format on save
  - Auto-test after change
  - Auto-commit to git
  - Auto-backup to cloud
```

## Use Cases

### Case 1: Client Project (Agency)

```yaml
base_profile: mistral-nemo
constraints:
  - confidential-data
  - tool-locked-vscode
```

**scenario**: 
- Client code is confidential (NDA)
- Cannot cache their logic
- Cannot test automatically (might expose secrets)
- Manual verification before each change

**workflow**:
```
1. Open client code
2. User requests change
3. Agent proposes code
4. User reviews code
5. User manually tests
6. No logs, no cache
7. After task: confirm cleanup
```

### Case 2: Startup (Pre-funding)

```yaml
base_profile: claude-sonnet
constraints:
  - confidential-data
  - offline-only  # Extra paranoid
```

**scenario**:
- Code is secret (competitors watching)
- No cloud backups (NSA paranoia)
- Must stay offline

**workflow**:
```
1. All processing local only
2. No upload to Claude servers
3. No model caching
4. Manual testing only
5. Daily local backup (user controlled)
```

### Case 3: Financial Services (Compliance)

```yaml
base_profile: mistral-nemo
constraints:
  - confidential-data
  - legal-compliance
  - regional-us
```

**scenario**:
- Confidential: Customer account data
- Legal: SOX audit requirements
- Regional: US financial data

**workflow**:
```
1. All data confidential (no caching)
2. All operations logged (for audit)
3. Immutable logs (can't hide evidence)
4. US region only
```

## Activation

### Via Profile Creation

```bash
mip_profile create secret-project \
  --base mistral-nemo \
  --constraints confidential-data
  
# Maria: "Applying confidential-data constraint.
#         Your settings:
#         ✅ Logging: OFF
#         ✅ Caching: OFF
#         ✅ External APIs: BLOCKED
#         ✅ Manual mode: ON
#         
#         Ready for confidential work."
```

### Via CLI

```bash
# OR just add to existing profile
mip_profile apply-constraint myprofile confidential-data

# Then revert if needed
mip_profile remove-constraint myprofile confidential-data
```

## Workflow: Step-by-Step

### Example Task: Modify payment logic (confidential)

```
User: "Change payment retry logic"

[BEFORE confidential-data]
1. Agent: "I'll modify payment.rs and run tests"
   ✅ Changes made
   ✅ Auto-tests run
   ✅ Results cached
   ✅ Changes logged
   [Leak risk: tests might expose amounts, keys]

[AFTER confidential-data]
1. Agent: "I'll propose changes, you review"
2. User: "Show me the changes"
3. Agent: Shows diff (no cache created)
4. User: "Looks good, apply"
5. Agent: Changes applied (logged locally only)
6. User: "Run tests locally?"
7. User: Runs `cargo test` manually (no cache)
8. Agent: "Tests passed?"
9. User: "Yes, done"
10. Maria: "Cleanup? (manual)"
    - Delete temp files
    - Clear clipboard
    - Clear browser cache
    - ✅ Done
```

## Configuration Examples

### Example 1: Minimal Confidential (Allow Some Caching)

```yaml
confidential_data:
  # Disable logging and external data
  logging: disabled
  external_api: blocked
  
  # But allow some caching (encrypted)
  caching:
    model_cache: encrypted_only
    response_cache: disabled
    intermediate_cache: disabled
```

### Example 2: Maximum Confidential (Paranoid Mode)

```yaml
confidential_data:
  # Everything off
  logging: disabled
  caching: disabled
  network: blocked
  monitoring: enabled
  clipboard_access: blocked
  screenshot_protection: enabled
  
  # Force manual cleanup
  cleanup:
    after_task: mandatory_user_confirmation
    temp_files: shred_with_dod_wipe
    memory_clear: explicit_user_triggered
```

## Performance Impact

### Normal Task (No Confidential)

```
Time: 5 minutes
- 2 min: Agent thinks (parallel)
- 1 min: Auto tests
- 2 min: Cache/upload operations
```

### Same Task (With Confidential)

```
Time: 15 minutes
- 5 min: Agent thinks (sequential, no cache)
- 5 min: User reviews + manual testing
- 5 min: Manual cleanup
= 3x slower (but fully controlled)
```

### Cost Impact

| Mode | External APIs | Storage | Total |
|------|---------------|---------|-------|
| Normal | Web, Analytics, Storage | Cloud caching | $0.50/task |
| Confidential | None (blocked) | Local temp only | $0/task |

## Troubleshooting

### Issue: "Agent keeps asking for permission"

→ This is expected! Confidential mode is conservative.

Solution: Trust the agent more by using `confidential-lite`:

```yaml
confidential_data:
  # Partial mode: cache + test, but no logging
  caching: encrypted_only
  testing: auto_local_only
  logging: disabled
```

### Issue: "Can't copy data to clipboard"

→ Intentional (clipboard access blocked).

Solution: Use manual file transfer or encrypted USB.

### Issue: "Task takes 3x longer"

→ Expected due to manual review + testing.

Solution: 
- Smaller tasks (T1 instead of T4)
- Pre-review with business team
- Use mistral-nemo (cheaper API fallback for review phase)

## Advanced: Encrypted Caching

For cases where you want some caching but encrypted:

```yaml
confidential_data:
  caching:
    mode: encrypted_only
    algorithm: aes256
    key_management: local_keyring
    auto_expire: 1h
```

Then:
- ✅ Agent can cache results (encrypted)
- ❌ But cache expires in 1 hour
- ✅ Key stored in OS keyring
- ❌ Key lost on logout

## References

- [MANAGEMENT.md](./MANAGEMENT.md) — How to apply constraints
- [CONSTRAINTS.md](./CONSTRAINTS.md) — All constraint types
- [Offline-only Constraint](./offline-only.md) — Combine for maximum isolation
- [PII-strict Constraint](./pii-strict.md) — For PII-specific data
