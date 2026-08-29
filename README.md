# Thai Herbal NHSO Support App

```
██████╗ ██╗   ██╗ ██████╗████████╗██╗  ██╗███████╗██████╗ ██████╗
██╔══██╗██║   ██║██╔════╝╚══██╔══╝██║  ██║██╔════╝██╔══██╗██╔══██╗
██████╔╝██║   ██║███████╗   ██║   ███████║█████╗  ██████╔╝██████╔╝
██╔══██╗██║   ██║╚════██║   ██║   ██║  ██║██╔══╝  ██╔══██╗██╔══██╗
██║  ██║╚██████╔╝██████╔╝   ██║   ██║  ██║███████╗██║  ██║██████╔╝
╚═╝  ╚═╝ ╚═════╝ ╚═════╝   ╚═╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝╚═════╝
```

---

## ◆ PULSE

The formulary should follow the clinician, not the other way around.
This is the Thai National Health Security Office's supported herbal
medicine list, rebuilt in Rust - Leptos SSR rendering the first paint on
the server, WASM hydrating the search, and a Google Sheets-backed CMS
feeding the page without a database in sight. A digital formulary for
Sabot Hospital's healthcare professionals: instant search, memoized
filtering, and answers from one trusted source.

| SSR ▣ | Instant search ▣ | Headless CMS ▣ | Type-safe ▣ |
|---|---|---|---|

*The formulary - render, search, filter, fetch - is sealed.*

> Built with Leptos 0.7 + Axum, styled in pure SCSS, fed by Google
> Sheets through a server function - the Vue original, ported honest.
>
> **suradet-ps**, artifact keeper

---

## ◆ IGNITION

Nightly, one tool, one command.

```
⟫ rustup toolchain install nightly --allow-downgrade
⟫ rustup default nightly
⟫ rustup target add wasm32-unknown-unknown
⟫ cargo install cargo-leptos --locked
⟫ cargo leptos watch
```

Open [http://localhost:3000](http://localhost:3000).

<details>
<summary>Environment</summary>

A `.env` file with the Google Apps Script endpoint:

```
GOOGLE_API_URL=https://script.google.com/macros/s/YOUR_SCRIPT_ID/exec
```

Sass (for styling): `⟫ npm install -g sass`

</details>

Production: `⟫ cargo leptos build --release` - a server binary in
`target/server/release/` plus static site files in `target/site/`, run
with `LEPTOS_OUTPUT_NAME`, `LEPTOS_SITE_ROOT`, `LEPTOS_SITE_ADDR`, and
`GOOGLE_API_URL` set.

---

## ◆ ANATOMY

One server, one signal graph, one sheet of truth.

- **Renders** - Axum serves the first HTML on the server - the
  formulary's first paint needs no WASM boot to exist - and Leptos
  hydrates the rest with fine-grained reactivity.
- **Fetches** - a server function pulls the herb data from Google
  Sheets via `GOOGLE_API_URL`; the CMS is a spreadsheet, the database
  is wherever the formulary already lives.
- **Searches** - client-side filtering over memoized signals answers
  keystroke by keystroke - the full list, filtered in place, no round
  trip per letter.
- **Types** - the API response flows into typed structs (`models.rs`)
  and out through typed components - end to end, no `any` at the
  seams.
- **Wears** - pure semantic SCSS, hand-authored, no utility framework:
  the style layer is as deliberate as the Rust layer.
- **Tests** - `cargo test`, `cargo clippy`, and `cargo fmt --all
  --check` guard the port from the original Vue days.

---

## ◆ RITUALS

**The core ceremony** - the formulary lookup:

1. Open the page. The SSR paint is already the formulary - no spinner,
   no blank screen waiting on WASM.
2. Type into the search bar. The filtered list answers each keystroke,
   memoized and instant.
3. Read the herb card: the NHSO-supported details, rendered from the
   typed model.
4. Trust the source: the data came from the spreadsheet the formulary
   was born in, fetched server-side, never scraped on the client.

**The ceremony of the first paint** - the page renders before it
hydrates. What the clinician sees first is HTML from the server, not a
loading promise.

**The ceremony of the sheet** - no CMS, no database, no API key in the
browser: the content lives in Google Sheets and arrives through a
server function. The formulary's source of truth stays where the
hospital already keeps it.

---

## ◆ ECHOES

**Where this artifact is heading**

```
render   ▸ Axum SSR first paint, Leptos hydration ─────────────────── ▸ sealed
fetch    ▸ Google Sheets via server function ──────────────────────── ▸ sealed
search   ▸ memoized client-side filtering ─────────────────────────── ▸ sealed
style    ▸ semantic SCSS, no utility framework ────────────────────── ▸ sealed
test     ▸ cargo test, clippy, fmt gates ──────────────────────────── ▸ sealed
```

**Raising the artifact** - the port's shape is documented in the
project structure: `api.rs`, `app.rs`, `models.rs`, `components/`.
End-to-end checks live under `end2end/`. Open an issue first to
discuss a change.

**Status** - dependencies are maintained through Renovate; deployment
is a Linux binary plus a static `site/` folder.

---

```
  ─────────────────────────────────────────
   A formulary that renders before it asks
   is a formulary that respects the clock.
  ─────────────────────────────────────────
```

Open source under the [MIT License](LICENSE).