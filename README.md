# Dojo — A Powerful Rust-based To-Do List 🥷✨

A sleek and fast README for **Dojo** — a simple, secure, and fast To-Do application written in Rust.

```
 ____   ____   ____   ____
|  _ \ |  _ \ |  _ \ |  _ \
| | | || | | || | | || | | |
| |_| || |_| || |_| || |_| |
|____/ |____/ |____/ |____/
   D O J O   -   T O D O
```

---

![crates.io](https://img.shields.io/badge/rust-stable-orange) ![license](https://img.shields.io/badge/license-MIT-blue) ![build](https://img.shields.io/badge/build-passing-brightgreen)

---

## 🔥 Overview
**Dojo** is a lightweight and fast task management (TODO) application built with Rust. Its goal is speed, simplicity, and a clean CLI user experience, making it perfect for developers or anyone looking for a small but powerful tool.

---

## 🚀 Features
- Extremely fast and lightweight (Rust 🦀).
- Clean and simple command-line interface.
- Local storage (SQLite depending on implementation).
- Basic operations: add, list, done, remove.
- Extensible and fully tested with `cargo test`.

---

## 🧩 Installation (for developers)
To install from source:

```bash
# Clone the repository
git clone https://github.com/Hoseinpy/dojo.git
cd dojo

# Build and run (development mode)
cargo run -- <command> [args]

# Or build a release version and use the binary
cargo build --release
# Binary will be in target/release/dojo
```

To install via `cargo install` (when published):

```bash
cargo install --git https://github.com/Hoseinpy/dojo.git --rev main
# Or from crates.io when available:
# cargo install dojo
```

---

## ⚙️ Usage (CLI)
Quick examples for using Dojo:

```bash
# Add a new task
dojo add Buy groceries

# List all tasks
dojo list

# Mark task as done
dojo done 3

# Remove a task
dojo remove 5
```

---

## 🧪 Testing
All tests can be run with `cargo test`:

```bash
# Run the test suite
cargo test
```

Please make sure all tests pass before submitting a Pull Request ✅

---

## 🛠️ Contributing
Want to help? Awesome! Follow these steps for a smooth contribution process:

1. Fork and work on a separate branch (`feature/<something>`).
2. Run `cargo fmt` and `cargo clippy` before your PR.
3. Ensure all tests pass.
4. Submit a clear PR explaining what issue it fixes or what feature it adds.

Keep issues/PRs simple and clear: what happened, steps to reproduce, and suggested solution.

---

## 📜 License
This project is licensed under **MIT** — free to use and modify.

---

## ❤️ Thanks
If Dojo helped you or you like it, give it a ⭐. Report bugs or suggest features via issues
