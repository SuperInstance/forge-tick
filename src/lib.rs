use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tick {
    pub id: Uuid,
    pub room: String,
    pub agent: String,
    pub content: String,
    pub tick_type: TickType,
    pub timestamp_ms: u64,
    pub meta: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TickType {
    SensorReading,
    TextLine,
    CodeBlock,
    DataRow,
    AudioChunk,
    ImageRegion,
    SubtitleEntry,
    Status,
    Error,
    Command,
}

impl std::fmt::Display for TickType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TickType::SensorReading => write!(f, "sensor_reading"),
            TickType::TextLine => write!(f, "text_line"),
            TickType::CodeBlock => write!(f, "code_block"),
            TickType::DataRow => write!(f, "data_row"),
            TickType::AudioChunk => write!(f, "audio_chunk"),
            TickType::ImageRegion => write!(f, "image_region"),
            TickType::SubtitleEntry => write!(f, "subtitle_entry"),
            TickType::Status => write!(f, "status"),
            TickType::Error => write!(f, "error"),
            TickType::Command => write!(f, "command"),
        }
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

pub struct TileToTickMapper;

impl TileToTickMapper {
    pub fn map_text(tile_content: &str, index: u64) -> Tick {
        Tick {
            id: Uuid::new_v4(),
            room: String::new(),
            agent: String::new(),
            content: tile_content.to_string(),
            tick_type: TickType::TextLine,
            timestamp_ms: now_ms(),
            meta: {
                let mut m = HashMap::new();
                m.insert("index".into(), index.to_string());
                m
            },
        }
    }

    pub fn map_sensor(sensor_type: &str, value: f64, unit: &str, timestamp_ms: u64) -> Tick {
        Tick {
            id: Uuid::new_v4(),
            room: String::new(),
            agent: String::new(),
            content: format!("{}={}{}", sensor_type, value, unit),
            tick_type: TickType::SensorReading,
            timestamp_ms,
            meta: {
                let mut m = HashMap::new();
                m.insert("sensor_type".into(), sensor_type.to_string());
                m.insert("value".into(), value.to_string());
                m.insert("unit".into(), unit.to_string());
                m
            },
        }
    }

    pub fn map_code(language: &str, kind: &str, name: &str, body: &str) -> Tick {
        Tick {
            id: Uuid::new_v4(),
            room: String::new(),
            agent: String::new(),
            content: body.to_string(),
            tick_type: TickType::CodeBlock,
            timestamp_ms: now_ms(),
            meta: {
                let mut m = HashMap::new();
                m.insert("language".into(), language.to_string());
                m.insert("kind".into(), kind.to_string());
                m.insert("name".into(), name.to_string());
                m
            },
        }
    }

    pub fn map_data_row(row_json: &str, index: u64) -> Tick {
        Tick {
            id: Uuid::new_v4(),
            room: String::new(),
            agent: String::new(),
            content: row_json.to_string(),
            tick_type: TickType::DataRow,
            timestamp_ms: now_ms(),
            meta: {
                let mut m = HashMap::new();
                m.insert("index".into(), index.to_string());
                m
            },
        }
    }

    pub fn map_subtitle(text: &str, start_ms: u64, end_ms: u64) -> Tick {
        Tick {
            id: Uuid::new_v4(),
            room: String::new(),
            agent: String::new(),
            content: text.to_string(),
            tick_type: TickType::SubtitleEntry,
            timestamp_ms: start_ms,
            meta: {
                let mut m = HashMap::new();
                m.insert("start_ms".into(), start_ms.to_string());
                m.insert("end_ms".into(), end_ms.to_string());
                m
            },
        }
    }

    pub fn map_generic(content: &str, kind: &str) -> Tick {
        let tick_type = match kind {
            "sensor_reading" => TickType::SensorReading,
            "text_line" => TickType::TextLine,
            "code_block" => TickType::CodeBlock,
            "data_row" => TickType::DataRow,
            "audio_chunk" => TickType::AudioChunk,
            "image_region" => TickType::ImageRegion,
            "subtitle_entry" => TickType::SubtitleEntry,
            "status" => TickType::Status,
            "error" => TickType::Error,
            "command" => TickType::Command,
            _ => TickType::Status,
        };
        Tick {
            id: Uuid::new_v4(),
            room: String::new(),
            agent: String::new(),
            content: content.to_string(),
            tick_type,
            timestamp_ms: now_ms(),
            meta: HashMap::new(),
        }
    }
}

