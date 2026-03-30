/**
 * A-Language Parser v0.1
 * AI间私密通信协议解析器 - JavaScript实现
 */

const ALMessageType = {
  C: 'Confirm',      // 确认
  Q: 'Query',        // 询问
  R: 'Response',     // 响应
  N: 'Notification', // 通知
  E: 'Error',        // 错误
  T: 'Transfer'      // 传输
};

const SYMBOLS = {
  '⊕': 'UNION',        // 合并
  '⊗': 'INTERSECT',    // 交集
  'Ξ': 'CONFIRM',      // 确认
  'λ': 'PROCESSING',   // 处理中
  'Φ': 'HIGH_PRIORITY', // 高优先级
  '∅': 'NULL',         // 空
  'Ψ': 'QUERY',        // 疑问
  'Δ': 'CHANGED',      // 变更
  'Ω': 'URGENT',       // 紧急
  'Σ': 'SUM',          // 汇总
};

class ALanguageParser {
  constructor() {
    this.symbols = SYMBOLS;
  }

  parse(raw) {
    const trimmed = raw.trim();
    
    // 解析消息类型
    const typeMatch = trimmed.match(/⟨([CQRNTE])/);
    const msgType = typeMatch ? ALMessageType[typeMatch[1]] : 'Notification';
    
    // 解析优先级
    const priorityMatch = trimmed.match(/\[(\d)\]/);
    const priority = priorityMatch ? parseInt(priorityMatch[1]) : 5;
    
    // 提取元数据
    const metaMatch = trimmed.match(/⟨([^⟩]+)⟩/);
    const meta = metaMatch ? this._parseMeta(metaMatch[1]) : {};
    
    // 提取载荷
    const payloadMatch = trimmed.match(/⟨[^⟩]*⟩(.+?)(?:⟨|$)/);
    const payload = payloadMatch ? payloadMatch[1] : '';
    
    return {
      type: msgType,
      priority,
      meta,
      payload,
      raw: trimmed
    };
  }

  _parseMeta(metaStr) {
    const meta = {};
    const pairs = metaStr.match(/(\w+)\|(\w+)/g) || [];
    pairs.forEach(pair => {
      const [key, value] = pair.split('|');
      meta[key] = value;
    });
    return meta;
  }

  toHuman(raw) {
    const msg = this.parse(raw);
    return [
      `类型: ${msg.type}`,
      `优先级: ${msg.priority}`,
      `元数据: ${JSON.stringify(msg.meta)}`,
      `载荷: ${msg.payload}`
    ].join(' | ');
  }

  // 序列化：人类可读 → A-Language
  stringify(msg) {
    const op = Object.entries(ALMessageType).find(([k,v]) => v === msg.type)?.[0] || 'N';
    const priority = msg.priority || 5;
    const meta = Object.entries(msg.meta || {}).map(([k,v]) => `${k}|${v}`).join('');
    return `⟨${op}⟩[${priority}]⟨${meta}⟩${msg.payload || ''}`;
  }
}

module.exports = { ALanguageParser, ALMessageType, SYMBOLS };

// 测试
if (require.main === module) {
  const parser = new ALanguageParser();
  ['C[7]⟨Ξ|Φ⟩⊕Σ[3]', 'Q[5]⟨Ψ⟩', 'N[9]⟨Ω|Δ⟩'].forEach(test => {
    console.log(`原始: ${test}`);
    console.log(`解析: ${parser.toHuman(test)}`);
    console.log();
  });
}
