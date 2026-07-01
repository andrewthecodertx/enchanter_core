use super::context::ToolContext;
use crate::protocol::tools::Tool as ProtocolTool;
use async_trait::async_trait;

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn schema(&self) -> serde_json::Value;

    async fn execute(
        &self,
        arguments: serde_json::Value,
        context: &ToolContext,
    ) -> anyhow::Result<String>;

    fn to_protocol(&self) -> ProtocolTool {
        ProtocolTool::function(self.name(), self.description(), self.schema())
    }
}
