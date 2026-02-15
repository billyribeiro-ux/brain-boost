-- Create extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create enums
CREATE TYPE subscription_tier AS ENUM ('free', 'premium');
CREATE TYPE user_status AS ENUM ('active', 'suspended', 'deleted');
CREATE TYPE level_status AS ENUM ('locked', 'active', 'completed');
CREATE TYPE session_slot AS ENUM ('morning', 'afternoon', 'evening');
CREATE TYPE session_status AS ENUM ('pending', 'in_progress', 'completed', 'skipped', 'missed');
CREATE TYPE exercise_type AS ENUM (
    'meditation', 'body_scan', 'nsdr', 'visualization', 'qigong',
    'diet_log', 'sleep_prep', 'sleep_log',
    'retrieval_practice', 'spaced_repetition', 'interleaving',
    'feynman_technique', 'reading', 'speed_drill',
    'puzzle', 'dual_nback', 'bias_busting',
    'exercise_light', 'exercise_brisk', 'hiit', 'cold_exposure',
    'new_skill', 'creative_skill',
    'cbt_reframe', 'journaling', 'distraction_setup',
    'trading_sim', 'rl_trading', 'custom_combo'
);

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    avatar_url TEXT,
    subscription subscription_tier NOT NULL DEFAULT 'free',
    status user_status NOT NULL DEFAULT 'active',
    chronotype VARCHAR(20),
    primary_goal VARCHAR(50),
    timezone VARCHAR(50) NOT NULL DEFAULT 'America/New_York',
    onboarding_completed BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Session schedule preferences
CREATE TABLE user_schedule (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    morning_time TIME NOT NULL DEFAULT '07:30',
    afternoon_time TIME NOT NULL DEFAULT '13:30',
    evening_time TIME NOT NULL DEFAULT '20:30',
    notification_enabled BOOLEAN NOT NULL DEFAULT true,
    haptic_enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id)
);

-- Level progress tracking
CREATE TABLE user_levels (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    level_number SMALLINT NOT NULL CHECK (level_number BETWEEN 1 AND 6),
    status level_status NOT NULL DEFAULT 'locked',
    current_week SMALLINT NOT NULL DEFAULT 1 CHECK (current_week BETWEEN 1 AND 6),
    current_day SMALLINT NOT NULL DEFAULT 1 CHECK (current_day BETWEEN 1 AND 7),
    grace_skip_used BOOLEAN NOT NULL DEFAULT false,
    compliance_rate DECIMAL(5,2) NOT NULL DEFAULT 0.00,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    restarted_count INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, level_number)
);

-- Daily session tracking
CREATE TABLE daily_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    level_number SMALLINT NOT NULL,
    week_number SMALLINT NOT NULL,
    day_number SMALLINT NOT NULL,
    day_date DATE NOT NULL,
    slot session_slot NOT NULL,
    status session_status NOT NULL DEFAULT 'pending',
    scheduled_time TIME,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    duration_planned_seconds INT NOT NULL,
    duration_actual_seconds INT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, day_date, slot)
);

-- Individual exercise completions within sessions
CREATE TABLE exercise_completions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id UUID NOT NULL REFERENCES daily_sessions(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    exercise_type exercise_type NOT NULL,
    exercise_name VARCHAR(200) NOT NULL,
    duration_seconds INT NOT NULL,
    accuracy_score DECIMAL(5,2),
    focus_rating SMALLINT CHECK (focus_rating BETWEEN 1 AND 10),
    difficulty_level SMALLINT,
    metadata JSONB DEFAULT '{}',
    completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Metrics snapshots (daily aggregated)
CREATE TABLE daily_metrics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    metric_date DATE NOT NULL,
    brain_score DECIMAL(5,2),
    stress_index DECIMAL(5,2),
    longevity_score DECIMAL(5,2),
    completion_rate DECIMAL(5,2),
    avg_accuracy DECIMAL(5,2),
    avg_focus DECIMAL(5,2),
    total_session_minutes INT,
    streak_days INT NOT NULL DEFAULT 0,
    hrv_reading DECIMAL(6,2),
    sleep_hours DECIMAL(4,2),
    sleep_quality SMALLINT CHECK (sleep_quality BETWEEN 1 AND 10),
    exercise_minutes INT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, metric_date)
);

