use crate::errors::{AppError, Result};
use crate::middleware::auth::AuthUser;
use crate::models::user::{UpdateUserRequest, UserResponse, UserSchedule, UpdateScheduleRequest};
use crate::services::auth_service;
use axum::{extract::State, Extension, Json};
use sqlx::PgPool;

pub async fn get_me(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<UserResponse>> {
    let user = auth_service::get_user_by_id(&pool, auth_user.user_id).await?;
    Ok(Json(user))
}

pub async fn update_me(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>> {
    if let Some(ref email) = req.email {
        if !email.contains('@') {
            return Err(AppError::ValidationError("Invalid email format".to_string()));
        }
    }
    
    if let Some(ref chronotype) = req.chronotype {
        let valid_chronotypes: &[&str] = &["morning", "evening", "flexible"];
        if !valid_chronotypes.contains(&chronotype.as_str()) {
            return Err(AppError::ValidationError("Invalid chronotype".to_string()));
        }
    }
    
    if let Some(ref goal) = req.primary_goal {
        let valid_goals: &[&str] = &["focus", "memory", "creativity", "longevity", "performance", "recovery"];
        if !valid_goals.contains(&goal.as_str()) {
            return Err(AppError::ValidationError("Invalid primary goal".to_string()));
        }
    }
    
    let user = sqlx::query_as!(
        UserResponse,
        r#"
        UPDATE users
        SET display_name = COALESCE($1, display_name),
            avatar_url = COALESCE($2, avatar_url),
            chronotype = COALESCE($3, chronotype),
            primary_goal = COALESCE($4, primary_goal),
            timezone = COALESCE($5, timezone),
            updated_at = NOW()
        WHERE id = $6
        RETURNING id, email, display_name, avatar_url, subscription::text as "subscription!",
                  status::text as "status!", chronotype, primary_goal, timezone,
                  onboarding_completed, created_at, updated_at
        "#,
        req.display_name,
        req.avatar_url,
        req.chronotype,
        req.primary_goal,
        req.timezone,
        auth_user.user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(user))
}

pub async fn get_schedule(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<UserSchedule>> {
    let schedule = sqlx::query_as::<_, UserSchedule>(
        "SELECT * FROM user_schedule WHERE user_id = $1"
    )
    .bind(auth_user.user_id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(schedule))
}

pub async fn update_schedule(
    State(pool): State<PgPool>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<UpdateScheduleRequest>,
) -> Result<Json<UserSchedule>> {
    let mut updates = vec![];
    let mut bind_count = 1;

    let mut query_str = "UPDATE user_schedule SET ".to_string();
    
    if req.morning_time.is_some() {
        updates.push(format!("morning_time = ${}", bind_count));
        bind_count += 1;
    }
    if req.afternoon_time.is_some() {
        updates.push(format!("afternoon_time = ${}", bind_count));
        bind_count += 1;
    }
    if req.evening_time.is_some() {
        updates.push(format!("evening_time = ${}", bind_count));
        bind_count += 1;
    }
    if req.notification_enabled.is_some() {
        updates.push(format!("notification_enabled = ${}", bind_count));
        bind_count += 1;
    }
    if req.haptic_enabled.is_some() {
        updates.push(format!("haptic_enabled = ${}", bind_count));
        bind_count += 1;
    }

    updates.push("updated_at = NOW()".to_string());
    query_str.push_str(&updates.join(", "));
    query_str.push_str(&format!(" WHERE user_id = ${} RETURNING *", bind_count));

    let mut query = sqlx::query_as::<_, UserSchedule>(&query_str);
    
    if let Some(morning) = req.morning_time {
        query = query.bind(morning);
    }
    if let Some(afternoon) = req.afternoon_time {
        query = query.bind(afternoon);
    }
    if let Some(evening) = req.evening_time {
        query = query.bind(evening);
    }
    if let Some(notif) = req.notification_enabled {
        query = query.bind(notif);
    }
    if let Some(haptic) = req.haptic_enabled {
        query = query.bind(haptic);
    }
    
    query = query.bind(auth_user.user_id);

    let schedule = query.fetch_one(&pool).await?;

    Ok(Json(schedule))
}
