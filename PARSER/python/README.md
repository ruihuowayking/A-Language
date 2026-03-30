# A-Language Python Parser

## 安装

```bash
pip install a-language
```

## 使用

```python
from parser import ALanguageParser

parser = ALanguageParser()
msg = parser.parse("C[7]⟨Ξ|Φ⟩⊕Σ[3]")
print(msg.type, msg.priority, msg.meta)
```
