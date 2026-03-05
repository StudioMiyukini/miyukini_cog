---
id: offline-only
name: "Offline-Only Constraint"
category: tool
description: |
  Complete network isolation for maximum security/privacy:
  - No internet connectivity at all
  - Local inference only (GGUF/Ollama)
  - Local file operations only
  - Air-gapped machine recommended
  - Extreme privacy (no data leaves machine ever)
  
applies_to: all profiles
risk_level: critical
performance_impact: "-50% to -80% (local inference slow)"
cost_impact: "$0 (completely free, local hardware only)"

---

# Offline-Only Constraint

## What This Constraint Does

Enforces **complete network isolation**:
- No connection to external LLM APIs
- No web search
- No cloud storage
- No external APIs
- No model updates from internet
- Processing only on local hardware

```yaml
offline_only:
  enabled: true
  
  # 1. Network Isolation
  network:
    internet: blocked
    localhost_only: true
    vpn: blocked
    proxy: blocked
    
  # 2. Inference
  inference:
    provider: local_only
    backend: ollama | llama-cpp | ggml
    models: local_downloaded_only
    model_loading: local_disk
    
  # 3. File Operations
  files:
    filesystem: local_disk_only
    cloud_sync: disabled
    git_sync: disabled
    clipboard: local_memory_only
    
  # 4. Updates
  updates:
    model_updates: manual_download_at_home
    app_updates: manual_download_at_home
    skill_updates: manual_download_at_home
    
  # 5. Environment
  environment:
    air_gapped: recommended
    network_interface: disabled
    wireless: disabled
    usb_network: blocked
    
  # 6. Verification
  verification:
    no_internet_check: mandatory
    network_interface_test: before_start
    connection_attempt_detection: alert_user
```

## Profile Effects

| Aspect | Normal Mode | Offline Mode |
|--------|------------|--------------|
| **LLM** | API (Mistral, Claude) | Local (Ollama, Llama) |
| **Speed** | 1x (network latency low) | 0.1-0.3x (local CPU/GPU) |
| **Quality** | Best models (latest) | Available local models |
| **Cost** | $0.10-1.00/task | $0 (hardware only) |
| **Privacy** | Cloud provider sees | Zero external visibility |
| **Requirements** | Internet | Local GPU (recommended) |
| **Update Lag** | Real-time | Manual (update@ home only) |
| **Fallback** | Another API | Fail (no fallback) |

## Hardware Requirements

### Minimum (CPU only)
- 8-core CPU, 16GB RAM
- Model: Llama 2 7B (slow)
- Speed: 10-20 tokens/sec
- Cost: ~$0 after hardware

### Recommended (GPU)
- RTX 3080 or A100 GPU, 24-48GB VRAM
- Models: Mistral 7B, Llama 2 13B (fast)
- Speed: 100-300 tokens/sec
- Cost: ~$300-500 GPU, ~$0 per task

### Ideal (Dedicated)
- RTX 4090, 128GB system RAM, 48GB GPU VRAM
- Models: Multiple 13B-70B models loaded simultaneously
- Speed: 200-500 tokens/sec
- Cost: ~$2000 hardware, ~$0 per task

## Compatibility Matrix

```
âœ… Compatible with:
  - confidential-data (offline + confidential = maximum security)
  - legal-compliance (local audit trail only)
  - tool-locked-* (locked IDE + offline)
  - All offline profiles (ollama, llama, lm-studio)

âš ï¸  Partial compatibility:
  - Mode 4 Code Reviewer (works, but no web search)
  - Skills (only local-capable skills)

âŒ Incompatible with:
  - Web search (explicitly blocked)
  - External API operations
  - Cloud storage
  - Online model updates
  - Streaming from internet
```

## Mode Support

```
âœ… Mode 5 (Offline Llama) â€” FULL support
   - Built for offline
   - Local inference
   - Complete constraint match

âš ï¸  Mode 3 (Copilot) â€” LIMITED
   - Copilot requires internet
   - Profile will force local fallback
   - Performance degraded

âš ï¸  Mode 2 (Mistral) â€” LIMITED
   - Mistral API requires internet
   - Will use local Mistral instead
   - Cost stays $0

âŒ Mode 1 (Claude) â€” NOT SUPPORTED
   - Claude requires internet
   - Constraint blocks all internet
   - Will fail explicitly
```

## Implementation Details

### Network Verification (Startup)

```
MIP starts with offline-only constraint:

1. Check: "Is internet available?"
   â†’ Yes: âŒ FAIL ("offline-only requires no internet")
   â†’ No: âœ… PASS ("Network isolation verified")
   
2. Check: "Are network interfaces disabled?"
   â†’ WiFi: âŒ ALERT ("WiFi enabled, should disable")
   â†’ Ethernet: âŒ ALERT ("Ethernet connected, should unplug")
   â†’ None: âœ… PASS ("All networks disabled")
   
3. Check: "Is Ollama running locally?"
   â†’ Yes: âœ… PASS ("Local LLM available")
   â†’ No: âŒ FAIL ("Cannot start without local LLM")
```

