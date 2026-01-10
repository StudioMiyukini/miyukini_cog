Miyukini — Stack Technique
TypeScript • Vite • Alpha → Bêta

> ⚠️ DOCUMENT TECHNIQUE DE RÉFÉRENCE  
> Toute implémentation doit respecter STRICTEMENT cette stack.  
> Toute technologie non listée ici est interdite par défaut.

---

## 1. Plateformes cibles

### 1.1 Applications supportées
- ✅ Web App (Desktop + Mobile)
- ✅ PWA (installable)
- ✅ Back-office Web

### 1.2 Plateformes exclues
- ✅ Android natif (webapp encapsulé dans l'application native)
- ❌ iOS natif
- ❌ Desktop natif

---

## 2. Langage & Standards

### 2.1 Langage
- **TypeScript strict**
- `strict: true` obligatoire
- Pas de JavaScript non typé

❌ `any` interdit (sauf cas documenté)

---

### 2.2 Convention de nommage (rappel)
- camelCase → code
- PascalCase → types / classes
- snake_case → base SQL
- kebab-case → routes / URLs

---

## 3. Front-end (Application utilisateur)

### 3.1 Bundler
- **Vite**

Raisons :
- rapidité
- simplicité
- excellent support TS
- compatible PWA

---

### 3.2 Framework UI
- **React**
- Fonctionnel uniquement (hooks)

❌ Class Components interdits

---

### 3.3 UI / Design System
- Design System interne Famitura
- Atomic Design strict :
  - atoms
  - molecules
  - organisms
- Styles :
  - CSS Modules ou
  - Tailwind CSS (choix unique)

❌ UI libraries complètes (MUI, Ant, Chakra) interdites

---

### 3.4 Architecture Front-end

src/
├─ app/
│ ├─ router/
│ ├─ providers/
│ └─ layout/
│
├─ core/
│ ├─ ui/
│ ├─ hooks/
│ ├─ services/
│ ├─ events/
│ └─ config/
│
├─ features/
│ ├─ todo/
│ ├─ shopping/
│ ├─ budget/
│ └─ medical/
│
├─ types/
└─ utils/

---

### 3.5 Gestion d’état
- **TanStack Query**
- **Zustand** (état UI local)

❌ Redux interdit  
❌ MobX interdit  

---

### 3.6 Routing
- **React Router**
- Routes déclaratives
- Un `NavGraph` par module

---

## 4. Offline-first & Sync

### 4.1 Stockage local
- **IndexedDB**
  - via Dexie.js ou équivalent typé
- LocalStorage UNIQUEMENT pour préférences

---

### 4.2 Stratégie offline
- Local → UI
- Sync en arrière-plan
- Résolution conflit :
  - last-write-wins
  - historique minimal

---

### 4.3 Background sync
- Service Workers
- Sync à :
  - ouverture
  - reconnexion
  - action utilisateur

---

## 5. Backend & API

### 5.1 Backend principal
- **Supabase**

Utilisé pour :
- Auth
- PostgreSQL
- Sync
- RLS
- Storage

---

### 5.2 API
- Supabase REST / RPC
- Versionnement :
  - v1 uniquement en alpha

❌ Backend custom non justifié interdit

---

### 5.3 Authentification

#### Alpha
- Email + mot de passe
- Supabase Auth déjà utilisable

#### Bêta
- Multi-device
- Sessions persistantes
- Refresh tokens

---

## 6. Base de données (SQL)

### 6.1 SGBD
- **PostgreSQL (Supabase)**

---

### 6.2 ORM / Accès DB
- Supabase client
- SQL explicite pour migrations

❌ Prisma interdit en alpha (complexité inutile)

---

### 6.3 Règles DB
- UUID universels
- Soft delete (`deleted_at`)
- Tables préfixées par module
- RLS obligatoire

---

## 7. Back-office (Admin)

### 7.1 Application
- Web App séparée
- **TypeScript + Vite + React**

---

### 7.2 UI
- UI simplifiée
- Pas de Design System complet
- Composants internes sobres

---

### 7.3 Accès
- Rôle `ADMIN` uniquement
- Auth Supabase séparée

---

## 8. Scan code-barres

### 8.1 Technologie
- API caméra navigateur
- Librairie JS locale (ZXing / Quagga)

---

### 8.2 Contraintes
- Scan local uniquement
- Pas d’API externe
- Fallback manuel obligatoire

---

## 9. Notifications

### 9.1 Notifications locales
- Web Notifications API
- Service Worker

---

### 9.2 Notifications push (bêta)
- FCM Web Push
- Pas de notifications médicales

---

## 10. Sécurité

### 10.1 Front-end
- CSP stricte
- Pas de secrets exposés
- Tokens sécurisés

---

### 10.2 Données médicales
- Isolation logique stricte
- RLS renforcée
- Aucune exposition back-office

---

## 11. Tests & Qualité

### 11.1 Tests unitaires
- **Vitest**

---

### 11.2 Tests UI
- **Playwright**

---

### 11.3 Qualité
- ESLint
- Prettier
- TypeScript strict

---

## 12. Outils & DevOps

### 12.1 Versioning
- Git
- Branches :
  - main
  - develop
  - feature/*

---

### 12.2 CI/CD
- GitHub Actions
- Build + lint + tests

---

## 13. Dépendances INTERDITES

- Redux
- Firebase Realtime DB
- Prisma
- ORM automatique opaque
- UI kits lourds
- JS non typé
- jQuery

---

## 14. Principe final

> La stack TypeScript/Vite de Famitura privilégie :
> - la lisibilité
> - la modularité
> - la maîtrise
> - la cohérence front / back / DB
>
> Toute déviation doit être documentée et validée.
