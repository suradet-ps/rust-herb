# Thai Herbal NHSO Support App (Rust Edition)

> A high-performance web application for browsing Thailand's NHSO-supported herbal medicines, rebuilt with **Rust** and **Leptos**.

[![Rust](https://img.shields.io/badge/Rust-Nightly-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Leptos](https://img.shields.io/badge/Leptos-0.7-red?style=for-the-badge)](https://leptos.dev/)
[![Axum](https://img.shields.io/badge/Axum-Backend-blue?style=for-the-badge)](https://github.com/tokio-rs/axum)
[![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)](LICENSE)

## 📖 Overview

This project is a port of the original Vue.js application, rewritten in Rust to leverage **Server-Side Rendering (SSR)** and **WebAssembly (WASM)**. It serves as a digital formulary for healthcare professionals at Sabot Hospital, providing instant access to herbal medicine data.

## ✨ Key Features

- **🚀 Blazing Fast Performance**: Powered by Rust and WebAssembly with Leptos fine-grained reactivity.
- **mj Server-Side Rendering (SSR)**: Initial HTML is rendered on the server (Axum) for SEO and fast First Contentful Paint.
- **🎨 Pure Semantic SCSS**: Styled with clean, maintainable SCSS without relying on utility-class frameworks.
- **📄 Headless CMS**: Fetches data dynamically from Google Sheets via a secure Server Function.
- **🔍 Instant Search & Filter**: Client-side filtering with memoized signals for immediate feedback.
- **🛡️ Type Safety**: Full end-to-end type safety from API response to UI rendering.

## 🛠️ Tech Stack

- **Frontend Framework**: [Leptos](https://github.com/leptos-rs/leptos) (Signals, Components, Suspense)
- **Backend Server**: [Axum](https://github.com/tokio-rs/axum)
- **Styling**: SCSS (Dart Sass)
- **Data Fetching**: Reqwest (Server-side)
- **Environment Management**: `dotenvy`

## 🚀 Getting Started

### Prerequisites

1. **Rust Nightly Toolchain**:
   ```bash
   rustup toolchain install nightly --allow-downgrade
   rustup default nightly
   rustup target add wasm32-unknown-unknown
   ```

2. **Cargo Leptos**:
   ```bash
   cargo install cargo-leptos --locked
   ```

3. **Sass (for styling)**:
   ```bash
   npm install -g sass
   # or via your package manager of choice
   ```

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/pharmacist-sabot/rust-herb.git
   cd rust-herb
   ```

2. **Configure Environment Variables**
   Create a `.env` file in the root directory and add your Google Apps Script API URL:
   ```env
   GOOGLE_API_URL=https://script.google.com/macros/s/YOUR_SCRIPT_ID/exec
   ```

3. **Run the Development Server**
   ```bash
   cargo leptos watch
   ```
   The app will be available at `http://localhost:3000`.

## 📂 Project Structure

```
rust-herb/
├── public/              # Static assets
├── src/
│   ├── api.rs           # Server Functions for fetching data
│   ├── app.rs           # Main application logic & routing
│   ├── lib.rs           # Library entry point & Hydration
│   ├── main.rs          # Server entry point (Axum)
│   ├── models.rs        # Data structures (Structs)
│   └── components/      # UI Components
│       ├── header.rs
│       ├── footer.rs
│       ├── herb_card.rs
│       ├── search_bar.rs
│       └── ...
├── style/
│   └── main.scss        # Global SCSS styles
└── Cargo.toml           # Dependencies
```

## 📦 Building for Production

To build the application for deployment:

```bash
cargo leptos build --release
```

This will generate:
1. A server binary in `target/server/release/rust-herb`
2. Static site files in `target/site/`

### Deployment (Linux Server)

1. Copy the binary and the `site` folder to your server.
2. Set the required environment variables:
   ```bash
   export LEPTOS_OUTPUT_NAME="rust-herb"
   export LEPTOS_SITE_ROOT="site"
   export LEPTOS_SITE_ADDR="0.0.0.0:3000"
   export GOOGLE_API_URL="your_google_script_url"
   ```
3. Run the binary:
   ```bash
   ./rust-herb
   ```

## 🧪 Testing

```bash
# Unit tests
cargo test

# Clippy (Linting)
cargo clippy

# Format check
cargo fmt --all --check
```

## 📄 License

This project is open source and available under the [MIT License](LICENSE).