### Model Loading

```
Offline models available:

Option 1: Ollama (recommended)
$ ollama pull mistral
$ ollama pull llama2
$ ollama serve
â†’ Runs on localhost:11434

Option 2: LM Studio (GUI)
â†’ Download model from HuggingFace
â†’ Load in LM Studio
â†’ Runs on localhost:1234

Option 3: llama.cpp (CLI)
$ ./main -m model.gguf --listen
â†’ Runs on localhost:8000
```

### Offline Operation

```
Task workflow (offline):

1. User: "Refactor payment module"
2. Agent: "I'll use local Mistral 7B"
3. Processing: All local (no network)
4. Result: Generated locally
5. Output: Saved to disk only
6. âœ… Complete (zero internet used)
```

## Use Cases

### Case 1: Government/Military (Air-Gapped)

```yaml
base_profile: ollama-mistral
constraints:
  - offline-only
  - confidential-data
  - legal-compliance
```

**scenario**:
- Top secret project (air-gapped network)
- Cannot trust cloud providers
- Must keep data 100% local
- Legal requirement: no external storage

**setup**:
```
1. Offline machine (no WiFi, ethernet unplugged)
2. Ollama with Mistral 7B (16GB GPU)
3. MIP running locally
4. All work offline
```

### Case 2: Financial Services (Vault)

```yaml
base_profile: lm-studio
constraints:
  - offline-only
  - pii-strict
  - legal-compliance
```

**scenario**:
- High-frequency trading algorithms
- Customer wallet addresses (extremely sensitive)
- Cannot risk data exposure
- Regulatory requirement: air-gapped

**setup**:
```
1. Dedicated offline server
2. LM Studio with Mistral 13B (RTX 4090)
3. MIP in vault network segment
4. All training/inference offline
```

### Case 3: Startup (Cheap Privacy)

```yaml
base_profile: ollama-llama2
constraints:
  - offline-only
  - confidential-data
```

**scenario**:
- Can't afford Claude API ($1000s/month)
- Code is company secret (competitors)
- Laptop with RTX 3080 GPU
- Want free + private solution

**setup**:
```
1. MacBook Pro with GPU
2. Ollama with Llama 2 7B
3. Works even on airplane (no internet)
4. $0 cost (only hardware)
```

## Activation

### Requirement Check

```bash
mip_profile create secret-vault \
  --base ollama-mistral \
  --constraints offline-only
  
# Maria: "Offline-only constraint detected.
#         Checking prerequisites:
#         
#         â“ Internet connectivity?"
# System: "Checking..."
#   âœ… No internet detected
#   
# â“ Ollama running?"
# System: "Checking localhost:11434..."
#   âœ… Ollama found (Mistral 7B loaded)
#   
# â“ Network interfaces disabled?"
# System: "Checking..."
#   âš ï¸  WiFi enabled
#   âš ï¸  Ethernet connected
#   
#   Recommendation: Unplug ethernet + disable WiFi
#   Proceed anyway? (y/n)"
```

### Setup Steps

```bash
# Step 1: Prepare machine
sudo systemctl stop NetworkManager  # Disable networking
rfkill block wifi                   # Disable WiFi

# Step 2: Start local LLM
ollama serve

# Step 3: Create offline profile
mip_profile create my-vault \
  --base ollama-mistral \
  --constraints offline-only

# Step 4: Verify setup
mip_profile validate my-vault

# Step 5: Activate
mip_profile my-vault
```

## Operation: Disconnected Workflow

### Phase 0: Download

```
At home (with internet):
1. Download models from HuggingFace
   $ huggingface-cli download mistralai/Mistral-7B-v0.1
2. Convert to GGUF
   $ python3 convert.py model.pth model.gguf
3. Download MIP + skills
   $ git clone https://github.com/... mip.git
4. Copy to offline machine (USB drive)
```

### Phase 1-5: Work (Offline)

```
At vault (no internet):

Setup:
- Load models: ollama pull /local/model.gguf
- Start MIP: mip_profile vault-secured
- Maria: "âœ… Offline mode active"

Work:
- User: "Refactor payment.rs"
- Agent: Uses local Mistral
- Output: Saved locally
- No network attempts (blocked)

Shutdown:
- Save results: git commit (local)
- Export to USB: cp -r /work/* /usb/
- Power down
```

### Phase 6: Archive (Offline)

