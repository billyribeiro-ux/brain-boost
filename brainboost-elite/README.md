# BrainBoost Elite

Advanced cognitive training system for peak mental performance. A 36-week program (6 levels × 6 weeks) with comprehensive cognitive enhancement exercises, AI coaching, and gamification.

## Tech Stack

### Backend
- **Rust** with Axum web framework
- **PostgreSQL 16** database
- **SQLx** for compile-time checked queries
- JWT authentication with bcrypt

### Frontend
- **Next.js 14** (App Router)
- **React 18.2+**
- **Tailwind CSS 3.4** with Neural Nexus theme
- **Framer Motion 10** for animations
- **Zustand 4** for state management
- **TanStack React Query 5** for data fetching

## Project Structure

```
brainboost-elite/
├── backend/          # Rust/Axum API server
│   ├── src/
│   │   ├── main.rs
│   │   ├── models/   # Database models
│   │   ├── routes/   # API endpoints
│   │   ├── services/ # Business logic
│   │   └── middleware/
│   └── migrations/   # SQL migrations
├── frontend/         # Next.js application
│   └── src/
│       ├── app/      # Next.js pages
│       ├── components/ # React components
│       ├── stores/   # Zustand stores
│       ├── lib/      # Utilities
│       └── types/    # TypeScript types
└── docker-compose.yml
```

## Getting Started

### Prerequisites
- Rust (latest stable)
- Node.js 18+
- Docker & Docker Compose
- PostgreSQL 16 (or use Docker)

### 1. Start PostgreSQL

```bash
docker compose up -d
```

### 2. Set up Backend

```bash
cd backend

# Copy environment file
cp .env.example .env

# Edit .env with your settings
# DATABASE_URL=postgresql://postgres:postgres@localhost:5432/brainboost_elite
# JWT_SECRET=your-secret-key-change-in-production-min-32-chars

# Run migrations
cargo install sqlx-cli
sqlx migrate run

# Start the backend server
cargo run
```

The backend will be available at `http://localhost:8080`

### 3. Set up Frontend

```bash
cd frontend

# Install dependencies
npm install

# Copy environment file
cp .env.example .env.local

# Edit .env.local
# NEXT_PUBLIC_API_URL=http://localhost:8080

# Start the development server
npm run dev
```

The frontend will be available at `http://localhost:3000`

## Features

### Core Functionality
- **6-Level Progressive System**: Neural Ignition → Synaptic Surge → Cognitive Catalyst → Mental Mastery → Peak Performance → Transhuman
- **Daily Sessions**: Morning, Afternoon, Evening (45-90 min/day)
- **Comprehensive Exercises**:
  - Mindfulness & Recovery (meditation, NSDR, body scan)
  - Hyper-Learning (spaced repetition, Feynman technique, speed drills)
  - Cognitive Enhancement (Dual N-Back, puzzles, bias busting)
  - Physical Optimization (HIIT, cold exposure)
  - Trading Simulator with bias detection
  - AI Coach with personalized guidance

### Design System: Neural Nexus
- **Dark Mode**: Near-black (#121212) with synaptic grid animations
- **Color Palette**:
  - Neural Blue (#007BFF) - Primary actions
  - Creative Purple (#6F42C1) - AI features
  - Success Green (#28A745) - Achievements
  - Warning Red (#DC3545) - Alerts
  - Orange Accent (#FD7E14) - Streaks
- **Typography**: Inter font family
- **Animations**: Framer Motion with spring physics

## API Endpoints

### Authentication
- `POST /auth/register` - Create new account
- `POST /auth/login` - Sign in
- `POST /auth/refresh` - Refresh token
- `POST /auth/logout` - Sign out

### User
- `GET /users/me` - Get current user
- `PATCH /users/me` - Update profile
- `GET /users/me/schedule` - Get session schedule
- `PUT /users/me/schedule` - Update schedule

### Progress
- `GET /progress/current` - Current level and today's sessions
- `GET /progress/levels` - All levels overview
- `POST /progress/advance-day` - Move to next day

### Exercises
- `GET /exercises/today` - Today's exercise plan
- `POST /exercises/complete` - Complete an exercise
- `GET /exercises/history` - Exercise history

### Metrics
- `GET /metrics/today` - Today's metrics
- `GET /metrics/trend?days=30` - Metrics trend
- `GET /metrics/brain-score` - Current brain score

### Trading
- `POST /trading/start` - Start trading session
- `POST /trading/decide` - Make trading decision
- `GET /trading/history` - Trading history
- `GET /trading/stats` - Trading statistics

### Coach
- `POST /coach/message` - Send message to AI coach
- `GET /coach/suggestions` - Get suggestions
- `GET /coach/history` - Message history

## Development

### Backend Development
```bash
cd backend
cargo watch -x run  # Auto-reload on changes
cargo test          # Run tests
cargo clippy        # Lint
```

### Frontend Development
```bash
cd frontend
npm run dev         # Development server
npm run build       # Production build
npm run lint        # Lint
```

### Database Migrations
```bash
cd backend
sqlx migrate add <migration_name>  # Create new migration
sqlx migrate run                   # Run migrations
sqlx migrate revert                # Revert last migration
```

## License

Proprietary - All rights reserved

## Author

John Smith - Apple Principal Engineer ICT Level 7
