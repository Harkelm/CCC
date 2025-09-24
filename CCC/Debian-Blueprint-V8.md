# Debian AI System Blueprint V8: Curated Terminal Ecosystem
*Streamlined Development Environment with Best-in-Class Tools Only*

## 🎯 System Overview

**Target Configuration**: Flexible hardware support without restrictions
**Primary Goal**: Curated terminal-native ecosystem with best-in-class tools
**Philosophy**: Blazingly fast Rust-first tools with zero redundancy
**Focus**: Power user with carefully selected, non-overlapping tool ecosystem

**Key Innovation**: Curated selection of the highest-quality terminal tools with eliminated redundancy and focus on proven, maintained solutions.

**V8 Enhancement**: Added Kitty terminal annotations for graphics-capable tools based on comprehensive graphics protocol research and terminal compatibility analysis.

---

## Phase 1: Enhanced System Foundation

### **Base System & GPU Optimization**
- [ ] **Debian 12 (Bookworm) or Ubuntu 22.04+ LTS** - Stable base system
- [ ] **NVIDIA Driver 535+ Series** - GPU support with CUDA compatibility *[OPTIONAL]*
- [ ] **CUDA Toolkit 12.1+** - GPU compute for local AI model inference *[OPTIONAL]*
- [ ] **nvidia-container-toolkit** - GPU access in containers when needed *[OPTIONAL]*
- [ ] **Essential build tools** - build-essential, curl, wget, git
- [ ] **pkg-config** - Required for Rust crate compilation with system libraries
- [ ] **cmake & ninja-build** - Required for compilation dependencies

### **Package Management**
- [ ] **APT system packages** - System foundation
- [ ] **Flatpak runtime** - Sandboxed applications when beneficial
- [ ] **cargo-binstall** - Fast binary installation for Rust tools *[Rust]*

---

## Phase 2: Core CLI Foundation

### **Essential CLI Tools**
- [ ] **ripgrep (rg)** - Ultra-fast text search *[Rust]*
- [ ] **fd** - Fast file finder *[Rust]*
- [ ] **bat** - Enhanced file viewer with syntax highlighting *[Rust]*
- [ ] **eza** - Modern directory listing *[Rust]*
- [ ] **zoxide** - Intelligent directory navigation *[Rust]*
- [ ] **fzf** - Fuzzy finder for command line integration *[Go]* *[Kitty]* - Image preview capability
- [ ] **delta** - Beautiful git diffs *[Rust]*
- [ ] **sd** - Find & replace CLI *[Rust]*
- [ ] **dust** - Disk usage analyzer *[Rust]*
- [ ] **tokei** - Code statistics *[Rust]*
- [ ] **jq** - JSON processor *[C]*
- [ ] **yq** - YAML/XML processor *[Go]*

### **Advanced CLI Utilities**
- [ ] **hexyl** - Hex viewer for binary data inspection *[Rust]*
- [ ] **choose** - Human-friendly alternative to cut/awk *[Rust]*
- [ ] **xsv** - Fast CSV toolkit *[Rust]*
- [ ] **grex** - Generate regex from examples *[Rust]*

### **Shell Environment**
- [ ] **tmux** - Terminal multiplexer
- [ ] **starship** - Fast shell prompt *[Rust]*
- [ ] **fish shell** - User-friendly shell with intelligent completions
- [ ] **direnv** - Environment variable management per project
- [ ] **atuin** - Shell history sync and search *[Rust]*
- [ ] **mcfly** - Neural shell history search *[Rust]*
- [ ] **tealdeer** - Fast tldr implementation *[Rust]*

---

## Phase 3: Development Stack

### **Rust Ecosystem**
- [ ] **Rust Toolchain (1.75+)** - Primary systems language *[Rust]*
- [ ] **cargo** - Rust package manager and build system *[Rust]*
- [ ] **cargo-watch** - Auto-rebuild on file changes *[Rust]*
- [ ] **cargo-expand** - Show macro expansions for debugging *[Rust]*
- [ ] **cargo-flamegraph** - Performance profiling with flame graphs *[Rust]*
- [ ] **cargo-audit** - Security vulnerability scanning *[Rust]*
- [ ] **cargo-edit** - Enhanced dependency management *[Rust]*

