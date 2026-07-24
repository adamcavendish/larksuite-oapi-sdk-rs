use serde_json::{Map, Value, json};

use crate::LarkError;

use super::types::{ChannelMention, ChannelResource};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct NormalizedContent {
    pub text: Option<String>,
    pub resources: Vec<ChannelResource>,
}

pub(super) fn parse_content(message_type: &str, content: &str) -> NormalizedContent {
    if message_type == "merge_forward" {
        return NormalizedContent {
            text: (!content.is_empty()).then(|| content.to_string()),
            resources: Vec::new(),
        };
    }

    let Ok(value) = serde_json::from_str::<Value>(content) else {
        return NormalizedContent {
            text: Some("[unsupported message]".into()),
            resources: Vec::new(),
        };
    };
    let Some(map) = value.as_object() else {
        return NormalizedContent {
            text: Some("[unsupported message]".into()),
            resources: Vec::new(),
        };
    };

    match message_type {
        "text" => text(map),
        "image" => image(map),
        "file" => file(map),
        "folder" => keyed_tag(map, "folder", "folder", false),
        "audio" => audio(map),
        "media" | "video" => video(map),
        "sticker" => sticker(map),
        "hongbao" => hongbao(map),
        "location" => location(map),
        "share_chat" => simple_id_tag(map, "chat_id", "group_card"),
        "share_user" => simple_id_tag(map, "user_id", "contact_card"),
        "system" => system(map),
        "vote" => vote(map),
        "video_chat" => video_chat(map),
        "calendar" | "general_calendar" | "share_calendar_event" => calendar(message_type, map),
        "todo" => todo(map),
        "post" => post(map),
        "interactive" => NormalizedContent {
            text: Some("[interactive card]".into()),
            resources: Vec::new(),
        },
        _ => NormalizedContent {
            text: Some("[unsupported message]".into()),
            resources: Vec::new(),
        },
    }
}

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

fn text(map: &Map<String, Value>) -> NormalizedContent {
    NormalizedContent {
        text: string_field(map, "text"),
        resources: Vec::new(),
    }
}

fn image(map: &Map<String, Value>) -> NormalizedContent {
    if let Some(image_key) = string_field(map, "image_key") {
        NormalizedContent {
            text: Some(format!("![image]({image_key})")),
            resources: vec![ChannelResource::new("image", image_key)],
        }
    } else {
        NormalizedContent {
            text: Some("[image]".into()),
            resources: Vec::new(),
        }
    }
}

fn file(map: &Map<String, Value>) -> NormalizedContent {
    keyed_tag(map, "file", "file", true)
}

