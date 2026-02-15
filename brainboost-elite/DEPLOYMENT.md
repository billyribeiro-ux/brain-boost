# BrainBoost Elite - Production Deployment Guide

## 🚀 Quick Start

### Prerequisites
- PostgreSQL 14+
- Rust 1.70+
- Node.js 18+
- pnpm 8+

### Environment Setup

#### Backend (.env)
```bash
DATABASE_URL=postgresql://localhost/brainboost_elite
JWT_SECRET=your-super-secret-jwt-key-change-in-production
FRONTEND_URL=http://localhost:3000
PORT=8080
RUST_LOG=info
ENVIRONMENT=production
VAPID_PUBLIC_KEY=your-vapid-public-key
VAPID_PRIVATE_KEY=your-vapid-private-key
```

#### Frontend (.env.local)
```bash
NEXT_PUBLIC_API_URL=http://localhost:8080
NEXT_PUBLIC_WS_URL=ws://localhost:8080/ws
NEXT_PUBLIC_SENTRY_DSN=your-sentry-dsn
NEXT_PUBLIC_GA_ID=your-google-analytics-id
```

## 📦 Local Development

### 1. Start PostgreSQL
```bash
# macOS with Homebrew
brew services start postgresql@14

# Or with Docker
docker run -d \
  --name brainboost-postgres \
  -e POSTGRES_DB=brainboost_elite \
  -e POSTGRES_PASSWORD=postgres \
  -p 5432:5432 \
  postgres:14
```

### 2. Run Database Migrations
```bash
cd backend
sqlx database create
sqlx migrate run
```

### 3. Seed Test Data (Optional)
```bash
psql -d brainboost_elite -f tests/seed_test_data.sql
```

### 4. Start Backend
```bash
cd backend
cargo run --release
# Server runs on http://localhost:8080
```

### 5. Start Frontend
```bash
cd frontend
pnpm install
pnpm dev
# App runs on http://localhost:3000
```

## 🧪 Running Tests

### Backend Tests
```bash
cd backend
cargo test
```

### Frontend Tests
```bash
cd frontend
pnpm test
```

### E2E Tests with Playwright
```bash
# Make sure backend and database are running
cd frontend
npx playwright test

# Run with UI
npx playwright test --ui

# Run specific suite
npx playwright test e2e/auth.spec.ts

# Run on specific browser
npx playwright test --project=chromium
```

## 🐳 Docker Deployment

### Build Backend Image
```bash
cd backend
docker build -t brainboost-backend:latest .
```

### Run with Docker Compose
```bash
docker-compose up -d
```

## ☁️ Production Deployment

### Backend (Fly.io / Railway / AWS)
1. Set all environment variables in your platform
2. Push Docker image or connect GitHub repo
3. Configure health check endpoint: `/health`
4. Set up PostgreSQL database
5. Run migrations on first deploy

### Frontend (Vercel)
1. Connect GitHub repository
2. Set environment variables in Vercel dashboard
3. Configure build settings:
   - Build Command: `pnpm build`
   - Output Directory: `.next`
   - Install Command: `pnpm install`
4. Deploy automatically on push to main

### CI/CD
GitHub Actions workflows automatically:
- Run tests on every PR
- Deploy to production on merge to main
- Perform security audits
- Run Lighthouse performance checks

## 📊 Monitoring

### Health Checks
- Backend: `GET /health`
- Database: Check connection pool status
- WebSocket: Monitor active connections

### Metrics
- Request latency
- Error rates
- Active users
- Database query performance

### Logs
Backend uses structured JSON logging:
```bash
# View logs
docker logs brainboost-backend

# Filter by level
docker logs brainboost-backend | grep ERROR
```

## 🔒 Security

### Required Actions Before Production
1. Change all default secrets
2. Enable HTTPS/TLS
3. Configure CORS properly
4. Set up rate limiting
5. Enable Sentry error tracking
6. Configure backup strategy

### Environment Variables to Change
- `JWT_SECRET` - Use cryptographically secure random string
- `VAPID_PUBLIC_KEY` / `VAPID_PRIVATE_KEY` - Generate with `web-push generate-vapid-keys`
- Database credentials
- API keys for third-party services

## 🐛 Troubleshooting

### Backend won't start
- Check DATABASE_URL is correct
- Ensure PostgreSQL is running
- Verify migrations have run
- Check port 8080 is available

### Frontend can't connect to backend
- Verify NEXT_PUBLIC_API_URL is correct
- Check CORS configuration
- Ensure backend is running
- Check network/firewall settings

### WebSocket connection fails
- Verify NEXT_PUBLIC_WS_URL is correct
- Check WebSocket proxy configuration
- Ensure /ws endpoint is accessible
- Check for connection limits

### E2E tests fail
- Ensure backend is running
- Check database is seeded with test data
- Verify test users exist (see seed_test_data.sql)
- Check browser compatibility

## 📈 Performance Optimization

### Backend
- Connection pooling configured (max 20 connections)
- Query optimization with indexes
- Caching for frequently accessed data
- Background jobs for heavy operations

### Frontend
- Code splitting with Next.js
- Image optimization
- Lazy loading components
- Service worker caching

### Database
- Indexes on foreign keys
- Composite indexes for common queries
- Regular VACUUM and ANALYZE
- Connection pooling

## 🎯 Production Checklist

- [ ] All environment variables set
- [ ] Database migrations run
- [ ] SSL/TLS certificates configured
- [ ] CORS properly configured
- [ ] Rate limiting enabled
- [ ] Error tracking (Sentry) configured
- [ ] Analytics (GA4) configured
- [ ] Backup strategy in place
- [ ] Monitoring alerts configured
- [ ] Health checks passing
- [ ] Load testing completed
- [ ] Security audit passed
- [ ] Documentation updated

## 📞 Support

For issues or questions:
- GitHub Issues: [repository]/issues
- Email: support@brainboost.app
- Documentation: /docs

## 🔄 Updates

To update to latest version:
```bash
git pull origin main
cd backend && cargo build --release
cd ../frontend && pnpm install && pnpm build
# Run any new migrations
sqlx migrate run
```

---

**BrainBoost Elite** - Unlock Your Super Brain 🧠✨