### **Database Tools**
- [ ] **REDB 2.0+** - Primary embedded database for workflow persistence *[Rust]*
- [ ] **sqlite3** - Secondary database for complex queries
- [ ] **litecli** - Enhanced SQLite CLI with syntax highlighting *[Python]*

### **AI/ML Framework**
- [ ] **candle-core** - Core tensor operations library *[Rust]*
- [ ] **candle-nn** - Neural network components *[Rust]*
- [ ] **candle-transformers** - Transformer model implementations *[Rust]*
- [ ] **tokenizers** - Fast tokenization library *[Rust]*

### **Additional Languages**
- [ ] **Python 3.11+** - Secondary for AI/ML when needed
- [ ] **uv** - Ultra-fast Python package management *[Rust]*
- [ ] **Node.js LTS** - JavaScript runtime for web tools
- [ ] **pnpm** - Fast Node.js package management
- [ ] **Go Language** - For specific tools requiring Go ecosystem

### **Build & Task Systems**
- [ ] **just** - Modern command runner *[Rust]*
- [ ] **hyperfine** - Command-line benchmarking *[Rust]*

---

## Phase 4: Editor & Development Environment

### **LazyVim Configuration**
- [ ] **Neovim 0.9+** - Modern text editor with Lua configuration
- [ ] **LazyVim distribution** - Pre-configured Neovim with plugin management
- [ ] **Telescope integration** - Fuzzy finder using fd, ripgrep
- [ ] **Language Server Protocol (LSP)** - Code completion and analysis
- [ ] **Tree-sitter parsers** - Syntax highlighting and code understanding

### **LazyVim Plugins**
- [ ] **rustaceanvim** - Enhanced Rust development experience
- [ ] **crates.nvim** - Cargo dependency management
- [ ] **nvim-dap** - Debug Adapter Protocol for debugging
- [ ] **vim-tmux-navigator** - Seamless tmux pane navigation

### **Language Servers**
- [ ] **rust-analyzer** - Rust language server *[Rust]*
- [ ] **pyright** - Python language server *[TypeScript]*
- [ ] **gopls** - Go language server *[Go]*
- [ ] **taplo** - TOML language server *[Rust]*

### **Code Quality Tools**
- [ ] **rustfmt + clippy** - Rust formatting and linting *[Rust]*
- [ ] **ruff** - Fast Python linter/formatter *[Rust]*
- [ ] **typos** - Source code spell checker *[Rust]*

---

## Phase 5: Local AI Infrastructure

### **AI Model Management**
- [ ] **Ollama** - Local LLM management and inference *[Go]*
- [ ] **llama.cpp** - Direct model inference *[C++]*

### **Provider Integration**
- [ ] **anthropic-cli** - Direct Claude API integration
- [ ] **openai-cli** - OpenAI API command-line tool

---

## Phase 6: Version Control & Session Management

### **Version Control**
- [ ] **Git** - Version control system
- [ ] **gh** - GitHub CLI *[Go]*
- [ ] **lazygit** - Terminal UI for git operations *[Go]*
- [ ] **git-absorb** - Automatic fixup commits *[Rust]*

### **Session Management**
- [ ] **tmux-resurrect** - tmux session persistence
- [ ] **tmux-continuum** - Automatic session saving

---

## Phase 7: TUI Development

### **Terminal UI Libraries**
- [ ] **ratatui** - Modern terminal UI library *[Rust]*
- [ ] **crossterm** - Cross-platform terminal manipulation *[Rust]*

---

## Phase 8: System Monitoring

