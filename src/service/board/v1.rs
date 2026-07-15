use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::constants::AccessTokenType;
use crate::error::LarkError;
use crate::req::RequestOption;
use crate::resp::ApiResp;
use crate::service::common::RestRequest;

// ── Domain types ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Board {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whiteboard_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_bot_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub extra: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BorderRadius {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_left: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_right: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bottom_right: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bottom_left: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompositeShape {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pie: Option<Pie>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circular_ring: Option<Pie>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trapezoid: Option<Trapezoid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cube: Option<Cube>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Connector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_object: Option<ConnectorAttachedObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_object: Option<ConnectorAttachedObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<ConnectorInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<ConnectorInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captions: Option<ConnectorCaption>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub turning_points: Vec<Point>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption_auto_direction: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption_position: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specified_coordinate: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption_position_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectorAttachedObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snap_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Point>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectorCaption {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<Text>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectorInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attached_object: Option<ConnectorAttachedObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Point>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arrow_style: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cube {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control_point: Option<Point>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DepartmentId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_department_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FillGradient {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub handle_positions: Vec<Point>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stops: Vec<GradientStop>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GradientStop {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Head {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<AuthInfo>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub fields: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<ClientInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Image {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Lifeline {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MindMap {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MindMapNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z_index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout_position: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collapsed: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MindMapRoot {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_style: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub up_children: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub down_children: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub left_children: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub right_children: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Paint {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lines: Vec<Point>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Pie {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_radial_line_angle: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub central_angle: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sector_ratio: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Point {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichText {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paragraphs: Vec<RichTextParagraph>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_element: Option<RichTextElementText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_element: Option<RichTextElementLink>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mention_user_element: Option<RichTextElementMentionUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mention_doc_element: Option<RichTextElementMentionDoc>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextElementLink {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub herf: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_style: Option<RichTextElementTextStyle>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextElementMentionDoc {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_style: Option<RichTextElementTextStyle>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextElementMentionUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_style: Option<RichTextElementTextStyle>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextElementText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_style: Option<RichTextElementTextStyle>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextElementTextStyle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_background_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_through: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RichTextParagraph {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paragraph_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub elements: Vec<RichTextElement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indent: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_begin_index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Section {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Shadow {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blur: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickyNote {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_author_info: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Style {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fill_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fill_opacity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border_style: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border_width: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border_opacity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub h_flip: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v_flip: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme_fill_color_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme_border_color_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fill_color_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border_color_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub border_dasharrays: Vec<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<BorderRadius>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shadow: Option<Shadow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inner_shadow: Option<Shadow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fill_gradient: Option<FillGradient>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Svg {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub svg_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Syntax {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub syntax_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Table {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<TableMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableCell {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub col_index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_info: Option<TableCellMergeInfo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableCellMergeInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_span: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub col_span: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub col_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub row_sizes: Vec<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub col_sizes: Vec<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Text {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub horizontal_align: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_background_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_through: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub angle: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme_text_color_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme_text_background_color_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<RichText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_color_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_background_color_type: Option<i32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Trapezoid {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_length: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WhiteboardNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<Text>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composite_shape: Option<CompositeShape>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connector: Option<Connector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section: Option<Section>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table: Option<Table>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub z_index: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifeline: Option<Lifeline>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paint: Option<Paint>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub svg: Option<Svg>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticky_note: Option<StickyNote>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mind_map_node: Option<MindMapNode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mind_map_root: Option<MindMapRoot>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mind_map: Option<MindMap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub syntax: Option<Syntax>,
}

// ── Request body types ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateBoardReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
}

// ── Response wrappers ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BoardData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whiteboard: Option<Board>,
}

// ── Request body types (whiteboard theme) ──

#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateThemeWhiteboardReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
}

// ── Request body types (whiteboard node) ──

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateWhiteboardNodeReqBody {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<WhiteboardNode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchDeleteWhiteboardNodeReqBody {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreatePlantumlWhiteboardNodeReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plant_uml_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagram_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overwrite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_type: Option<i32>,
}

impl_resp!(CreateBoardResp, BoardData);
impl_resp!(GetBoardResp, BoardData);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
}

