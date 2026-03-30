//! A-Language Parser v0.1
//! AI间私密通信协议解析器 - Rust实现

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum MessageType {
    Confirm,
    Query,
    Response,
    Notification,
    Error,
    Transfer,
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
    pub fn new() -> Self { ALanguageParser }

    pub fn parse<'a>(&self, raw: &'a str) -> ALMessage<'a> {
        let trimmed = raw.trim();
        
        // 找 ⟨符号 的位置，类型字母在它前面
        let open_angle = "⟨";
        
        // 找到第一个 ⟨，类型字母就在它前面
        let type_idx = match trimmed.find(open_angle) {
            Some(idx) if idx > 0 => idx - 1,
            _ => 0,
        };
        let type_char = trimmed.chars().nth(type_idx).unwrap_or('N');
        let msg_type = MessageType::from_op(type_char);
        
        // 找 [优先级]
        let mut priority = 5u8;
        if let Some(start) = trimmed.find('[') {
            if let Some(end) = trimmed.find(']') {
                if end > start {
                    priority = trimmed[start+1..end].parse().unwrap_or(5);
                }
            }
        }
        
        // 找 ⟨...⟩ 提取元数据
        let mut meta = HashMap::new();
        if let Some(start) = trimmed.find(open_angle) {
            if let Some(end) = trimmed.find("⟩") {
                if end > start + 3 {
                    let meta_str = &trimmed[start+1..end];
                    for pair in meta_str.split('|') {
                        if let Some((k, v)) = pair.split_once(':') {
                            meta.insert(k.to_string(), v.to_string());
                        }
                    }
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