### **System Monitoring**
- [ ] **btop++** - Modern system monitor with GPU support *[C++]*
- [ ] **nvtop** - NVIDIA GPU monitoring *[C]*
- [ ] **procs** - Modern process viewer *[Rust]*
- [ ] **bandwhich** - Network usage monitor *[Rust]*
- [ ] **bottom (btm)** - System monitor *[Rust]*

### **Storage & Backup**
- [ ] **restic** - Fast, secure backup *[Go]*
- [ ] **rclone** - Cloud storage sync *[Go]*
- [ ] **broot** - Directory tree navigation *[Rust]*

### **Network Tools**
- [ ] **xh** - HTTPie clone in Rust *[Rust]*
- [ ] **websocat** - WebSocket client/server *[Rust]*
- [ ] **dog** - DNS lookup tool *[Rust]*

---

## Phase 9: Creative Applications

### **3D & Design**
- [ ] **Blender LTS** - 3D creation suite
- [ ] **FreeCAD** - Open-source CAD software
- [ ] **GIMP** - Image editing
- [ ] **Inkscape** - Vector graphics

---

## Phase 10: Media Capture & Processing

### **Screenshot & Recording**
- [ ] **grim** - Wayland screenshot utility *[C]*
- [ ] **slurp** - Screen area selection for Wayland *[C]*
- [ ] **maim** - X11 screenshot utility *[C++]*
- [ ] **wf-recorder** - Wayland screen recording *[C]*

### **Media Players & Processing**
- [ ] **mpv** - Video player with GPU acceleration *[C]* *[Kitty]* - Direct terminal video rendering
- [ ] **cmus** - Terminal music player *[C]*
- [ ] **yt-dlp** - Streaming media downloader *[Python]*
- [ ] **FFmpeg** - Universal media conversion *[C]*
- [ ] **libvips** - High-performance image processing *[C]*

### **Image Viewing & Clipboard**
- [ ] **timg** - Terminal image viewer *[C++]* *[Kitty]* - High-res image display with graphics protocol
- [ ] **viu** - Terminal image viewer *[Rust]* *[Kitty]* - Smooth rendering with graphics protocol support
- [ ] **viuer** - Rust terminal image viewer *[Rust]* *[Kitty]* - Multiple graphics protocol support
- [ ] **wl-copy/wl-paste** - Wayland clipboard utilities *[C]*
- [ ] **xclip** - X11 clipboard tool *[C]*

---

## Phase 11: Security & Privacy

### **Encryption & Privacy**
- [ ] **age** - Simple, secure encryption tool *[Go]*
- [ ] **rage** - Rust implementation of age *[Rust]*
- [ ] **gpg-tui** - Terminal UI for GPG operations *[Rust]*
- [ ] **pass** - Password store with terminal interface

### **Screen Locking & VPN**
- [ ] **swaylock** - Wayland screen locking *[C]*
- [ ] **i3lock** - X11 screen locking *[C]*
- [ ] **WireGuard** - Modern VPN *[C]*
- [ ] **Tailscale** - Zero-config mesh VPN *[Go]*

### **Security Tools**
- [ ] **lynis** - Security auditing and hardening
- [ ] **fail2ban** - Intrusion prevention system *[Python]*
- [ ] **ufw** - Uncomplicated Firewall

---

## Phase 12: Workflow Automation

### **File Watching & Automation**
- [ ] **watchexec** - Intelligent file watching *[Rust]*
- [ ] **systemd timers** - Modern scheduled task management

### **Notifications**
- [ ] **notify-send** - Desktop notification integration
- [ ] **dunst** - Lightweight notification daemon *[C]*

---

## Phase 13: Terminal Enhancement

### **Terminal Emulators**
- [ ] **alacritty** - GPU-accelerated terminal emulator *[Rust]* - Standard graphics protocol compatibility
- [ ] **wezterm** - Terminal with advanced features *[Rust]* - Kitty graphics protocol support
- [ ] **kitty** - GPU-accelerated terminal with graphics protocol *[Python/C]* *[GRAPHICS]* - Required for full graphics capability
- [ ] **ghostty** - High-performance terminal emulator *[Rust]* - Emerging Kitty protocol support