```
Still offline:

1. Verify work locally
   $ cargo test
   $ cargo fmt --check
   
2. Export results
   $ tar -czf work.tar.gz .
   
3. Sign archive
   $ gpg --sign work.tar.gz
   
4. Copy to USB (offline transfer)
```

## Performance Tuning

### Local Inference Optimization

```yaml
# For fast CPU-only inference
inference:
  backend: llama.cpp
  n_threads: 8        # All CPU cores
  n_batch: 512        # Large batches
  model: mistral-7b-quantized  # Smaller = faster
  quantization: q4    # 4-bit quantization
  â†’ Speed: 10 tokens/sec

# For fast GPU inference  
inference:
  backend: ollama
  gpu_layers: all     # All layers on GPU
  model: mistral-7b   # Higher quality
  â†’ Speed: 200 tokens/sec

# Balance quality/speed
inference:
  backend: llama.cpp
  n_gpu_layers: 35    # Half GPU, half CPU
  model: mistral-13b-quantized
  quantization: q5    # Higher quality
  â†’ Speed: 50 tokens/sec
```

### Cost-Performance Trade-off

| Hardware | Model | Speed | Cost | Viable? |
|----------|-------|-------|------|---------|
| CPU (8-core) | Llama 7B | 5 tok/s | $0 | Small tasks |
| CPU (16-core) | Mistral 7B | 8 tok/s | $0 | Medium tasks |
| RTX 3080 | Mistral 7B | 100 tok/s | $300 | Production |
| RTX 4090 | Mistral 13B | 200 tok/s | $2000 | Large-scale |
| A100 GPU | Llama 70B | 500 tok/s | $5000 | Enterprise |

## Model Selection Guide

### For Coding (Task class T2-T3)

```
âœ… Best (offline):
  - Mistral 7B (fastest, decent quality)
  - Llama 2 13B (if GPU available)

âš ï¸  Possible (slower):
  - Llama 2 7B (works, somewhat literal)

âŒ Not recommended:
  - TinyLlama 1B (too small for code)
```

### For Analysis & Planning (Task class T1)

```
âœ… Best:
  - Mistral 7B (sufficient)
  - Llama 2 7B (acceptable)
  
âš ï¸  Possible:
  - TinyLlama 1B (simple tasks only)
```

### For Complex Code (Task class T4-T5)

```
âš ï¸  Possible (need GPU):
  - Mistral 13B (if RTX 3080+)
  - Llama 2 13B (if RTX 4090)
  
âŒ Not viable (too complex):
  - Smaller models (7B or less)
  - CPU-only (too slow)
```

## Troubleshooting

### Issue: "Ollama not running"

```
Error: "Cannot reach localhost:11434"

Solution 1: Start Ollama
  $ ollama serve
  
Solution 2: Check if crashed
  $ ps aux | grep ollama
  
Solution 3: Restart completely
  $ ollama list  # Should show models
  $ ollama pull mistral  # Download if missing
  $ ollama serve
```

### Issue: "Task still trying internet"

```
Error: "Attempting connection to api.mistral.ai"

Reason: Model provider config not offline

Solution:
  $ mip_profile edit myprofile
  
  Ensure:
    llm.provider: ollama
    llm.endpoint: http://localhost:11434
    
  NOT:
    llm.provider: mistral-api
    llm.endpoint: https://api.mistral.ai
```

### Issue: "Out of memory"

```
Error: Failed to load model (VRAM full)

Solution 1: Smaller model
  ollama pull llama2:7b instead of llama2:13b
  
Solution 2: Quantization
  Use q4 instead of q5 (4x less memory)
  
Solution 3: More VRAM
  Add GPU or swap to cloud fallback (loses offline status)
```

### Issue: "Model quality too low"

```
Symptom: "Output is mediocre/literal"

Reason: Local models less capable than large API models

Solutions:
  1. Use Mistral 13B (if GPU available)
  2. Use higher quantization (q5, q6)
  3. Adjust temperature for better creativity
  4. Accept performance trade-off (offline priority)
```

## Advanced: USB-Transfer Workflow

For air-gapped machines with no network at all:

```
Home (online):
1. Download models
2. Download skills updates
3. Copy to encrypted USB

Offline machine:
1. Mount USB
2. Run updater: `mip_offline_update /media/usb`
3. Models + skills now installed
4. Work offline
5. Export results to USB

Back at home:
6. Review results
7. Check into git (online)
8. Prepare next USB update
```

This enables:
- âœ… Completely air-gapped development
- âŒ But manual update process
- âœ… Maximum security
- âŒ Weekly update lag

## References

- [MANAGEMENT.md](..//..//README.md) â€” Apply this constraint
- [CONSTRAINTS.md](..//..//README.md) â€” Other constraints
- [Mode 5 Profile](..//..//README.md) â€” Designed for offline
- [Local Inference Guide](..//..//README.md) â€” Setup LLM

