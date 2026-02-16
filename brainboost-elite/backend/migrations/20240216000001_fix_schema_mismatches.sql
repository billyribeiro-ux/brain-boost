-- Migration: Fix schema mismatches for SQLx 0.8 compatibility
-- Agent 13: Schema Migration Specialist

-- Add missing columns to user_levels table
ALTER TABLE user_levels
ADD COLUMN IF NOT EXISTS grace_skips_used INTEGER DEFAULT 0;

-- Add comments for documentation
COMMENT ON COLUMN user_levels.grace_skips_used IS 'Number of grace skips used by user';

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_user_levels_user_status ON user_levels(user_id, status);
CREATE INDEX IF NOT EXISTS idx_refresh_tokens_hash ON refresh_tokens(token_hash);
CREATE INDEX IF NOT EXISTS idx_exercise_completions_user ON exercise_completions(user_id);
CREATE INDEX IF NOT EXISTS idx_micro_lessons_topic ON micro_lessons(topic_id);

