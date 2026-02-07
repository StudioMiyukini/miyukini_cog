# MiyukiniAdmin — Reference Implementation Guidelines

## 1. Contexte

Ce document fournit les **guidelines d'implementation de reference** pour MiyukiniAdmin. Il definit les principes, patterns et contraintes a respecter lors de l'implementation.

## 2. Portee / Scope

Ce document definit :
- Les principes d'implementation
- L'architecture technique recommandee
- Les patterns a utiliser
- Les anti-patterns a eviter
- Les contraintes techniques

Ce document **ne couvre pas** :
- Le code source detaille
- Les specifications de tests
- La documentation API

**Document complementaire :** Pour l'implementation des **controles et de la securite** (detection etat environnement, verrou bootstrap, reponse securitaire, recovery automatique, auth, RBAC), voir [MiyukiniAdmin - Implementation Security and Controls](./MiyukiniAdmin%20-%20Implementation%20Security%20and%20Controls.md).

---

## 3. Principes d'Implementation

### 3.1 Principes Fondamentaux

| Principe | Description |
|----------|-------------|
| **Auto-suffisance** | Backend + Frontend complets, pas de dependance externe |
| **Isolation** | Aucun composant partage avec d'autres Operateurs |
| **Tracabilite** | Toute action est loggee avec contexte complet |
| **Securite maximale** | MFA, chiffrement, audit, zero-trust |
| **Explicite** | Aucune action silencieuse ou implicite |

### 3.2 Stack Technique Recommandee

| Composant | Technologie | Justification |
|-----------|-------------|---------------|
| **Backend** | Rust | Performance, securite memoire |
| **Frontend** | TypeScript + React | Typage fort, ecosysteme mature |
| **State** | Redux/Zustand | Etat previsible, devtools |
| **API interne** | gRPC ou REST | Communication backend-frontend |
| **Storage local** | SQLite | Logs locaux, cache |

---

## 4. Architecture Technique

### 4.1 Structure du Projet

```
miyukini_admin/
├── backend/
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── config/
│   │   ├── api/
│   │   │   ├── handlers/
│   │   │   └── routes.rs
│   │   ├── services/
│   │   │   ├── monitoring.rs
│   │   │   ├── database.rs
│   │   │   ├── security.rs
│   │   │   └── testing.rs
│   │   ├── bridge/
│   │   │   └── bonding_brother.rs
│   │   ├── audit/
│   │   │   └── logger.rs
│   │   └── models/
│   └── Cargo.toml
├── frontend/
│   ├── src/
│   │   ├── App.tsx
│   │   ├── components/
│   │   │   ├── dashboard/
│   │   │   ├── database/
│   │   │   ├── security/
│   │   │   └── common/
│   │   ├── hooks/
│   │   ├── services/
│   │   ├── store/
│   │   └── types/
│   └── package.json
├── shared/
│   └── types/
└── config/
```

### 4.2 Diagramme de Composants

```
┌─────────────────────────────────────────────────────────────────┐
│                      MiyukiniAdmin                              │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                   Frontend (React)                       │   │
│  │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐        │   │
│  │  │Dashboard│ │Database │ │Security │ │ Tests   │        │   │
│  │  │   View  │ │   View  │ │   View  │ │   View  │        │   │
│  │  └────┬────┘ └────┬────┘ └────┬────┘ └────┬────┘        │   │
│  │       └───────────┴───────────┴───────────┘              │   │
│  │                         │                                │   │
│  │                   [API Client]                           │   │
│  └─────────────────────────┬───────────────────────────────┘   │
│                            │                                    │
│                     [Internal API]                              │
│                            │                                    │
│  ┌─────────────────────────┴───────────────────────────────┐   │
│  │                   Backend (Rust)                         │   │
│  │                                                          │   │
│  │  ┌─────────────────────────────────────────────────┐    │   │
│  │  │                  API Layer                       │    │   │
│  │  │  [Handlers] [Middleware: Auth, Audit, Rate]      │    │   │
│  │  └─────────────────────────┬───────────────────────┘    │   │
│  │                            │                             │   │
│  │  ┌─────────────────────────┴───────────────────────┐    │   │
│  │  │               Service Layer                      │    │   │
│  │  │  ┌──────────┐ ┌──────────┐ ┌──────────┐        │    │   │
│  │  │  │Monitoring│ │ Database │ │ Security │        │    │   │
│  │  │  │ Service  │ │ Service  │ │ Service  │        │    │   │
│  │  │  └──────────┘ └──────────┘ └──────────┘        │    │   │
│  │  └─────────────────────────┬───────────────────────┘    │   │
│  │                            │                             │   │
│  │  ┌─────────────────────────┴───────────────────────┐    │   │
│  │  │              Admin Bridge                        │    │   │
│  │  │       (Interface vers BondingBrother)            │    │   │
│  │  └─────────────────────────┬───────────────────────┘    │   │
│  │                            │                             │   │
│  │  ┌─────────────────────────┴───────────────────────┐    │   │
│  │  │              Audit Logger                        │    │   │
│  │  │       (Tracabilite complete)                     │    │   │
│  │  └─────────────────────────────────────────────────┘    │   │
│  │                                                          │   │
│  └──────────────────────────────────────────────────────────┘   │
│                            │                                    │
└────────────────────────────┼────────────────────────────────────┘
                             │
                             ▼
                    [BondingBrother]
```

