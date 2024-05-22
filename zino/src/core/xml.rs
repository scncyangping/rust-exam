//! xml writer
//!
//! # Author
//!
//! - Yapi
//!
//! # Date
//!
//! 2024/05/21

use core::str;

use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename = "response")]
struct XmlResponse<T> {
    property: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename = "property")]
struct XmlRspItem {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    content: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename = "request")]
struct AuthXmlRequest<T> {
    property: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename = "property")]
struct AuthReqItem {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    content: String,
}

impl<T> AuthXmlRequest<T>
where
    T: Serialize,
{
    fn new() -> Self {
        Self {
            property: Vec::new(),
        }
    }
    /// add item
    fn with_item(&mut self, t: T) -> &mut Self {
        self.property.push(t);
        self
    }
    /// to string
    fn to_string(&self) -> Result<String, String> {
        quick_xml::se::to_string(&self).map_err(|e| e.to_string())
    }
    /// convert to xml string
    fn to_xml_string(&self) -> Result<String, String> {
        self.to_string()
            .map(|d| format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>{}", d))
    }
}

pub fn build_from_xml<'a, T>(pa: &'a str) -> Result<T, String>
where
    T: Deserialize<'a>,
{
    from_str::<T>(pa).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use crate::core::xml::build_from_xml;

    use super::{AuthReqItem, AuthXmlRequest, XmlResponse, XmlRspItem};

    #[test]
    fn test_to_xml_serialize() {
        let mut v = AuthXmlRequest::new();
        let x = v
            .with_item(AuthReqItem {
                name: "tag".to_string(),
                content: "body1".to_string(),
            })
            .with_item(AuthReqItem {
                name: "tag2".to_string(),
                content: "body2".to_string(),
            })
            .to_xml_string();

        println!("{}", x.unwrap())
    }

    #[test]
    fn test_xml_de_serialize() {
        let xml: &str = r#"
    <?xml version="1.0" encoding="UTF-8"?>
        <response>
            <property name="title">Programming Rust</property>
            <property name="author">Jim Blandy</property>
        </response>
    "#;
        // 反序列化 XML 数据为 XmlResponse<XmlRspItem> 结构体
        let response: XmlResponse<XmlRspItem> = build_from_xml(xml).unwrap();
        // 输出反序列化后的结构体
        println!("Deserialized response: {:#?}", response);
    }
}
