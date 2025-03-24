use crate::common::em::PreMatchTypeEnum;
use crate::common::string;
use crate::types::AsyncMatchFn;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) fn eq(source: &str, target: &str) -> anyhow::Result<bool> {
    anyhow::Ok(source == target)
}

pub(crate) fn not_eq(source: &str, target: &str) -> anyhow::Result<bool> {
    anyhow::Ok(source != target)
}

pub(crate) fn contains(source: &str, target: &str) -> anyhow::Result<bool> {
    anyhow::Ok(source.contains(target))
}

pub(crate) fn not_contains(source: &str, target: &str) -> anyhow::Result<bool> {
    anyhow::Ok(!source.contains(target))
}

pub(crate) fn regex(source: &str, target: &str) -> anyhow::Result<bool> {
    let re = Regex::new(target)?;
    anyhow::Ok(re.is_match(source))
}

pub fn cmd_string_match(
    match_type: PreMatchTypeEnum,
    node_id: String,
    value: String,
) -> AsyncMatchFn {
    Arc::new(move |data: Arc<RwLock<HashMap<String, String>>>| {
        let match_type = match_type.clone();
        let item_value = value.clone();
        let key = format!("node-{}-cmd-output", node_id);
        Box::pin(async move {
            let data_read = data.read().await;
            let s = match data_read.get(&key) {
                Some(v) => v,
                None => return anyhow::Ok(false), // 确保返回 Result<bool>
            };
            let result = match match_type {
                PreMatchTypeEnum::Eq => eq(s.as_str(), &item_value),
                PreMatchTypeEnum::Reg => string::regex(s.as_str(), &item_value),
                PreMatchTypeEnum::Contains => contains(s.as_str(), &item_value),
                PreMatchTypeEnum::NotContains => not_contains(s.as_str(), &item_value),
                PreMatchTypeEnum::NotEq => not_eq(s.as_str(), &item_value),
            };
            result
        })
    })
}