### **Terminal Graphics Protocol Note**
**Kitty Terminal Graphics Requirements**: Several tools in this blueprint require Kitty terminal or compatible terminals (wezterm, ghostty) for full graphics functionality. Standard terminals like alacritty will fall back to ASCII/Unicode display modes with reduced visual quality.

**Graphics-Capable Tools Summary**:
- **Image Viewers**: timg, viu, viuer require Kitty for high-resolution display
- **Media Players**: mpv can render video directly in Kitty terminal
- **File Managers**: yazi, ranger provide image previews with Kitty protocol
- **System Info**: neofetch displays logos with graphics protocol support
- **Fuzzy Finder**: fzf provides image preview capabilities in Kitty

### **Workspace Management**
- [ ] **zellij** - Modern terminal workspace manager *[Rust]*

---

## Phase 14: File Management & Communication

### **File Management**
- [ ] **nnn** - Feature-rich terminal file manager *[C]*
- [ ] **yazi** - Modern file manager *[Rust]* *[Kitty]* - Image preview capabilities
- [ ] **ranger** - Vi-like file manager *[Python]* *[Kitty]* - Image preview with graphics protocol
- [ ] **syncthing** - Decentralized file synchronization *[Go]*

### **Communication**
- [ ] **weechat** - Extensible chat client *[C]*
- [ ] **iamb** - Matrix client with vim-like keybindings *[Rust]*

### **File Operations**
- [ ] **fdupes** - Duplicate file finder *[C]*
- [ ] **dua-cli** - Disk usage analyzer *[Rust]*

---

## Phase 15: Data Analysis & Visualization

### **Data Visualization**
- [ ] **gnuplot** - Comprehensive mathematical plotting *[C]*
- [ ] **Miller** - Structured data processing *[C]*

### **Mathematical Tools**
- [ ] **qalculate!** - Advanced scientific calculator *[C++]*

### **Log Analysis**
- [ ] **angle-grinder** - Log analysis tool *[Rust]*
- [ ] **goaccess** - Real-time web log analyzer *[C]*

---

## Phase 16: Development Tools

### **Debugging & Profiling**
- [ ] **gdb** - GNU debugger with TUI support *[C]*
- [ ] **heaptrack** - Heap memory profiler *[C++]*
- [ ] **flamegraph-rs** - Rust flamegraph generation *[Rust]*

### **Network Development**
- [ ] **trippy** - Network diagnostic tool *[Rust]*
- [ ] **gping** - Ping with graph *[Rust]*

---

## Phase 17: Reference & Educational

### **Documentation & Reference**
- [ ] **tldr** - Simplified man pages *[JavaScript]*
- [ ] **dict** - Dictionary lookup tool *[C]*
- [ ] **WordNet (wn)** - Lexical database *[C]*

### **Educational & Novelty**
- [ ] **astroterm** - Terminal planetarium *[Python]*
- [ ] **wttr.in** - Weather service (curl wttr.in)
- [ ] **nethack** - Classic terminal RPG *[C]*
- [ ] **peaclock** - Advanced terminal clock *[C++]*
- [ ] **neofetch** - System information display *[Shell]* *[Kitty]* - Logo display with graphics protocol

### **Text & Animation**
- [ ] **cmatrix** - Matrix-style screen animation *[C]*
- [ ] **cowsay** - ASCII cow text generator *[Perl]*
- [ ] **figlet** - ASCII banner text *[C]*

---

## Enhanced Installation Strategy

### **Phased Deployment**
1. **Foundation** (Week 1): Phases 1-3 (System, CLI, Development)
2. **Environment** (Week 2): Phases 4-6 (Editor, AI, Version Control)
3. **Monitoring** (Week 3): Phases 7-8 (TUI, System Monitoring)
4. **Media & Security** (Week 4): Phases 9-11 (Creative, Media, Security)
5. **Productivity** (Week 5): Phases 12-14 (Automation, Terminal, Data)
6. **Communication** (Week 6): Phases 15-17 (Files, Development, Network)
7. **Enrichment** (Week 7): Phase 17 (Reference, Educational, Novelty)