---

## 5. Implementation Backend

### 5.1 Structure du Main

```rust
// main.rs
use miyukini_admin::{config, server, services};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = config::load()?;
    
    // Initialize audit logger (FIRST - before any action)
    let audit = services::audit::init(&config)?;
    
    // Initialize services
    let monitoring = services::monitoring::init(&config, &audit)?;
    let database = services::database::init(&config, &audit)?;
    let security = services::security::init(&config, &audit)?;
    
    // Initialize Admin Bridge (connection to BondingBrother)
    let bridge = services::bridge::init(&config, &audit)?;
    
    // Start server
    server::run(config, monitoring, database, security, bridge).await
}
```

### 5.2 Admin Bridge Pattern

```rust
// bridge/bonding_brother.rs
pub struct AdminBridge {
    client: BondingBrotherClient,
    audit: AuditLogger,
}

impl AdminBridge {
    /// Execute a capability through BondingBrother
    /// ALL interactions with cores MUST go through this method
    pub async fn execute<T: Capability>(
        &self,
        capability: T,
        context: RequestContext,
    ) -> Result<T::Response, BridgeError> {
        // 1. Log the request
        let request_id = self.audit.log_request(&capability, &context)?;
        
        // 2. Build admin request
        let request = AdminRequest {
            request_id,
            timestamp: Utc::now(),
            source: "miyukini_admin",
            operator_id: context.operator_id,
            capability: capability.name(),
            parameters: capability.parameters(),
            context: context.clone(),
        };
        
        // 3. Send through BondingBrother
        let response = self.client.send(request).await?;
        
        // 4. Log the response
        self.audit.log_response(&request_id, &response)?;
        
        // 5. Return result
        capability.parse_response(response)
    }
}
```

### 5.3 Audit Logger Pattern

```rust
// audit/logger.rs
pub struct AuditLogger {
    storage: AuditStorage,
}

impl AuditLogger {
    /// Log ANY action - this is non-optional
    pub fn log_action(&self, action: &Action) -> Result<AuditId, AuditError> {
        let entry = AuditEntry {
            id: AuditId::new(),
            timestamp: Utc::now(),
            action_type: action.action_type(),
            operator_id: action.operator_id(),
            details: action.details(),
            justification: action.justification(),
            result: None, // Will be updated
        };
        
        self.storage.store(entry)
    }
    
    /// Update action result - must be called after action completes
    pub fn log_result(
        &self, 
        audit_id: &AuditId, 
        result: ActionResult
    ) -> Result<(), AuditError> {
        self.storage.update_result(audit_id, result)
    }
}
```

### 5.4 Service Pattern

```rust
// services/monitoring.rs
pub struct MonitoringService {
    bridge: Arc<AdminBridge>,
    audit: Arc<AuditLogger>,
}

impl MonitoringService {
    pub async fn get_system_metrics(&self, ctx: RequestContext) -> Result<SystemMetrics, Error> {
        // All requests go through bridge -> BondingBrother -> CaringNanny
        self.bridge.execute(
            capabilities::admin::metrics::System {},
            ctx,
        ).await
    }
    
    pub async fn get_db_metrics(&self, ctx: RequestContext) -> Result<DbMetrics, Error> {
        self.bridge.execute(
            capabilities::admin::metrics::Database {},
            ctx,
        ).await
    }
}
```

---

## 6. Implementation Frontend

### 6.1 Structure des Composants

