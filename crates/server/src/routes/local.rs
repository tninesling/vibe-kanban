use axum::{
    Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, patch, post},
};
use deployment::Deployment;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use utils::response::ApiResponse;
use uuid::Uuid;

use crate::{DeploymentImpl, error::ApiError};

#[derive(Serialize, Deserialize)]
pub struct LocalProject {
    pub id: String,
    pub organization_id: String,
    pub name: String,
    pub color: String,
    pub sort_order: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct CreateProject {
    pub organization_id: String,
    pub name: String,
    pub color: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateProject {
    pub name: Option<String>,
    pub color: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct LocalIssue {
    pub id: String,
    pub project_id: String,
    pub issue_number: i32,
    pub simple_id: String,
    pub status_id: String,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub sort_order: f64,
    pub parent_issue_id: Option<String>,
    pub parent_issue_sort_order: Option<f64>,
    pub start_date: Option<String>,
    pub target_date: Option<String>,
    pub completed_at: Option<String>,
    pub extension_metadata: Option<String>,
    pub creator_user_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct CreateIssue {
    pub project_id: String,
    pub status_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub sort_order: Option<f64>,
}

#[derive(Deserialize)]
pub struct UpdateIssue {
    pub status_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub sort_order: Option<f64>,
    pub completed_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LocalStatus {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub color: String,
    pub sort_order: i32,
    pub hidden: bool,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct LocalTag {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub color: String,
}

const DEFAULT_STATUSES: &[(&str, &str, &str, i32)] = &[
    (
        "Backlog",
        "#6b7280",
        "00000000-0000-0000-0000-000000000010",
        0,
    ),
    (
        "In Progress",
        "#3b82f6",
        "00000000-0000-0000-0000-000000000011",
        1,
    ),
    (
        "Review",
        "#f59e0b",
        "00000000-0000-0000-0000-000000000012",
        2,
    ),
    ("Done", "#22c55e", "00000000-0000-0000-0000-000000000013", 3),
];

const DEFAULT_TAGS: &[(&str, &str, &str)] = &[
    ("Bug", "#ef4444", "00000000-0000-0000-0000-000000000020"),
    ("Feature", "#22c55e", "00000000-0000-0000-0000-000000000021"),
    (
        "Enhancement",
        "#8b5cf6",
        "00000000-0000-0000-0000-000000000022",
    ),
];

fn project_from_row(row: &sqlx::sqlite::SqliteRow) -> LocalProject {
    LocalProject {
        id: row.get("id"),
        organization_id: row.get("organization_id"),
        name: row.get("name"),
        color: row.get("color"),
        sort_order: row.get("sort_order"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn issue_from_row(row: &sqlx::sqlite::SqliteRow) -> LocalIssue {
    LocalIssue {
        id: row.get("id"),
        project_id: row.get("project_id"),
        issue_number: row.get("issue_number"),
        simple_id: row.get("simple_id"),
        status_id: row.get("status_id"),
        title: row.get("title"),
        description: row.get("description"),
        priority: row.get("priority"),
        sort_order: row.get("sort_order"),
        parent_issue_id: row.get("parent_issue_id"),
        parent_issue_sort_order: row.get("parent_issue_sort_order"),
        start_date: row.get("start_date"),
        target_date: row.get("target_date"),
        completed_at: row.get("completed_at"),
        extension_metadata: row.get("extension_metadata"),
        creator_user_id: row.get("creator_user_id"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

fn status_from_row(row: &sqlx::sqlite::SqliteRow) -> LocalStatus {
    let hidden_int: i32 = row.get("hidden");
    LocalStatus {
        id: row.get("id"),
        project_id: row.get("project_id"),
        name: row.get("name"),
        color: row.get("color"),
        sort_order: row.get("sort_order"),
        hidden: hidden_int != 0,
        created_at: row.get("created_at"),
    }
}

fn tag_from_row(row: &sqlx::sqlite::SqliteRow) -> LocalTag {
    LocalTag {
        id: row.get("id"),
        project_id: row.get("project_id"),
        name: row.get("name"),
        color: row.get("color"),
    }
}

pub fn router() -> Router<DeploymentImpl> {
    Router::new()
        .route("/local/projects", get(list_projects))
        .route("/local/projects", post(create_project))
        .route("/local/projects/{id}", get(get_project))
        .route("/local/projects/{id}", patch(update_project))
        .route("/local/projects/{id}", delete(delete_project))
        .route("/local/projects/{id}/issues", get(list_issues))
        .route("/local/projects/{id}/issues", post(create_issue))
        .route("/local/issues/{id}", patch(update_issue))
        .route("/local/issues/{id}", delete(delete_issue))
        .route("/local/projects/{id}/statuses", get(list_statuses))
        .route("/local/statuses", post(create_status))
        .route("/local/statuses/{id}", patch(update_status))
        .route("/local/statuses/{id}", delete(delete_status))
        .route("/local/projects/{id}/tags", get(list_tags))
        .route("/local/projects/{id}/tags", post(create_tag))
        .route("/local/issues/bulk", post(bulk_update_issues))
        .route("/local/issues/{id}/tags", get(list_issue_tags))
        .route("/local/issue-tags", post(create_issue_tag))
        .route("/local/issue-tags/{id}", delete(delete_issue_tag))
        .route(
            "/local/issue-tags/by-project/{project_id}",
            get(list_project_issue_tags),
        )
}

async fn ensure_default_statuses(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    project_id: &str,
) -> Result<(), sqlx::Error> {
    let existing_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM local_statuses WHERE project_id = ?1")
            .bind(project_id)
            .fetch_one(pool)
            .await?;
    if existing_count > 0 {
        return Ok(());
    }
    for (name, color, _id, sort_order) in DEFAULT_STATUSES {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO local_statuses (id, project_id, name, color, sort_order, hidden, created_at) VALUES (?1, ?2, ?3, ?4, ?5, 0, datetime('now'))"
        )
        .bind(&id)
        .bind(project_id)
        .bind(name)
        .bind(color)
        .bind(sort_order)
        .execute(pool)
        .await?;
    }
    Ok(())
}

async fn ensure_default_tags(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    project_id: &str,
) -> Result<(), sqlx::Error> {
    let existing_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM local_tags WHERE project_id = ?1")
            .bind(project_id)
            .fetch_one(pool)
            .await?;
    if existing_count > 0 {
        return Ok(());
    }
    for (name, color, _id) in DEFAULT_TAGS {
        let id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO local_tags (id, project_id, name, color) VALUES (?1, ?2, ?3, ?4)")
            .bind(&id)
            .bind(project_id)
            .bind(name)
            .bind(color)
            .execute(pool)
            .await?;
    }
    Ok(())
}

async fn list_projects(
    State(deployment): State<DeploymentImpl>,
    params: Query<serde_json::Value>,
) -> Result<Json<ApiResponse<Vec<LocalProject>>>, ApiError> {
    let org_id = params
        .get("organization_id")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Seed a default project on first request if the table is empty
    let count: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM local_projects")
        .fetch_one(&deployment.db().pool)
        .await?;
    if count == 0 {
        seed_project(&deployment, "00000000-0000-0000-0000-000000000002").await?;
    }

    let rows = sqlx::query(
        "SELECT id, organization_id, name, color, sort_order, created_at, updated_at FROM local_projects WHERE organization_id = ?1 ORDER BY sort_order"
    )
    .bind(org_id)
    .fetch_all(&deployment.db().pool)
    .await?
    .iter()
    .map(project_from_row)
    .collect();

    Ok(Json(ApiResponse::success(rows)))
}

async fn seed_project(deployment: &DeploymentImpl, project_id: &str) -> Result<(), ApiError> {
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT OR IGNORE INTO local_projects (id, organization_id, name, color, sort_order, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, 0, ?5, ?5)"
    )
    .bind(project_id)
    .bind("00000000-0000-0000-0000-000000000001")
    .bind("Local Dev Project")
    .bind("#f59e0b")
    .bind(&now)
    .execute(&deployment.db().pool)
    .await?;
    ensure_default_statuses(&deployment.db().pool, project_id).await?;
    ensure_default_tags(&deployment.db().pool, project_id).await?;
    Ok(())
}

async fn create_project(
    State(deployment): State<DeploymentImpl>,
    Json(req): axum::Json<CreateProject>,
) -> Result<Json<ApiResponse<LocalProject>>, ApiError> {
    let id = Uuid::new_v4().to_string();
    let color = req.color.unwrap_or_else(|| "#6b7280".to_string());
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO local_projects (id, organization_id, name, color, sort_order, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, (SELECT COALESCE(MAX(sort_order), -1) + 1 FROM local_projects WHERE organization_id = ?2), ?5, ?5)"
    )
    .bind(&id)
    .bind(&req.organization_id)
    .bind(&req.name)
    .bind(&color)
    .bind(&now)
    .execute(&deployment.db().pool)
    .await?;

    ensure_default_statuses(&deployment.db().pool, &id).await?;
    ensure_default_tags(&deployment.db().pool, &id).await?;

    let row = sqlx::query(
        "SELECT id, organization_id, name, color, sort_order, created_at, updated_at FROM local_projects WHERE id = ?1"
    )
    .bind(&id)
    .fetch_one(&deployment.db().pool)
    .await?;

    Ok(Json(ApiResponse::success(project_from_row(&row))))
}

async fn get_project(
    State(deployment): State<DeploymentImpl>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<LocalProject>>, ApiError> {
    let row = sqlx::query(
        "SELECT id, organization_id, name, color, sort_order, created_at, updated_at FROM local_projects WHERE id = ?1"
    )
    .bind(&id)
    .fetch_one(&deployment.db().pool)
    .await?;
    Ok(Json(ApiResponse::success(project_from_row(&row))))
}

async fn update_project(
    State(deployment): State<DeploymentImpl>,
    Path(id): Path<String>,
    Json(req): axum::Json<UpdateProject>,
) -> Result<Json<ApiResponse<LocalProject>>, ApiError> {
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "UPDATE local_projects SET name = COALESCE(?1, name), color = COALESCE(?2, color), sort_order = COALESCE(?3, sort_order), updated_at = ?4 WHERE id = ?5"
    )
    .bind(&req.name)
    .bind(&req.color)
    .bind(req.sort_order)
    .bind(&now)
    .bind(&id)
    .execute(&deployment.db().pool)
    .await?;

    let row = sqlx::query(
        "SELECT id, organization_id, name, color, sort_order, created_at, updated_at FROM local_projects WHERE id = ?1"
    )
    .bind(&id)
    .fetch_one(&deployment.db().pool)
    .await?;

    Ok(Json(ApiResponse::success(project_from_row(&row))))
}

async fn delete_project(
    State(deployment): State<DeploymentImpl>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    sqlx::query("DELETE FROM local_issue_tags WHERE issue_id IN (SELECT id FROM local_issues WHERE project_id = ?1)")
        .bind(&id)
        .execute(&deployment.db().pool)
        .await?;
    sqlx::query("DELETE FROM local_issues WHERE project_id = ?1")
        .bind(&id)
        .execute(&deployment.db().pool)
        .await?;
    sqlx::query("DELETE FROM local_tags WHERE project_id = ?1")
        .bind(&id)
        .execute(&deployment.db().pool)
        .await?;
    sqlx::query("DELETE FROM local_statuses WHERE project_id = ?1")
        .bind(&id)
        .execute(&deployment.db().pool)
        .await?;
    sqlx::query("DELETE FROM local_projects WHERE id = ?1")
        .bind(&id)
        .execute(&deployment.db().pool)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn list_issues(
    State(deployment): State<DeploymentImpl>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<Vec<LocalIssue>>>, ApiError> {
    let rows = sqlx::query(
        "SELECT id, project_id, issue_number, simple_id, status_id, title, description, priority, sort_order, parent_issue_id, parent_issue_sort_order, start_date, target_date, completed_at, extension_metadata, creator_user_id, created_at, updated_at FROM local_issues WHERE project_id = ?1 ORDER BY sort_order"
    )
    .bind(&id)
    .fetch_all(&deployment.db().pool)
    .await?
    .iter()
    .map(issue_from_row)
    .collect();

    Ok(Json(ApiResponse::success(rows)))
}

async fn create_issue(
    State(deployment): State<DeploymentImpl>,
    Json(req): axum::Json<CreateIssue>,
) -> Result<Json<ApiResponse<LocalIssue>>, ApiError> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    let max_num: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(issue_number), 0) + 1 FROM local_issues WHERE project_id = ?1",
    )
    .bind(&req.project_id)
    .fetch_one(&deployment.db().pool)
    .await?;

    let simple_id = format!("DEV-{max_num}");
    let sort_order = req.sort_order.unwrap_or(max_num as f64);
    let status_id = req
        .status_id
        .unwrap_or_else(|| "00000000-0000-0000-0000-000000000010".to_string());
    let priority = &req.priority;

    sqlx::query(
        "INSERT INTO local_issues (id, project_id, issue_number, simple_id, status_id, title, description, priority, sort_order, parent_issue_id, parent_issue_sort_order, start_date, target_date, completed_at, extension_metadata, creator_user_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?17)"
    )
    .bind(&id)
    .bind(&req.project_id)
    .bind(max_num)
    .bind(&simple_id)
    .bind(&status_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(priority)
    .bind(sort_order)
    .bind(&None::<String>)
    .bind(&None::<f64>)
    .bind(&None::<String>)
    .bind(&None::<String>)
    .bind(&None::<String>)
    .bind(&None::<String>)
    .bind(&None::<String>)
    .bind(&now)
    .execute(&deployment.db().pool)
    .await?;

    let row = sqlx::query(
        "SELECT id, project_id, issue_number, simple_id, status_id, title, description, priority, sort_order, parent_issue_id, parent_issue_sort_order, start_date, target_date, completed_at, extension_metadata, creator_user_id, created_at, updated_at FROM local_issues WHERE id = ?1"
    )
    .bind(&id)
    .fetch_one(&deployment.db().pool)
    .await?;

    Ok(Json(ApiResponse::success(issue_from_row(&row))))
}

async fn update_issue(
    State(deployment): State<DeploymentImpl>,
    Path(id): Path<String>,
    Json(req): axum::Json<UpdateIssue>,
) -> Result<Json<ApiResponse<LocalIssue>>, ApiError> {
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "UPDATE local_issues SET status_id = COALESCE(?1, status_id), title = COALESCE(?2, title), description = COALESCE(?3, description), priority = COALESCE(?4, priority), sort_order = COALESCE(?5, sort_order), completed_at = COALESCE(?6, completed_at), updated_at = ?7 WHERE id = ?8"
    )
    .bind(&req.status_id)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.priority)
    .bind(req.sort_order)
    .bind(&req.completed_at)
    .bind(&now)
    .bind(&id)
    .execute(&deployment.db().pool)
    .await?;

    let row = sqlx::query(
        "SELECT id, project_id, issue_number, simple_id, status_id, title, description, priority, sort_order, parent_issue_id, parent_issue_sort_order, start_date, target_date, completed_at, extension_metadata, creator_user_id, created_at, updated_at FROM local_issues WHERE id = ?1"
    )
    .bind(&id)
    .fetch_one(&deployment.db().pool)
    .await?;

    Ok(Json(ApiResponse::success(issue_from_row(&row))))
}

async fn delete_issue(
    State(deployment): State<DeploymentImpl>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    sqlx::query("DELETE FROM local_issue_tags WHERE issue_id = ?1")
        .bind(&id)
        .execute(&deployment.db().pool)
        .await?;
    sqlx::query("DELETE FROM local_issues WHERE id = ?1")
        .bind(&id)
        .execute(&deployment.db().pool)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn list_statuses(
    State(deployment): State<DeploymentImpl>,
    Path(project_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<LocalStatus>>>, ApiError> {
    // Seed project and statuses if the project doesn't exist
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM local_projects WHERE id = ?1")
        .bind(&project_id)
        .fetch_one(&deployment.db().pool)
        .await?;
    if count == 0 {
        seed_project(&deployment, &project_id).await?;
    } else {
        ensure_default_statuses(&deployment.db().pool, &project_id).await?;
    }
    let rows = sqlx::query(
        "SELECT id, project_id, name, color, sort_order, CAST(hidden AS INTEGER) as hidden, created_at FROM local_statuses WHERE project_id = ?1 ORDER BY sort_order"
    )
    .bind(&project_id)
    .fetch_all(&deployment.db().pool)
    .await?
    .iter()
    .map(status_from_row)
    .collect();

    Ok(Json(ApiResponse::success(rows)))
}

async fn list_tags(
    State(deployment): State<DeploymentImpl>,
    Path(project_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<LocalTag>>>, ApiError> {
    ensure_default_tags(&deployment.db().pool, &project_id).await?;
    let rows =
        sqlx::query("SELECT id, project_id, name, color FROM local_tags WHERE project_id = ?1")
            .bind(&project_id)
            .fetch_all(&deployment.db().pool)
            .await?
            .iter()
            .map(tag_from_row)
            .collect();

    Ok(Json(ApiResponse::success(rows)))
}

async fn create_tag(
    State(deployment): State<DeploymentImpl>,
    Json(req): axum::Json<serde_json::Value>,
) -> Result<Json<ApiResponse<LocalTag>>, ApiError> {
    let id = Uuid::new_v4().to_string();
    let project_id = req.get("project_id").and_then(|v| v.as_str()).unwrap_or("");
    let name = req.get("name").and_then(|v| v.as_str()).unwrap_or("");
    let color = req
        .get("color")
        .and_then(|v| v.as_str())
        .unwrap_or("#6b7280");

    sqlx::query("INSERT INTO local_tags (id, project_id, name, color) VALUES (?1, ?2, ?3, ?4)")
        .bind(&id)
        .bind(project_id)
        .bind(name)
        .bind(color)
        .execute(&deployment.db().pool)
        .await?;

    let row = sqlx::query("SELECT id, project_id, name, color FROM local_tags WHERE id = ?1")
        .bind(&id)
        .fetch_one(&deployment.db().pool)
        .await?;

    Ok(Json(ApiResponse::success(tag_from_row(&row))))
}

#[derive(Serialize, Deserialize)]
pub struct LocalIssueTag {
    pub id: String,
    pub issue_id: String,
    pub tag_id: String,
}

fn issue_tag_from_row(row: &sqlx::sqlite::SqliteRow) -> LocalIssueTag {
    LocalIssueTag {
        id: row.get("id"),
        issue_id: row.get("issue_id"),
        tag_id: row.get("tag_id"),
    }
}

async fn bulk_update_issues(
    State(deployment): State<DeploymentImpl>,
    Json(req): axum::Json<serde_json::Value>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let now = chrono::Utc::now().to_rfc3339();
    if let Some(updates) = req.get("updates").and_then(|v| v.as_array()) {
        for update in updates {
            let id = update.get("id").and_then(|v| v.as_str()).unwrap_or("");
            if id.is_empty() {
                continue;
            }
            if let Some(changes) = update.get("changes") {
                let status_id = changes.get("status_id").and_then(|v| v.as_str());
                let sort_order = changes.get("sort_order").and_then(|v| v.as_f64());
                sqlx::query(
                    "UPDATE local_issues SET status_id = COALESCE(?1, status_id), sort_order = COALESCE(?2, sort_order), updated_at = ?3 WHERE id = ?4"
                )
                .bind(status_id)
                .bind(sort_order)
                .bind(&now)
                .bind(id)
                .execute(&deployment.db().pool)
                .await?;
            }
        }
    }
    Ok(Json(ApiResponse::success(())))
}

async fn list_issue_tags(
    State(deployment): State<DeploymentImpl>,
    Path(issue_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<LocalIssueTag>>>, ApiError> {
    let rows = sqlx::query("SELECT id, issue_id, tag_id FROM local_issue_tags WHERE issue_id = ?1")
        .bind(&issue_id)
        .fetch_all(&deployment.db().pool)
        .await?
        .iter()
        .map(issue_tag_from_row)
        .collect();
    Ok(Json(ApiResponse::success(rows)))
}

async fn create_issue_tag(
    State(deployment): State<DeploymentImpl>,
    Json(req): axum::Json<serde_json::Value>,
) -> Result<Json<ApiResponse<LocalIssueTag>>, ApiError> {
    let id = Uuid::new_v4().to_string();
    let issue_id = req.get("issue_id").and_then(|v| v.as_str()).unwrap_or("");
    let tag_id = req.get("tag_id").and_then(|v| v.as_str()).unwrap_or("");

    sqlx::query("INSERT INTO local_issue_tags (id, issue_id, tag_id) VALUES (?1, ?2, ?3)")
        .bind(&id)
        .bind(issue_id)
        .bind(tag_id)
        .execute(&deployment.db().pool)
        .await?;

    let row = sqlx::query("SELECT id, issue_id, tag_id FROM local_issue_tags WHERE id = ?1")
        .bind(&id)
        .fetch_one(&deployment.db().pool)
        .await?;

    Ok(Json(ApiResponse::success(issue_tag_from_row(&row))))
}

async fn delete_issue_tag(
    State(deployment): State<DeploymentImpl>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    sqlx::query("DELETE FROM local_issue_tags WHERE id = ?1")
        .bind(&id)
        .execute(&deployment.db().pool)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn list_project_issue_tags(
    State(deployment): State<DeploymentImpl>,
    Path(project_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<LocalIssueTag>>>, ApiError> {
    let rows = sqlx::query(
        "SELECT it.id, it.issue_id, it.tag_id FROM local_issue_tags it JOIN local_issues i ON i.id = it.issue_id WHERE i.project_id = ?1"
    )
    .bind(&project_id)
    .fetch_all(&deployment.db().pool)
    .await?
    .iter()
    .map(issue_tag_from_row)
    .collect();
    Ok(Json(ApiResponse::success(rows)))
}

#[derive(Deserialize)]
pub struct CreateStatus {
    pub project_id: String,
    pub name: String,
    pub color: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateStatus {
    pub name: Option<String>,
    pub color: Option<String>,
    pub sort_order: Option<i32>,
    pub hidden: Option<bool>,
}

async fn create_status(
    State(deployment): State<DeploymentImpl>,
    Json(req): axum::Json<CreateStatus>,
) -> Result<Json<ApiResponse<LocalStatus>>, ApiError> {
    let id = Uuid::new_v4().to_string();
    let color = req.color.unwrap_or_else(|| "#6b7280".to_string());
    let sort_order = req.sort_order.unwrap_or(0);
    let now = chrono::Utc::now().to_rfc3339();

    // Seed project if it doesn't exist
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM local_projects WHERE id = ?1")
        .bind(&req.project_id)
        .fetch_one(&deployment.db().pool)
        .await?;
    if count == 0 {
        seed_project(&deployment, &req.project_id).await?;
    }

    sqlx::query(
        "INSERT INTO local_statuses (id, project_id, name, color, sort_order, hidden, created_at) VALUES (?1, ?2, ?3, ?4, ?5, 0, ?6)"
    )
    .bind(&id)
    .bind(&req.project_id)
    .bind(&req.name)
    .bind(&color)
    .bind(sort_order)
    .bind(&now)
    .execute(&deployment.db().pool)
    .await?;

    let row = sqlx::query(
        "SELECT id, project_id, name, color, sort_order, CAST(hidden AS INTEGER) as hidden, created_at FROM local_statuses WHERE id = ?1"
    )
    .bind(&id)
    .fetch_one(&deployment.db().pool)
    .await?;

    Ok(Json(ApiResponse::success(status_from_row(&row))))
}

async fn update_status(
    State(deployment): State<DeploymentImpl>,
    Path(id): Path<String>,
    Json(req): axum::Json<UpdateStatus>,
) -> Result<Json<ApiResponse<LocalStatus>>, ApiError> {
    sqlx::query(
        "UPDATE local_statuses SET name = COALESCE(?1, name), color = COALESCE(?2, color), sort_order = COALESCE(?3, sort_order), hidden = COALESCE(?4, hidden) WHERE id = ?5"
    )
    .bind(&req.name)
    .bind(&req.color)
    .bind(req.sort_order)
    .bind(req.hidden.map(|h| h as i32))
    .bind(&id)
    .execute(&deployment.db().pool)
    .await?;

    let row = sqlx::query(
        "SELECT id, project_id, name, color, sort_order, CAST(hidden AS INTEGER) as hidden, created_at FROM local_statuses WHERE id = ?1"
    )
    .bind(&id)
    .fetch_one(&deployment.db().pool)
    .await?;

    Ok(Json(ApiResponse::success(status_from_row(&row))))
}

async fn delete_status(
    State(deployment): State<DeploymentImpl>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    sqlx::query("DELETE FROM local_statuses WHERE id = ?1")
        .bind(&id)
        .execute(&deployment.db().pool)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
