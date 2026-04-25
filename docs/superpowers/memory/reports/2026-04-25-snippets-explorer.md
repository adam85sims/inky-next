# Memory Update Report: Snippets & File Explorer

**Date:** 2026-04-25
**Verified Commit:** d3c0446

## Summary of Changes
- Established foundational multi-file project management patterns in Inky Next.
- Documented the Svelte component responsibilities in `module-cards.md`.
- Formalized the file synchronization and snippet insertion contracts in `contracts.md`.
- Added a lesson on robust JSON parsing for compiler output in `lessons/robust-inklecate-json-parsing.md`.

## Durable Knowledge Captured
- **Module Cards:** Responsibilities for `Editor`, `FileExplorer`, and `SnippetMenu`.
- **Contracts:** Multi-file sync logic and the `window` bridge for snippet insertion.
- **Lessons:** Concatenated JSON parsing for compiler stdout.

## Doc Gaps & Uncertainties
- **Path Management:** Currently uses simple relative paths (`./`); may need absolute path normalization in the future.
- **Main Ink Designation:** The logic for identifying the "root" file is currently implicit or manual; needs a more robust "Set as Main Ink" feature.