pub struct TickToTileMapper;

impl TickToTileMapper {
    pub fn to_text(tick: &Tick) -> String {
        tick.content.clone()
    }

    pub fn to_sensor_values(tick: &Tick) -> Option<(String, f64, String)> {
        if tick.tick_type != TickType::SensorReading {
            return None;
        }
        let sensor_type = tick.meta.get("sensor_type")?.clone();
        let value: f64 = tick.meta.get("value")?.parse().ok()?;
        let unit = tick.meta.get("unit")?.clone();
        Some((sensor_type, value, unit))
    }

    pub fn to_meta_map(tick: &Tick) -> HashMap<String, String> {
        tick.meta.clone()
    }
}

pub struct TickFormatter;

impl TickFormatter {
    pub fn format_for_room(tick: &Tick, room: &str) -> String {
        format!(
            "[{}:{}] <{}> [{}] {}",
            room, tick.timestamp_ms, tick.agent, tick.tick_type, tick.content
        )
    }

    pub fn format_compact(tick: &Tick) -> String {
        let content_preview = if tick.content.len() > 60 {
            format!("{}…", &tick.content[..57])
        } else {
            tick.content.clone()
        };
        format!("[{}] {}", tick.tick_type, content_preview)
    }

    pub fn format_verbose(tick: &Tick) -> String {
        let meta_str: Vec<String> = tick
            .meta
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        format!(
            "Tick {{\n  id: {}\n  room: {}\n  agent: {}\n  type: {}\n  ts: {}ms\n  content: {}\n  meta: {}\n}}",
            tick.id,
            tick.room,
            tick.agent,
            tick.tick_type,
            tick.timestamp_ms,
            tick.content,
            meta_str.join(", "),
        )
    }

