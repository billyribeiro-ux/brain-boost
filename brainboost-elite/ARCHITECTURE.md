# BrainBoost Elite - System Architecture

## 🏗️ High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         Frontend                             │
│  Next.js 14 + React 18 + TypeScript + Tailwind CSS         │
│                                                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │  Pages   │  │Components│  │  Stores  │  │  Hooks   │  │
│  │ (Routes) │  │   (UI)   │  │ (Zustand)│  │ (Logic)  │  │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘  │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Service Worker (PWA + Offline + Push Notifications) │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ HTTP/REST + WebSocket
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                         Backend                              │
│         Rust + Axum + SQLx + PostgreSQL                     │
│                                                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │  Routes  │  │ Services │  │  Models  │  │Middleware│  │
│  │  (API)   │  │(Business)│  │  (Data)  │  │  (Auth)  │  │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘  │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  WebSocket Manager (Real-time Updates)               │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Background Jobs (Reminders, Metrics, Monitoring)    │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    PostgreSQL Database                       │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │  Users   │  │ Sessions │  │ Exercises│  │ Metrics  │  │
│  │ Levels   │  │Flashcards│  │ Learning │  │ Trading  │  │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## 📊 Database Schema

### Core Tables
- **users** - User accounts and profiles
- **user_levels** - 6-level progression tracking
- **daily_sessions** - Morning/Afternoon/Evening sessions
- **exercise_completions** - Exercise results and metrics

### Elite Features
- **flashcards** - Spaced repetition (SM-2 algorithm)
- **learning_topics** - Hyper-learning topics
- **micro_lessons** - Ultra-spacing lessons
- **trading_sessions** - Cognitive bias training
- **daily_metrics** - Brain score, stress, longevity

### Supporting Tables
- **user_schedule** - Notification preferences
- **refresh_tokens** - JWT refresh tokens
- **user_achievements** - Gamification unlocks

## 🔄 Data Flow

### 1. Authentication Flow
```
User → Frontend → POST /auth/register
                → Backend validates & hashes password
                → Creates user + initial level
                → Returns JWT tokens
                → Frontend stores in memory + localStorage
```

### 2. Exercise Completion Flow
```
User completes exercise
  → Frontend POST /exercises/{session_id}/complete
  → Backend records completion
  → Updates session status
  → Calculates metrics (brain score, accuracy)
  → Triggers WebSocket update
  → Frontend updates UI optimistically
  → Receives confirmation via WebSocket
```

### 3. Real-time Updates Flow
```
Backend Job runs (every 5 minutes)
  → Checks for upcoming sessions
  → Sends SessionReminder via WebSocket
  → Frontend receives message
  → Displays notification
  → Updates UI state
```

## 🎯 Key Design Patterns

### Frontend

**State Management**
- Zustand for global state (auth, metrics, sessions)
- React Query for server state caching
- Optimistic updates for better UX

**Component Architecture**
- Atomic design (atoms → molecules → organisms)
- Reusable UI components
- Feature-based folder structure

**Performance**
- Code splitting with Next.js dynamic imports
- Image optimization with next/image
- Lazy loading for heavy components
- Service worker caching

### Backend

**Layered Architecture**
```
Routes (HTTP handlers)
  ↓
Services (Business logic)
  ↓
Models (Data structures)
  ↓
Database (PostgreSQL)
```

**Concurrency**
- Tokio async runtime
- Connection pooling (SQLx)
- Background job scheduler
- WebSocket connection manager

**Security**
- JWT authentication
- Password hashing (bcrypt)
- CORS middleware
- Rate limiting
- Input validation

## 🔌 API Design

### RESTful Endpoints
```
Auth:
  POST   /auth/register
  POST   /auth/login
  POST   /auth/refresh
  POST   /auth/logout

Users:
  GET    /users/me
  PUT    /users/me
  GET    /users/schedule
  PUT    /users/schedule

Progress:
  GET    /progress/levels
  GET    /progress/current
  POST   /progress/advance

Exercises:
  GET    /exercises/today
  POST   /exercises/{id}/complete
  GET    /exercises/history

Metrics:
  GET    /metrics
  GET    /metrics/trend

Learning:
  POST   /learning/topics
  GET    /learning/topics
  GET    /learning/topics/{id}/next-lesson
  POST   /learning/lessons/{id}/complete

Flashcards:
  POST   /flashcards
  GET    /flashcards/due
  POST   /flashcards/{id}/review

Trading:
  POST   /trading/sessions
  GET    /trading/stats

Coach:
  POST   /coach/message
  GET    /coach/suggestions
```

### WebSocket Messages
```
Server → Client:
  - SessionReminder
  - MetricsUpdate
  - CoachSuggestion
  - AchievementUnlocked
  - StreakMilestone
  - StressShieldAlert
  - ReadinessUpdate
  - LevelAdvancement
  - MissedDayWarning
  - SystemNotification

Client → Server:
  - Ping (heartbeat)
  - Subscribe (to topics)
  - Unsubscribe
```

## 🚀 Performance Characteristics

### Backend
- **Request latency**: < 50ms (p95)
- **Throughput**: 1000+ req/s
- **WebSocket connections**: 10,000+ concurrent
- **Database queries**: < 10ms average

### Frontend
- **First Contentful Paint**: < 1.5s
- **Time to Interactive**: < 3.5s
- **Lighthouse Score**: 90+ (all categories)
- **Bundle size**: < 200KB (gzipped)

## 🔐 Security Architecture

### Authentication
- JWT access tokens (15min expiry)
- Refresh tokens (7 day expiry)
- Secure HTTP-only cookies option
- Token rotation on refresh

### Authorization
- Role-based access control
- Resource ownership validation
- Middleware guards on routes

### Data Protection
- Password hashing (bcrypt, cost 12)
- SQL injection prevention (parameterized queries)
- XSS protection (React escaping)
- CSRF tokens for state-changing operations

## 📈 Scalability Considerations

### Horizontal Scaling
- Stateless backend (JWT auth)
- WebSocket sticky sessions
- Database read replicas
- CDN for static assets

### Caching Strategy
- Browser caching (service worker)
- API response caching (Redis future)
- Database query caching
- Static asset caching (CDN)

### Database Optimization
- Indexes on foreign keys
- Composite indexes for common queries
- Connection pooling
- Query optimization

## 🧪 Testing Strategy

### Unit Tests
- Backend: Rust tests for services
- Frontend: Jest for utilities and hooks

### Integration Tests
- API endpoint tests
- Database integration tests
- WebSocket message tests

### E2E Tests
- Playwright for full user flows
- 80+ test scenarios
- Multi-browser testing
- Mobile responsive testing

## 🔄 CI/CD Pipeline

### On Pull Request
1. Lint code (Clippy, ESLint)
2. Type check (TypeScript)
3. Run unit tests
4. Run integration tests
5. Security audit
6. Build verification

### On Merge to Main
1. Run full test suite
2. Build Docker image
3. Deploy backend (Railway/Fly.io)
4. Deploy frontend (Vercel)
5. Run smoke tests
6. Notify team

## 📊 Monitoring & Observability

### Metrics
- Request rate, latency, errors
- WebSocket connections
- Database query performance
- Memory and CPU usage

### Logging
- Structured JSON logs
- Log levels (ERROR, WARN, INFO, DEBUG)
- Request/response logging
- Error stack traces

### Alerting
- Error rate thresholds
- Performance degradation
- Database connection issues
- High memory usage

---

**Built with ❤️ for optimal cognitive performance**
