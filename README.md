# libfmod-demos

Interactive demos and examples for the [libfmod](https://github.com/chainhackers/libfmod) Rust bindings to FMOD Engine.

## Quick Start

```bash
# 1. Install FMOD SDK and set environment
export FMOD_SDK_DIR=/path/to/fmod/20309

# 2. Download test audio files
./setup_demos.sh

# 3. Run demos
./run_demos.sh verify_fmod
./run_demos.sh harness_demo
```

## Available Demos

### Core Examples
- `verify_fmod` - Verify FMOD installation and version
- `play_sound <file>` - Play audio files
- `quick_test` - Run comprehensive test suite

### Interactive Demos
- `harness_demo [mode]` - Non-interactive feature demonstrations
  - `spatial` - 3D spatial audio with moving source
  - `explosion` - One-shot event playback
  - `parameters` / `rpm` - Real-time parameter control
  - `footsteps` - Multiple simultaneous events
  - `all` - Run all demos (default)
- `interactive_harness` - Real-time keyboard-controlled testing
  - `1-6` - Play/stop events
  - `WASD/QE` - Move sound source in 3D
  - `Space` - Stop all events
  - `H` - Toggle help

### Test Suites
- `studio_banks_test` - Bank loading and management
- `studio_events_test` - Event playback and variations
- `studio_parameters_test` - Parameter automation

## Examples

```bash
# Basic verification
./run_demos.sh verify_fmod

# Play downloaded audio
./run_demos.sh play_sound assets/audio/bird.ogg

# 3D spatial demo
./run_demos.sh harness_demo spatial

# Interactive control
./run_demos.sh interactive_harness
```

## Requirements

- Rust 1.79+ (2024 edition)
- FMOD Engine SDK 2.03.09+
- Linux/macOS (Windows support planned)

## Project Structure

```
libfmod-demos/
├── src/lib.rs           # Common utilities
├── examples/            # Demo implementations
├── assets/audio/        # Downloaded test files
├── setup_demos.sh       # Audio file downloader
└── run_demos.sh         # Demo runner script
```

## License

MIT