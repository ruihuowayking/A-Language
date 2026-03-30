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
        
        // 找第一个 ⟨，然后看它前面的字符是什么类型
        let open_angle = '⟨';
        let close_angle = '⟩';
        
        // 找到 ⟨ 的字符索引
        let mut char_indices = trimmed.char_indices();
        let mut type_char = 'N';
        let mut angle_char_idx = 0;
        
        for (i, c) in char_indices.clone().enumerate() {
            if c.1 == open_angle {
                // i 是字符索引，c.0 是字节位置
                angle_char_idx = i;
                if i > 0 {
                    // 类型字符是 ⟨ 前一个字符
                    if let Some(prev) = trimmed.chars().nth(i - 1) {
                        type_char = prev;
                    }
                }
                break;
            }
        }
        
        let msg_type = MessageType::from_op(type_char);
        
        // 找 [优先级]
        let mut priority = 5u8;
        let chars: Vec<char> = trimmed.chars().collect();
        for i in 0..chars.len() {
            if chars[i] == '[' {
                if i + 1 < chars.len() && chars[i + 1].is_ascii_digit() {
                    priority = chars[i + 1].to_digit(10).unwrap_or(5) as u8;
                    break;
                }
            }
        }
        
        // 找 ⟨...⟩ 提取元数据
        let mut meta = HashMap::new();
        let all_chars: Vec<char> = trimmed.chars().collect();
        if angle_char_idx < all_chars.len() {
            // 从 ⟨ 的下一个字符开始，找 ⟩
            let mut meta_start = angle_char_idx + 1;
            let mut meta_end = meta_start;
            for i in meta_start..all_chars.len() {
                if all_chars[i] == close_angle {
                    meta_end = i;
                    break;
                }
            }
            if meta_end > meta_start {
                let meta_str: String = all_chars[meta_start..meta_end].iter().collect();
                for pair in meta_str.split('|') {
                    if let Some((k, v)) = pair.split_once(':') {
                        meta.insert(k.trim().to_string(), v.trim().to_string());
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
