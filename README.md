# Subculture Save Editor

A small savegame editor for the 1997 game **Sub Culture** by Criterion Games.

Built in **Rust** (a language I do not know ;) ) using **egui / eframe**, this tool allows direct editing of selected player sub's params, weapon ammo, and cargo values by modifying the binary save file.

---

## Features

Currently supported:

- Credits
- Shields
- Hull strength
- Radiation shields
- Ripper rounds
- Depth charges
- Cargo

All edits are performed directly on the save file using known offsets.

---

## How it works

Sub Culture save files store player state as a mostly flat binary structure:
- Little-endian
- Mix of `int32` and `float32`
- No compression or checksums

This editor:
1. Loads the save file into memory
2. Reads known offsets into UI fields
3. Writes modified values back in-place

There's no more magic to that, just simple file editing.

---

## Controls

- **Ctrl+L** — Load save file (opens a file picker)
- **Ctrl+S** — Save changes

---

## Usage

1. Find your saved game in game's root folder (0.sav, 1.sav, auto.sav).
2. Open the save file using the editor.
3. Modify desired values.
4. Save.
5. Load the save in-game.

(!) It's always a good idea to keep a backup of your original save file!

---

## Build & Run

### Build

#### Windows

```bash
cargo build --target x86_64-pc-windows-gnu --release
```

#### Linux

```bash
cargo build --release
```

##### Statically-linked binary

```bash
chmod +x build-linux-static.sh
./build-linux-static.sh
```

#### MacOS (Universal Binary)

```bash
chmod +x build-macos.sh
./build-macos.sh
```

### ... or run a development build directly from source:

```bash
cargo run
```

Dependencies are managed via Cargo.

## Disclaimer

This tool is intended for educational and preservation purposes.
Sub Culture © 1997 Criterion Games.

Use at your own risk.

## Status

This utility has reached a state in which I'm happy with it, so I likely won't be improving it further, but if you have any ideas on what could be added - let me know.