impl_resp!(ThemeWhiteboardResp, ThemeData);
impl_resp!(UpdateThemeWhiteboardResp, ());

#[derive(Debug, Clone)]
pub struct DownloadAsImageWhiteboardResp {
    pub api_resp: ApiResp,
    pub file_name: Option<String>,
    pub data: Vec<u8>,
}

impl DownloadAsImageWhiteboardResp {
    pub fn success(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WhiteboardNodeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<WhiteboardNode>,
}

impl_resp!(CreateWhiteboardNodeResp, WhiteboardNodeData);
impl_resp!(CreatePlantumlWhiteboardNodeResp, WhiteboardNodeData);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchDeleteWhiteboardNodeData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl_resp!(BatchDeleteWhiteboardNodeResp, BatchDeleteWhiteboardNodeData);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WhiteboardNodeListData {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<WhiteboardNode>,
}

impl_resp!(ListWhiteboardNodeResp, WhiteboardNodeListData);

#[derive(Debug, Clone, Copy)]
pub struct CreateWhiteboardQuery<'a> {
    pub body: &'a CreateBoardReqBody,
}

impl<'a> CreateWhiteboardQuery<'a> {
    pub fn new(body: &'a CreateBoardReqBody) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetWhiteboardQuery<'a> {
    pub whiteboard_id: &'a str,
}

impl<'a> GetWhiteboardQuery<'a> {
    pub fn new(whiteboard_id: &'a str) -> Self {
        Self { whiteboard_id }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DownloadAsImageWhiteboardQuery<'a> {
    pub whiteboard_id: &'a str,
}

impl<'a> DownloadAsImageWhiteboardQuery<'a> {
    pub fn new(whiteboard_id: &'a str) -> Self {
        Self { whiteboard_id }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ThemeWhiteboardQuery<'a> {
    pub whiteboard_id: &'a str,
}

impl<'a> ThemeWhiteboardQuery<'a> {
    pub fn new(whiteboard_id: &'a str) -> Self {
        Self { whiteboard_id }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UpdateThemeWhiteboardQuery<'a> {
    pub whiteboard_id: &'a str,
    pub body: &'a UpdateThemeWhiteboardReqBody,
}

impl<'a> UpdateThemeWhiteboardQuery<'a> {
    pub fn new(whiteboard_id: &'a str, body: &'a UpdateThemeWhiteboardReqBody) -> Self {
        Self {
            whiteboard_id,
            body,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreateWhiteboardNodeQuery<'a> {
    pub whiteboard_id: &'a str,
    pub body: &'a CreateWhiteboardNodeReqBody,
}

impl<'a> CreateWhiteboardNodeQuery<'a> {
    pub fn new(whiteboard_id: &'a str, body: &'a CreateWhiteboardNodeReqBody) -> Self {
        Self {
            whiteboard_id,
            body,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CreatePlantumlWhiteboardNodeQuery<'a> {
    pub whiteboard_id: &'a str,
    pub body: &'a CreatePlantumlWhiteboardNodeReqBody,
}

#[derive(Debug, Clone, Copy)]
pub struct BatchDeleteWhiteboardNodeQuery<'a> {
    pub whiteboard_id: &'a str,
    pub body: &'a BatchDeleteWhiteboardNodeReqBody,
    pub client_token: Option<&'a str>,
}

impl<'a> BatchDeleteWhiteboardNodeQuery<'a> {
    pub fn new(whiteboard_id: &'a str, body: &'a BatchDeleteWhiteboardNodeReqBody) -> Self {
        Self {
            whiteboard_id,
            body,
            client_token: None,
        }
    }

    pub fn client_token(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.client_token = value.into();
        self
    }
}

impl<'a> CreatePlantumlWhiteboardNodeQuery<'a> {
    pub fn new(whiteboard_id: &'a str, body: &'a CreatePlantumlWhiteboardNodeReqBody) -> Self {
        Self {
            whiteboard_id,
            body,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListWhiteboardNodeQuery<'a> {
    pub whiteboard_id: &'a str,
}

impl<'a> ListWhiteboardNodeQuery<'a> {
    pub fn new(whiteboard_id: &'a str) -> Self {
        Self { whiteboard_id }
    }
}

// ── Resources ──

pub struct WhiteboardResource<'a> {
    config: &'a Config,
}

impl<'a> WhiteboardResource<'a> {
    pub async fn create(
        &self,
        body: &CreateBoardReqBody,
        option: &RequestOption,
    ) -> Result<CreateBoardResp, LarkError> {
        self.create_by_query(&CreateWhiteboardQuery::new(body), option)
            .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateWhiteboardQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateBoardResp, LarkError> {
        RestRequest::new(
            self.config,
            http::Method::POST,
            "/open-apis/board/v1/whiteboards",
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_response::<BoardData, CreateBoardResp>()
        .await
    }

    pub async fn get(
        &self,
        whiteboard_id: &str,
        option: &RequestOption,
    ) -> Result<GetBoardResp, LarkError> {
        self.get_by_query(&GetWhiteboardQuery::new(whiteboard_id), option)
            .await
    }

    pub async fn get_by_query(
        &self,
        query: &GetWhiteboardQuery<'_>,
        option: &RequestOption,
    ) -> Result<GetBoardResp, LarkError> {
        let path = format!("/open-apis/board/v1/whiteboards/{}", query.whiteboard_id);
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_response::<BoardData, GetBoardResp>()
        .await
    }

    pub async fn download_as_image(
        &self,
        whiteboard_id: &str,
        option: &RequestOption,
    ) -> Result<DownloadAsImageWhiteboardResp, LarkError> {
        self.download_as_image_by_query(&DownloadAsImageWhiteboardQuery::new(whiteboard_id), option)
            .await
    }

    pub async fn download_as_image_by_query(
        &self,
        query: &DownloadAsImageWhiteboardQuery<'_>,
        option: &RequestOption,
    ) -> Result<DownloadAsImageWhiteboardResp, LarkError> {
        let path = format!(
            "/open-apis/board/v1/whiteboards/{}/download_as_image",
            query.whiteboard_id
        );
        let download = RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .download()
        .await?;
        Ok(DownloadAsImageWhiteboardResp {
            api_resp: download.api_resp,
            file_name: download.file_name,
            data: download.data,
        })
    }

    pub async fn theme(
        &self,
        whiteboard_id: &str,
        option: &RequestOption,
    ) -> Result<ThemeWhiteboardResp, LarkError> {
        self.theme_by_query(&ThemeWhiteboardQuery::new(whiteboard_id), option)
            .await
    }

    pub async fn theme_by_query(
        &self,
        query: &ThemeWhiteboardQuery<'_>,
        option: &RequestOption,
    ) -> Result<ThemeWhiteboardResp, LarkError> {
        let path = format!(
            "/open-apis/board/v1/whiteboards/{}/theme",
            query.whiteboard_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .send_response::<ThemeData, ThemeWhiteboardResp>()
        .await
    }

    pub async fn update_theme(
        &self,
        whiteboard_id: &str,
        body: &UpdateThemeWhiteboardReqBody,
        option: &RequestOption,
    ) -> Result<UpdateThemeWhiteboardResp, LarkError> {
        self.update_theme_by_query(
            &UpdateThemeWhiteboardQuery::new(whiteboard_id, body),
            option,
        )
        .await
    }

    pub async fn update_theme_by_query(
        &self,
        query: &UpdateThemeWhiteboardQuery<'_>,
        option: &RequestOption,
    ) -> Result<UpdateThemeWhiteboardResp, LarkError> {
        let path = format!(
            "/open-apis/board/v1/whiteboards/{}/update_theme",
            query.whiteboard_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<(), UpdateThemeWhiteboardResp>()
        .await
    }
}

pub struct WhiteboardNodeResource<'a> {
    config: &'a Config,
}

impl<'a> WhiteboardNodeResource<'a> {
    pub async fn batch_delete(
        &self,
        whiteboard_id: &str,
        body: &BatchDeleteWhiteboardNodeReqBody,
        client_token: Option<&str>,
        option: &RequestOption,
    ) -> Result<BatchDeleteWhiteboardNodeResp, LarkError> {
        let query =
            BatchDeleteWhiteboardNodeQuery::new(whiteboard_id, body).client_token(client_token);
        self.batch_delete_by_query(&query, option).await
    }

    pub async fn batch_delete_by_query(
        &self,
        query: &BatchDeleteWhiteboardNodeQuery<'_>,
        option: &RequestOption,
    ) -> Result<BatchDeleteWhiteboardNodeResp, LarkError> {
        let path = format!(
            "/open-apis/board/v1/whiteboards/{}/nodes/batch_delete",
            query.whiteboard_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .query("client_token", query.client_token)
        .json_body(query.body)?
        .send_response::<BatchDeleteWhiteboardNodeData, BatchDeleteWhiteboardNodeResp>()
        .await
    }

    pub async fn create(
        &self,
        whiteboard_id: &str,
        body: &CreateWhiteboardNodeReqBody,
        option: &RequestOption,
    ) -> Result<CreateWhiteboardNodeResp, LarkError> {
        self.create_by_query(&CreateWhiteboardNodeQuery::new(whiteboard_id, body), option)
            .await
    }

    pub async fn create_by_query(
        &self,
        query: &CreateWhiteboardNodeQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreateWhiteboardNodeResp, LarkError> {
        let path = format!(
            "/open-apis/board/v1/whiteboards/{}/nodes",
            query.whiteboard_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .json_body(query.body)?
        .send_response::<WhiteboardNodeData, CreateWhiteboardNodeResp>()
        .await
    }

    pub async fn create_plantuml(
        &self,
        whiteboard_id: &str,
        body: &CreatePlantumlWhiteboardNodeReqBody,
        option: &RequestOption,
    ) -> Result<CreatePlantumlWhiteboardNodeResp, LarkError> {
        self.create_plantuml_by_query(
            &CreatePlantumlWhiteboardNodeQuery::new(whiteboard_id, body),
            option,
        )
        .await
    }

    pub async fn create_plantuml_by_query(
        &self,
        query: &CreatePlantumlWhiteboardNodeQuery<'_>,
        option: &RequestOption,
    ) -> Result<CreatePlantumlWhiteboardNodeResp, LarkError> {
        let path = format!(
            "/open-apis/board/v1/whiteboards/{}/nodes/plantuml",
            query.whiteboard_id
        );
        RestRequest::new(
            self.config,
            http::Method::POST,
            path,
            vec![AccessTokenType::User, AccessTokenType::Tenant],
            option,
        )
        .json_body(query.body)?
        .send_response::<WhiteboardNodeData, CreatePlantumlWhiteboardNodeResp>()
        .await
    }

    pub async fn list(
        &self,
        whiteboard_id: &str,
        option: &RequestOption,
    ) -> Result<ListWhiteboardNodeResp, LarkError> {
        self.list_by_query(&ListWhiteboardNodeQuery::new(whiteboard_id), option)
            .await
    }

    pub async fn list_by_query(
        &self,
        query: &ListWhiteboardNodeQuery<'_>,
        option: &RequestOption,
    ) -> Result<ListWhiteboardNodeResp, LarkError> {
        let path = format!(
            "/open-apis/board/v1/whiteboards/{}/nodes",
            query.whiteboard_id
        );
        RestRequest::new(
            self.config,
            http::Method::GET,
            path,
            vec![AccessTokenType::Tenant, AccessTokenType::User],
            option,
        )
        .send_response::<WhiteboardNodeListData, ListWhiteboardNodeResp>()
        .await
    }
}

// ── Version struct ──

pub struct V1<'a> {
    pub whiteboard: WhiteboardResource<'a>,
    pub whiteboard_node: WhiteboardNodeResource<'a>,
}

impl<'a> V1<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            whiteboard: WhiteboardResource { config },
            whiteboard_node: WhiteboardNodeResource { config },
        }
    }
}
