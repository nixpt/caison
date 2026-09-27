//! Crush AI-native Semantic Object Notation (CAISON) �� canonical types.
//!
//! CAISON is a human-readable configuration and serialization format that
//! blends JSON structure with AI-native primitives: confidence weights,
//! semantic keys, annotations, and synthesized values.
//!
//! These types are the single source of truth for CAISON across the crush
//! ecosystem. They implement serde's `Serialize` and `Deserialize` for
//! zero-code JSON/MessagePack/CBOR interop.

pub mod parser;
pub use parser::CaisonParser;

pub mod project;
pub use project::ProjectError;

pub mod print;
pub use print::{print_document, print_node};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Core Types ─────────────────────────────────────────────────────────────

/// A key in a CAISON object — either an exact match or a semantic intent anchor.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CaisonKey {
    /// Standard exact-match string key: `name: value`
    Exact(String),
    /// Semantic fuzzy-match anchor: `~"Billing or refund issues": handler`
    #[serde(rename = "~")]
    Semantic(String),
}

/// Core value types in the CAISON data model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CaisonValue {
    /// String literal: `"hello"`
    String(String),
    /// Numeric value (all numbers are f64): `42`, `3.14`
    Number(f64),
    /// Boolean: `true`, `false`
    Boolean(bool),
    /// Null placeholder: `null`
    Null,
    /// Key-value object: `{ key: value, ... }`
    Object(HashMap<String, CaisonNode>),
    /// Ordered sequence: `[a, b, c]`
    Array(Vec<CaisonNode>),
    /// AI-synthesized placeholder: `@synthesize("a complimentary color")`
    #[serde(rename = "@synthesize")]
    Synthesize(String),
}

/// A node in the CAISON tree — a value with optional AI metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaisonNode {
    /// The node's value.
    pub value: CaisonValue,
    /// Confidence/probability weight: `value ~0.95`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    /// Annotations attached to this node: `@wip { owner: "foreman" }`
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<CaisonAnnotation>,
}

/// Metadata annotation attached to a node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaisonAnnotation {
    /// Annotation name: `@wip`, `@temporary`, `@decision`, etc.
    pub name: String,
    /// Optional parenthesized argument: `@caison("1.5")`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<String>,
    /// Key-value properties: `@wip { owner: "foreman" }`
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, String>,
}

/// The root document structure for a `.caison` file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaisonDocument {
    /// Document format version (from `@caison { version: "1.0" }`).
    #[serde(default = "default_version")]
    pub version: String,
    /// The root value node.
    pub root: CaisonNode,
}

fn default_version() -> String {
    "1.0".to_string()
}

// ── Convenience constructors ───────────────────────────────────────────────

impl CaisonNode {
    pub fn new(value: CaisonValue) -> Self {
        Self {
            value,
            confidence: None,
            annotations: vec![],
        }
    }

    pub fn with_confidence(mut self, c: f64) -> Self {
        self.confidence = Some(c);
        self
    }

    pub fn with_annotation(mut self, ann: CaisonAnnotation) -> Self {
        self.annotations.push(ann);
        self
    }
}

impl CaisonValue {
    /// Convenience: create a string value.
    pub fn string(s: impl Into<String>) -> Self {
        CaisonValue::String(s.into())
    }
    /// Convenience: create a number value.
    pub fn number(n: f64) -> Self {
        CaisonValue::Number(n)
    }
    /// Convenience: create a boolean value.
    pub fn bool_value(b: bool) -> Self {
        CaisonValue::Boolean(b)
    }
}

// ── JSON serialization ─────────────────────────────────────────────────────

impl CaisonDocument {
    /// Serialize this CAISON document to a JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Serialize this CAISON document to compact JSON bytes.
    pub fn to_json_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    /// Deserialize a CAISON document from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Deserialize a CAISON document from JSON bytes.
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }
}

// ── Display ────────────────────────────────────────────────────────────────

impl std::fmt::Display for CaisonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CaisonValue::String(s) => write!(f, "\"{s}\""),
            CaisonValue::Number(n) => write!(f, "{n}"),
            CaisonValue::Boolean(b) => write!(f, "{b}"),
            CaisonValue::Null => write!(f, "null"),
            CaisonValue::Object(_) => write!(f, "{{object}}"),
            CaisonValue::Array(_) => write!(f, "[array]"),
            CaisonValue::Synthesize(p) => write!(f, "@synthesize({p:?})"),
        }
    }
}

