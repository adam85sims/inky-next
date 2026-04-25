![](resources/icon-small.jpg)

# Inky

**Inky** is an editor for [ink](http://www.inklestudios.com/ink), inkle's markup language for writing interactive narrative in games. It's an IDE (integrated development environment) that lets you play in the editor as you write, and fix any bugs in your code.

## Inky Next (Beta)

We are currently migrating Inky to a modern stack: **Tauri + SvelteKit + Monaco Editor**. This version is located in the `inky-next` directory and provides:
- **Improved Performance**: Native Rust backend using Tauri.
- **Modern Editor**: Powered by Monaco (the engine behind VS Code).
- **Theme Persistence**: Light and dark mode support that stays consistent across restarts.
- **Cross-platform**: Leaner builds for Mac, Windows, and Linux.

---

## Features

- **Play as you write**: The play pane remembers the choices that you made, so when Inky recompiles, it fast-forwards to the last point you were at in the flow.
- **Syntax highlighting**
- **As-you-type error highlighting**: Inky is constantly compiling, allowing you to fix errors early.
- **Issue browser**: Lists errors, warnings and TODOs in your **ink**, and allows you to jump to the exact line number and file in the source.
- **Support multi-file projects**: Inky automatically infers your story's structure from the `INCLUDE` lines.
- **Export to JSON**: Compiled JSON format for use in other runtimes like [inkjs](https://github.com/y-lohse/inkjs).

---

## Help develop Inky!

### Prerequisites
- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/tools/install) (for Inky Next)

### Building Inky Next (Tauri version)
1. `cd inky-next`
2. `npm install`
3. `npm run tauri dev` to run in development mode.
4. `npm run tauri build` to create a production bundle.

### Building Legacy Inky (Electron version)
- Install Node.js
- `cd app`
- `npm install`
- `npm start`

---

## License
**Inky** and **ink** are released under the MIT license. Copyright (c) 2016-2026 inkle Ltd.
