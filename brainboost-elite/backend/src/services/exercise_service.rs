use crate::errors::{AppError, Result};
use crate::models::exercise::{CompleteExerciseRequest, ExerciseCompletion};
use crate::models::progress::DailySession;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedExercise {
    pub exercise_type: String,
    pub name: String,
    pub description: String,
    pub duration_seconds: i32,
    pub instructions: Vec<String>,
    pub difficulty: u8,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExercisePlan {
    pub level: u8,
    pub week: u8,
    pub slot: String,
    pub exercises: Vec<PlannedExercise>,
}

pub fn get_daily_plan(level: u8, week: u8, slot: &str) -> Vec<PlannedExercise> {
    match level {
        1 => get_level_1_plan(week, slot),
        2 => get_level_2_plan(week, slot),
        3 => get_level_3_plan(week, slot),
        4 => get_level_4_plan(week, slot),
        5 => get_level_5_plan(week, slot),
        6 => get_level_6_plan(week, slot),
        _ => vec![],
    }
}

fn get_level_1_plan(week: u8, slot: &str) -> Vec<PlannedExercise> {
    match slot {
        "morning" => {
            let (meditation_instructions, meditation_difficulty) = match week {
                1..=2 => (
                    vec![
                        "Find a quiet spot. Close your eyes.".to_string(),
                        "Breathe in for 4 seconds, hold for 2, out for 4.".to_string(),
                        "Focus only on your breath.".to_string(),
                        "When thoughts arise, gently return to breath.".to_string(),
                    ],
                    2,
                ),
                3..=4 => (
                    vec![
                        "Same position. Now count each exhale from 1 to 10.".to_string(),
                        "If you lose count, start over.".to_string(),
                        "This trains sustained attention.".to_string(),
                    ],
                    3,
                ),
                _ => (
                    vec![
                        "Breathe in for 4s, hold for 4s, out for 6s, hold for 2s.".to_string(),
                        "This box-plus breathing activates parasympathetic nervous system.".to_string(),
                    ],
                    4,
                ),
            };

            vec![
                PlannedExercise {
                    exercise_type: "meditation".to_string(),
                    name: "Basic Breath Focus".to_string(),
                    description: "Foundation meditation practice".to_string(),
                    duration_seconds: 600,
                    instructions: meditation_instructions,
                    difficulty: meditation_difficulty,
                    metadata: serde_json::json!({"type": "breath_focus"}),
                },
                PlannedExercise {
                    exercise_type: "diet_log".to_string(),
                    name: "Breakfast Log".to_string(),
                    description: "Log your breakfast intake".to_string(),
                    duration_seconds: 300,
                    instructions: vec!["Log what you ate for breakfast.".to_string(), "App suggests omega-3 rich foods.".to_string()],
                    difficulty: 1,
                    metadata: serde_json::json!({"meal": "breakfast"}),
                },
            ]
        }
        "afternoon" => {
            let (card_count, instructions, difficulty) = match week {
                1..=2 => (
                    10,
                    vec![
                        "See the question. Try to recall the answer before flipping.".to_string(),
                        "Rate difficulty 1-5.".to_string(),
                        "App schedules next review.".to_string(),
                    ],
                    2,
                ),
                3..=4 => (
                    15,
                    vec![
                        "More cards now. Some show answers — recall the question.".to_string(),
                        "Building bidirectional recall.".to_string(),
                    ],
                    4,
                ),
                _ => (
                    20,
                    vec![
                        "Cards now mix topics. This interleaving feels harder but builds stronger connections.".to_string(),
                    ],
                    5,
                ),
            };

            vec![PlannedExercise {
                exercise_type: "spaced_repetition".to_string(),
                name: "Flashcard Review".to_string(),
                description: format!("{} flashcards with spaced repetition", card_count),
                duration_seconds: 900,
                instructions,
                difficulty,
                metadata: serde_json::json!({"card_count": card_count}),
            }]
        }
        "evening" => {
            let (exercise_instructions, exercise_difficulty) = match week {
                1..=2 => (
                    vec![
                        "Walk at a comfortable pace. No phone.".to_string(),
                        "Notice your surroundings.".to_string(),
                        "This is brain-body integration.".to_string(),
                    ],
                    1,
                ),
                3..=4 => (
                    vec![
                        "Increase pace until slightly breathless.".to_string(),
                        "Aim for 100-120 steps/min.".to_string(),
                        "BDNF starts increasing here.".to_string(),
                    ],
                    3,
                ),
                _ => (
                    vec![
                        "Find hills or use treadmill incline.".to_string(),
                        "Push for moderate intensity.".to_string(),
                        "Heart rate 60-70% of max.".to_string(),
                    ],
                    4,
                ),
            };

            vec![
                PlannedExercise {
                    exercise_type: "exercise_light".to_string(),
                    name: "Evening Walk".to_string(),
                    description: "Light physical activity".to_string(),
                    duration_seconds: 600,
                    instructions: exercise_instructions,
                    difficulty: exercise_difficulty,
                    metadata: serde_json::json!({"intensity": "light"}),
                },
                PlannedExercise {
                    exercise_type: "sleep_prep".to_string(),
                    name: "Wind-Down Checklist".to_string(),
                    description: "Prepare for quality sleep".to_string(),
                    duration_seconds: 300,
                    instructions: vec![
                        "Dim all lights. No screens within 30 min.".to_string(),
                        "Set alarm. Log planned bedtime.".to_string(),
                        "Rate today's energy 1-10.".to_string(),
                    ],
                    difficulty: 1,
                    metadata: serde_json::json!({"type": "wind_down"}),
                },
            ]
        }
        _ => vec![],
    }
}

fn get_level_2_plan(week: u8, slot: &str) -> Vec<PlannedExercise> {
    match slot {
        "morning" => {
            let (meditation_instructions, meditation_difficulty) = match week {
                7..=8 => (
                    vec![
                        "Start with breath focus (2 min).".to_string(),
                        "Then slowly scan from feet to head, noting sensations without judgment.".to_string(),
                        "Spend 10-15 seconds per body region.".to_string(),
                    ],
                    3,
                ),
                9..=10 => (
                    vec![
                        "Same scan, but now actively release tension at each point.".to_string(),
                        "Clench then release each muscle group as you scan.".to_string(),
                    ],
                    4,
                ),
                _ => (
                    vec![
                        "After body scan, enter a state of conscious rest.".to_string(),
                        "Body heavy, mind aware.".to_string(),
                        "This is Non-Sleep Deep Rest — more restorative than naps.".to_string(),
                    ],
                    5,
                ),
            };

            vec![
                PlannedExercise {
                    exercise_type: "body_scan".to_string(),
                    name: "Body Scan Meditation".to_string(),
                    description: "Progressive body awareness".to_string(),
                    duration_seconds: 720,
                    instructions: meditation_instructions,
                    difficulty: meditation_difficulty,
                    metadata: serde_json::json!({"type": "body_scan"}),
                },
                PlannedExercise {
                    exercise_type: "distraction_setup".to_string(),
                    name: "Focus Environment Setup".to_string(),
                    description: "Prepare distraction-free workspace".to_string(),
                    duration_seconds: 180,
                    instructions: vec![
                        "Close all unnecessary apps.".to_string(),
                        "Enable Do Not Disturb.".to_string(),
                        "Clear your desk/space.".to_string(),
                        "Set intention: 'I will focus completely for the next session.'".to_string(),
                    ],
                    difficulty: 1,
                    metadata: serde_json::json!({}),
                },
            ]
        }
        "afternoon" => {
            let (topic_count, instructions, difficulty) = match week {
                7..=8 => (
                    2,
                    vec![
                        "Questions alternate between 2 of your active topics.".to_string(),
                        "Don't settle into one — the switching builds transfer ability.".to_string(),
                    ],
                    3,
                ),
                9..=10 => (
                    3,
                    vec![
                        "Three topics now interleaved.".to_string(),
                        "The confusion is the learning signal — embrace the difficulty.".to_string(),
                    ],
                    5,
                ),
                _ => (
                    3,
                    vec![
                        "Same 3 topics but now timed — 30 seconds per question.".to_string(),
                        "Speed + accuracy builds processing power.".to_string(),
                    ],
                    6,
                ),
            };

            vec![
                PlannedExercise {
                    exercise_type: "interleaving".to_string(),
                    name: "Interleaved Practice".to_string(),
                    description: format!("Mix {} topics in quiz form", topic_count),
                    duration_seconds: 600,
                    instructions,
                    difficulty,
                    metadata: serde_json::json!({"topic_count": topic_count}),
                },
                PlannedExercise {
                    exercise_type: "feynman_technique".to_string(),
                    name: "Explain Simply".to_string(),
                    description: "Teach to learn deeply".to_string(),
                    duration_seconds: 600,
                    instructions: vec![
                        "Pick a concept you studied.".to_string(),
                        "Explain it out loud or in writing as simply as possible. No jargon.".to_string(),
                        "If you stumble, that's a gap — note it.".to_string(),
                    ],
                    difficulty: 3,
                    metadata: serde_json::json!({}),
                },
            ]
        }
        "evening" => {
            let (exercise_type, exercise_instructions, exercise_difficulty) = match week {
                7..=8 => (
                    "exercise_brisk",
                    vec![
                        "Jog at conversational pace or brisk walk.".to_string(),
                        "10 minutes continuous.".to_string(),
                        "BDNF elevation begins.".to_string(),
                    ],
                    3,
                ),
                9..=10 => (
                    "hiit",
                    vec![
                        "Warm up 2 min. Then 30s sprint / 30s rest × 8 rounds.".to_string(),
                        "Cool down 2 min.".to_string(),
                        "Heart rate 80-90% max on sprints.".to_string(),
                    ],
                    5,
                ),
                _ => (
                    "hiit",
                    vec![
                        "Same HIIT protocol.".to_string(),
                        "Then 1 minute cold water (shower) or cold air exposure.".to_string(),
                        "Norepinephrine surge for focus.".to_string(),
                    ],
                    7,
                ),
            };

            vec![
                PlannedExercise {
                    exercise_type: exercise_type.to_string(),
                    name: "Cardio Exercise".to_string(),
                    description: "Physical training for brain health".to_string(),
                    duration_seconds: 600,
                    instructions: exercise_instructions,
                    difficulty: exercise_difficulty,
                    metadata: serde_json::json!({}),
                },
                PlannedExercise {
                    exercise_type: "reading".to_string(),
                    name: "Active Reading".to_string(),
                    description: "Read with comprehension questions".to_string(),
                    duration_seconds: 300,
                    instructions: vec![
                        "Read once. Then answer questions without looking back.".to_string(),
                        "Trains active reading.".to_string(),
                    ],
                    difficulty: 2,
                    metadata: serde_json::json!({"word_count": 300}),
                },
            ]
        }
        _ => vec![],
    }
}

fn get_level_3_plan(week: u8, slot: &str) -> Vec<PlannedExercise> {
    match slot {
        "morning" => {
            let (meditation_instructions, meditation_difficulty) = match week {
                13..=14 => (
                    vec![
                        "5 min breath focus.".to_string(),
                        "Then 10 min: vividly visualize achieving your primary goal.".to_string(),
                        "See details — location, feelings, sounds.".to_string(),
                        "This primes motor planning circuits.".to_string(),
                    ],
                    4,
                ),
                15..=16 => (
                    vec![
                        "Visualize novel scenarios: solving a hard problem, inventing something, navigating a maze.".to_string(),
                        "Creative visualization builds new neural pathways.".to_string(),
                    ],
                    5,
                ),
                _ => (
                    vec![
                        "5 min breath focus. 5 min body scan to deep rest state.".to_string(),
                        "5 min visualization while in NSDR state — this is hypnagogic learning.".to_string(),
                    ],
                    6,
                ),
            };

            vec![
                PlannedExercise {
                    exercise_type: "visualization".to_string(),
                    name: "Goal Visualization".to_string(),
                    description: "Mental rehearsal and creative visualization".to_string(),
                    duration_seconds: 900,
                    instructions: meditation_instructions,
                    difficulty: meditation_difficulty,
                    metadata: serde_json::json!({"type": "goal_visualization"}),
                },
                PlannedExercise {
                    exercise_type: "new_skill".to_string(),
                    name: "Micro-Skill Practice".to_string(),
                    description: "5-minute skill burst".to_string(),
                    duration_seconds: 300,
                    instructions: vec![
                        "5 min language burst or music practice.".to_string(),
                        "Focus on pattern recognition, not memorization.".to_string(),
                    ],
                    difficulty: 3,
                    metadata: serde_json::json!({}),
                },
            ]
        }
        "afternoon" => {
            vec![
                PlannedExercise {
                    exercise_type: "interleaving".to_string(),
                    name: "Speed Interleaving".to_string(),
                    description: "Rapid topic switching".to_string(),
                    duration_seconds: 600,
                    instructions: vec![
                        "Rapid-fire questions across 3+ topics.".to_string(),
                        "45 seconds each. Focus on accuracy first.".to_string(),
                    ],
                    difficulty: 5,
                    metadata: serde_json::json!({"topic_count": 3}),
                },
                PlannedExercise {
                    exercise_type: "feynman_technique".to_string(),
                    name: "Complex Explanation".to_string(),
                    description: "Explain multi-step processes".to_string(),
                    duration_seconds: 600,
                    instructions: vec![
                        "Explain a process with 4+ steps.".to_string(),
                        "Must be sequential and complete.".to_string(),
                        "AI challenges weak links.".to_string(),
                    ],
                    difficulty: 5,
                    metadata: serde_json::json!({}),
                },
                PlannedExercise {
                    exercise_type: "puzzle".to_string(),
                    name: "Cognitive Puzzles".to_string(),
                    description: "Pattern recognition training".to_string(),
                    duration_seconds: 300,
                    instructions: vec![
                        "Timed Sudoku or chess puzzles.".to_string(),
                        "Trains pattern recognition and working memory.".to_string(),
                    ],
                    difficulty: 4,
                    metadata: serde_json::json!({}),
                },
            ]
        }
        "evening" => {
            vec![
                PlannedExercise {
                    exercise_type: "hiit".to_string(),
                    name: "HIIT Cardio".to_string(),
                    description: "High-intensity intervals".to_string(),
                    duration_seconds: 600,
                    instructions: vec![
                        "20s all-out / 40s rest × 10 rounds.".to_string(),
                        "Maximum BDNF release.".to_string(),
                    ],
                    difficulty: 6,
                    metadata: serde_json::json!({}),
                },
                PlannedExercise {
                    exercise_type: "cbt_reframe".to_string(),
                    name: "Stress Reframe".to_string(),
                    description: "Cognitive behavioral technique".to_string(),
                    duration_seconds: 300,
                    instructions: vec![
                        "Before sleep: identify one stressful thought.".to_string(),
                        "Write it down. Ask: 'What evidence supports this? What evidence contradicts?'".to_string(),
                        "Reframe.".to_string(),
                    ],
                    difficulty: 3,
                    metadata: serde_json::json!({}),
                },
            ]
        }
        _ => vec![],
    }
}

fn get_level_4_plan(_week: u8, slot: &str) -> Vec<PlannedExercise> {
    match slot {
        "morning" => {
            vec![
                PlannedExercise {
                    exercise_type: "meditation".to_string(),
                    name: "AI-Personalized Meditation".to_string(),
                    description: "Data-driven meditation selection".to_string(),
                    duration_seconds: 900,
                    instructions: vec![
                        "AI analyzes HRV/sleep/stress to choose: breath focus, body scan, or visualization.".to_string(),
                        "Trust the algorithm.".to_string(),
                    ],
                    difficulty: 5,
                    metadata: serde_json::json!({"ai_selected": true}),
                },
                PlannedExercise {
                    exercise_type: "dual_nback".to_string(),
                    name: "Dual N-Back (3-back)".to_string(),
                    description: "Elite working memory training".to_string(),
                    duration_seconds: 300,
                    instructions: vec![
                        "3-back dual task.".to_string(),
                        "Track visual + audio 3 items back.".to_string(),
                        "This is elite working memory.".to_string(),
                    ],
                    difficulty: 7,
                    metadata: serde_json::json!({"n_level": 3}),
                },
            ]
        }
        "afternoon" => {
            vec![PlannedExercise {
                exercise_type: "custom_combo".to_string(),
                name: "Core Stack Marathon".to_string(),
                description: "Full cognitive training stack".to_string(),
                duration_seconds: 1800,
                instructions: vec![
                    "10 min Feynman (complex topic)".to_string(),
                    "10 min interleaving (4+ topics, timed)".to_string(),
                    "10 min puzzles (increasing difficulty)".to_string(),
                    "AI paces you.".to_string(),
                ],
                difficulty: 7,
                metadata: serde_json::json!({}),
            }]
        }
        "evening" => {
            vec![
                PlannedExercise {
                    exercise_type: "hiit".to_string(),
                    name: "High-Intensity Focus".to_string(),
                    description: "Maximum BDNF protocol".to_string(),
                    duration_seconds: 900,
                    instructions: vec![
                        "15 min HIIT: 30s on/30s off.".to_string(),
                        "Target 85-95% max HR on intervals.".to_string(),
                    ],
                    difficulty: 7,
                    metadata: serde_json::json!({}),
                },
                PlannedExercise {
                    exercise_type: "journaling".to_string(),
                    name: "CBT Reflection".to_string(),
                    description: "Cognitive reframing practice".to_string(),
                    duration_seconds: 300,
                    instructions: vec![
                        "Identify strongest negative thought today.".to_string(),
                        "Apply ABCDE model: Adversity, Belief, Consequence, Dispute, Effect.".to_string(),
                    ],
                    difficulty: 5,
                    metadata: serde_json::json!({}),
                },
            ]
        }
        _ => vec![],
    }
}

fn get_level_5_plan(_week: u8, slot: &str) -> Vec<PlannedExercise> {
    match slot {
        "morning" => {
            vec![
                PlannedExercise {
                    exercise_type: "nsdr".to_string(),
                    name: "Deep NSDR".to_string(),
                    description: "Non-Sleep Deep Rest protocol".to_string(),
                    duration_seconds: 1200,
                    instructions: vec![
                        "Full Non-Sleep Deep Rest protocol.".to_string(),
                        "Guided yoga nidra. Body asleep, mind awake.".to_string(),
                        "Maximum memory consolidation.".to_string(),
                    ],
                    difficulty: 7,
                    metadata: serde_json::json!({"type": "yoga_nidra"}),
                },
                PlannedExercise {
                    exercise_type: "creative_skill".to_string(),
                    name: "Creative Burst".to_string(),
                    description: "Rapid creative expression".to_string(),
                    duration_seconds: 300,
                    instructions: vec![
                        "5 min creative burst: draw, compose, or write from a random prompt.".to_string(),
                        "Process over product.".to_string(),
                    ],
                    difficulty: 5,
                    metadata: serde_json::json!({}),
                },
            ]
        }
        "afternoon" => {
            vec![PlannedExercise {
                exercise_type: "custom_combo".to_string(),
                name: "Cognitive Marathon".to_string(),
                description: "High-intensity multi-technique rotation".to_string(),
                duration_seconds: 2100,
                instructions: vec![
                    "7 min IL (5+ topics) + 7 min FT (complex, timed)".to_string(),
                    "7 min PG (chess + N-Back) + 7 min SPD (speed drills)".to_string(),
                    "7 min DNB (3-back). Rotate every 7 min. No breaks.".to_string(),
                ],
                difficulty: 8,
                metadata: serde_json::json!({}),
            }]
        }
        "evening" => {
            vec![
                PlannedExercise {
                    exercise_type: "exercise_brisk".to_string(),
                    name: "Recovery Exercise".to_string(),
                    description: "AI-selected recovery protocol".to_string(),
                    duration_seconds: 900,
                    instructions: vec![
                        "AI selects today's protocol based on readiness.".to_string(),
                        "Could be HIIT, cold, or Qigong.".to_string(),
                    ],
                    difficulty: 6,
                    metadata: serde_json::json!({"ai_selected": true}),
                },
                PlannedExercise {
                    exercise_type: "journaling".to_string(),
                    name: "Deep Reflection".to_string(),
                    description: "Progress tracking and reframing".to_string(),
                    duration_seconds: 300,
                    instructions: vec![
                        "Write: What improved this week?".to_string(),
                        "What's my biggest cognitive gain?".to_string(),
                        "Where do I still struggle?".to_string(),
                    ],
                    difficulty: 4,
                    metadata: serde_json::json!({}),
                },
            ]
        }
        _ => vec![],
    }
}

fn get_level_6_plan(_week: u8, slot: &str) -> Vec<PlannedExercise> {
    match slot {
        "morning" => {
            vec![
                PlannedExercise {
                    exercise_type: "meditation".to_string(),
                    name: "AI-Blended Meditation".to_string(),
                    description: "Fully adaptive meditation protocol".to_string(),
                    duration_seconds: 1500,
                    instructions: vec![
                        "AI selects optimal blend based on all historical data, HRV, sleep, stress.".to_string(),
                        "Can be any combination of breath focus, body scan, NSDR, QTC, visualization.".to_string(),
                        "Instructions are AI-generated dynamically.".to_string(),
                    ],
                    difficulty: 8,
                    metadata: serde_json::json!({"ai_selected": true, "adaptive": true}),
                },
                PlannedExercise {
                    exercise_type: "dual_nback".to_string(),
                    name: "AI-Novelty Task".to_string(),
                    description: "Advanced cognitive challenge".to_string(),
                    duration_seconds: 600,
                    instructions: vec![
                        "AI selects from: advanced N-Back (4-back+), novel puzzle types, speed processing records.".to_string(),
                        "Always something new.".to_string(),
                    ],
                    difficulty: 9,
                    metadata: serde_json::json!({"ai_selected": true, "n_level": 4}),
                },
            ]
        }
        "afternoon" => {
            vec![PlannedExercise {
                exercise_type: "custom_combo".to_string(),
                name: "Full Booster Stack".to_string(),
                description: "Complete adaptive cognitive training".to_string(),
                duration_seconds: 2400,
                instructions: vec![
                    "AI rotates ALL techniques in optimized order:".to_string(),
                    "IL, FT, PG, DNB, SPD, BBS, RL-Trading.".to_string(),
                    "Pacing and difficulty fully adaptive.".to_string(),
                ],
                difficulty: 9,
                metadata: serde_json::json!({"ai_selected": true, "adaptive": true}),
            }]
        }
        "evening" => {
            vec![PlannedExercise {
                exercise_type: "custom_combo".to_string(),
                name: "Recovery Optimization".to_string(),
                description: "AI-balanced recovery protocol".to_string(),
                duration_seconds: 1200,
                instructions: vec![
                    "AI-balanced protocol: exercise type (HIIT/CE/QTC) based on accumulated fatigue".to_string(),
                    "QS logging + HD logging + CBT/journaling.".to_string(),
                    "Longevity-focused.".to_string(),
                ],
                difficulty: 7,
                metadata: serde_json::json!({"ai_selected": true, "longevity_focused": true}),
            }]
        }
        _ => vec![],
    }
}

pub async fn get_today_sessions(pool: &PgPool, user_id: Uuid) -> Result<Vec<DailySession>> {
    let today = Utc::now().date_naive();
    
    let existing = sqlx::query_as::<_, DailySession>(
        "SELECT * FROM daily_sessions WHERE user_id = $1 AND day_date = $2 ORDER BY slot"
    )
    .bind(user_id)
    .bind(today)
    .fetch_all(pool)
    .await?;
    
    if !existing.is_empty() {
        return Ok(existing);
    }
    
    let current_level = sqlx::query!(
        "SELECT level_number, current_week, current_day FROM user_levels WHERE user_id = $1 AND status = 'active'",
        user_id
    )
    .fetch_one(pool)
    .await?;
    
    let duration_map = match current_level.level_number {
        1 => 900,
        2 => 1000,
        3 => 1200,
        4 => 1400,
        5 => 1600,
        _ => 1800,
    };
    
    for slot in ["morning", "afternoon", "evening"] {
        sqlx::query(
            r#"
            INSERT INTO daily_sessions 
            (user_id, level_number, week_number, day_number, day_date, slot, duration_planned_seconds)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(user_id)
        .bind(current_level.level_number)
        .bind(current_level.current_week)
        .bind(current_level.current_day)
        .bind(today)
        .bind(slot)
        .bind(duration_map)
        .execute(pool)
        .await?;
    }
    
    let sessions = sqlx::query_as::<_, DailySession>(
        "SELECT * FROM daily_sessions WHERE user_id = $1 AND day_date = $2 ORDER BY slot"
    )
    .bind(user_id)
    .bind(today)
    .fetch_all(pool)
    .await?;
    
    Ok(sessions)
}

pub async fn complete_exercise(pool: &PgPool, user_id: Uuid, req: CompleteExerciseRequest) -> Result<ExerciseCompletion> {
    // IDOR Protection: Verify session belongs to user
    let session_check = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM daily_sessions WHERE id = $1 AND user_id = $2)"
    )
    .bind(req.session_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    if !session_check {
        return Err(AppError::NotFound("Session not found or access denied".to_string()));
    }

    let metadata = req.metadata.unwrap_or(serde_json::json!({}));

    let completion = sqlx::query_as::<_, ExerciseCompletion>(
        r#"
        INSERT INTO exercise_completions 
        (session_id, user_id, exercise_type, exercise_name, duration_seconds, 
         accuracy_score, focus_rating, difficulty_level, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
    )
    .bind(req.session_id)
    .bind(user_id)
    .bind(&req.exercise_type)
    .bind(&req.exercise_name)
    .bind(req.duration_seconds)
    .bind(req.accuracy_score)
    .bind(req.focus_rating)
    .bind(req.difficulty_level)
    .bind(metadata)
    .fetch_one(pool)
    .await?;
    
    let session = sqlx::query_as::<_, DailySession>(
        "SELECT * FROM daily_sessions WHERE id = $1"
    )
    .bind(req.session_id)
    .fetch_one(pool)
    .await?;
    
    let exercise_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM exercise_completions WHERE session_id = $1"
    )
    .bind(req.session_id)
    .fetch_one(pool)
    .await?;
    
    let plan = get_daily_plan(session.level_number as u8, session.week_number as u8, &session.slot);
    
    if exercise_count >= plan.len() as i64 {
        sqlx::query(
            "UPDATE daily_sessions SET status = 'completed', completed_at = NOW() WHERE id = $1"
        )
        .bind(req.session_id)
        .execute(pool)
        .await?;
    }

    Ok(completion)
}

pub async fn get_exercise_history(pool: &PgPool, user_id: Uuid, limit: i64) -> Result<Vec<ExerciseCompletion>> {
    let exercises = sqlx::query_as::<_, ExerciseCompletion>(
        r#"
        SELECT * FROM exercise_completions
        WHERE user_id = $1
        ORDER BY completed_at DESC
        LIMIT $2
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(exercises)
}