impl std::fmt::Display for CaisonNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)?;
        if let Some(c) = self.confidence {
            write!(f, " ~{c}")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for CaisonKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CaisonKey::Exact(s) => write!(f, "{s}"),
            // The internal map-key form: `~` + bare intent text. `to_caison` detects
            // the `~` and re-quotes it; the JSON projection strips the `~` (SPEC §6).
            CaisonKey::Semantic(s) => write!(f, "~{s}"),
        }
    }
}

// ── Tests ──��────────────────────────────────────────────────────────────────

// ── CAISON serialization ─────────────────────────────────────────────────────

impl CaisonDocument {
    pub fn to_caison(&self) -> String {
        let mut out = String::new();
        if self.version != "1.0" {
            out.push_str(&format!("@caison {{ version: \"{}\" }}\n", self.version));
        }

        if let CaisonValue::Object(obj) = &self.root.value {
            let mut items: Vec<_> = obj.iter().collect();
            items.sort_by_key(|(k, _)| *k);
            for (k, v) in items {
                if k.starts_with('~') {
                    out.push_str(&format!("~\"{}\": ", k.trim_start_matches('~')));
                } else if k.chars().all(|c| c.is_alphanumeric() || c == '_') && !k.is_empty() {
                    out.push_str(&format!("{}: ", k));
                } else {
                    out.push_str(&format!("\"{}\": ", k));
                }
                out.push_str(&v.to_caison(0));
                out.push('\n');
            }
        } else {
            out.push_str(&self.root.to_caison(0));
            out.push('\n');
        }

        out
    }
}

impl CaisonNode {
    pub fn to_caison(&self, indent: usize) -> String {
        let mut out = String::new();
        for ann in &self.annotations {
            out.push_str(&format!("@{}", ann.name));
            if let Some(args) = &ann.args {
                out.push_str(&format!("(\"{}\")", args));
            }
            if !ann.properties.is_empty() {
                out.push_str(" {");
                let mut props: Vec<_> = ann.properties.iter().collect();
                props.sort_by_key(|(k, _)| *k);
                for (i, (k, v)) in props.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    out.push_str(&format!(" {}: \"{}\"", k, v.replace('"', "\\\"")));
                }
                out.push_str(" }");
            }
            out.push('\n');
            out.push_str(&" ".repeat(indent * 4));
        }

        out.push_str(&self.value.to_caison(indent));

        if let Some(c) = self.confidence {
            out.push_str(&format!(" ~{}", c));
        }
        out
    }
}

