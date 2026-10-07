# auto-mkv-ripper

A desktop app for automating MakeMKV. A quick up front setup lets you rip and upload automatically. Built with [Tauri](https://tauri.app) and [SvelteKit](https://kit.svelte.dev).

## Features

- Scan and rip Blu-ray discs via MakeMKV (`makemkvcon`)
- TMDB/TVDB Lookup
- Movie Data and Title Mapping for automation
- Configurable MakeMKV path, output directory, and TMDB API key
- Upcoming upload to SFTP for Jellyfin/Plex media servers or Cloud Storage.
- Cross-platform: Windows and Linux

## Installation

Grab the latest release for your platform from the [Releases](../../releases) page:

- **Windows**: `.msi` installer, NSIS `.exe` installer, or a portable `.exe` (no install required)
- **Linux**: `.deb`, `.rpm`, or `.AppImage`

### Requirements

- [MakeMKV](https://www.makemkv.com/) must be installed separately
  - **Windows**: point the app at your `makemkvcon64.exe` install path in Settings
  - **Linux**: installed as a Flatpak (`com.makemkv.MakeMKV`)
- [MKVToolNix](https://mkvtoolnix.download/) (`mkvmerge`) is required for anime support (v0.5.0+)
  - **Windows**: point the app at your `mkvmerge.exe` install path in Settings
  - **Linux**: installed as a Flatpak (`org.bunkus.mkvtoolnix-gui`)

## Roadmap

### v0.3.0 — Enhanced Search
- [ ] Search TMDB by name
- [ ] Search by ID using the `id:` prefix (e.g. `id:9966`)

### v0.4.0 — TV Shows
- [ ] TVDB lookup
- [ ] TV season support
- [ ] Episode mapping

### v0.5.0 — Anime & Public Beta
- [ ] TVDB lookup for anime
- [ ] Anime episode mapping
- [ ] Discs with combined episode files will try to find the best chapter split by average episode length
- [ ] Ability to split SUB and DUB
- [ ] Movies, TV, and anime fully supported; ready for wider testing and feedback

### v1.0.0 — Stable Release
- [ ] Bug fixes
- [ ] UI polish
- [ ] Code cleanup

### Beyond v1.0
- [ ] SFTP media management (browse and manage media already on the server)

## Development

### Prerequisites

- [Bun](https://bun.sh)
- [Rust](https://www.rust-lang.org/tools/install)
- Platform-specific Tauri dependencies — see the [Tauri prerequisites guide](https://tauri.app/start/prerequisites/)

### Setup

```bash
bun install
bun run tauri dev
```

### Building

```bash
bun run tauri build
```

Bundled installers will be output to `src-tauri/target/release/bundle/`.

## Tech Stack

- **Frontend**: SvelteKit + TypeScript, styled with [Pico CSS](https://picocss.com)
- **Backend**: Rust via Tauri 2
- **Package manager**: Bun (use `bun`, not `npm`/`yarn`/`pnpm` — mixing package managers will cause dependency drift between the frontend and the Rust crates)

## License

TBD
