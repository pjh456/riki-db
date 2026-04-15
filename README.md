# riki-db

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> 从零开始，用 Rust 构建一个简单的、可持久化的键值存储引擎。

**riki-db** 是一个用于学习 Rust 系统编程和存储引擎原理的教学项目。它从一个纯内存的 `HashMap` 开始，逐步演进为一个支持文件持久化、并发访问和日志压缩的嵌入式 KV 存储。

对于从 C++ 转向 Rust 的开发者来说，这个项目是实践 **所有权、生命周期、并发安全、Trait 抽象** 以及 **零成本抽象** 的绝佳练习。

## 特性演进路线 (Roadmap)

| 版本 | 目标 | 关键技术点 | 状态 |
|:---|:---|:---|:---:|
| **v0.1.0** | 内存存储引擎 | `HashMap`、`Option`、API 设计 | 计划中 |
| **v0.2.0** | 单文件持久化 (Append-Only Log) | 序列化 (`bincode`/`serde`)、文件 I/O、`fs2` 文件锁 | 待开发 |
| **v0.3.0** | 并发安全与 Trait 抽象 | `DashMap`、`Arc`、内部可变性、自定义 `KvStore` trait | 待开发 |
| **v0.4.0** | 日志压缩与性能优化 | Bitcask 模型 / LSM-Tree、后台线程、`BTreeMap` | 待开发 |

## 快速开始

### 环境要求
- Rust 稳定版 (1.70+)
- Git

### 克隆与构建
```bash
git clone https://github.com/pjh123/riki-db.git
cd riki-db

# 构建项目
cargo build

# 运行测试
cargo test

# 运行内置 CLI 示例
cargo run -- set mykey "Hello, RikiDB!"
cargo run -- get mykey
```

### 作为库使用

在你的 `Cargo.toml` 中添加依赖（发布后可用）：
```toml
[dependencies]
riki_db = { git = "https://github.com/pjh456/riki-db.git" }
```

然后：
```rust
use riki_db::MemoryStore;

fn main() {
    let mut store = MemoryStore::new();
    store.set("key".to_string(), "value".to_string());
    assert_eq!(store.get("key"), Some(&"value".to_string()));
}
```

## 📁 项目结构
```
riki-db/
├── Cargo.toml
├── README.md
├── .gitignore
├── src/
│   ├── lib.rs          # 库根，对外暴露核心类型
│   ├── main.rs         # CLI 调试入口（仅用于开发测试）
│   ├── error.rs        # 自定义错误类型 (后续版本引入)
│   └── storage/
│       ├── mod.rs      # 存储模块入口
│       ├── memory.rs   # v0.1 内存存储实现
│       └── disk.rs     # v0.2+ 磁盘存储实现
└── tests/
    └── integration.rs  # 集成测试
```

## 版本迭代详解

### v0.1.0 – 纯内存存储
- **学习重点**：
  - 定义 `struct MemoryStore` 包含 `HashMap<String, String>`。
  - 使用 `Option<T>` 处理可能不存在的键。
  - 理解 `&self` 与 `&mut self` 的区别。
- **API**：
  - `fn set(&mut self, key: String, value: String)`
  - `fn get(&self, key: &str) -> Option<&String>`
  - `fn remove(&mut self, key: &str) -> Option<String>`

### v0.2.0 – 追加写日志 (Append-Only Log)
- **学习重点**：
  - 使用 `serde` 和 `bincode` 将命令序列化为二进制。
  - 用 `BufWriter` 提升写入性能，并用 `flush` 强制落盘。
  - 启动时 **重放日志** 恢复状态。
  - 利用 `fs2` 获取文件锁，防止多进程写坏数据。

### v0.3.0 – 并发与抽象
- **学习重点**：
  - 将 `HashMap` 替换为 `DashMap`，实现无锁并发读、分段锁写。
  - 方法接收者从 `&mut self` 变为 `&self`（内部可变性模式）。
  - 定义 `KvStore` trait，让 `MemoryStore` 和 `DiskStore` 实现同一接口。

### v0.4.0 – 压缩与后台任务
- **学习重点**：
  - 实现简单的 **Bitcask** 模型：生成 Hint 文件加速启动。
  - 或者挑战 **LSM-Tree**：用 `BTreeMap` 作为 MemTable，实现 SSTable 合并。
  - 使用 `std::thread` 或 `tokio::spawn` 在后台执行压缩任务。

## 参考资源

- [《Rust 程序设计语言》](https://doc.rust-lang.org/book/) – 官方权威教程
- [PingCAP Talent Plan (TinyKV)](https://github.com/pingcap/talent-plan) – 工业级教学项目
- [Bitcask 论文](https://riak.com/assets/bitcask-intro.pdf) – 简单高效的存储模型
- [《设计数据密集型应用》](https://dataintensive.net/) – 理解存储引擎的绝佳读物

## 贡献

欢迎任何形式的讨论、Issue 和 PR！

## 许可证

MIT © [pjh456]
```
