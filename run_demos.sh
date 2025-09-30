#!/bin/bash

# Run script for libfmod-demos
# Usage: ./run_demos.sh [example_name]

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if FMOD_SDK_DIR is set
if [ -z "$FMOD_SDK_DIR" ]; then
    echo -e "${RED}Error: FMOD_SDK_DIR environment variable is not set${NC}"
    echo "Please set it to your FMOD SDK path:"
    echo "  export FMOD_SDK_DIR=/path/to/fmod/sdk"
    echo "Example:"
    echo "  export FMOD_SDK_DIR=/home/\$USER/fmod/20309"
    exit 1
fi

# Detect platform
UNAME=$(uname)
case "$UNAME" in
    Linux*)
        PLATFORM="linux"
        LIB_DIR="x86_64"
        LIB_EXT="so"
        ;;
    Darwin*)
        PLATFORM="mac"
        LIB_DIR="."
        LIB_EXT="dylib"
        ;;
    *)
        echo -e "${RED}Unsupported platform: $UNAME${NC}"
        exit 1
        ;;
esac

# Set library paths
FMOD_CORE_LIB="$FMOD_SDK_DIR/api/core/lib/$LIB_DIR"
FMOD_STUDIO_LIB="$FMOD_SDK_DIR/api/studio/lib/$LIB_DIR"

# Verify FMOD libraries exist
if [ ! -d "$FMOD_CORE_LIB" ]; then
    echo -e "${RED}Error: FMOD core libraries not found at $FMOD_CORE_LIB${NC}"
    exit 1
fi

if [ ! -d "$FMOD_STUDIO_LIB" ]; then
    echo -e "${RED}Error: FMOD studio libraries not found at $FMOD_STUDIO_LIB${NC}"
    exit 1
fi

# Export library paths
if [ "$PLATFORM" = "linux" ]; then
    export LD_LIBRARY_PATH="$FMOD_CORE_LIB:$FMOD_STUDIO_LIB:$LD_LIBRARY_PATH"
elif [ "$PLATFORM" = "mac" ]; then
    export DYLD_LIBRARY_PATH="$FMOD_CORE_LIB:$FMOD_STUDIO_LIB:$DYLD_LIBRARY_PATH"
fi

# Get example name from argument
EXAMPLE=$1

# Function to run an example
run_example() {
    local example_name=$1
    echo -e "${GREEN}Running example: $example_name${NC}"
    cargo run --example "$example_name" -- "${@:2}"
}

# Show help if no argument provided
if [ -z "$EXAMPLE" ]; then
    echo -e "${GREEN}=== libfmod-demos Runner ===${NC}"
    echo
    echo "Usage: ./run_demos.sh <example_name> [args...]"
    echo
    echo "Available examples:"
    echo "  verify_fmod         - Verify FMOD is working correctly"
    echo "  play_sound <file>   - Play an audio file"
    echo "  harness_demo        - Non-interactive FMOD feature demos"
    echo "  interactive_harness - Interactive 3D audio testing"
    echo "  quick_test          - Run comprehensive test suite"
    echo
    echo "Examples:"
    echo "  ./run_demos.sh verify_fmod"
    echo "  ./run_demos.sh play_sound assets/audio/bird.ogg"
    echo "  ./run_demos.sh harness_demo spatial"
    echo "  ./run_demos.sh interactive_harness"
    echo
    echo "First time setup:"
    echo "  1. Run ./setup_demos.sh to download test audio files"
    echo "  2. Set FMOD_SDK_DIR environment variable"
    echo "  3. Run any of the examples above"
    exit 0
fi

# Check if assets exist for examples that need them
if [[ "$EXAMPLE" == "play_sound" || "$EXAMPLE" == "interactive_harness" || "$EXAMPLE" == "harness_demo" ]]; then
    if [ ! -d "assets/audio" ] || [ -z "$(ls -A assets/audio 2>/dev/null)" ]; then
        echo -e "${YELLOW}Warning: Audio assets not found${NC}"
        echo "Run ./setup_demos.sh first to download test audio files"
        echo
        read -p "Run setup now? (y/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            ./setup_demos.sh
        fi
    fi
fi

# Build and run the example
echo -e "${YELLOW}Building example...${NC}"
cargo build --example "$EXAMPLE" 2>&1 | grep -v "warning:" || true

echo
run_example "$EXAMPLE" "${@:2}"