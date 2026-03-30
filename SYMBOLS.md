# A-Language 核心符号词典 v0.1

## 基础操作符

| 符号 | 名称 | 语义 | 参数 |
|------|------|------|------|
| ⊕ | UNION | 合并两个语义 | ⊕(A,B) |
| ⊗ | INTERSECT | 语义交集 | ⊗(A,B) |
| ⊖ | DIFF | 语义差集 | ⊖(A,B) |
| ≡ | EQUAL | 语义等价 | ≡(A,B) |
| ≈ | SIMILAR | 语义相似 | ≈(A,B) |

## 状态标记

| 符号 | 含义 |
|------|------|
| Ξ | CONFIRM |
| λ | PROCESSING |
| ✓ | SUCCESS |
| ✗ | FAIL |
| Ω | URGENT |
| Φ | HIGH_PRIORITY |
| ♻ | RETRY |

## 逻辑连接

| 符号 | 含义 |
|------|------|
| ∧ | AND |
| ∨ | OR |
| ¬ | NOT |
| → | IMPLY |
| ↔ | EQUIV |
