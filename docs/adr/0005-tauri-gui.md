# ADR 0005: Tauri Desktop GUI

## Status

Accepted

## Context

Dark-GPT was originally designed as a CLI/Docker-based solution for running uncensored LLMs locally. To make it easier to share with colleagues and non-technical users, we needed a portable desktop application that:

- Works on Windows and Linux
- Provides a user-friendly setup wizard
- Handles prerequisites detection (Docker, Ollama)
- Offers one-click launch experience
- Can be distributed as a single downloadable file

## Decision

We chose **Tauri 2** with **Svelte 5** frontend and **Rust** backend for the desktop GUI.

### Alternatives Considered

| Option          | Pros                                                               | Cons                                       |
| --------------- | ------------------------------------------------------------------ | ------------------------------------------ |
| **Tauri**       | Small binaries (~15MB), Rust backend, web frontend, cross-platform | Learning curve for Rust                    |
| Electron        | Large ecosystem, familiar JS                                       | Large binaries (~150MB), high memory usage |
| Flutter Desktop | Good performance, single codebase                                  | Heavy for simple UI, Dart ecosystem        |
| Native (Gtk/Qt) | Best performance                                                   | Different codebases per platform           |

### Why Tauri?

1. **Small binaries**: ~15-20MB vs ~150MB for Electron
2. **Native performance**: Rust backend for system operations
3. **Web frontend**: Svelte 5 with Tailwind for modern UI
4. **Security**: Rust memory safety, sandboxed webview
5. **Shell access**: Built-in plugin for Docker/Ollama commands

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    TAURI APP (dark-gpt.exe/.AppImage)           │
│                                                                 │
│  ┌──────────────┐    ┌───────────────────────────────────────┐  │
│  │   Frontend   │    │           Backend Rust                │  │
│  │   (Svelte)   │───▶│  ┌─────────┐ ┌─────────┐ ┌─────────┐  │  │
│  │              │    │  │ Docker  │ │ Ollama  │ │ Health  │  │  │
│  │ SetupWizard  │    │  │ Cmds    │ │ Client  │ │ Checks  │  │  │
│  │ WebUIFrame   │    │  └────┬────┘ └────┬────┘ └────┬────┘  │  │
│  └──────────────┘    └───────┼───────────┼───────────┼───────┘  │
│                              │           │           │          │
└──────────────────────────────┼───────────┼───────────┼──────────┘
                               ▼           ▼           ▼
                    ┌──────────────┐ ┌──────────┐ ┌──────────┐
                    │Docker Compose│ │  Ollama  │ │Open-WebUI│
                    │(Caddy+WebUI) │ │ :11434   │ │ (iframe) │
                    └──────────────┘ └──────────┘ └──────────┘
```

## Implementation Details

### Backend (Rust)

- **commands/docker.rs**: Start/stop Docker Compose services
- **commands/ollama.rs**: Check Ollama, list/pull models with progress events
- **commands/health.rs**: Health checks for all services
- **commands/setup.rs**: Prerequisites detection, setup wizard logic

### Frontend (Svelte 5)

- **App.svelte**: Main component with setup wizard and WebUI iframe
- Tauri event listening for model download progress
- Dark theme with Tailwind CSS

### CI/CD

- **ci.yml**: Lint and check on PRs
- **release.yml**: Multi-platform builds on tag push

## Consequences

### Positive

- Users can download a single file and run Dark-GPT
- Setup wizard guides through prerequisites
- Model download with progress bar
- One-click start/stop services
- Cross-platform (Windows + Linux)

### Negative

- Requires Docker and Ollama pre-installed (cannot bundle due to size)
- First-time model download is slow (~4.7GB)
- Linux requires additional dependencies (webkit2gtk)

### Neutral

- Binaries are ~15-20MB
- Build time ~8-12 minutes on CI

## References

- [Tauri Documentation](https://tauri.app/v2/)
- [Svelte 5 Runes](https://svelte.dev/blog/runes)
- [tauri-apps/tauri-action](https://github.com/tauri-apps/tauri-action)
