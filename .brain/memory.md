# Decision Log & Lessons Learned

## Decisions
- **Stack:** Chose SvelteKit + Monaco + Tauri for performance, small binary size, and better Linux stability.
- **Backend Architecture:** Used a stateful Rust backend to track the active `inklecate` process and manage unique temporary directories for compilation sessions.
- **Interactive Mode:** Switched from static compilation (`-c`) to interactive play mode (`-i -p`) to support real-time story choices and streaming output.
- **Sidecar:** Bundled `inklecate` as a sidecar to ensure platform-native execution and avoid WASM limitations for complex Ink scripts.
- **Project Structure:** Isolated the new implementation in an `inky-next` folder to avoid conflicts with the legacy Electron codebase during migration.

## Lessons & Gotchas
- **Sidecar Naming:** Tauri sidecars require a specific naming convention including the target triple (e.g., `inklecate-x86_64-unknown-linux-gnu`).
- **Interactive stdin:** When spawning a sidecar with interactive mode, the process must be kept alive in `AppState` to allow writing choices to its `stdin`.
- **Monaco Sync:** Monaco's internal state needs explicit synchronization when the underlying Svelte store is updated externally (e.g., via "File Open").
- **Tauri Permissions:** The `dialog` and `fs` APIs must be explicitly enabled in `tauri.conf.json` and `Cargo.toml` features.
- **Linux Headless:** Developing Tauri apps in a headless environment requires careful handling of system dependencies (like `libsoup`, `webkit2gtk`) which may not be present for compilation checks.
- **Ubuntu 24.04 WebKit Mismatch:** Tauri v1 explicitly requests `webkit2gtk-4.0`. On modern Ubuntu (24.04+) which only ships `4.1`, you must manually install `libwebkit2gtk-4.1-dev` and symlink both the `.pc` files (for `pkg-config`) and `.so` files (for `rust-lld`) to `4.0` equivalents to compile successfully.
- **Sidecar API Permission:** To use `tauri::api::process::Command` in Rust for spawning sidecars, the `shell-all` or `shell-execute` feature must be present in the `tauri` Cargo dependency, and the `"shell"` allowlist in `tauri.conf.json` must be set to `true` or explicitly allow `"sidecar": true`.
