use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PreMatchTypeEnum {
    Eq,
    Reg,
    #[default]
    Contains,
    NotContains,
    NotEq,
}
