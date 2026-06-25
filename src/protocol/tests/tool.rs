use serde::Deserialize;
use serde_json::json;

use crate::protocol::tool::{Tool, ToolCall, ToolCallFunction, ToolChoice};

fn weather_tool() -> Tool {
    Tool::function(
        "get_weather",
        "Get the current weather for a location.",
        json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City and country"
                }
            },
            "required": ["location"]
        }),
    )
}

#[test]
fn tool_name_and_description() {
    let tool = weather_tool();
    assert_eq!(tool.name(), "get_weather");
    assert_eq!(
        tool.description(),
        "Get the current weather for a location."
    );
}

#[test]
fn tool_serializes_with_type_tag() {
    let tool = weather_tool();
    let json = serde_json::to_string(&tool).unwrap();
    assert!(json.contains(r#""type":"function""#));
    assert!(json.contains(r#""name":"get_weather""#));
}

#[test]
fn tool_choice_auto_serializes() {
    let json = serde_json::to_string(&ToolChoice::Auto).unwrap();
    assert_eq!(json, r#""auto""#);
}

#[test]
fn tool_choice_none_serializes() {
    let json = serde_json::to_string(&ToolChoice::None).unwrap();
    assert_eq!(json, r#""none""#);
}

#[test]
fn tool_choice_required_serializes() {
    let json = serde_json::to_string(&ToolChoice::Required).unwrap();
    assert_eq!(json, r#""required""#);
}

#[test]
fn tool_choice_force_specific_function() {
    let choice = ToolChoice::force("get_weather");
    let json = serde_json::to_string(&choice).unwrap();
    assert!(json.contains(r#""type":"function""#));
    assert!(json.contains(r#""name":"get_weather""#));
}

#[test]
fn tool_call_parse_arguments() {
    let call = ToolCall {
        id: "call_abc123".to_string(),
        r#type: "function".to_string(),
        function: ToolCallFunction {
            name: "get_weather".to_string(),
            arguments: r#"{"location":"Paris, France"}"#.to_string(),
        },
    };

    #[derive(Deserialize)]
    struct Args {
        location: String,
    }

    let args: Args = call.parse_arguments().unwrap();
    assert_eq!(args.location, "Paris, France");
}

#[test]
fn tool_call_parse_arguments_error() {
    let call = ToolCall {
        id: "call_abc123".to_string(),
        r#type: "function".to_string(),
        function: ToolCallFunction {
            name: "get_weather".to_string(),
            arguments: r#"not valid json"#.to_string(),
        },
    };

    #[derive(Deserialize)]
    struct Args {
        location: String,
    }

    assert!(call.parse_arguments::<Args>().is_err());
}

#[test]
fn roundtrip_tool_call() {
    let original = ToolCall {
        id: "call_xyz".to_string(),
        r#type: "function".to_string(),
        function: ToolCallFunction {
            name: "get_weather".to_string(),
            arguments: r#"{"location":"Tokyo"}"#.to_string(),
        },
    };

    let json = serde_json::to_string(&original).unwrap();
    let restored: ToolCall = serde_json::from_str(&json).unwrap();
    assert_eq!(original, restored);
}
