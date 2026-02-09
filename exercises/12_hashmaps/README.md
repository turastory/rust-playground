# 12. 해시맵 (HashMaps)

## 학습 목표

- HashMap<K, V> 생성 및 사용
- 키-값 삽입 및 접근
- Entry API

## HashMap

키-값 쌍을 저장합니다 (JavaScript Object/Map과 유사):

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("blue", 10);
map.insert("red", 50);
```

## Node.js와 비교

| JavaScript | Rust |
|-----------|------|
| `const obj = {}` | `let mut map = HashMap::new()` |
| `obj.key = value` | `map.insert(key, value)` |
| `obj.key` or `obj['key']` | `map.get(&key)` |

## 참고 자료

- [The Rust Book - Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
