# Session State

**Current Goal:** Troubleshoot Linux build environment and Tauri compile errors for Inky Next.

**Status:** Completed.

**Summary of Work:**
- Diagnosed missing `javascriptcoregtk-4.0` due to Ubuntu 24.04+ using WebKit 4.1.
- Installed `libwebkit2gtk-4.1-dev` and symlinked `.pc` and `.so` files to `4.0` equivalents to satisfy Tauri v1 dependencies.
- Updated `tauri` dependency to include `shell-all` and modified `tauri.conf.json` to allow sidecars.
- Successfully compiled and ran `inky-next` via `npm run tauri dev`.
- Updated `README.md` to document the Linux build workarounds.

**Next Steps:**
- User to test the running application locally.
- Consider migrating to Tauri v2 in the future to natively support WebKitGTK 4.1.
