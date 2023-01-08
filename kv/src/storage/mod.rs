mod memory;
use crate::{error::KvError, Kvpair, Value};
pub use memory::MemTable;

pub trait Storage {
    /// 从一个HashTable里获取一个key的value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 从一个HashTable里设置一个key的value 返回旧的value
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    /// 查看HashTable中是否有key
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    /// 删除一个key
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    // 获取所有的kv pair
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    // 遍历hashTable 返回kv pair的iterator
    //fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}