```typescript
// components/dashboard/Dashboard.tsx
import { useSystemMetrics, useDbMetrics, useOperators } from '@/hooks';
import { MetricsCard, OperatorsTable, AlertsPanel } from '@/components/common';

export function Dashboard() {
  const { metrics: systemMetrics, isLoading: systemLoading } = useSystemMetrics();
  const { metrics: dbMetrics, isLoading: dbLoading } = useDbMetrics();
  const { operators, isLoading: operatorsLoading } = useOperators();

  return (
    <div className="dashboard">
      <section className="status-cards">
        <HealthScoreCard score={systemMetrics?.healthScore} />
        <TrustLevelBadge level={systemMetrics?.trustLevel} />
        <SecurityLevelBadge level={systemMetrics?.securityLevel} />
      </section>
      
      <section className="metrics-grid">
        <MetricsCard title="CPU" value={systemMetrics?.cpu} loading={systemLoading} />
        <MetricsCard title="RAM" value={systemMetrics?.ram} loading={systemLoading} />
        <MetricsCard title="Disk" value={systemMetrics?.disk} loading={systemLoading} />
        <MetricsCard title="Network" value={systemMetrics?.network} loading={systemLoading} />
      </section>
      
      <section className="operators">
        <OperatorsTable operators={operators} loading={operatorsLoading} />
      </section>
      
      <section className="alerts">
        <AlertsPanel />
      </section>
    </div>
  );
}
```

### 6.2 Hooks Pattern

```typescript
// hooks/useSystemMetrics.ts
import { useQuery } from '@tanstack/react-query';
import { adminApi } from '@/services/api';

export function useSystemMetrics(refreshInterval = 5000) {
  return useQuery({
    queryKey: ['metrics', 'system'],
    queryFn: () => adminApi.getSystemMetrics(),
    refetchInterval: refreshInterval,
    staleTime: refreshInterval - 1000,
  });
}
```

### 6.3 API Client Pattern

```typescript
// services/api.ts
class AdminApiClient {
  private baseUrl: string;
  private authToken: string | null = null;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> {
    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      ...options,
      headers: {
        'Content-Type': 'application/json',
        ...(this.authToken && { Authorization: `Bearer ${this.authToken}` }),
        ...options.headers,
      },
    });

    if (!response.ok) {
      throw new ApiError(response.status, await response.text());
    }

    return response.json();
  }

  // Metrics
  getSystemMetrics() {
    return this.request<SystemMetrics>('/api/metrics/system');
  }

  getDbMetrics() {
    return this.request<DbMetrics>('/api/metrics/db');
  }

  // Security
  getSecurityLevel() {
    return this.request<SecurityLevel>('/api/security/level');
  }

  async changeSecurityLevel(newLevel: number, justification: string) {
    return this.request<ChangeResult>('/api/security/level', {
      method: 'POST',
      body: JSON.stringify({ newLevel, justification }),
    });
  }
}

export const adminApi = new AdminApiClient('/');
```

---

## 7. Patterns Obligatoires

### 7.1 Pattern: Audit Everything

```rust
// TOUT doit passer par l'audit
pub async fn any_operation(
    audit: &AuditLogger,
    ctx: RequestContext,
    operation: impl Fn() -> Result<T>
) -> Result<T> {
    let audit_id = audit.log_action(&Action::from_context(&ctx))?;
    
    match operation() {
        Ok(result) => {
            audit.log_result(&audit_id, ActionResult::Success)?;
            Ok(result)
        }
        Err(e) => {
            audit.log_result(&audit_id, ActionResult::Error(e.to_string()))?;
            Err(e)
        }
    }
}
```

### 7.2 Pattern: Confirmation Required

```typescript
// Pour toute action critique
function useCriticalAction<T>(
  action: (params: T) => Promise<void>,
  options: { requireJustification: boolean }
) {
  const [showConfirm, setShowConfirm] = useState(false);
  const [justification, setJustification] = useState('');
  
  const execute = async (params: T) => {
    if (options.requireJustification && justification.length < 50) {
      throw new Error('Justification required (min 50 chars)');
    }
    
    setShowConfirm(false);
    await action({ ...params, justification });
  };
  
  return {
    showConfirm,
    setShowConfirm,
    justification,
    setJustification,
    execute,
  };
}
```

### 7.3 Pattern: Never Silent

```typescript
// Toast/notification obligatoire pour toute action
function useActionWithFeedback<T>(
  action: () => Promise<T>,
  options: { 
    loadingMessage: string;
    successMessage: string;
    errorMessage: string;
  }
) {
  const [isLoading, setIsLoading] = useState(false);
  const { toast } = useToast();
  
  const execute = async () => {
    setIsLoading(true);
    toast.info(options.loadingMessage);
    
    try {
      const result = await action();
      toast.success(options.successMessage);
      return result;
    } catch (error) {
      toast.error(`${options.errorMessage}: ${error.message}`);
      throw error;
    } finally {
      setIsLoading(false);
    }
  };
  
  return { execute, isLoading };
}
```