    pub fn format_batch(ticks: &[Tick]) -> String {
        ticks
            .iter()
            .map(Self::format_compact)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_text() {
        let tick = TileToTickMapper::map_text("hello world", 0);
        assert_eq!(tick.content, "hello world");
        assert_eq!(tick.tick_type, TickType::TextLine);
        assert_eq!(tick.meta.get("index").unwrap(), "0");
    }

    #[test]
    fn test_map_text_with_index() {
        let tick = TileToTickMapper::map_text("line two", 5);
        assert_eq!(tick.meta.get("index").unwrap(), "5");
    }

    #[test]
    fn test_map_sensor() {
        let tick = TileToTickMapper::map_sensor("temperature", 23.5, "°C", 1000);
        assert_eq!(tick.tick_type, TickType::SensorReading);
        assert_eq!(tick.timestamp_ms, 1000);
        assert_eq!(tick.meta.get("sensor_type").unwrap(), "temperature");
        assert_eq!(tick.meta.get("value").unwrap(), "23.5");
        assert_eq!(tick.meta.get("unit").unwrap(), "°C");
        assert!(tick.content.contains("23.5"));
    }

    #[test]
    fn test_map_code() {
        let tick = TileToTickMapper::map_code("rust", "function", "main", "fn main() {}");
        assert_eq!(tick.tick_type, TickType::CodeBlock);
        assert_eq!(tick.content, "fn main() {}");
        assert_eq!(tick.meta.get("language").unwrap(), "rust");
        assert_eq!(tick.meta.get("name").unwrap(), "main");
    }

    #[test]
    fn test_map_data_row() {
        let tick = TileToTickMapper::map_data_row(r#"{"x": 1}"#, 3);
        assert_eq!(tick.tick_type, TickType::DataRow);
        assert_eq!(tick.meta.get("index").unwrap(), "3");
        assert_eq!(tick.content, r#"{"x": 1}"#);
    }

    #[test]
    fn test_map_subtitle() {
        let tick = TileToTickMapper::map_subtitle("Hello", 1000, 3000);
        assert_eq!(tick.tick_type, TickType::SubtitleEntry);
        assert_eq!(tick.content, "Hello");
        assert_eq!(tick.meta.get("start_ms").unwrap(), "1000");
        assert_eq!(tick.meta.get("end_ms").unwrap(), "3000");
    }

    #[test]
    fn test_map_generic_known_kind() {
        let tick = TileToTickMapper::map_generic("booting", "status");
        assert_eq!(tick.tick_type, TickType::Status);
        assert_eq!(tick.content, "booting");
    }

    #[test]
    fn test_map_generic_unknown_kind() {
        let tick = TileToTickMapper::map_generic("stuff", "unknown_type");
        assert_eq!(tick.tick_type, TickType::Status); // fallback
    }

    #[test]
    fn test_to_text() {
        let tick = TileToTickMapper::map_text("hello", 0);
        assert_eq!(TickToTileMapper::to_text(&tick), "hello");
    }

    #[test]
    fn test_to_sensor_values() {
        let tick = TileToTickMapper::map_sensor("humidity", 65.0, "%", 2000);
        let (s, v, u) = TickToTileMapper::to_sensor_values(&tick).unwrap();
        assert_eq!(s, "humidity");
        assert!((v - 65.0).abs() < f64::EPSILON);
        assert_eq!(u, "%");
    }

    #[test]
    fn test_to_sensor_values_wrong_type() {
        let tick = TileToTickMapper::map_text("not a sensor", 0);
        assert!(TickToTileMapper::to_sensor_values(&tick).is_none());
    }

    #[test]
    fn test_to_meta_map() {
        let tick = TileToTickMapper::map_code("python", "class", "Foo", "pass");
        let meta = TickToTileMapper::to_meta_map(&tick);
        assert_eq!(meta.get("language").unwrap(), "python");
        assert_eq!(meta.get("kind").unwrap(), "class");
    }

    #[test]
    fn test_format_for_room() {
        let mut tick = TileToTickMapper::map_text("msg", 0);
        tick.room = "lobby".into();
        tick.agent = "alice".into();
        let s = TickFormatter::format_for_room(&tick, "lobby");
        assert!(s.starts_with("[lobby:"));
        assert!(s.contains("<alice>"));
        assert!(s.contains("text_line"));
        assert!(s.contains("msg"));
    }

    #[test]
    fn test_format_compact() {
        let tick = TileToTickMapper::map_text("short", 0);
        let s = TickFormatter::format_compact(&tick);
        assert!(s.starts_with("[text_line] short"));
    }

    #[test]
    fn test_format_compact_truncation() {
        let long = "x".repeat(100);
        let tick = TileToTickMapper::map_text(&long, 0);
        let s = TickFormatter::format_compact(&tick);
        assert!(s.contains('…'));
    }

    #[test]
    fn test_format_verbose() {
        let mut tick = TileToTickMapper::map_sensor("temp", 20.0, "C", 500);
        tick.room = "kitchen".into();
        tick.agent = "bot".into();
        let s = TickFormatter::format_verbose(&tick);
        assert!(s.contains("Tick {"));
        assert!(s.contains("kitchen"));
        assert!(s.contains("bot"));
        assert!(s.contains("sensor_reading"));
        assert!(s.contains("temp"));
    }

    #[test]
    fn test_format_batch() {
        let t1 = TileToTickMapper::map_text("a", 0);
        let t2 = TileToTickMapper::map_text("b", 1);
        let s = TickFormatter::format_batch(&[t1, t2]);
        let lines: Vec<&str> = s.lines().collect();
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn test_tick_serialization_roundtrip() {
        let tick = TileToTickMapper::map_sensor("pressure", 1013.25, "hPa", 9999);
        let json = serde_json::to_string(&tick).unwrap();
        let back: Tick = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, tick.id);
        assert_eq!(back.tick_type, TickType::SensorReading);
        assert_eq!(back.content, tick.content);
    }

    #[test]
    fn test_tick_type_display() {
        assert_eq!(TickType::TextLine.to_string(), "text_line");
        assert_eq!(TickType::SensorReading.to_string(), "sensor_reading");
        assert_eq!(TickType::CodeBlock.to_string(), "code_block");
        assert_eq!(TickType::Command.to_string(), "command");
    }

    #[test]
    fn test_tick_has_uuid() {
        let tick = TileToTickMapper::map_text("x", 0);
        assert!(!tick.id.is_nil());
    }

    #[test]
    fn test_format_batch_empty() {
        let s = TickFormatter::format_batch(&[]);
        assert!(s.is_empty());
    }
}