-- Spaced repetition flashcards
CREATE TABLE flashcards (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    topic VARCHAR(200) NOT NULL,
    question TEXT NOT NULL,
    answer TEXT NOT NULL,
    difficulty SMALLINT NOT NULL DEFAULT 1 CHECK (difficulty BETWEEN 1 AND 5),
    ease_factor DECIMAL(4,2) NOT NULL DEFAULT 2.50,
    interval_days INT NOT NULL DEFAULT 1,
    repetitions INT NOT NULL DEFAULT 0,
    next_review_date DATE NOT NULL DEFAULT CURRENT_DATE,
    last_reviewed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Hyper-Learning topics
CREATE TABLE learning_topics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    topic_name VARCHAR(200) NOT NULL,
    description TEXT,
    total_micro_lessons INT NOT NULL DEFAULT 0,
    completed_micro_lessons INT NOT NULL DEFAULT 0,
    estimated_mastery_days INT,
    mastery_percentage DECIMAL(5,2) NOT NULL DEFAULT 0.00,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE micro_lessons (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    topic_id UUID NOT NULL REFERENCES learning_topics(id) ON DELETE CASCADE,
    lesson_order SMALLINT NOT NULL,
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    duration_minutes SMALLINT NOT NULL DEFAULT 10,
    next_scheduled_at TIMESTAMPTZ,
    completed BOOLEAN NOT NULL DEFAULT false,
    score DECIMAL(5,2),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Trading simulator data
CREATE TABLE trading_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    scenario_type VARCHAR(50) NOT NULL,
    starting_balance DECIMAL(12,2) NOT NULL DEFAULT 100000.00,
    ending_balance DECIMAL(12,2),
    total_trades INT NOT NULL DEFAULT 0,
    winning_trades INT NOT NULL DEFAULT 0,
    expected_value_score DECIMAL(8,4),
    bias_detections JSONB DEFAULT '[]',
    process_score DECIMAL(5,2),
    decision_speed_ms INT,
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE trading_decisions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    trading_session_id UUID NOT NULL REFERENCES trading_sessions(id) ON DELETE CASCADE,
    decision_type VARCHAR(20) NOT NULL,
    asset_symbol VARCHAR(20) NOT NULL,
    price_at_decision DECIMAL(12,4) NOT NULL,
    quantity DECIMAL(12,4),
    reasoning TEXT,
    ai_feedback TEXT,
    bias_detected VARCHAR(50)[],
    was_optimal BOOLEAN,
    pnl DECIMAL(12,2),
    decision_time_ms INT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- AI Coach conversation history
CREATE TABLE coach_messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(20) NOT NULL,
    content TEXT NOT NULL,
    suggestion_type VARCHAR(50),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Streaks and achievements
CREATE TABLE achievements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT NOT NULL,
    icon_name VARCHAR(50) NOT NULL,
    category VARCHAR(50) NOT NULL,
    requirement_value INT NOT NULL,
    requirement_type VARCHAR(50) NOT NULL
);

CREATE TABLE user_achievements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    achievement_id UUID NOT NULL REFERENCES achievements(id) ON DELETE CASCADE,
    unlocked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, achievement_id)
);

-- Refresh tokens for JWT
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    revoked BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_daily_sessions_user_date ON daily_sessions(user_id, day_date);
CREATE INDEX idx_daily_sessions_status ON daily_sessions(user_id, status);
CREATE INDEX idx_exercise_completions_session ON exercise_completions(session_id);
CREATE INDEX idx_exercise_completions_user ON exercise_completions(user_id, completed_at);
CREATE INDEX idx_daily_metrics_user_date ON daily_metrics(user_id, metric_date);
CREATE INDEX idx_flashcards_review ON flashcards(user_id, next_review_date);
CREATE INDEX idx_trading_sessions_user ON trading_sessions(user_id, created_at);
CREATE INDEX idx_coach_messages_user ON coach_messages(user_id, created_at);
CREATE INDEX idx_user_levels_user ON user_levels(user_id, level_number);
CREATE INDEX idx_refresh_tokens_user ON refresh_tokens(user_id, revoked);
