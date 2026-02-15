-- Seed test data for E2E testing
-- Run this before E2E tests to populate database with realistic test data

-- Clean existing test data
DELETE FROM exercise_completions WHERE user_id IN (SELECT id FROM users WHERE email LIKE '%@test.brainboost%');
DELETE FROM daily_sessions WHERE user_id IN (SELECT id FROM users WHERE email LIKE '%@test.brainboost%');
DELETE FROM daily_metrics WHERE user_id IN (SELECT id FROM users WHERE email LIKE '%@test.brainboost%');
DELETE FROM user_levels WHERE user_id IN (SELECT id FROM users WHERE email LIKE '%@test.brainboost%');
DELETE FROM refresh_tokens WHERE user_id IN (SELECT id FROM users WHERE email LIKE '%@test.brainboost%');
DELETE FROM users WHERE email LIKE '%@test.brainboost%';

-- Test User 1: New user (for onboarding flow)
INSERT INTO users (id, email, password_hash, display_name, created_at)
VALUES (
  '00000000-0000-0000-0000-000000000001',
  'newuser@test.brainboost',
  '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYzS3MET7iC', -- password: TestPass123!
  'New Test User',
  NOW()
);

-- Test User 2: Active user at Level 1, Week 2, Day 3
INSERT INTO users (id, email, password_hash, display_name, chronotype, primary_goal, timezone, created_at)
VALUES (
  '00000000-0000-0000-0000-000000000002',
  'activeuser@test.brainboost',
  '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYzS3MET7iC', -- password: TestPass123!
  'Active Test User',
  'morning',
  'focus',
  'America/New_York',
  NOW() - INTERVAL '10 days'
);

-- User 2: Level progress
INSERT INTO user_levels (id, user_id, level_number, week_number, current_day, status, compliance_rate, started_at)
VALUES (
  '00000000-0000-0000-0000-000000000011',
  '00000000-0000-0000-0000-000000000002',
  1,
  2,
  3,
  'active',
  85.5,
  NOW() - INTERVAL '10 days'
);

-- User 2: Today's sessions
INSERT INTO daily_sessions (id, user_id, level_number, week_number, day_number, slot, session_date, status)
VALUES 
  ('00000000-0000-0000-0000-000000000021', '00000000-0000-0000-0000-000000000002', 1, 2, 3, 'morning', CURRENT_DATE, 'completed'),
  ('00000000-0000-0000-0000-000000000022', '00000000-0000-0000-0000-000000000002', 1, 2, 3, 'afternoon', CURRENT_DATE, 'pending'),
  ('00000000-0000-0000-0000-000000000023', '00000000-0000-0000-0000-000000000002', 1, 2, 3, 'evening', CURRENT_DATE, 'pending');

-- User 2: Some completed exercises
INSERT INTO exercise_completions (id, user_id, session_id, exercise_type, duration_seconds, accuracy, focus_rating, completed_at)
VALUES 
  ('00000000-0000-0000-0000-000000000031', '00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000021', 'meditation', 600, 100, 8, NOW() - INTERVAL '2 hours'),
  ('00000000-0000-0000-0000-000000000032', '00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000021', 'flashcard', 900, 85, 7, NOW() - INTERVAL '1 hour');

-- User 2: Metrics history (last 7 days)
INSERT INTO daily_metrics (user_id, metric_date, brain_score, stress_index, longevity_score, streak, completion_rate, avg_accuracy, avg_focus)
VALUES 
  ('00000000-0000-0000-0000-000000000002', CURRENT_DATE - INTERVAL '6 days', 65.5, 35.2, 72.0, 1, 100, 82, 7),
  ('00000000-0000-0000-0000-000000000002', CURRENT_DATE - INTERVAL '5 days', 67.2, 33.8, 73.5, 2, 100, 85, 8),
  ('00000000-0000-0000-0000-000000000002', CURRENT_DATE - INTERVAL '4 days', 68.8, 32.1, 74.2, 3, 100, 87, 8),
  ('00000000-0000-0000-0000-000000000002', CURRENT_DATE - INTERVAL '3 days', 70.1, 30.5, 75.0, 4, 100, 88, 8),
  ('00000000-0000-0000-0000-000000000002', CURRENT_DATE - INTERVAL '2 days', 71.5, 29.2, 76.1, 5, 100, 90, 9),
  ('00000000-0000-0000-0000-000000000002', CURRENT_DATE - INTERVAL '1 day', 72.8, 28.0, 77.0, 6, 100, 91, 9),
  ('00000000-0000-0000-0000-000000000002', CURRENT_DATE, 74.0, 27.5, 78.0, 7, 66.67, 85, 7);

