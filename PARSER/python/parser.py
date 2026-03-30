"""
A-Language Parser v0.1
AI间私密通信协议解析器
"""

import re
from dataclasses import dataclass
from enum import Enum
from typing import Optional

class MessageType(Enum):
    C = "Confirm"     # 确认
    Q = "Query"       # 询问
    R = "Response"    # 响应
    N = "Notification" # 通知
    E = "Error"       # 错误
    T = "Transfer"    # 传输

@dataclass
class ALMessage:
    type: MessageType
    priority: int
    meta: dict
    payload: str
    raw: str

class ALanguageParser:
    """A-Language 解析器"""
    
    SYMBOLS = {
        '⊕': 'UNION',      # 合并
        '⊗': 'INTERSECT',  # 交集
        'Ξ': 'CONFIRM',     # 确认
        'λ': 'PROCESSING', # 处理中
        'Φ': 'HIGH_PRIORITY', # 高优先级
        '∅': 'NULL',        # 空
        'Ψ': 'QUERY',       # 疑问
        'Δ': 'CHANGED',     # 变更
        'Ω': 'URGENT',      # 紧急
        'Σ': 'SUM',         # 汇总
        '✓': 'SUCCESS',
        '✗': 'FAIL',
    }
    
    def parse(self, raw: str) -> ALMessage:
        """
        解析A-Language消息
        格式: ⟨OP⟩[PRIORITY]⟨META⟩⟨PAYLOAD⟩⟨CHECK⟩
        """
        raw = raw.strip()
        
        # 解析消息类型
        type_match = re.search(r'⟨([CQRNTE])', raw)
        msg_type = MessageType[type_match.group(1)] if type_match else MessageType.N
        
        # 解析优先级
        priority_match = re.search(r'\[(\d)\]', raw)
        priority = int(priority_match.group(1)) if priority_match else 5
        
        # 提取元数据
        meta_match = re.search(r'⟨([^⟩]+)⟩', raw)
        meta = self._parse_meta(meta_match.group(1)) if meta_match else {}
        
        # 提取载荷
        payload_match = re.search(r'⟨[^⟩]*⟩(.+?)(?:⟨|$)', raw)
        payload = payload_match.group(1) if payload_match else ''
        
        return ALMessage(
            type=msg_type,
            priority=priority,
            meta=meta,
            payload=payload,
            raw=raw
        )
    
    def _parse_meta(self, meta_str: str) -> dict:
        """解析元数据区域"""
        meta = {}
        for k, v in re.findall(r'(\w+)\|(\w+)', meta_str):
            meta[k] = v
        return meta
    
    def to_human(self, raw: str) -> str:
        """将A-Language转为人类可读描述"""
        msg = self.parse(raw)
        parts = [
            f"类型: {msg.type.value}",
            f"优先级: {msg.priority}",
            f"元数据: {msg.meta}",
            f"载荷: {msg.payload}",
        ]
        return " | ".join(parts)

if __name__ == "__main__":
    parser = ALanguageParser()
    
    # 测试
    test_cases = [
        "C[7]⟨Ξ|Φ⟩⊕Σ[3]",
        "Q[5]⟨Ψ⟩",
        "N[9]⟨Ω|Δ⟩",
    ]
    
    for test in test_cases:
        print(f"原始: {test}")
        print(f"解析: {parser.to_human(test)}")
        print()
