use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct A {
    #[serde(
        rename = "aa",
        serialize_with = "serialize_aa",
        deserialize_with = "deserialize_aa"
    )]
    AA: String,
    BB: i32,
}

// 序列化函数
fn serialize_aa<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(value)
}

// 反序列化函数
fn deserialize_aa<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s)
}

fn main() {
    let instance = A {
        AA: "Hello".to_string(),
        BB: 42,
    };

    // 序列化
    let serialized = serde_json::to_string(&instance).unwrap();
    println!("Serialized: {}", serialized); // 输出: {"aa":"Hello","BB":42}

    // 反序列化
    let json_data = r#"{"a_a":"Hello","BB":42}"#;
    let deserialized: A = serde_json::from_str(json_data).unwrap();
    println!("Deserialized: {:?}", deserialized); // 输出: A { AA: "Hello", BB: 42 }
}
