# A-Language Rust Parser

## 安装

```toml
[dependencies]
a-language = { git = "https://github.com/ruihuowayking/A-Language", package = "a-language" }
```

## 使用

```rust
use a_language::{ALanguageParser, MessageType};

let parser = ALanguageParser::new();
let msg = parser.parse("C[7]⟨Ξ|Φ⟩⊕Σ[3]");
assert!(matches!(msg.msg_type, MessageType::Confirm));
```
