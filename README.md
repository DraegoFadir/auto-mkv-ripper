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

### v0.5.0 — Anime
- [ ] TVDB lookup for anime
- [ ] Anime episode mapping
- [ ] Chapter splitting for discs with combined episode files (via MKVToolNix/`mkvmerge`)
- [ ] Optional SUB/DUB splitting
- [ ] Movies, TV, and anime fully supported; ready for wider testing and feedback

### v1.0.0 — Stable Release
- [ ] Bug fixes
- [ ] UI polish
- [ ] Code cleanup

### Beyond v1.0
- [ ] SFTP media management (browse and manage media already on the server)
- [ ] Custom output folders and file names
- [ ] Retry individual mapped titles

## Known Issues

- The **Stop** button on mapped titles on the Rip page is currently disabled.
- The **Reset** button is still clickable during a rip or upload.
- Mapped titles aren't validated before ripping. If two titles map to the same episode, the later one will overwrite the earlier file, so double-check your mapping before starting.

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

## Environment Setup

The app uses API keys that are compiled into the build at build time. They are not stored in the repo, so you'll need to provide your own key to build locally.

### 1. Get a TMDB API key
Create a free account and request an API key at https://www.themoviedb.org/settings/api

### 2. Create your `.env` file
Copy the example file:

**Linux / macOS**
```bash
cp src-tauri/.env.example src-tauri/.env
```

**Windows (PowerShell)**
```powershell
Copy-Item src-tauri/.env.example src-tauri/.env
```

### 3. Add your key
Open `src-tauri/.env`, uncomment the line, and add your key:

```dotenv
TMDB_API_KEY=your_key_here
```

> ⚠️ `.env` is gitignored. Never commit it or share your key.

### Troubleshooting

**Build fails with `Error loading .env file`**
The `.env` file is missing. Make sure it exists at `src-tauri/.env`.

**Build fails with a missing `TMDB_API_KEY` error**
The `.env` file exists but the key isn't set. Make sure the line is uncomment and has a value.

**Changed your key but the app still uses the old one**
The key is compiled into the build, and Cargo may reuse the old compiled value. Force a rebuild:
```bash
cd src-tauri && cargo clean
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
