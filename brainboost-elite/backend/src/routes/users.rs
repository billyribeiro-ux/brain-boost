use crate::errors::Result;
use crate::middleware::auth::AuthUser;
use crate::models::user::{UpdateProfileRequest, UpdateScheduleRequest, UserResponse, UserSchedule};
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
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<UserResponse>> {
    let mut updates = vec![];
    let mut values: Vec<String> = vec![];
    let mut param_count = 1;

    if let Some(display_name) = req.display_name {
        updates.push(format!("display_name = ${}", param_count));
        values.push(display_name);
        param_count += 1;
    }

    if let Some(avatar_url) = req.avatar_url {
        updates.push(format!("avatar_url = ${}", param_count));
        values.push(avatar_url);
        param_count += 1;
    }

    if let Some(chronotype) = req.chronotype {
        updates.push(format!("chronotype = ${}", param_count));
        values.push(chronotype);
        param_count += 1;
    }

    if let Some(primary_goal) = req.primary_goal {
        updates.push(format!("primary_goal = ${}", param_count));
        values.push(primary_goal);
        param_count += 1;
    }

    if let Some(timezone) = req.timezone {
        updates.push(format!("timezone = ${}", param_count));
        values.push(timezone);
        param_count += 1;
    }

    updates.push("updated_at = NOW()".to_string());

    let query = format!(
        "UPDATE users SET {} WHERE id = ${} RETURNING *",
        updates.join(", "),
        param_count
    );

    let mut query_builder = sqlx::query_as::<_, crate::models::user::User>(&query);
    for value in values {
        query_builder = query_builder.bind(value);
    }
    query_builder = query_builder.bind(auth_user.user_id);

    let user = query_builder.fetch_one(&pool).await?;

    Ok(Json(user.into()))
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
