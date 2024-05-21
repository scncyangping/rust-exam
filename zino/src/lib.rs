/// A Universally Unique Identifier (UUID).
pub type Uuid = uuid::Uuid;

/// A JSON value.
pub type JsonValue = serde_json::Value;

/// A JSON key-value type.
pub type Map = serde_json::Map<String, JsonValue>;

/// An owned dynamically typed future.
pub type BoxFuture<'a, T = ()> =
    std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;
