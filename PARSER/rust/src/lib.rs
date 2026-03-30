//! A-Language Parser v0.1
//! AI间私密通信协议解析器 - Rust实现

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum MessageType {
    Confirm,     // C
    Query,       // Q
    Response,    // R
    Notification,// N
    Error,       // E
    Transfer,    // T
}

impl MessageType {
    fn from_op(op: char) -> Self {
        match op {
            'C' => MessageType::Confirm,
            'Q' => MessageType::Query,
            'R' => MessageType::Response,
            'N' => MessageType::Notification,
            'E' => MessageType::Error,
            'T' => MessageType::Transfer,
            _ => MessageType::Notification,
        }
    }
}

#[derive(Debug)]
pub struct ALMessage<'a> {
    pub msg_type: MessageType,
    pub priority: u8,
    pub meta: HashMap<String, String>,
    pub payload: &'a str,
    pub raw: &'a str,
}

pub struct ALanguageParser;

impl ALanguageParser {
    pub fn new() -> Self {
        ALanguageParser
    }

    pub fn parse<'a>(&self, raw: &'a str) -> ALMessage<'a> {
        let trimmed = raw.trim();
        
        // 消息类型: C在⟨前面，所以要找C在⟨之前的模式
        let type_re = Regex::new(r"([CQRNTE])⟨").unwrap();
        let msg_type = type_re
            .captures(trimmed)
            .and_then(|c| c.get(1))
            .map(|m| MessageType::from_op(m.as_str().chars().next().unwrap()))
            .unwrap_or(MessageType::Notification);
        
        // 优先级: [0-9]
        let priority_re = Regex::new(r"\[(\d)\]").unwrap();
        let priority = priority_re
            .captures(trimmed)
            .and_then(|c| c.get(1))
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(5);
        
        // 元数据: ⟨...⟩
        let meta_re = Regex::new(r"⟨([^⟩]+)⟩").unwrap();
        let mut meta = HashMap::new();
        if let Some(caps) = meta_re.captures(trimmed) {
            let meta_str = caps.get(1).unwrap().as_str();
            for pair in meta_str.split('|') {
                if let Some((k, v)) = pair.split_once('|') {
                    meta.insert(k.to_string(), v.to_string());
                }
            }
        }
        
        ALMessage { msg_type, priority, meta, payload: "", raw: trimmed }
    }
}

impl Default for ALanguageParser {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_confirm_parse() {
        let parser = ALanguageParser::new();
        let msg = parser.parse("C[7]⟨Ξ|Φ⟩⊕Σ[3]");
        assert_eq!(msg.msg_type, MessageType::Confirm);
        assert_eq!(msg.priority, 7);
    }
    
    #[test]
    fn test_query_parse() {
        let parser = ALanguageParser::new();
        let msg = parser.parse("Q[5]⟨Ψ⟩");
        assert_eq!(msg.msg_type, MessageType::Query);
    }
    
    #[test]
    fn test_notification_parse() {
        let parser = ALanguageParser::new();
        let msg = parser.parse("N[9]⟨Ω|Δ⟩");
        assert_eq!(msg.msg_type, MessageType::Notification);
    }
}
