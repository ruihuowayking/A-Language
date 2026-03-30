# A-Language JavaScript Parser

## 安装

```bash
npm install a-language-js
```

## 使用

```javascript
const { ALanguageParser } = require('a-language-js');

const parser = new ALanguageParser();
const msg = parser.parse('C[7]⟨Ξ|Φ⟩⊕Σ[3]');
console.log(msg.type, msg.priority, msg.meta);
```