---

## 8. Anti-Patterns a Eviter

### 8.1 Direct Core Access

```rust
// INTERDIT - acces direct au core
async fn bad_example() {
    let result = strong_father::decide(intent);  // VIOLATION!
}

// CORRECT - toujours via bridge
async fn good_example(bridge: &AdminBridge) {
    let result = bridge.execute(
        capabilities::StrongFather::Decide { intent },
        context,
    ).await;
}
```

### 8.2 Silent Actions

```typescript
// INTERDIT - action sans feedback
async function badExample() {
  await api.changeSecurityLevel(3);  // VIOLATION - silencieux
}

// CORRECT - toujours avec feedback
async function goodExample() {
  toast.info('Changing security level...');
  try {
    await api.changeSecurityLevel(3);
    toast.success('Security level changed');
  } catch (e) {
    toast.error(`Failed: ${e.message}`);
  }
}
```

### 8.3 Untracked Operations

```rust
// INTERDIT - operation sans audit
fn bad_example(db: &Database) {
    db.execute(query);  // VIOLATION - pas d'audit
}

// CORRECT - toujours avec audit
fn good_example(db: &Database, audit: &AuditLogger, ctx: &RequestContext) {
    let audit_id = audit.log_action(&Action::DbQuery { query: &query }, ctx)?;
    let result = db.execute(query);
    audit.log_result(&audit_id, result.into())?;
}
```

---

## 9. Securite

### 9.1 Authentication

```rust
// Middleware d'authentification obligatoire
pub async fn auth_middleware(
    req: Request,
    next: Next,
) -> Result<Response, AuthError> {
    // 1. Extract token
    let token = req.headers()
        .get("Authorization")
        .ok_or(AuthError::MissingToken)?;
    
    // 2. Validate token
    let session = validate_token(token)?;
    
    // 3. Verify MFA if required
    if session.requires_mfa && !session.mfa_verified {
        return Err(AuthError::MfaRequired);
    }
    
    // 4. Attach session to request
    req.extensions_mut().insert(session);
    
    next.run(req).await
}
```

### 9.2 Rate Limiting

```rust
// Rate limiter par capacite
pub struct RateLimiter {
    limits: HashMap<String, RateLimit>,
}

impl RateLimiter {
    pub fn check(&self, capability: &str, operator_id: &str) -> Result<(), RateLimitError> {
        let limit = self.limits.get(capability)
            .unwrap_or(&RateLimit::default());
        
        let key = format!("{}:{}", capability, operator_id);
        let count = self.increment(&key)?;
        
        if count > limit.max_requests {
            return Err(RateLimitError::Exceeded {
                capability: capability.to_string(),
                retry_after: limit.window,
            });
        }
        
        Ok(())
    }
}
```

---

## 10. Tests

### 10.1 Structure des Tests

```
tests/
├── unit/
│   ├── services/
│   ├── bridge/
│   └── audit/
├── integration/
│   ├── api/
│   └── bridge/
└── e2e/
    ├── dashboard.spec.ts
    ├── database.spec.ts
    └── security.spec.ts
```

### 10.2 Test Pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_security_level_change_requires_approval() {
        // Setup
        let bridge = MockBridge::new();
        let audit = MockAuditLogger::new();
        let service = SecurityService::new(bridge, audit);
        
        // Execute
        let result = service.change_level(
            3,
            "Test justification with enough characters",
            test_context(),
        ).await;
        
        // Verify
        assert!(bridge.was_called_with("admin.security.level.write"));
        assert!(audit.has_entry_for("SECURITY_LEVEL_CHANGE"));
    }
}
```

---

## 11. Documents Associes

- [MiyukiniAdmin - Documentation Fondatrice](../foundation/MiyukiniAdmin%20-%20Documentation%20Fondatrice.md)
- [MiyukiniAdmin - Architecture & Flows](../architecture/MiyukiniAdmin%20-%20Architecture%20&%20Flows.md)
- [MiyukiniAdmin - Invariants & Guarantees](../contracts/governance/MiyukiniAdmin%20-%20Invariants%20&%20Guarantees.md)

---

**Date de creation :** 2026-01-28  
**Version :** 1.0.0  
**Statut :** Document de reference
