# Advanced Note Taking App (Rust + egui)

A modern, cross-platform note-taking app written in Rust using the `eframe`/`egui` GUI library. Supports creating, editing, deleting, and searching notes with a responsive, dark/light mode interface.

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain recommended)

### Installation

1. **Clone the repository:**

   ```sh
   git clone https://github.com/ChrisSch-dev/advanced_note_app.git
   cd advanced_note_app
   ```

2. **Build and run:**

   ```sh
   cargo run
   ```

   This will launch the app window.

---

## 🖥️ Usage

- **Sidebar**
  - `➕ New Note`: Create a new note.
  - Click on a note title to view or edit it.
  - Use the search bar to filter notes by title or content.

- **Note Editor**
  - Enter a title and content for your note.
  - `💾 Save` to save your changes.
  - `❌ Cancel` to discard edits.

- **Note Viewer**
  - View note details, creation, and last edited timestamps.
  - `✏️ Edit` to modify the note.
  - `🗑️ Delete` to remove the note.

- **Theme Toggle**
  - Top right icon (`🌙`/`🔆`) toggles dark/light mode.

---

## 🗃️ File Structure

```
src/
  ├── main.rs        # Entry point
  ├── app.rs         # Main app logic and GUI
  ├── note.rs        # Note struct and logic
  ├── storage.rs     # (Stub) Persistence layer
  └── theme.rs       # Theme/dark mode handling
```

---

## 💾 Persistence

All notes are stored in a JSON File.

---


## 🤝 Contributing

Contributions are welcome! Feel free to open issues or PRs for features, bugfixes, or ideas.

---

## 📄 License

[MIT](./LICENSE)