use std::time::{Duration, Instant};

use crate::service::im::v1::MessageType;

#[derive(Debug, Clone)]
pub struct StreamUpdate {
    pub message_id: String,
    pub content: String,
    throttle: Duration,
    last_flush: Option<Instant>,
}

impl StreamUpdate {
    pub fn new(message_id: impl Into<String>, throttle: Duration) -> Self {
        Self {
            message_id: message_id.into(),
            content: String::new(),
            throttle,
            last_flush: None,
        }
    }

    pub fn push(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn force_due(&mut self) {
        self.last_flush = None;
    }

    pub(super) fn should_flush(&self) -> bool {
        !self.content.is_empty()
            && self
                .last_flush
                .is_none_or(|last| last.elapsed() >= self.throttle)
    }

    pub(super) fn pending_content(&self) -> String {
        self.content.clone()
    }

    pub(super) fn mark_flushed(&mut self) {
        self.last_flush = Some(Instant::now());
    }
}

pub fn text_content(text: &str) -> Result<String, serde_json::Error> {
    serde_json::to_string(&serde_json::json!({ "text": text }))
}

pub fn split_text(text: &str, max_chars: usize) -> Vec<String> {
    let max_chars = max_chars.max(1);
    let mut chunks = Vec::new();
    let mut current = String::new();
    for ch in text.chars() {
        if current.chars().count() >= max_chars {
            chunks.push(std::mem::take(&mut current));
        }
        current.push(ch);
    }
    if !current.is_empty() {
        chunks.push(current);
    }
    chunks
}

pub fn split_markdown(markdown: &str, max_chars: usize) -> Vec<String> {
    let max_chars = max_chars.max(1);
    let mut chunks = Vec::new();
    let mut current = String::new();
    for line in markdown.split_inclusive('\n') {
        if !current.is_empty() && current.chars().count() + line.chars().count() > max_chars {
            chunks.push(std::mem::take(&mut current));
        }
        if line.chars().count() > max_chars {
            chunks.extend(split_text(line, max_chars));
        } else {
            current.push_str(line);
        }
    }
    if !current.is_empty() {
        chunks.push(current);
    }
    chunks
}

pub(super) fn extract_text_content(message_type: &str, content: &str) -> Option<String> {
    if message_type != MessageType::TEXT {
        return None;
    }
    serde_json::from_str::<serde_json::Value>(content)
        .ok()
        .and_then(|value| {
            value
                .get("text")
                .and_then(|text| text.as_str())
                .map(str::to_owned)
        })
        .or_else(|| Some(content.to_string()).filter(|s| !s.is_empty()))
}
