use crate::errors::{AppError, Result};
use chrono::{Duration, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningTopic {
    pub id: Uuid,
    pub user_id: Uuid,
    pub topic_name: String,
    pub description: Option<String>,
    pub mastery_percentage: f64,
    pub estimated_mastery_days: i32,
    pub created_at: NaiveDateTime,
    pub lessons: Vec<MicroLesson>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MicroLesson {
    pub id: Uuid,
    pub topic_id: Uuid,
    pub lesson_number: i32,
    pub title: String,
    pub content: String,
    pub duration_minutes: i32,
    pub next_scheduled_at: NaiveDateTime,
    pub completed: bool,
    pub score: Option<f64>,
    pub completed_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize)]
pub struct MasteryPrediction {
    pub estimated_days_remaining: i32,
    pub predicted_mastery_date: NaiveDateTime,
    pub current_pace: String,
    pub daily_minutes_needed: i32,
}

fn get_topic_curriculum(topic_name: &str) -> Vec<(String, String, i32)> {
    let mut curricula: HashMap<&str, Vec<(String, String, i32)>> = HashMap::new();
    
    curricula.insert("options_trading", vec![
        ("What are Options?".to_string(), "Options are financial derivatives that give buyers the right, but not the obligation, to buy or sell an underlying asset at an agreed-upon price and date. Unlike stocks, options have expiration dates and can be used for hedging, speculation, or income generation. The two basic types are calls (right to buy) and puts (right to sell). Understanding options opens up sophisticated trading strategies that can profit in any market condition.".to_string(), 8),
        ("Calls vs Puts".to_string(), "A call option gives you the right to buy stock at a specific price (strike price) before expiration. You profit when the stock price rises above your strike plus premium paid. A put option gives you the right to sell stock at the strike price. You profit when the stock falls below your strike minus premium. Calls are bullish bets, puts are bearish. Both can be bought or sold (written), creating four basic positions with different risk profiles.".to_string(), 10),
        ("Strike Price & Expiry".to_string(), "The strike price is the agreed-upon price at which the option can be exercised. Options are available at multiple strikes above and below the current stock price. Expiration dates range from weekly to years out (LEAPS). The relationship between strike, stock price, and time to expiration determines the option's value. In-the-money (ITM) options have intrinsic value, at-the-money (ATM) are near the current price, and out-of-the-money (OTM) are pure time value bets.".to_string(), 12),
        ("Intrinsic vs Extrinsic Value".to_string(), "An option's price has two components: intrinsic value and extrinsic (time) value. Intrinsic value is how much the option is ITM (stock price minus strike for calls, strike minus stock price for puts). Extrinsic value is everything else - time until expiration, volatility, interest rates. As expiration approaches, extrinsic value decays to zero (theta decay). Deep ITM options are mostly intrinsic value, OTM options are all extrinsic.".to_string(), 12),
        ("The Greeks: Delta".to_string(), "Delta measures how much an option's price changes for each $1 move in the underlying stock. Call deltas range from 0 to 1.0, put deltas from 0 to -1.0. A delta of 0.50 means the option moves $0.50 for every $1 stock move. Delta also approximates the probability of expiring ITM. ATM options have ~0.50 delta. Delta changes as the stock moves (that's gamma). Understanding delta is crucial for position sizing and risk management.".to_string(), 15),
        ("The Greeks: Gamma, Theta, Vega".to_string(), "Gamma measures how fast delta changes - it's highest for ATM options near expiration. Theta measures time decay - how much value the option loses each day. Short-dated options have higher theta. Vega measures sensitivity to volatility changes - higher volatility increases option prices. Long options are positive vega (benefit from vol increase), short options are negative vega. These Greeks interact: high gamma means unstable delta, high theta means rapid decay.".to_string(), 15),
        ("Covered Calls".to_string(), "A covered call means owning 100 shares of stock and selling a call option against it. You collect premium income but cap your upside at the strike price. If the stock stays below the strike, you keep the premium and the stock. If it rises above, your shares get called away at the strike. This strategy generates income in flat to moderately bullish markets. The premium provides downside cushion but doesn't protect against major drops.".to_string(), 12),
        ("Protective Puts".to_string(), "Buying a put while owning stock creates a protective put (married put). It's portfolio insurance - you pay premium for downside protection. If the stock crashes, your put gains offset stock losses. If the stock rises, you lose the premium but profit on shares. The put's strike determines your protection level. This strategy is expensive but eliminates catastrophic risk. It's like buying insurance on your house - you hope to never use it.".to_string(), 12),
        ("Bull and Bear Spreads".to_string(), "A bull call spread buys a lower strike call and sells a higher strike call. This reduces cost but caps profit. A bear put spread buys a higher strike put and sells a lower strike put. Spreads define your max profit and max loss upfront. They're cheaper than naked options and have better probability of profit. The trade-off is limited upside. Spreads are ideal when you have a directional view but want defined risk.".to_string(), 15),
        ("Iron Condors".to_string(), "An iron condor sells an OTM call spread and an OTM put spread simultaneously. You profit if the stock stays within a range. Maximum profit is the net premium collected. Maximum loss is the spread width minus premium. This is a neutral strategy for low-volatility environments. You want the stock to stay between your short strikes. Time decay works in your favor. Iron condors have high probability of profit but limited reward.".to_string(), 15),
        ("Straddles and Strangles".to_string(), "A straddle buys a call and put at the same strike (usually ATM). You profit from big moves in either direction. A strangle buys OTM call and put at different strikes. Strangles are cheaper but need bigger moves. Both strategies are volatility plays - you're betting on movement, not direction. They're expensive and require significant price swings to overcome the double premium cost. Best used before earnings or major events.".to_string(), 15),
        ("Options Pricing Models".to_string(), "The Black-Scholes model calculates theoretical option prices using stock price, strike, time to expiration, volatility, interest rates, and dividends. It assumes constant volatility and log-normal price distribution. Real markets violate these assumptions, creating pricing inefficiencies. The model gives us the Greeks and implied volatility. Understanding that options are priced on expected future volatility, not past volatility, is key to finding edge.".to_string(), 15),
        ("Implied Volatility".to_string(), "Implied volatility (IV) is the market's expectation of future price movement, derived from option prices. High IV means expensive options, low IV means cheap options. IV expands before events (earnings) and contracts after. IV rank compares current IV to its 52-week range. Buy options when IV is low, sell when IV is high. IV crush after earnings can destroy long option positions even if you're directionally correct.".to_string(), 12),
        ("Risk Management".to_string(), "Never risk more than 1-2% of your account on a single trade. Size positions based on max loss, not max profit. Use stop losses or defined-risk spreads. Avoid holding options through expiration unless you want assignment. Close winners at 50% of max profit. Cut losers at 2x the premium received (for credit spreads). Diversify across time frames and strategies. Track your trades and learn from mistakes. Risk management is what separates professionals from gamblers.".to_string(), 15),
        ("Building a Trading Plan".to_string(), "A trading plan defines your strategy, entry/exit rules, position sizing, and risk parameters. Document your edge: why should this strategy work? Set clear criteria for trade selection. Define maximum position size and portfolio heat. Establish rules for when to trade and when to stay flat. Track every trade with entry reasoning and outcome. Review monthly to identify patterns. A plan removes emotion from decisions and creates consistency. Without a plan, you're gambling.".to_string(), 15),
    ]);

    curricula.insert("python_basics", vec![
        ("Variables & Types".to_string(), "Variables store data in Python. You create them with assignment: x = 5. Python has dynamic typing - the type is inferred from the value. Main types: int (whole numbers), float (decimals), str (text), bool (True/False). Check type with type(x). Variables are case-sensitive. Use descriptive names: user_age not ua. You can assign multiple variables at once: x, y = 1, 2. Understanding types is fundamental to avoiding errors.".to_string(), 8),
        ("Strings & String Methods".to_string(), "Strings are text in quotes: 'hello' or \"hello\". Concatenate with +: 'hello' + ' world'. Access characters with indexing: s[0] is first char. Slice with s[1:4]. Useful methods: .upper(), .lower(), .strip(), .split(), .replace(). F-strings format variables: f'Hello {name}'. Strings are immutable - methods return new strings. Multi-line strings use triple quotes. Raw strings r'' ignore escape characters.".to_string(), 10),
        ("Numbers & Math".to_string(), "Python supports int and float. Operations: + - * / (division always returns float). // is integer division, % is modulo (remainder), ** is power. Order of operations follows PEMDAS. Import math module for advanced functions: math.sqrt(), math.sin(), math.pi. Round with round(x, 2). Convert types: int('5'), float('3.14'), str(42). Be careful with float precision - 0.1 + 0.2 != 0.3 exactly due to binary representation.".to_string(), 10),
        ("Booleans & Conditionals".to_string(), "Booleans are True or False. Comparison operators: == != < > <= >=. Logical operators: and, or, not. If statements: if condition: do_something. Elif for multiple conditions. Else for default case. Indentation matters! Truthy/falsy: 0, empty string, None, empty list are falsy. Everything else is truthy. Ternary operator: x if condition else y. Use is for None checks: if x is None.".to_string(), 12),
        ("Lists & Tuples".to_string(), "Lists are ordered, mutable collections: [1, 2, 3]. Access with index: lst[0]. Negative indexing: lst[-1] is last. Slice: lst[1:3]. Methods: .append(), .extend(), .insert(), .remove(), .pop(). List comprehensions: [x*2 for x in range(10)]. Tuples are immutable: (1, 2, 3). Use tuples for fixed data. Lists for changing data. Both support len(), in operator, iteration.".to_string(), 12),
        ("Dictionaries & Sets".to_string(), "Dictionaries map keys to values: {'name': 'Alice', 'age': 30}. Access: d['name']. Add/update: d['key'] = value. Methods: .keys(), .values(), .items(), .get(). Dict comprehensions: {k: v for k, v in pairs}. Sets are unordered unique elements: {1, 2, 3}. Set operations: union |, intersection &, difference -. Use sets to remove duplicates: set(list). Dicts are fast for lookups.".to_string(), 12),
        ("For Loops & While Loops".to_string(), "For loops iterate over sequences: for item in list: process(item). Range generates numbers: range(10), range(1, 11), range(0, 10, 2). Enumerate adds index: for i, item in enumerate(list). While loops run until condition false: while x < 10: x += 1. Break exits loop early. Continue skips to next iteration. Avoid infinite loops! For loops are preferred when you know iteration count.".to_string(), 12),
        ("Functions & Parameters".to_string(), "Define functions with def: def greet(name): return f'Hello {name}'. Call with greet('Alice'). Parameters can have defaults: def func(x, y=10). *args collects extra positional arguments. **kwargs collects keyword arguments. Return multiple values: return x, y. Functions are first-class objects - can be passed as arguments. Docstrings document functions: '''Description'''. Keep functions focused on one task.".to_string(), 15),
        ("Error Handling".to_string(), "Try/except catches errors: try: risky_code() except ValueError: handle_error(). Catch specific exceptions. Use else for code that runs if no exception. Finally runs regardless: finally: cleanup(). Raise exceptions: raise ValueError('message'). Create custom exceptions by inheriting Exception. Don't catch all exceptions - be specific. Errors are better than silent failures. Use assertions for debugging: assert x > 0.".to_string(), 12),
        ("File I/O".to_string(), "Open files with with open('file.txt', 'r') as f: content = f.read(). Modes: 'r' read, 'w' write (overwrites), 'a' append, 'r+' read/write. Read methods: .read(), .readline(), .readlines(). Write with .write(). With statement auto-closes files. Use pathlib for paths: from pathlib import Path. Check if file exists: Path('file.txt').exists(). Always use with for file operations.".to_string(), 12),
        ("List Comprehensions".to_string(), "List comprehensions create lists concisely: [x**2 for x in range(10)]. Add condition: [x for x in range(10) if x % 2 == 0]. Nested: [x*y for x in range(3) for y in range(3)]. Dict comprehensions: {x: x**2 for x in range(5)}. Set comprehensions: {x**2 for x in range(10)}. Generator expressions use (): (x**2 for x in range(10)) - memory efficient for large sequences. Comprehensions are Pythonic and fast.".to_string(), 12),
        ("Modules & Imports".to_string(), "Import modules: import math, then use math.sqrt(). Import specific items: from math import sqrt. Import with alias: import numpy as np. Create your own modules by saving .py files. __name__ == '__main__' checks if file is run directly. Packages are folders with __init__.py. Standard library has tons of modules: os, sys, datetime, random, json, re. Install external packages with pip. Organize code into modules for reusability.".to_string(), 12),
    ]);

    curricula.insert("machine_learning", vec![
        ("Introduction to ML".to_string(), "Machine Learning is teaching computers to learn patterns from data without explicit programming. Three types: supervised (labeled data), unsupervised (find patterns), reinforcement (learn from rewards). ML powers recommendations, image recognition, natural language processing. Key concepts: training data, features, labels, models, predictions. The goal is generalization - performing well on unseen data. ML is statistics + optimization + computer science.".to_string(), 10),
        ("Supervised Learning Basics".to_string(), "Supervised learning uses labeled examples to learn mappings from inputs to outputs. Classification predicts categories (spam/not spam). Regression predicts continuous values (house prices). Training process: feed examples, compute error, adjust model parameters. Common algorithms: linear regression, logistic regression, decision trees, neural networks. Split data into train/validation/test sets. Overfitting is when model memorizes training data but fails on new data.".to_string(), 12),
        ("Features & Feature Engineering".to_string(), "Features are measurable properties used as model inputs. Good features are informative, independent, and numerous enough. Feature engineering transforms raw data into useful features: scaling (normalize 0-1), encoding (convert categories to numbers), polynomial features (x² from x), interaction features (x*y). Domain knowledge is crucial. More features aren't always better - curse of dimensionality. Feature selection removes irrelevant features. Quality features matter more than complex models.".to_string(), 12),
        ("Linear Regression".to_string(), "Linear regression fits a line to data: y = mx + b. Finds best m (slope) and b (intercept) by minimizing squared errors. Multiple regression uses multiple features: y = w₁x₁ + w₂x₂ + ... + b. Gradient descent iteratively adjusts weights to reduce error. Assumptions: linear relationship, independent errors, constant variance. R² measures fit quality (0-1). Regularization (L1/L2) prevents overfitting by penalizing large weights. Simple but powerful baseline.".to_string(), 15),
        ("Classification & Logistic Regression".to_string(), "Classification assigns inputs to discrete categories. Binary classification has two classes. Logistic regression uses sigmoid function to output probabilities 0-1. Decision boundary separates classes. Threshold (usually 0.5) converts probability to class. Metrics: accuracy, precision, recall, F1-score. Confusion matrix shows true/false positives/negatives. Multi-class uses softmax. Logistic regression is linear classifier - can't learn complex boundaries without feature engineering.".to_string(), 15),
        ("Decision Trees & Random Forests".to_string(), "Decision trees split data based on feature values, creating if-then rules. Each node asks a question, branches are answers, leaves are predictions. Learns non-linear patterns. Prone to overfitting. Random forests combine many trees (ensemble) - each trained on random data subset with random features. Predictions are averaged/voted. More robust than single trees. Feature importance shows which features matter most. Handles mixed data types well.".to_string(), 15),
        ("Neural Networks Fundamentals".to_string(), "Neural networks are layers of connected neurons. Each neuron: weighted sum of inputs + bias, then activation function (ReLU, sigmoid, tanh). Input layer receives features, hidden layers learn representations, output layer makes predictions. Backpropagation computes gradients, optimizer updates weights. Deep learning uses many layers. More layers = more complex patterns. Requires lots of data and compute. Universal function approximators - can learn any pattern given enough capacity.".to_string(), 15),
        ("Training & Optimization".to_string(), "Training minimizes loss function (error measure). Gradient descent: compute gradient, update weights in opposite direction. Learning rate controls step size - too high diverges, too low is slow. Stochastic gradient descent (SGD) uses mini-batches for efficiency. Momentum accelerates convergence. Adam optimizer adapts learning rates per parameter. Epochs are full passes through data. Batch size affects training dynamics. Early stopping prevents overfitting.".to_string(), 15),
        ("Overfitting & Regularization".to_string(), "Overfitting is when model learns training data too well, including noise. Signs: training accuracy high, validation accuracy low. Solutions: more data, simpler model, regularization, dropout, early stopping. L1 regularization (Lasso) encourages sparsity. L2 regularization (Ridge) penalizes large weights. Dropout randomly disables neurons during training. Cross-validation tests generalization. Bias-variance tradeoff: simple models underfit (high bias), complex models overfit (high variance). Sweet spot in middle.".to_string(), 15),
        ("Model Evaluation".to_string(), "Never test on training data! Split: 70% train, 15% validation, 15% test. Validation tunes hyperparameters. Test estimates real-world performance. K-fold cross-validation: split into K parts, train on K-1, test on 1, repeat. Metrics depend on problem: accuracy, precision, recall, F1, AUC-ROC for classification; MSE, MAE, R² for regression. Confusion matrix visualizes errors. Learning curves show if more data helps. Always establish baseline (simple model) first.".to_string(), 12),
        ("Unsupervised Learning".to_string(), "Unsupervised learning finds patterns in unlabeled data. Clustering groups similar items: K-means, hierarchical, DBSCAN. Dimensionality reduction compresses data while preserving structure: PCA, t-SNE, UMAP. Anomaly detection finds outliers. Association rules discover relationships (market basket analysis). Use cases: customer segmentation, data visualization, feature learning. No ground truth - evaluation is subjective. Often preprocessing step for supervised learning.".to_string(), 12),
        ("Real-World ML Pipeline".to_string(), "Production ML: collect data, explore/visualize, clean (handle missing values, outliers), engineer features, split data, train models, tune hyperparameters, evaluate, deploy, monitor. Data quality matters most. Start simple, iterate. Version control data and models. Monitor for data drift (input distribution changes) and concept drift (relationship changes). Retrain periodically. A/B test new models. Document everything. 80% of ML is data work, 20% is modeling.".to_string(), 15),
    ]);

    let normalized = topic_name.to_lowercase().replace(" ", "_");
    
    if let Some(curriculum) = curricula.get(normalized.as_str()) {
        curriculum.clone()
    } else {
        vec![
            ("Introduction & Key Concepts".to_string(), format!("Welcome to {}! This lesson introduces the fundamental concepts and terminology you'll need. We'll cover the basic principles that form the foundation of this topic. Understanding these core ideas will make everything else easier to grasp. Take your time with this foundational material.", topic_name), 8),
            ("Fundamental Principles".to_string(), format!("Now we dive deeper into the core principles of {}. These are the building blocks that everything else is built upon. We'll explore why these principles matter and how they connect to real-world applications.", topic_name), 10),
            ("Core Terminology".to_string(), format!("Every field has its own language. In this lesson, we'll master the essential terminology of {}. Knowing the right words helps you think more clearly and communicate effectively with others in the field.", topic_name), 10),
            ("Basic Techniques".to_string(), format!("Time to get practical! This lesson covers the basic techniques and methods used in {}. We'll start with simple approaches that you can apply immediately. These techniques form your core toolkit.", topic_name), 12),
            ("Intermediate Concepts".to_string(), format!("Building on the basics, we now explore intermediate concepts in {}. These ideas require understanding the fundamentals first. You'll see how different concepts connect and reinforce each other.", topic_name), 12),
            ("Advanced Principles".to_string(), format!("Ready to level up? This lesson tackles advanced principles in {}. These concepts separate beginners from experts. We'll explore nuances and edge cases that matter in real applications.", topic_name), 15),
            ("Common Patterns".to_string(), format!("Experts recognize patterns that beginners miss. This lesson reveals the common patterns in {}. Once you see these patterns, you'll solve problems faster and avoid common pitfalls.", topic_name), 12),
            ("Problem-Solving Approaches".to_string(), format!("How do experts approach problems in {}? This lesson teaches you systematic problem-solving strategies. You'll learn frameworks for breaking down complex challenges into manageable steps.", topic_name), 15),
            ("Real-World Applications".to_string(), format!("Theory meets practice! This lesson shows how {} is applied in the real world. We'll examine case studies, common use cases, and practical considerations. This is where everything clicks together.", topic_name), 15),
            ("Mastery Review & Integration".to_string(), format!("Congratulations on reaching the final lesson! We'll review everything you've learned about {} and see how it all integrates. This lesson helps you consolidate your knowledge and identify areas for continued growth.", topic_name), 12),
        ]
    }
}

pub async fn create_learning_topic(
    pool: &PgPool,
    user_id: Uuid,
    topic_name: String,
    description: Option<String>,
) -> Result<LearningTopic> {
    let topic_id = Uuid::new_v4();
    let curriculum = get_topic_curriculum(&topic_name);
    let lesson_count = curriculum.len();
    
    let estimated_days = match lesson_count {
        1..=5 => 14,
        6..=10 => 30,
        11..=15 => 45,
        _ => 60,
    };
    
    sqlx::query(
        r#"
        INSERT INTO learning_topics (id, user_id, topic_name, description, estimated_mastery_days)
        VALUES ($1, $2, $3, $4, $5)
        "#
    )
    .bind(topic_id)
    .bind(user_id)
    .bind(&topic_name)
    .bind(&description)
    .bind(estimated_days)
    .execute(pool)
    .await?;
    
    let now = Utc::now().naive_utc();
    let mut lessons = Vec::new();

    let intervals = vec![0, 1, 24, 72, 120, 192, 312, 504];

    for (i, (title, content, duration)) in curriculum.iter().enumerate() {
        let lesson_id = Uuid::new_v4();
        let hours_offset = if i < intervals.len() {
            intervals[i]
        } else {
            intervals.last().unwrap() + (i - intervals.len() + 1) as i64 * 312
        };

        let next_scheduled = now + Duration::hours(hours_offset);
        
        sqlx::query(
            r#"
            INSERT INTO micro_lessons 
            (id, topic_id, lesson_number, title, content, duration_minutes, next_scheduled_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(lesson_id)
        .bind(topic_id)
        .bind((i + 1) as i32)
        .bind(title)
        .bind(content)
        .bind(*duration)
        .bind(next_scheduled)
        .execute(pool)
        .await?;
        
        lessons.push(MicroLesson {
            id: lesson_id,
            topic_id,
            lesson_number: (i + 1) as i32,
            title: title.clone(),
            content: content.clone(),
            duration_minutes: *duration,
            next_scheduled_at: next_scheduled,
            completed: false,
            score: None,
            completed_at: None,
        });
    }
    
    Ok(LearningTopic {
        id: topic_id,
        user_id,
        topic_name,
        description,
        mastery_percentage: 0.0,
        estimated_mastery_days: estimated_days,
        created_at: now,
        lessons,
    })
}

pub async fn get_user_topics(pool: &PgPool, user_id: Uuid) -> Result<Vec<LearningTopic>> {
    let topics = sqlx::query!(
        r#"
        SELECT id, user_id, topic_name, description, mastery_percentage, 
               estimated_mastery_days, created_at, is_active
        FROM learning_topics
        WHERE user_id = $1 AND is_active = true
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;
    
    // Optimization: Fetch all lessons in one query to avoid N+1 problem
    let topic_ids: Vec<Uuid> = topics.iter().map(|t| t.id).collect();

    let all_lessons = if !topic_ids.is_empty() {
        sqlx::query_as::<_, MicroLesson>(
            "SELECT * FROM micro_lessons WHERE topic_id = ANY($1) ORDER BY topic_id, lesson_number"
        )
        .bind(&topic_ids)
        .fetch_all(pool)
        .await?
    } else {
        Vec::new()
    };

    // Group lessons by topic_id
    let mut lessons_by_topic: HashMap<Uuid, Vec<MicroLesson>> = HashMap::new();
    for lesson in all_lessons {
        lessons_by_topic.entry(lesson.topic_id).or_insert_with(Vec::new).push(lesson);
    }

    let mut result = Vec::new();

    for topic in topics {
        let lessons = lessons_by_topic.remove(&topic.id).unwrap_or_default();

        result.push(LearningTopic {
            id: topic.id,
            user_id: topic.user_id,
            topic_name: topic.topic_name,
            description: topic.description,
            mastery_percentage: topic.mastery_percentage.to_string().parse::<f64>().unwrap_or(0.0),
            estimated_mastery_days: topic.estimated_mastery_days.unwrap_or(30),
            created_at: topic.created_at.naive_utc(),
            lessons,
        });
    }

    Ok(result)
}

pub async fn get_next_lesson(pool: &PgPool, user_id: Uuid) -> Result<Option<MicroLesson>> {
    let now = Utc::now().naive_utc();
    
    let lesson = sqlx::query_as::<_, MicroLesson>(
        r#"
        SELECT ml.* FROM micro_lessons ml
        JOIN learning_topics lt ON ml.topic_id = lt.id
        WHERE lt.user_id = $1 
        AND lt.is_active = true
        AND ml.completed = false
        AND ml.next_scheduled_at <= $2
        ORDER BY ml.next_scheduled_at ASC
        LIMIT 1
        "#
    )
    .bind(user_id)
    .bind(now)
    .fetch_optional(pool)
    .await?;
    
    Ok(lesson)
}

pub async fn complete_lesson(
    pool: &PgPool,
    user_id: Uuid,
    lesson_id: Uuid,
    score: f64,
) -> Result<MicroLesson> {
    let now = Utc::now().naive_utc();

    // IDOR Protection: Verify lesson belongs to user
    let _lesson = sqlx::query_as::<_, MicroLesson>(
        r#"
        SELECT ml.* FROM micro_lessons ml
        JOIN learning_topics lt ON ml.topic_id = lt.id
        WHERE ml.id = $1 AND lt.user_id = $2
        "#
    )
    .bind(lesson_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Lesson not found or access denied".to_string()))?;

    // Update the lesson
    let lesson = sqlx::query_as::<_, MicroLesson>(
        "SELECT * FROM micro_lessons WHERE id = $1"
    )
    .bind(lesson_id)
    .fetch_one(pool)
    .await?;
    
    if score < 70.0 {
        sqlx::query(
            "UPDATE micro_lessons SET next_scheduled_at = $1 WHERE id = $2"
        )
        .bind(now + Duration::hours(4))
        .bind(lesson_id)
        .execute(pool)
        .await?;
    } else {
        sqlx::query(
            r#"
            UPDATE micro_lessons 
            SET completed = true, score = $1, completed_at = $2
            WHERE id = $3
            "#
        )
        .bind(score)
        .bind(now)
        .bind(lesson_id)
        .execute(pool)
        .await?;
        
        let stats = sqlx::query!(
            r#"
            SELECT 
                COUNT(*) as total,
                COUNT(CASE WHEN completed THEN 1 END) as completed_count
            FROM micro_lessons
            WHERE topic_id = $1
            "#,
            lesson.topic_id
        )
        .fetch_one(pool)
        .await?;
        
        let mastery = if stats.total.unwrap_or(0) > 0 {
            (stats.completed_count.unwrap_or(0) as f64 / stats.total.unwrap_or(1) as f64) * 100.0
        } else {
            0.0
        };
        
        sqlx::query(
            "UPDATE learning_topics SET mastery_percentage = $1 WHERE id = $2"
        )
        .bind(mastery)
        .bind(lesson.topic_id)
        .execute(pool)
        .await?;
    }
    
    let updated = sqlx::query_as::<_, MicroLesson>(
        "SELECT * FROM micro_lessons WHERE id = $1"
    )
    .bind(lesson_id)
    .fetch_one(pool)
    .await?;
    
    Ok(updated)
}

pub async fn get_mastery_prediction(
    pool: &PgPool,
    user_id: Uuid,
    topic_id: Uuid,
) -> Result<MasteryPrediction> {
    // IDOR Protection: Verify topic belongs to user
    let topic_check = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM learning_topics WHERE id = $1 AND user_id = $2)"
    )
    .bind(topic_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    if !topic_check {
        return Err(AppError::NotFound("Topic not found or access denied".to_string()));
    }

    let stats = sqlx::query!(
        r#"
        SELECT
            COUNT(*) as total,
            COUNT(CASE WHEN completed THEN 1 END) as completed_count,
            AVG(CASE WHEN completed THEN score END) as avg_score
        FROM micro_lessons
        WHERE topic_id = $1
        "#,
        topic_id
    )
    .fetch_one(pool)
    .await?;
    
    let total = stats.total.unwrap_or(0) as i32;
    let completed = stats.completed_count.unwrap_or(0) as i32;
    let remaining = total - completed;
    let avg_score = stats.avg_score
        .map(|bd| bd.to_string().parse::<f64>().unwrap_or(75.0))
        .unwrap_or(75.0);

    let pace = if avg_score >= 90.0 { "ahead" } else if avg_score >= 75.0 { "on_track" } else { "behind" };
    let days_per_lesson = if avg_score >= 90.0 { 2 } else if avg_score >= 75.0 { 3 } else { 4 };
    let estimated_days_remaining = remaining * days_per_lesson;
    let daily_minutes = if remaining > 0 { 15 } else { 0 };
    
    Ok(MasteryPrediction {
        estimated_days_remaining,
        predicted_mastery_date: Utc::now().naive_utc() + Duration::days(estimated_days_remaining as i64),
        current_pace: pace.to_string(),
        daily_minutes_needed: daily_minutes,
    })
}
