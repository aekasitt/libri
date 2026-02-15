# Libri Speed Reader

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://github.com/aekasitt/libri/blob/master/LICENSE)
[![Top](https://img.shields.io/github/languages/top/aekasitt/libri)](https://github.com/aekasitt/libri)
[![Languages](https://img.shields.io/github/languages/count/aekasitt/libri)](https://github.com/aekasitt/libri)
[![Size](https://img.shields.io/github/repo-size/aekasitt/libri)](https://github.com/aekasitt/libri)
[![Last commit](https://img.shields.io/github/last-commit/aekasitt/libri/master)](https://github.com/aekasitt/libri)
[![Fork](https://img.shields.io/badge/fork-filipesabella/speed--reader-beige?logo=github)](https://github.com/filipsabella/speed-reader)
[![Fork](https://img.shields.io/badge/fork-lizidev/leptos--devtools-beige?logo=github)](https://github.com/lizidev/leptos-devtools)

[![Libri banner](https://github.com/aekasitt/libri/blob/master/static/libri-banner.svg)](static/libri-banner.svg)

### Technical stack

Built with Rust, Leptos, and WebAssembly.

### Usage

#### Activating the Speed Reader

There are three ways to activate Libri:

1. **Extension Icon**: Select text on any webpage, then click the Libri extension icon
2. **Keyboard Shortcut**: Select text and press `Ctrl+Shift+L` (or `Cmd+Shift+L` on Mac)
3. **Context Menu**: Right-click selected text and choose "Speed Read Selected Text"

#### Controls

Once activated, use these controls:

| Key | Action |
|-----|--------|
| `Space` | Pause/Resume |
| `Escape` | Close reader |
| `↑` | Increase speed |
| `↓` | Decrease speed |
| `←` | Previous word (when paused) |
| `→` | Next word (when paused) |

You can also click the `+` and `-` buttons to adjust speed, or click the background to close.


### Configurations

Settings can be customized through the extension (settings UI coming soon). Default settings:

- **Speed**: 400 WPM
- **Font**: Monospace
- **Colors**: Dark theme with orange highlight
- **Size**: 90% width, auto height

### Contributions

#### Prerequisites

TODO: Probably need language-aware word-splitting

#### Getting started

1. **Clone the repository**

```bash
git clone https://github.com/aekasitt/libri.git
cd libri
```

2. **Install dependencies**

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install wasm target
rustup target add wasm32-unknown-unknown

# Install Trunk (WASM build tool)
cargo install trunk

# Install Node dependencies
cd extension
pnpm install
```

3. **Build the extension**

```bash
pnpm build
```

4. **Load the extension in Chrome**

- Open `chrome://extensions` in your browser
- Enable "Developer mode" (toggle in top-right corner)
- Click "Load unpacked"
- Select the `extension/dist` directory

## Acknowledgments

- [filipesabella/speed-reader](https://github.com/filipesabella/speed-reader)
- [lizidev/leptos-devtools](https://github.com/lizidev/leptos-devtools)

## License

This project is licensed under the terms of the MIT license.
