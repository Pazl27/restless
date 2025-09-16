# 🚀 Restless

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://rustlang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/yourusername/restless)

> A blazingly fast, keyboard-driven terminal user interface for testing REST APIs

Restless is a modern, intuitive TUI (Terminal User Interface) application built in Rust that makes HTTP API testing a breeze. Say goodbye to switching between your terminal and browser – test your APIs directly from the comfort of your command line with a beautiful, responsive interface.

![Restless Demo](docs/demo.gif)

## ✨ Features

- 🎯 **Intuitive TUI**: Clean, organized interface with vim-like navigation
- 🚀 **Fast & Lightweight**: Built in Rust for maximum performance
- 📋 **Multi-Tab Support**: Test multiple endpoints simultaneously
- 🔧 **Full HTTP Support**: GET, POST, PUT, DELETE methods
- 📝 **Request Builder**: Easy header, parameter, and body configuration
- 🎨 **Syntax Highlighting**: JSON response formatting and highlighting
- ⌨️ **Keyboard-Driven**: Complete keyboard navigation for power users
- 💾 **Session Management**: Tab state preservation
- 🛡️ **Error Handling**: Comprehensive error reporting and validation

## 📸 Screenshots

<details>
<summary>Click to view screenshots</summary>

### Main Interface
![Main Interface](docs/main-interface.png)

### Response Viewer
![Response Viewer](docs/response-viewer.png)

### Help Screen
![Help Screen](docs/help-screen.png)

</details>

## 🚀 Quick Start

### Installation

#### From Source (Recommended)
```bash
git clone https://github.com/yourusername/restless.git
cd restless
cargo build --release
./target/release/restless
```

#### Using Cargo
```bash
cargo install restless
restless
```

