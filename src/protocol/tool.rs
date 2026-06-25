use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionSpec {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Tool {
    Function { function: FunctionSpec },
}

impl Tool {
    pub fn function(
        name: impl Into<String>,
        description: impl Into<String>,
        parameters: serde_json::Value,
    ) -> Self {
        Tool::Function {
            function: FunctionSpec {
                name: name.into(),
                description: description.into(),
                parameters,
            },
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Tool::Function { function } => &function.name,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Tool::Function { function } => &function.description,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolChoice {
    Auto,
    Required,
    None,
    Function(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolChoiceFunction {
    pub name: String,
}

impl ToolChoice {
    pub fn force(name: impl Into<String>) -> Self {
        ToolChoice::Function(name.into())
    }
}

impl Serialize for ToolChoice {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        match self {
            ToolChoice::Auto => s.serialize_str("auto"),
            ToolChoice::Required => s.serialize_str("required"),
            ToolChoice::None => s.serialize_str("none"),
            ToolChoice::Function(name) => {
                let mut map = s.serialize_map(Some(2))?;
                map.serialize_entry("type", "function")?;
                map.serialize_entry("function", &serde_json::json!({ "name": name }))?;
                map.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for ToolChoice {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(d)?;
        match &value {
            serde_json::Value::String(s) => match s.as_str() {
                "auto" => Ok(ToolChoice::Auto),
                "required" => Ok(ToolChoice::Required),
                "none" => Ok(ToolChoice::None),
                other => Err(serde::de::Error::unknown_variant(
                    other,
                    &["auto", "required", "none"],
                )),
            },
            serde_json::Value::Object(map) => {
                let name = map
                    .get("function")
                    .and_then(|f| f.get("name"))
                    .and_then(|n| n.as_str())
                    .ok_or_else(|| serde::de::Error::missing_field("function.name"))?;
                Ok(ToolChoice::Function(name.to_string()))
            }
            _ => Err(serde::de::Error::custom("expected string or object")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: ToolCallFunction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

impl ToolCall {
    pub fn parse_arguments<T>(&self) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_str(&self.function.arguments).map_err(|e| {
            anyhow::anyhow!(
                "failed to parse arguments for '{}': {e}",
                self.function.name
            )
        })
    }
}
