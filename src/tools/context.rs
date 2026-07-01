use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

pub struct ToolContext {
    pub http: Arc<reqwest::Client>,

    extensions: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ToolContext {
    pub fn new() -> Self {
        Self {
            http: Arc::new(reqwest::Client::new()),
            extensions: HashMap::new(),
        }
    }

    pub fn with_client(client: reqwest::Client) -> Self {
        Self {
            http: Arc::new(client),
            extensions: HashMap::new(),
        }
    }

    pub fn insert<T: Any + Send + Sync>(&mut self, value: T) {
        self.extensions.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn get<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.extensions
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref())
    }

    pub fn get_mut<T: Any + Send + Sync>(&mut self) -> Option<&mut T> {
        self.extensions
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut())
    }
}

impl Default for ToolContext {
    fn default() -> Self {
        Self::new()
    }
}
