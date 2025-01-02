use chrono::{DateTime, Local};
use sea_orm::sea_query::ConditionExpression;
use sea_orm::{
    ActiveModelTrait, ActiveValue, DbConn, DbErr, EntityTrait, IdenStatic, IntoActiveModel,
    PaginatorTrait, Select, SelectModel, Value,
};
use sea_orm::{Iterable, QueryFilter};
use serde::{Deserialize, Serialize};
use std::cmp::max;

pub(crate) struct SeaRepo;
impl SeaRepo {
    #[allow(dead_code)]
    pub async fn page_with_default<E>(
        db: &DbConn,
        pg: (u64, u64),
        ces: Option<Vec<ConditionExpression>>,
    ) -> anyhow::Result<(u64, Vec<E::Model>)>
        where
            E: EntityTrait,
            Select<E>: for<'a> PaginatorTrait<'a, DbConn, Selector = SelectModel<E::Model>>,
    {
        let mut ens = E::find();
        if let Some(ft) = ces {
            for exp in ft {
                match exp {
                    ConditionExpression::Condition(cond) => ens = ens.filter(cond),
                    ConditionExpression::SimpleExpr(sim) => ens = ens.filter(sim),
                }
            }
        }

        let ens = ens.paginate(db, pg.1);

        let count = ens.num_items().await?;
        let res = ens.fetch_page(max(pg.0 - 1, 0)).await?;
        anyhow::Ok((count, res))
    }
    #[allow(dead_code)]
    pub async fn update_with_default<E>(
        db: &DbConn,
        mut model: E::ActiveModel,
    ) -> anyhow::Result<E::Model>
        where
            E: EntityTrait,
            E::Model: IntoActiveModel<E::ActiveModel>,
            <E as EntityTrait>::ActiveModel: Send,
    {
        E::Column::iter().for_each(|e| {
            if FIELD_UPDATED_AT == e.as_str() && model.get(e).is_not_set() {
                model.set(
                    e,
                    Value::ChronoDateTimeLocal(Some(Box::new(default_time()))),
                )
            }
        });
        anyhow::Ok(model.update(db).await?)
    }

    /// Inserts an ActiveModel instance into the database.
    #[allow(dead_code)]
    pub async fn insert_with_default<E, D>(db: &DbConn, data: D) -> anyhow::Result<String>
        where
            E: EntityTrait,
            E::Model: IntoActiveModel<E::ActiveModel>,
            D: Serialize,
            for<'de> <E as EntityTrait>::Model: Deserialize<'de>,
            <E as EntityTrait>::ActiveModel: Send,
    {
        let mut id = String::new();
        let mut model = E::ActiveModel::from_json(serde_json::to_value(data)?)?;
        E::Column::iter().for_each(|e| match e.as_str() {
            FIELD_ID => match model.get(e) {
                ActiveValue::Set(value) => {
                    id = value.to_string();
                }
                ActiveValue::Unchanged(value) => {
                    id = value.to_string();
                }
                ActiveValue::NotSet => {
                    id = default_id();
                    model.set(e, Value::String(Some(Box::new(id.clone()))))
                }
            },
            FIELD_CREATED_AT | FIELD_UPDATED_AT => {
                if model.get(e).is_not_set() {
                    model.set(
                        e,
                        Value::ChronoDateTimeLocal(Some(Box::new(default_time()))),
                    )
                }
            }
            _ => {}
        });
        match model.insert(db).await {
            Ok(_) => Ok(id),
            // Optional: handle specific case gracefully
            Err(DbErr::RecordNotInserted) => Ok(id),
            Err(e) => anyhow::bail!(e),
        }
    }

    /// Convert to sea_orm model
    #[allow(dead_code)]
    pub fn convert_to_model<E, D>(data: D) -> anyhow::Result<E::Model>
        where
            E: EntityTrait,
            D: Serialize,
            for<'de> <E as EntityTrait>::Model: Deserialize<'de>,
    {
        let vl = serde_json::to_value(data)?;
        let data: E::Model = serde_json::from_value(vl)?;
        anyhow::Ok(data)
    }
}

const FIELD_ID: &str = "id";
const FIELD_CREATED_AT: &str = "created_at";
const FIELD_UPDATED_AT: &str = "updated_at";

fn default_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
fn default_time() -> DateTime<Local> {
    Local::now()
}
