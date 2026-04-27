use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── Top-level panel ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panel {
    pub id: String,
    pub title: Option<Title>,
    #[serde(rename = "type")]
    pub panel_type: PanelType,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub attributes: HashMap<char, AttributeDef>,
    pub body: Body,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<ModelDef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub init: Option<InitSection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reinit: Option<ReinitSection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proc_section: Option<ProcSection>,
    pub metadata: Metadata,
}

// ─── Title ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_var: Option<String>,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_var: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

// ─── Panel type ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PanelType {
    Dialog,
    Menu,
    List,
    Help,
    Form,
    Edit,
    Browse,
    Tutorial,
    Changelog,
    Sample,
}

// ─── Attributes ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeDef {
    #[serde(rename = "type")]
    pub field_type: FieldType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intensity: Option<Intensity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caps: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scroll: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub justification: Option<Justification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mouse: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attn: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum FieldType {
    Prot,
    Input,
    Output,
    Sel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Intensity {
    High,
    Low,
    Non,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Justification {
    Left,
    Right,
    Asis,
}

// ─── Body ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    pub rows: Vec<BodyRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BodyRow {
    Command {
        variable: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        scroll: Option<ScrollField>,
    },
    Blank,
    Text {
        content: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        style: Option<String>,
    },
    FieldRow {
        fields: Vec<Field>,
    },
    Input {
        variable: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        attribute: Option<char>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        width: Option<usize>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        field_connector: bool,
    },
    Output {
        variable: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        style: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        indent: Option<usize>,
    },
    Divider {
        style: DividerStyle,
    },
    Box {
        style: BoxStyle,
        rows: Vec<BodyRow>,
    },
    InlineGroup {
        fields: Vec<Field>,
    },
    ColumnHeader {
        columns: Vec<String>,
    },
    ColumnRuler,
    Raw {
        content: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollField {
    pub variable: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Field {
    Text {
        content: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        style: Option<String>,
    },
    Input {
        variable: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        attribute: Option<char>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        width: Option<usize>,
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        field_connector: bool,
    },
    Output {
        variable: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        attribute: Option<char>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DividerStyle {
    Single,
    Double,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BoxStyle {
    Asterisk,
    Announcement,
    Single,
    Double,
}

// ─── Model (for list/table panels) ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selection_field: Option<SelectionField>,
    pub columns: Vec<ModelColumn>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionField {
    pub variable: String,
    pub width: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelColumn {
    pub variable: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attribute: Option<char>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<usize>,
}

// ─── Init / Reinit ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitSection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_panel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zvars: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbols: Option<bool>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub assignments: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditionals: Vec<Conditional>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReinitSection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub assignments: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conditional {
    pub condition: String,
    pub then_assignments: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub then_cursor: Option<String>,
}

// ─── Proc ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcSection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validations: Vec<Validation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub navigation: Option<Navigation>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub assignments: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validation {
    pub field: String,
    pub rules: Vec<ValidationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ValidationRule {
    NonBlank,
    Boolean,
    Numeric {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        range: Option<NumericRange>,
    },
    Alpha,
    Hex,
    Picture {
        format: String,
    },
    List {
        values: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumericRange {
    pub min: RangeValue,
    pub max: RangeValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RangeValue {
    Literal(i64),
    Variable(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Navigation {
    pub source_variable: String,
    pub routes: Vec<NavRoute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavRoute {
    pub value: String,
    pub action: NavAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum NavAction {
    Panel { target: String },
    List { targets: Vec<String> },
    Up,
    Blank,
    Ctc { command: String },
}

// ─── Metadata ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub source_file: String,
    pub category: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parse_warnings: Vec<String>,
}