fn keyed_tag(
    map: &Map<String, Value>,
    fallback: &str,
    tag: &str,
    include_resource: bool,
) -> NormalizedContent {
    let Some(file_key) = string_field(map, "file_key") else {
        return NormalizedContent {
            text: Some(format!("[{fallback}]")),
            resources: Vec::new(),
        };
    };
    let file_name = string_field(map, "file_name").unwrap_or_default();
    let name_attr = if file_name.is_empty() {
        String::new()
    } else {
        format!(r#" name="{}""#, escape_attr(&file_name))
    };
    let mut resources = Vec::new();
    if include_resource {
        resources.push(ChannelResource {
            resource_type: tag.into(),
            file_key: file_key.clone(),
            file_name,
            duration_ms: None,
            cover_image_key: String::new(),
        });
    }
    NormalizedContent {
        text: Some(format!(r#"<{tag} key="{file_key}"{name_attr}/>"#)),
        resources,
    }
}

fn audio(map: &Map<String, Value>) -> NormalizedContent {
    let Some(file_key) = string_field(map, "file_key") else {
        return NormalizedContent {
            text: Some("[audio]".into()),
            resources: Vec::new(),
        };
    };
    let duration_ms = int_field(map, "duration").map(|v| v as i32);
    let duration_attr = duration_ms
        .and_then(format_duration)
        .map(|duration| format!(r#" duration="{duration}""#))
        .unwrap_or_default();
    NormalizedContent {
        text: Some(format!(r#"<audio key="{file_key}"{duration_attr}/>"#)),
        resources: vec![ChannelResource {
            resource_type: "audio".into(),
            file_key,
            file_name: String::new(),
            duration_ms,
            cover_image_key: String::new(),
        }],
    }
}

fn video(map: &Map<String, Value>) -> NormalizedContent {
    let Some(file_key) = string_field(map, "file_key") else {
        return NormalizedContent {
            text: Some("[video]".into()),
            resources: Vec::new(),
        };
    };
    let file_name = string_field(map, "file_name").unwrap_or_default();
    let cover_image_key = string_field(map, "image_key").unwrap_or_default();
    let duration_ms = int_field(map, "duration").map(|v| v as i32);
    let mut attrs = String::new();
    if !file_name.is_empty() {
        attrs.push_str(&format!(r#" name="{}""#, escape_attr(&file_name)));
    }
    if let Some(duration) = duration_ms.and_then(format_duration) {
        attrs.push_str(&format!(r#" duration="{duration}""#));
    }
    NormalizedContent {
        text: Some(format!(r#"<video key="{file_key}"{attrs}/>"#)),
        resources: vec![ChannelResource {
            resource_type: "video".into(),
            file_key,
            file_name,
            duration_ms,
            cover_image_key,
        }],
    }
}

fn sticker(map: &Map<String, Value>) -> NormalizedContent {
    if let Some(file_key) = string_field(map, "file_key") {
        NormalizedContent {
            text: Some(format!(r#"<sticker key="{file_key}"/>"#)),
            resources: vec![ChannelResource::new("sticker", file_key)],
        }
    } else {
        NormalizedContent {
            text: Some("[sticker]".into()),
            resources: Vec::new(),
        }
    }
}

fn hongbao(map: &Map<String, Value>) -> NormalizedContent {
    let attr = string_field(map, "text")
        .filter(|s| !s.is_empty())
        .map(|text| format!(r#" text="{}""#, escape_attr(&text)))
        .unwrap_or_default();
    NormalizedContent {
        text: Some(format!("<hongbao{attr}/>")),
        resources: Vec::new(),
    }
}

fn location(map: &Map<String, Value>) -> NormalizedContent {
    let mut attr = String::new();
    if let Some(name) = string_field(map, "name").filter(|s| !s.is_empty()) {
        attr.push_str(&format!(r#" name="{}""#, escape_attr(&name)));
    }
    if let (Some(lat), Some(lng)) = (
        string_field(map, "latitude"),
        string_field(map, "longitude"),
    ) {
        attr.push_str(&format!(r#" coords="lat:{lat},lng:{lng}""#));
    }
    NormalizedContent {
        text: Some(format!("<location{attr}/>")),
        resources: Vec::new(),
    }
}

fn simple_id_tag(map: &Map<String, Value>, field: &str, tag: &str) -> NormalizedContent {
    NormalizedContent {
        text: Some(format!(
            r#"<{tag} id="{}"/>"#,
            string_field(map, field).unwrap_or_default()
        )),
        resources: Vec::new(),
    }
}

fn system(map: &Map<String, Value>) -> NormalizedContent {
    let Some(template) = string_field(map, "template").filter(|s| !s.is_empty()) else {
        return NormalizedContent {
            text: Some("[system message]".into()),
            resources: Vec::new(),
        };
    };
    let mut out = template;
    for (key, value) in map {
        if key == "template" {
            continue;
        }
        let replacement = if let Some(s) = value.as_str() {
            s.to_string()
        } else if let Some(items) = value.as_array() {
            items
                .iter()
                .map(value_to_string)
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            value_to_string(value)
        };
        out = out.replace(&format!("{{{key}}}"), &replacement);
    }
    let out = out.trim();
    NormalizedContent {
        text: Some(if out.is_empty() {
            "[system message]".into()
        } else {
            out.to_string()
        }),
        resources: Vec::new(),
    }
}

fn vote(map: &Map<String, Value>) -> NormalizedContent {
    let topic = string_field(map, "topic").unwrap_or_default();
    let options = map
        .get("options")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if topic.is_empty() && options.is_empty() {
        return NormalizedContent {
            text: Some("<vote>\n[vote]\n</vote>".into()),
            resources: Vec::new(),
        };
    }
    let mut lines = Vec::new();
    if !topic.is_empty() {
        lines.push(topic);
    }
    lines.extend(options.into_iter().map(|option| format!("• {option}")));
    NormalizedContent {
        text: Some(format!("<vote>\n{}\n</vote>", lines.join("\n"))),
        resources: Vec::new(),
    }
}

fn video_chat(map: &Map<String, Value>) -> NormalizedContent {
    let mut lines = Vec::new();
    if let Some(topic) = string_field(map, "topic").filter(|s| !s.is_empty()) {
        lines.push(format!("📹 {topic}"));
    }
    if let Some(dt) = millis_field(map, "start_time").and_then(millis_to_datetime) {
        lines.push(format!("🕙 {dt}"));
    }
    let inner = if lines.is_empty() {
        "[video chat]".into()
    } else {
        lines.join("\n")
    };
    NormalizedContent {
        text: Some(format!("<meeting>\n{inner}\n</meeting>")),
        resources: Vec::new(),
    }
}

fn calendar(message_type: &str, map: &Map<String, Value>) -> NormalizedContent {
    let mut lines = Vec::new();
    if let Some(summary) = string_field(map, "summary").filter(|s| !s.is_empty()) {
        lines.push(format!("📅 {summary}"));
    }
    let start = millis_field(map, "start_time").and_then(millis_to_datetime);
    let end = millis_field(map, "end_time").and_then(millis_to_datetime);
    match (start, end) {
        (Some(start), Some(end)) => lines.push(format!("🕙 {start} ~ {end}")),
        (Some(start), None) => lines.push(format!("🕙 {start}")),
        _ => {}
    }
    let inner = if lines.is_empty() {
        "[calendar event]".into()
    } else {
        lines.join("\n")
    };
    let tag = match message_type {
        "calendar" => "calendar_invite",
        "share_calendar_event" => "calendar_share",
        _ => "calendar",
    };
    NormalizedContent {
        text: Some(format!("<{tag}>\n{inner}\n</{tag}>")),
        resources: Vec::new(),
    }
}

fn todo(map: &Map<String, Value>) -> NormalizedContent {
    let Some(summary) = map.get("summary").and_then(Value::as_object) else {
        return NormalizedContent {
            text: Some("<todo>\n[todo]\n</todo>".into()),
            resources: Vec::new(),
        };
    };
    let mut lines = Vec::new();
    if let Some(title) = string_field(summary, "title").filter(|s| !s.is_empty()) {
        lines.push(title);
    }
    if let Some(content) = summary.get("content") {
        let (text, _) = render_rich_text_blocks(content);
        if !text.is_empty() {
            lines.push(text);
        }
    }
    if let Some(due) = millis_field(map, "due_time").and_then(millis_to_datetime) {
        lines.push(format!("Due: {due}"));
    }
    let inner = if lines.is_empty() {
        "[todo]".into()
    } else {
        lines.join("\n")
    };
    NormalizedContent {
        text: Some(format!("<todo>\n{inner}\n</todo>")),
        resources: Vec::new(),
    }
}

fn post(map: &Map<String, Value>) -> NormalizedContent {
    // Receive events may place the post document directly at the root instead of under a locale.
    let post = if map.contains_key("title")
        || map.contains_key("content")
        || map.contains_key("content_v2")
    {
        Some(map)
    } else {
        map.get("zh_cn")
            .or_else(|| map.get("en_us"))
            .or_else(|| map.values().find(|value| value.is_object()))
            .and_then(Value::as_object)
    };
    let Some(post) = post else {
        return NormalizedContent {
            text: Some("[post]".into()),
            resources: Vec::new(),
        };
    };

    if let Some(content_v2) = post.get("content_v2").and_then(Value::as_array)
        && !content_v2.is_empty()
    {
        return render_content_v2(content_v2);
    }

    let mut parts = Vec::new();
    if let Some(title) = string_field(post, "title").filter(|s| !s.is_empty()) {
        parts.push(format!("**{title}**"));
    }
    let mut resources = Vec::new();
    if let Some(content) = post.get("content") {
        let (body, body_resources) = render_rich_text_blocks(content);
        if !body.is_empty() {
            parts.push(body);
        }
        resources = body_resources;
    }
    NormalizedContent {
        text: Some(if parts.is_empty() {
            "[post]".into()
        } else {
            parts.join("\n\n")
        }),
        resources,
    }
}

fn render_content_v2(blocks: &[Value]) -> NormalizedContent {
    let mut lines = Vec::new();
    let mut resources = Vec::new();
    for paragraph in blocks {
        let Some(elements) = paragraph.as_array() else {
            continue;
        };
        let mut line = String::new();
        for element in elements {
            let Some(el) = element.as_object() else {
                continue;
            };
            match string_field(el, "tag").as_deref() {
                Some("md") => {
                    let (text, mut found) =
                        process_md_text(&string_field(el, "text").unwrap_or_default());
                    line.push_str(&text);
                    resources.append(&mut found);
                }
                Some("text") => line.push_str(&string_field(el, "text").unwrap_or_default()),
                Some("at") => line.push_str(&render_at_text(el)),
                Some("img") => {
                    if let Some(image_key) = string_field(el, "image_key") {
                        line.push_str(&format!("![image]({image_key})"));
                        resources.push(ChannelResource::new("image", image_key));
                    }
                }
                _ => {}
            }
        }
        if !line.is_empty() {
            lines.push(line);
        }
    }
    NormalizedContent {
        text: Some(lines.join("\n")),
        resources,
    }
}

fn render_rich_text_blocks(content: &Value) -> (String, Vec<ChannelResource>) {
    let Some(blocks) = content.as_array() else {
        return (String::new(), Vec::new());
    };
    let mut lines = Vec::new();
    let mut resources = Vec::new();
    for paragraph in blocks {
        let Some(elements) = paragraph.as_array() else {
            continue;
        };
        let mut line = String::new();
        for element in elements {
            let Some(el) = element.as_object() else {
                continue;
            };
            match string_field(el, "tag").as_deref() {
                Some("text") => line.push_str(&string_field(el, "text").unwrap_or_default()),
                Some("a") => {
                    let text = string_field(el, "text").unwrap_or_default();
                    let href = string_field(el, "href").unwrap_or_default();
                    if href.is_empty() {
                        line.push_str(&text);
                    } else {
                        line.push_str(&format!("[{text}]({href})"));
                    }
                }
                Some("at") => {
                    line.push_str(&render_at_text(el));
                }
                Some("img") => {
                    if let Some(image_key) = string_field(el, "image_key") {
                        line.push_str(&format!("![image]({image_key})"));
                        resources.push(ChannelResource::new("image", image_key));
                    }
                }
                Some("md") => {
                    let (text, mut found) =
                        process_md_text(&string_field(el, "text").unwrap_or_default());
                    line.push_str(&text);
                    resources.append(&mut found);
                }
                _ => {}
            }
        }
        if !line.is_empty() {
            lines.push(line);
        }
    }
    (lines.join("\n"), resources)
}

fn render_at_text(el: &Map<String, Value>) -> String {
    let user_id = string_field(el, "user_id").unwrap_or_default();
    if user_id == "all" {
        "@all".into()
    } else {
        format!(
            "@{}",
            string_field(el, "user_name")
                .or_else(|| string_field(el, "text"))
                .unwrap_or(user_id)
        )
    }
}

fn process_md_text(text: &str) -> (String, Vec<ChannelResource>) {
    let mut out = String::new();
    let mut resources = Vec::new();
    let mut in_code = false;
    for line in text.split_inclusive('\n') {
        let line_without_newline = line.trim_end_matches('\n');
        let newline = if line.ends_with('\n') { "\n" } else { "" };
        if line_without_newline.trim_start().starts_with("```") {
            in_code = !in_code;
            out.push_str(line);
            continue;
        }
        if in_code {
            out.push_str(line);
            continue;
        }
        let (mentions_replaced, mut line_resources) = replace_at_mentions(line_without_newline);
        let (images_replaced, mut image_resources) = extract_markdown_images(&mentions_replaced);
        out.push_str(&images_replaced);
        out.push_str(newline);
        resources.append(&mut line_resources);
        resources.append(&mut image_resources);
    }
    (out, resources)
}

fn replace_at_mentions(text: &str) -> (String, Vec<ChannelResource>) {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find("<at ") {
        out.push_str(&rest[..start]);
        let after_start = &rest[start..];
        let Some(end_open) = after_start.find('>') else {
            out.push_str(after_start);
            return (out, Vec::new());
        };
        let after_open = &after_start[end_open + 1..];
        let Some(close) = after_open.find("</at>") else {
            out.push_str(after_start);
            return (out, Vec::new());
        };
        let attrs = &after_start[..end_open + 1];
        let name = &after_open[..close];
        if attrs.contains(r#"user_id="all""#) {
            out.push_str("@all");
        } else {
            out.push('@');
            out.push_str(name);
        }
        rest = &after_open[close + "</at>".len()..];
    }
    out.push_str(rest);
    (out, Vec::new())
}

fn extract_markdown_images(text: &str) -> (String, Vec<ChannelResource>) {
    let mut resources = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("![") {
        let Some(open_paren) = rest[start..].find("](").map(|idx| start + idx) else {
            break;
        };
        let Some(close_paren) = rest[open_paren + 2..]
            .find(')')
            .map(|idx| open_paren + 2 + idx)
        else {
            break;
        };
        let key = &rest[open_paren + 2..close_paren];
        if !key.is_empty() {
            resources.push(ChannelResource::new("image", key.to_string()));
        }
        rest = &rest[close_paren + 1..];
    }
    (text.to_string(), resources)
}

fn string_field(map: &Map<String, Value>, key: &str) -> Option<String> {
    map.get(key).and_then(|value| {
        value
            .as_str()
            .map(str::to_string)
            .or_else(|| value.as_i64().map(|v| v.to_string()))
            .or_else(|| value.as_u64().map(|v| v.to_string()))
    })
}

fn int_field(map: &Map<String, Value>, key: &str) -> Option<i64> {
    map.get(key)
        .and_then(|value| value.as_i64().or_else(|| value.as_str()?.parse().ok()))
}

fn millis_field(map: &Map<String, Value>, key: &str) -> Option<i64> {
    int_field(map, key)
}

fn value_to_string(value: &Value) -> String {
    value
        .as_str()
        .map(str::to_string)
        .unwrap_or_else(|| value.to_string())
}

fn escape_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn format_duration(ms: i32) -> Option<String> {
    if ms <= 0 {
        return None;
    }
    let seconds = ms / 1000;
    Some(format!("{}:{:02}", seconds / 60, seconds % 60))
}

fn millis_to_datetime(ms: i64) -> Option<String> {
    let seconds = ms.checked_div(1000)?.checked_add(8 * 60 * 60)?;
    let days = seconds.div_euclid(86_400);
    let seconds_of_day = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = seconds_of_day / 3_600;
    let minute = (seconds_of_day % 3_600) / 60;
    let second = seconds_of_day % 60;
    Some(format!(
        "{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}"
    ))
}

fn civil_from_days(days_since_epoch: i64) -> (i64, i64, i64) {
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if month <= 2 { 1 } else { 0 };
    (year, month, day)
}
