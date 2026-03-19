# Text Paster

A lightweight, cross-platform desktop application for managing and pasting text presets using customizable keyboard shortcuts. Built with [Tauri](https://tauri.app/) and [Next.js](https://nextjs.org/) for a seamless, native experience.

## Features

- **5 Customizable Text Presets** - Store and quickly access your frequently used text snippets
- **Global Hotkeys** - Two hotkey modes: Numpad (1-5) or Ctrl+1-5 for instant pasting
- **Internationalization** - Full support for German (Deutsch) and English with dynamic language switching
- **Modern UI** - Clean, responsive interface with dark mode support
- **Persistent Storage** - All presets and settings are automatically saved to localStorage
- **Lightweight** - Minimal resource usage thanks to Tauri's efficient architecture

## Requirements

- Node.js 18+
- npm or yarn
- Rust 1.77.2+

## Installation

1. Clone the repository:

```bash
git clone https://github.com/DrSenpai/text-paster.git
cd text-paster
```

2. Install dependencies:

```bash
npm install
```

## Getting Started

Start the development server:

```bash
npm run tauri dev
```

The application will open in a window at approximately 800x600 pixels. The Next.js frontend runs on `http://localhost:3000`.

## Usage

### Adding Presets

1. Enter your text in the preset fields (1-5)
2. Click "Strings speichern" / "Save Strings"
3. Your presets are automatically saved and will persist across sessions

### Using Hotkeys

1. Go to Settings (gear icon)
2. Choose your preferred hotkey mode:
   - **Numpad 1-5**: Use Numpad1-5 to paste presets
   - **Ctrl+1-5**: Use Ctrl+1-5 to paste presets
3. Click "Speichern" / "Save"
4. Use your configured hotkeys anywhere to paste presets globally

### Changing Language

1. Open Settings
2. Select your preferred language (Deutsch / English)
3. The UI will update immediately

## Project Structure

```
text-paster/
├── src/                          # Next.js frontend
│   ├── app/
│   │   ├── page.tsx             # Main application page
│   │   ├── settings/page.tsx    # Settings page
│   │   ├── hooks/useTranslation.ts
│   │   ├── components/
│   │   ├── translations.ts
│   │   └── globals.css
│   └── ...
├── src-tauri/                    # Tauri backend
│   ├── src/main.rs              # Rust command handlers
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── ...
├── package.json
└── README.md
```

## Technology Stack

- **Frontend**: Next.js 16, React, TypeScript, Tailwind CSS
- **Backend**: Tauri 2, Rust
- **Clipboard**: Global hotkey handling via tauri-plugin-global-shortcut
- **Storage**: Browser localStorage for persistence
- **Internationalization**: Custom translation system

## Building for Production

```bash
npm run tauri build
```

The compiled executable will be in `src-tauri/target/release/`.

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Feel free to open issues and pull requests.

---

**Developed by**: [DrSenpai](https://github.com/DrSenpai)  
**Last Updated**: 2026
