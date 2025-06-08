# Potato Browser

**Potato Browser** is a minimal, scriptable Rust-based browser using the Chromium Embedded Framework (CEF).

---

## 🚀 Features

- Embeds Chromium in a native Rust window
- Uses [`cef-rs`](https://github.com/tauri-apps/cef-rs) bindings
- Loads DuckDuckGo by default
- Good starting point for CEF experimentation

---

## 🧰 Requirements

- Rust (latest stable)
- Git, CMake, Python3
- CEF binaries from `cef-rs`

---

## 📦 Setup

### 1. Clone and enter the project

```sh
git clone https://github.com/YOUR_USERNAME/PotatoBrowser.git
cd PotatoBrowser
```

```sh
cargo run -p export-cef-dir -- --force $HOME/.local/share/cef
```


```sh
# Potato Browser CEF setup
export CEF_PATH="$HOME/.local/share/cef"
export LD_LIBRARY_PATH="$CEF_PATH/Release:$LD_LIBRARY_PATH"

```

`cargo run -p potato-browser`