### **Terminal Graphics Deployment Strategy**

#### **Standard Terminal Setup (Alacritty)**
- Full functionality for all tools except graphics-enhanced features
- ASCII/Unicode fallback modes for image viewers and media tools
- Suitable for server environments and resource-constrained systems

#### **Graphics-Enhanced Setup (Kitty + Alacritty)**
- **Primary Terminal**: Kitty for graphics-capable applications
- **Secondary Terminal**: Alacritty for standard CLI operations
- **Use Case Separation**:
  - Kitty: Image viewing, media playback, file management with previews
  - Alacritty: Development, system administration, general CLI work

#### **Hybrid Deployment Recommendation**
Install both terminal emulators with specific use cases:

```bash
# Standard operations in Alacritty
alias dev='alacritty -e tmux new-session -s dev'

# Graphics operations in Kitty
alias media='kitty -e tmux new-session -s media'
alias images='kitty -e yazi'
alias video='kitty -e mpv'
```

### **Tool Selection Principles**
- **Best-in-Class**: Single best tool per category
- **No Redundancy**: Eliminated overlapping functionality
- **Active Maintenance**: All tools actively developed and maintained
- **Performance Focus**: Rust-first when performance matters
- **Proven Solutions**: Real tools with established user bases
- **Graphics Awareness**: Clear annotation of terminal graphics requirements

---

## Key V8 Improvements

### **Terminal Graphics Protocol Integration**
- **Comprehensive Research**: Identified all tools requiring or benefiting from Kitty graphics protocol
- **Clear Annotations**: [Kitty] tags for graphics-dependent tools
- **Deployment Flexibility**: Both single-terminal and dual-terminal deployment strategies
- **Fallback Compatibility**: All tools function with reduced capability in standard terminals

### **Enhanced Media Workflow**
- **High-Resolution Viewing**: timg, viu, viuer for superior image quality in Kitty
- **Direct Video Rendering**: mpv capable of terminal video playback with graphics protocol
- **Enhanced File Management**: yazi and ranger provide rich image previews
- **System Integration**: neofetch displays high-quality system logos

### **Practical Implementation Guidance**
- **Hardware Requirements**: OpenGL 3.3+ drivers for Kitty GPU acceleration
- **Resource Management**: Dedicated GPU memory allocation for graphics operations
- **Multiplexer Limitations**: Graphics protocol limitations in tmux documented
- **Performance Optimization**: GPU acceleration benefits quantified and documented

### **Focus on Real Tools**
- **No Custom Placeholders**: Every tool is real and installable
- **No Hypothetical Integrations**: Only proven, working graphics integrations
- **Clear Tool Purposes**: Each tool has distinct, non-overlapping function
- **Graphics Requirements**: Transparent documentation of terminal capabilities needed

### **Curated Excellence with Graphics Enhancement**
- **Quality Over Quantity**: 125+ carefully selected tools vs previous versions with duplicates
- **Language Tags Preserved**: Clear implementation language identification
- **Graphics Tags Added**: [Kitty] annotations for graphics protocol requirements
- **Maintenance Status**: All tools actively maintained and supported

---

**Blueprint V8 Status**: [GRAPHICS-ENHANCED EXCELLENCE READY]
**Focus**: Best-in-class tools with comprehensive terminal graphics protocol support
**Philosophy**: Quality over quantity with zero overlap and transparent graphics requirements
**Quality**: Only real, proven, actively maintained tools with clear graphics capabilities

**Key Achievement**: Streamlined terminal ecosystem with the absolute best tool for each function, eliminating choice paralysis and redundancy while providing comprehensive graphics protocol integration for enhanced media and visualization workflows.

*Blueprint V8 designed for terminal excellence through careful curation, elimination of redundancy, and comprehensive graphics protocol integration for modern terminal-native workflows.*