#### Download Binary
Download the latest release from the [releases page](https://github.com/yourusername/restless/releases).

### System Requirements

- **Terminal**: Any modern terminal emulator
- **Minimum Size**: 80x24 characters
- **Rust Version**: 1.70+ (for building from source)

## 🎮 Usage

### Basic Workflow

1. **Start Restless**: Run `restless` in your terminal
2. **Enter URL**: Press `u` to edit the URL field
3. **Select Method**: Press `m` to choose HTTP method (GET, POST, PUT, DELETE)
4. **Configure Request**: Navigate to Values section and add headers, parameters, or body
5. **Send Request**: Press `Enter` to execute the request
6. **View Response**: Navigate to Response section to see results

### Example: Testing a JSON API

```bash
# 1. Set URL
https://jsonplaceholder.typicode.com/posts/1

# 2. Add headers (if needed)
Content-Type: application/json
Authorization: Bearer your-token-here

# 3. Add query parameters
userId=1
_format=json

# 4. Send request and view formatted JSON response
```

## ⌨️ Keyboard Shortcuts

### Global Navigation
| Key | Action |
|-----|--------|
| `Ctrl+j` | Navigate down between sections |
| `Ctrl+k` | Navigate up between sections |
| `?` | Show/hide help |
| `q` | Quit application |

### URL Section
| Key | Action |
|-----|--------|
| `u` | Edit URL |
| `m` | Open method dropdown |
| `↑/↓` | Navigate method dropdown |
| `Enter` | Select method / Send request |
| `Esc` | Exit edit mode |

### Values Section
| Key | Action |
|-----|--------|
| `h/l` or `←/→` | Switch between Body/Headers/Params |
| `i` | Enter edit mode for current tab |
| `Enter` | Add header/parameter |
| `Tab` | Switch between key/value fields |
| `Esc` | Exit edit mode |

### Response Section
| Key | Action |
|-----|--------|
| `h/b` | Switch between Headers/Body |
| `j/k` | Scroll response content |
| `↑/↓` | Scroll response content |

### Tab Management
| Key | Action |
|-----|--------|
| `t` | Create new tab |
| `x` | Close current tab |
| `Tab` | Next tab |
| `Shift+Tab` | Previous tab |

## 🏗️ Project Structure

Restless is built with a modular architecture for maintainability and extensibility:

```
src/
├── app/                 # Application state management
│   ├── app.rs          # Main app structure and logic
│   ├── tab.rs          # Tab management
│   └── mod.rs          # Module exports
├── handlers/            # Event handling
│   ├── keyboard.rs     # Keyboard event processing
│   ├── navigation.rs   # Navigation helpers
│   ├── request.rs      # HTTP request handling
│   ├── tab.rs          # Tab operations
│   └── mod.rs          # Handler coordination
├── logic/              # Core business logic
│   ├── request.rs      # HTTP request logic
│   ├── response.rs     # Response processing
│   └── mod.rs          # Logic exports
├── ui/                 # User interface
│   ├── components.rs   # UI components
│   ├── layouts.rs      # Layout management
│   ├── popups.rs       # Modal dialogs
│   ├── renderer.rs     # Main UI renderer
│   └── mod.rs          # UI coordination
├── terminal/           # Terminal management
│   └── mod.rs          # Terminal setup/cleanup
├── error.rs           # Error handling
└── main.rs            # Application entry point
```

## 🔧 Configuration

Restless automatically validates your terminal size and requests. Minimum requirements:

- **Terminal Size**: 80x24 characters
- **Network**: Internet connection for HTTP requests
- **Keyboard**: Standard keyboard input support

### Supported Content Types

- ✅ **JSON**: Automatic formatting and syntax highlighting
- ✅ **XML**: Basic formatting support
- ✅ **Plain Text**: Raw text display
- ✅ **HTML**: Raw HTML display

### HTTP Features

- ✅ **Methods**: GET, POST, PUT, DELETE
- ✅ **Headers**: Custom header support
- ✅ **Query Parameters**: URL parameter builder
- ✅ **Request Body**: JSON, XML, or plain text
- ✅ **Response**: Status codes, headers, and body
- ✅ **Timeouts**: 30-second request timeout

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/restless.git
cd restless

# Install dependencies and run
cargo run

# Run tests
cargo test

# Run with network tests
cargo test -- --ignored
```

### Code Style

This project uses:
- **Formatting**: `cargo fmt`
- **Linting**: `cargo clippy`
- **Testing**: Comprehensive unit and integration tests

## 🐛 Troubleshooting

### Common Issues

<details>
<summary><strong>Terminal too small error</strong></summary>

```
Error: Terminal width too small: 70 (minimum: 80)
```

**Solution**: Resize your terminal to at least 80x24 characters.
</details>

<details>
<summary><strong>Network timeout errors</strong></summary>

```
Request failed: Request timeout after 30 seconds
```

**Solution**: Check your internet connection and the target server availability.
</details>

<details>
<summary><strong>Invalid URL format</strong></summary>

```
Validation error: URL must start with http:// or https://
```

**Solution**: Ensure your URL includes the protocol (http:// or https://).
</details>

### Getting Help

- 📖 Press `?` in the application for built-in help
- 🐛 [Report bugs](https://github.com/yourusername/restless/issues)
- 💡 [Request features](https://github.com/yourusername/restless/issues)
- 💬 [Discussions](https://github.com/yourusername/restless/discussions)

## 📋 Roadmap

- [ ] **Configuration File**: Save/load request collections
- [ ] **Authentication**: Bearer token, API key, Basic auth
- [ ] **Environment Variables**: Template support
- [ ] **Response History**: Previous request/response storage
- [ ] **Export/Import**: Share request configurations
- [ ] **Themes**: Customizable color schemes
- [ ] **Plugins**: Extensible architecture

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Ratatui](https://ratatui.rs/) for the terminal UI
- HTTP client powered by [Reqwest](https://github.com/seanmonstar/reqwest)
- Error handling with [Thiserror](https://github.com/dtolnay/thiserror) and [Anyhow](https://github.com/dtolnay/anyhow)
- Inspired by [Postman](https://www.postman.com/) and [Insomnia](https://insomnia.rest/)

---

<div align="center">

**[⬆ Back to top](#-restless)**

Made with ❤️ by the Restless team

</div>