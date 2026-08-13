use serde_json::{Value, json};

use crate::LarkError;

use super::types::ChannelMention;

pub(super) fn compose_mentions_text_prefix(mentions: &[ChannelMention]) -> String {
    let parts: Vec<_> = mentions
        .iter()
        .filter_map(|mention| {
            let id = mention.id.as_ref().and_then(|id| {
                id.user_id()
                    .or_else(|| id.open_id())
                    .or_else(|| id.union_id())
            })?;
            Some(format!(
                r#"<at user_id="{}">{}</at>"#,
                escape_attr(id),
                mention.name
            ))
        })
        .collect();
    if parts.is_empty() {
        String::new()
    } else {
        format!("{} ", parts.join(" "))
    }
}

pub(super) fn markdown_to_post(
    title: &str,
    markdown: &str,
    mentions: &[ChannelMention],
) -> Result<String, LarkError> {
    let mut content = Vec::new();
    let mention_elements: Vec<_> = mentions
        .iter()
        .filter_map(|mention| {
            let user_id = mention.id.as_ref().and_then(|id| {
                id.user_id()
                    .or_else(|| id.open_id())
                    .or_else(|| id.union_id())
            })?;
            Some(json!({ "tag": "at", "user_id": user_id, "user_name": mention.name }))
        })
        .collect();
    if !mention_elements.is_empty() {
        let mut row = Vec::new();
        for element in mention_elements {
            row.push(element);
            row.push(json!({ "tag": "text", "text": " " }));
        }
        content.push(Value::Array(row));
    }
    content.push(json!([{ "tag": "md", "text": markdown }]));

    Ok(serde_json::to_string(&json!({
        "zh_cn": {
            "title": title,
            "content": content,
        }
    }))?)
}

fn escape_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
