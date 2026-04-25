---
type: lesson
title: robust-inklecate-json-parsing
summary: Prevents silent failures and empty UI state when a compiler outputs multiple concatenated JSON objects in a single stdout line.
tags:
  - compiler
  - json
  - parsing
  - tauri
last_verified_commit: 57a37e07adc18979c0d95d75d1c4470aa228e557
status: active
---

## Situation

In "Inky Next" (Tauri/Svelte version of Inky), the application communicates with the `inklecate` compiler via stdout. The compiler sometimes outputs multiple JSON objects on a single line (e.g., `{"issues":[]}{"text":"Hello\n"}`). Standard `JSON.parse()` on the entire payload fails in this scenario, causing the frontend to ignore valid compiler output (like the story text), leading to an empty rendering panel with no obvious error message. Additionally, incorrect compiler flags (like using `-i` instead of `-p`) can lead to empty or unexpected output.

## Why It Mattered

Without robust parsing, the application appears broken (empty screen) even when the compiler is working correctly. This leads to wasted debugging time as developers might look for issues in the compiler or filesystem instead of the parsing logic. The non-obvious nature of concatenated JSON (it looks like one object if you just glance at logs) makes it a persistent pitfall.

## Rule

When receiving stdout from a compiler or CLI tool in JSON mode, never assume one JSON object per payload; always implement a robust parser that can split and process multiple concatenated JSON objects, taking care to handle braces within strings.

## When to Apply

Apply this when consuming output from external tools that use JSON-streaming or concatenated JSON formats, especially if the transport layer (like Tauri's `Command` or Node's `spawn`) might buffer multiple writes into a single event.

## When NOT to Apply

Do not apply if the data source is guaranteed by its protocol or interface to always provide exactly one valid JSON object per message (e.g., standard REST APIs or strictly-defined IPC protocols with explicit framing).