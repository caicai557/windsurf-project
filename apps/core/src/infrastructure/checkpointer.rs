use std::marker::PhantomData;
use std::time::Duration;

use chrono::{DateTime, Utc};
use moka::future::Cache;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Sqlite, SqlitePool, Transaction};
use thiserror::Error;

use crate::domain::workflow::{Action, Transition, WorkflowBehavior, WorkflowRecord, WorkflowStatus};

#[derive(Debug, Error)]
pub enum CheckpointerError {
    #[error("workflow instance {0} not found")]
    NotFound(String),
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("invalid workflow status: {0}")]
    InvalidStatus(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct NewWorkflowInstance<S> {
    pub id: String,
    pub account_id: String,
    pub flow_definition_id: String,
    pub state: S,
    pub status: WorkflowStatus,
}

pub struct Checkpointer<B: WorkflowBehavior> {
    pool: SqlitePool,
    cache: Cache<String, B::State>,
    _behavior: PhantomData<B>,
}

impl<B> Checkpointer<B>
where
    B: WorkflowBehavior,
    B::State: Serialize + DeserializeOwned,
{
    pub fn new(pool: SqlitePool) -> Self {
        let cache = Cache::builder()
            .max_capacity(1024)
            .time_to_live(Duration::from_secs(3600))
            .build();
        Self {
            pool,
            cache,
            _behavior: PhantomData,
        }
    }

    pub async fn bootstrap_instance(
        &self,
        params: NewWorkflowInstance<B::State>,
    ) -> Result<(), CheckpointerError> {
        let state_json = serde_json::to_string(&params.state)?;
        sqlx::query(
            r#"INSERT OR IGNORE INTO workflow_instances
            (id, account_id, flow_definition_id, state, status)
            VALUES (?, ?, ?, ?, ?)"#,
        )
        .bind(&params.id)
        .bind(&params.account_id)
        .bind(&params.flow_definition_id)
        .bind(state_json)
        .bind(params.status.as_str())
        .execute(&self.pool)
        .await?;

        self.cache
            .insert(params.id.clone(), params.state.clone())
            .await;
        Ok(())
    }

    pub async fn process_event(
        &self,
        instance_id: &str,
        event: B::Event,
    ) -> Result<Vec<Action>, CheckpointerError> {
        let mut tx = self.pool.begin().await?;
        let record = self.fetch_record_tx(&mut tx, instance_id).await?;
        let current_state = self.state_from_cache(instance_id, &record).await?;

        let transition = B::compute_next(current_state, event);
        let serialized_state = serde_json::to_string(&transition.next_state)?;

        sqlx::query(
            r#"UPDATE workflow_instances
            SET state = ?, status = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?"#,
        )
        .bind(serialized_state)
        .bind(transition.status.as_str())
        .bind(instance_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        self.cache
            .insert(instance_id.to_string(), transition.next_state.clone())
            .await;

        Ok(transition.actions)
    }

    pub async fn load_record(
        &self,
        instance_id: &str,
    ) -> Result<Option<WorkflowRecord>, CheckpointerError> {
        let row = sqlx::query_as::<_, WorkflowRow>(
            r#"SELECT id, account_id, flow_definition_id, state, status, created_at, updated_at
                FROM workflow_instances WHERE id = ?"#,
        )
        .bind(instance_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(self.row_to_record(row)?))
        } else {
            Ok(None)
        }
    }

    async fn state_from_cache(
        &self,
        instance_id: &str,
        record: &WorkflowRecord,
    ) -> Result<B::State, CheckpointerError> {
        if let Some(state) = self.cache.get(instance_id).await {
            return Ok(state);
        }

        let deserialized = serde_json::from_value(record.state.clone())?;
        self.cache
            .insert(instance_id.to_string(), deserialized.clone())
            .await;
        Ok(deserialized)
    }

    async fn fetch_record_tx(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        instance_id: &str,
    ) -> Result<WorkflowRecord, CheckpointerError> {
        let row = sqlx::query_as::<_, WorkflowRow>(
            r#"SELECT id, account_id, flow_definition_id, state, status, created_at, updated_at
                FROM workflow_instances WHERE id = ?"#,
        )
        .bind(instance_id)
        .fetch_optional(&mut *tx)
        .await?;

        match row {
            Some(row) => self.row_to_record(row),
            None => Err(CheckpointerError::NotFound(instance_id.to_string())),
        }
    }

    fn row_to_record(&self, row: WorkflowRow) -> Result<WorkflowRecord, CheckpointerError> {
        let status = WorkflowStatus::from_str(&row.status)
            .ok_or_else(|| CheckpointerError::InvalidStatus(row.status.clone()))?;
        let state_value: Value = serde_json::from_str(&row.state)?;

        Ok(WorkflowRecord {
            id: row.id,
            account_id: row.account_id,
            flow_definition_id: row.flow_definition_id,
            state: state_value,
            status,
            created_at: DateTime::<Utc>::from_utc(row.created_at, Utc),
            updated_at: DateTime::<Utc>::from_utc(row.updated_at, Utc),
        })
    }
}

#[derive(FromRow)]
struct WorkflowRow {
    id: String,
    account_id: String,
    flow_definition_id: String,
    state: String,
    status: String,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
}