impl CaisonValue {
    pub fn to_caison(&self, indent: usize) -> String {
        match self {
            CaisonValue::String(s) => format!(
                "\"{}\"",
                s.replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace('\n', "\\n")
                    .replace('\r', "\\r")
                    .replace('\t', "\\t")
            ),
            CaisonValue::Number(n) => n.to_string(),
            CaisonValue::Boolean(b) => b.to_string(),
            CaisonValue::Null => "null".to_string(),
            CaisonValue::Synthesize(s) => format!("@synthesize(\"{}\")", s),
            CaisonValue::Array(arr) => {
                if arr.is_empty() {
                    return "[]".to_string();
                }
                let mut out = String::from("[\n");
                let child_indent = indent + 1;
                for item in arr {
                    out.push_str(&" ".repeat(child_indent * 4));
                    out.push_str(&item.to_caison(child_indent));
                    out.push_str(",\n");
                }
                out.push_str(&" ".repeat(indent * 4));
                out.push(']');
                out
            }
            CaisonValue::Object(obj) => {
                if obj.is_empty() {
                    return "{}".to_string();
                }
                let mut out = String::from("{\n");
                let child_indent = indent + 1;
                let mut items: Vec<_> = obj.iter().collect();
                items.sort_by_key(|(k, _)| *k);
                for (k, v) in items {
                    out.push_str(&" ".repeat(child_indent * 4));
                    if k.starts_with('~') {
                        out.push_str(&format!("~\"{}\": ", k.trim_start_matches('~')));
                    } else if k.chars().all(|c| c.is_alphanumeric() || c == '_') && !k.is_empty() {
                        out.push_str(&format!("{}: ", k));
                    } else {
                        out.push_str(&format!("\"{}\": ", k));
                    }
                    out.push_str(&v.to_caison(child_indent));
                    out.push_str(",\n");
                }
                out.push_str(&" ".repeat(indent * 4));
                out.push('}');
                out
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::CaisonParser;

    #[test]
    fn inline_annotation_attaches_to_its_value() {
        // `port: 8080 @wip { owner: "foreman" }` — the annotation is a SUFFIX on
        // the value, and the value stays `8080` (not "8080 @wip ...").
        let doc =
            CaisonParser::new("port: 8080 @wip { owner: \"foreman\" }\nhost: \"localhost\"\n")
                .parse()
                .unwrap();
        let CaisonValue::Object(map) = &doc.root.value else {
            panic!("root is not an object");
        };
        let port = map.get("port").expect("port present");
        assert_eq!(port.value, CaisonValue::Number(8080.0));
        assert_eq!(port.annotations.len(), 1);
        assert_eq!(port.annotations[0].name, "wip");
        assert_eq!(
            port.annotations[0]
                .properties
                .get("owner")
                .map(String::as_str),
            Some("foreman")
        );
        // the next pair is unaffected
        assert_eq!(
            map.get("host").expect("host present").value,
            CaisonValue::string("localhost")
        );
    }

    #[test]
    fn semantic_key_roundtrips_and_projects_to_bare_text() {
        // The internal map key keeps the `~"…"` marker (so the printer can
        // round-trip it); SPEC §6 strips it in the JSON projection.
        let doc = CaisonParser::new("~\"billing or refund issues\": \"route to support\"\n")
            .parse()
            .unwrap();
        let CaisonValue::Object(map) = &doc.root.value else {
            panic!("root is not an object");
        };
        assert!(map.contains_key("~billing or refund issues"));
        // the printer restores the semantic marker
        let printed = doc.root.to_caison(0);
        assert!(
            printed.contains("~\"billing or refund issues\""),
            "{printed}"
        );
    }

    #[test]
    fn confidence_suffix_is_not_mistaken_for_a_semantic_key() {
        let doc = CaisonParser::new("temperature: 21.5 ~0.8\n")
            .parse()
            .unwrap();
        let CaisonValue::Object(map) = &doc.root.value else {
            panic!("root is not an object");
        };
        let t = map.get("temperature").expect("temperature present");
        assert_eq!(t.value, CaisonValue::Number(21.5));
        assert_eq!(t.confidence, Some(0.8));
    }

    #[test]
    fn test_json_roundtrip() {
        let doc = CaisonDocument {
            version: "1.0".into(),
            root: CaisonNode::new(CaisonValue::Object(HashMap::from([
                (
                    "name".into(),
                    CaisonNode::new(CaisonValue::string("test")).with_confidence(0.95),
                ),
                ("count".into(), CaisonNode::new(CaisonValue::number(42.0))),
            ]))),
        };

        let json = doc.to_json().unwrap();
        let doc2 = CaisonDocument::from_json(&json).unwrap();
        assert_eq!(doc.version, doc2.version);
        assert_eq!(doc.root.value, doc2.root.value);
    }

    #[test]
    fn test_json_roundtrip_array() {
        let doc = CaisonDocument {
            version: "1.0".into(),
            root: CaisonNode::new(CaisonValue::Array(vec![
                CaisonNode::new(CaisonValue::string("a")),
                CaisonNode::new(CaisonValue::string("b")),
            ])),
        };

        let json = doc.to_json().unwrap();
        let doc2 = CaisonDocument::from_json(&json).unwrap();
        assert_eq!(doc.root.value, doc2.root.value);
    }

    #[test]
    fn test_json_roundtrip_annotations() {
        let mut props = HashMap::new();
        props.insert("owner".into(), "foreman".into());
        let doc = CaisonDocument {
            version: "1.0".into(),
            root: CaisonNode::new(CaisonValue::Null).with_annotation(CaisonAnnotation {
                name: "wip".into(),
                args: None,
                properties: props,
            }),
        };

        let json = doc.to_json().unwrap();
        let doc2 = CaisonDocument::from_json(&json).unwrap();
        assert_eq!(doc2.root.annotations.len(), 1);
        assert_eq!(doc2.root.annotations[0].name, "wip");
    }

    #[test]
    fn test_synthesize() {
        let doc = CaisonDocument {
            version: "1.0".into(),
            root: CaisonNode::new(CaisonValue::Synthesize("a warm color".into())),
        };
        let json = doc.to_json().unwrap();
        // Synthesize values serialise as plain strings with #[serde(untagged)],
        // so they deserialise as String. The semantic distinction is preserved
        // at the CAISON level; JSON is a lossy transport for the @synthesize tag.
        let doc2 = CaisonDocument::from_json(&json).unwrap();
        assert_eq!(doc2.root.value, CaisonValue::String("a warm color".into()));
    }
}