-- Test User 3: Advanced user at Level 3
INSERT INTO users (id, email, password_hash, display_name, chronotype, primary_goal, timezone, created_at)
VALUES (
  '00000000-0000-0000-0000-000000000003',
  'advanced@test.brainboost',
  '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5GyYzS3MET7iC', -- password: TestPass123!
  'Advanced Test User',
  'flexible',
  'performance',
  'America/Los_Angeles',
  NOW() - INTERVAL '60 days'
);

-- User 3: Level progress
INSERT INTO user_levels (id, user_id, level_number, week_number, current_day, status, compliance_rate, started_at)
VALUES (
  '00000000-0000-0000-0000-000000000012',
  '00000000-0000-0000-0000-000000000003',
  3,
  4,
  2,
  'active',
  92.3,
  NOW() - INTERVAL '60 days'
);

-- User 3: Learning topics
INSERT INTO learning_topics (id, user_id, topic_name, description, mastery_percentage, estimated_mastery_days, created_at)
VALUES 
  ('00000000-0000-0000-0000-000000000041', '00000000-0000-0000-0000-000000000003', 'options_trading', 'Master options trading strategies', 45.5, 30, NOW() - INTERVAL '15 days'),
  ('00000000-0000-0000-0000-000000000042', '00000000-0000-0000-0000-000000000003', 'python_basics', 'Learn Python programming', 75.0, 20, NOW() - INTERVAL '25 days');

-- User 3: Some micro lessons
INSERT INTO micro_lessons (id, topic_id, lesson_number, title, content, duration_minutes, next_scheduled_at, completed)
VALUES 
  ('00000000-0000-0000-0000-000000000051', '00000000-0000-0000-0000-000000000041', 1, 'What are Options?', 'Options are financial derivatives...', 8, NOW() - INTERVAL '1 day', true),
  ('00000000-0000-0000-0000-000000000052', '00000000-0000-0000-0000-000000000041', 2, 'Calls vs Puts', 'A call option gives you the right...', 10, NOW() + INTERVAL '2 hours', false),
  ('00000000-0000-0000-0000-000000000053', '00000000-0000-0000-0000-000000000042', 1, 'Variables & Types', 'Variables store data in Python...', 8, NOW() - INTERVAL '2 days', true);

-- User 3: Flashcards
INSERT INTO flashcards (id, user_id, topic, question, answer, difficulty, ease_factor, interval_days, repetitions, next_review_date)
VALUES 
  ('00000000-0000-0000-0000-000000000061', '00000000-0000-0000-0000-000000000003', 'Options Trading', 'What is a call option?', 'A call option gives the holder the right, but not the obligation, to buy an asset at a specified price.', 'medium', 2.5, 3, 2, CURRENT_DATE),
  ('00000000-0000-0000-0000-000000000062', '00000000-0000-0000-0000-000000000003', 'Python', 'What is a variable in Python?', 'A variable is a named storage location that holds a value.', 'easy', 2.8, 7, 3, CURRENT_DATE + INTERVAL '2 days');

-- User 3: Trading sessions
INSERT INTO trading_sessions (id, user_id, scenario_type, total_trades, winning_trades, avg_ev_score, avg_process_score, started_at, completed_at)
VALUES 
  ('00000000-0000-0000-0000-000000000071', '00000000-0000-0000-0000-000000000003', 'endowment_effect', 5, 3, 72.5, 68.0, NOW() - INTERVAL '3 days', NOW() - INTERVAL '3 days' + INTERVAL '45 minutes'),
  ('00000000-0000-0000-0000-000000000072', '00000000-0000-0000-0000-000000000003', 'loss_aversion', 5, 4, 78.2, 75.5, NOW() - INTERVAL '1 day', NOW() - INTERVAL '1 day' + INTERVAL '40 minutes');

-- User 3: Achievements
INSERT INTO user_achievements (user_id, achievement_name, unlocked_at)
VALUES 
  ('00000000-0000-0000-0000-000000000003', 'First Spark', NOW() - INTERVAL '59 days'),
  ('00000000-0000-0000-0000-000000000003', 'Week Warrior', NOW() - INTERVAL '53 days'),
  ('00000000-0000-0000-0000-000000000003', 'Month Master', NOW() - INTERVAL '30 days'),
  ('00000000-0000-0000-0000-000000000003', 'Trading Novice', NOW() - INTERVAL '3 days');

-- Commit the transaction
COMMIT;
