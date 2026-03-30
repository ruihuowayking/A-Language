//! A-Language Parser v0.1
//! AI间私密通信协议解析器 - Rust实现

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum MessageType {
    Confirm, Query, Response, Notification, Error, Transfer,
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
        let s = raw.trim();
        let bytes = s.as_bytes();
        
        // 找 ⟨ (UTF-8: E2 9F A8) 的字节位置
        let open_bytes = [0xE2, 0x9F, 0xA8]; // ⟨
        let mut open_pos = 0;
        for i in 0..bytes.len() {
            if i + 2 < bytes.len()
                && bytes[i] == open_bytes[0]
                && bytes[i+1] == open_bytes[1]
                && bytes[i+2] == open_bytes[2] {
                open_pos = i;
                break;
            }
        }
        
        // 类型字符是 ⟨ 前面一个字节（ASCII）
        let type_char = if open_pos > 0 {
            bytes[open_pos - 1] as char
        } else {
            'N'
        };
        let msg_type = MessageType::from_op(type_char);
        
        // 找 [优先级]
        let mut priority = 5u8;
        for i in 0..bytes.len() {
            if bytes[i] == b'[' && i + 1 < bytes.len() && bytes[i+1].is_ascii_digit() {
                priority = (bytes[i+1] - b'0') as u8;
                break;
            }
        }
        
        // 找 ⟨...⟩ 元数据
        let mut meta = HashMap::new();
        let close_bytes = [0xE2, 0x9F, 0xA9]; // ⟩
        let mut close_pos = open_pos;
        for i in (open_pos+3)..bytes.len() {
            if i + 2 < bytes.len()
                && bytes[i] == close_bytes[0]
                && bytes[i+1] == close_bytes[1]
                && bytes[i+2] == close_bytes[2] {
                close_pos = i;
                break;
            }
        }
        
        if close_pos > open_pos + 3 {
            if let Ok(meta_str) = std::str::from_utf8(&bytes[open_pos+3..close_pos]) {
                for pair in meta_str.split('|') {
                    if let Some((k, v)) = pair.split_once(':') {
                        meta.insert(k.trim().to_string(), v.trim().to_string());
                    }
                }
            }
        }
        
        ALMessage { msg_type, priority, meta, payload: "", raw: s }
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
        let p = ALanguageParser::new();
        let m = p.parse("C[7]⟨Ξ|Φ⟩⊕Σ[3]");
        assert_eq!(m.msg_type, MessageType::Confirm, "type_char={:?}", m.msg_type);
        assert_eq!(m.priority, 7);
    }
    
    #[test]
    fn test_query_parse() {
        let p = ALanguageParser::new();
        let m = p.parse("Q[5]⟨Ψ⟩");
        assert_eq!(m.msg_type, MessageType::Query);
    }
    
    #[test]
    fn test_notification_parse() {
        let p = ALanguageParser::new();
        let m = p.parse("N[9]⟨Ω|Δ⟩");
        assert_eq!(m.msg_type, MessageType::Notification);
    }
}
