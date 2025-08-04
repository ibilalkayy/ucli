# 🧰 Younix — Minimalist Unix Command-Line Tools in Rust

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

**Younix** is a collection of simple, fast, and safe Unix command-line tools reimagined in Rust. Perfect for learning systems programming, handling files, and working with text streams using modern tools.

> 🚀 Built in Rust — blazing fast, memory-safe, and beginner-friendly!

---

## 📦 Available Commands

| Command | Description                              |
| ------- | ---------------------------------------- |
| `cat`   | Output contents of a file line-by-line   |
| `ls`    | List directory contents                  |
| `grep`  | Search for matching lines in a file      |
| `view`  | View file content interactively (`less`) |
| `sort`  | Sort lines in a file                     |
| `wc`    | Count lines, words, and bytes in a file  |

### 🔧 Example Usages

```bash
younix cat file.txt
younix ls -a
younix grep "error" logs.txt -i -n
younix wc file.txt -l -w -c
```

Each command supports relevant flags similar to classic Unix tools.

---

## 🦀 Build from Source

```bash
git clone https://github.com/ibilalkayy/younix
cd younix
cargo build --release
```

This will generate a `younix` binary in `target/release/younix`.

---

## 🙌 Contributing

Contributions are welcome! Feel free to open issues, suggest new features, or submit pull requests. Help make Younix a better learning and productivity tool for everyone.
