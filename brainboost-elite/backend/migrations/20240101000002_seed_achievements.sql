-- Seed achievements data
INSERT INTO achievements (id, name, description, icon_name, category, requirement_value, requirement_type) VALUES
(uuid_generate_v4(), 'First Spark', 'Complete your first exercise session', 'zap', 'level', 1, 'sessions_completed'),
(uuid_generate_v4(), 'Week Warrior', 'Complete 7 consecutive days', 'flame', 'streak', 7, 'streak_days'),
(uuid_generate_v4(), 'Neural Ignition Complete', 'Graduate Level 1', 'brain', 'level', 1, 'level_completed'),
(uuid_generate_v4(), 'Synaptic Surge Complete', 'Graduate Level 2', 'brain', 'level', 2, 'level_completed'),
(uuid_generate_v4(), 'Month Master', 'Maintain a 30-day streak', 'trophy', 'streak', 30, 'streak_days'),
(uuid_generate_v4(), 'Recall Champion', 'Achieve 95% accuracy on flashcards', 'target', 'mastery', 95, 'flashcard_accuracy'),
(uuid_generate_v4(), 'Trading Rookie', 'Complete 10 trading simulations', 'trending-up', 'trading', 10, 'trading_sessions'),
(uuid_generate_v4(), 'Bias Buster', 'Detect and correct 50 biases', 'shield', 'trading', 50, 'biases_corrected'),
(uuid_generate_v4(), 'Zen Mind', 'Complete 100 meditation sessions', 'heart', 'wellness', 100, 'meditation_sessions'),
(uuid_generate_v4(), 'Iron Streak', 'Maintain a 100-day streak', 'award', 'streak', 100, 'streak_days'),
(uuid_generate_v4(), 'Transhuman', 'Reach Level 6', 'star', 'level', 6, 'level_completed'),
(uuid_generate_v4(), 'Speed Demon', 'Score 90%+ on speed processing drills', 'zap', 'mastery', 90, 'speed_drill_score'),
(uuid_generate_v4(), 'Feynman Master', 'Complete 50 Feynman Technique sessions', 'book-open', 'mastery', 50, 'feynman_sessions'),
(uuid_generate_v4(), 'Cold Warrior', 'Complete 30 cold exposure sessions', 'snowflake', 'wellness', 30, 'cold_sessions'),
(uuid_generate_v4(), 'Brain Score 90', 'Achieve a Brain Score of 90+', 'award', 'mastery', 90, 'brain_score');
