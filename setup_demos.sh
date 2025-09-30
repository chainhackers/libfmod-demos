#!/bin/bash

# Setup script for libfmod-demos
# Downloads public domain test audio files from Internet Archive

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}=== libfmod-demos Setup ===${NC}"
echo

# Create assets directory structure
echo "Creating asset directories..."
mkdir -p assets/audio

# Function to download and verify a file
download_file() {
    local url=$1
    local filename=$2
    local expected_md5=$3

    echo -n "Downloading $filename... "

    if [ -f "assets/audio/$filename" ]; then
        echo -e "${YELLOW}already exists, skipping${NC}"
        return 0
    fi

    curl -L -s -o "assets/audio/$filename" "$url"

    if [ $? -eq 0 ]; then
        if [ ! -z "$expected_md5" ]; then
            actual_md5=$(md5sum "assets/audio/$filename" | cut -d' ' -f1)
            if [ "$actual_md5" == "$expected_md5" ]; then
                echo -e "${GREEN}✓${NC}"
            else
                echo -e "${RED}✗ checksum mismatch${NC}"
                rm "assets/audio/$filename"
                return 1
            fi
        else
            echo -e "${GREEN}✓${NC}"
        fi
    else
        echo -e "${RED}✗ download failed${NC}"
        return 1
    fi
}

# Download test audio files from Internet Archive (CC0/Public Domain)
echo -e "${YELLOW}Downloading test audio files from Internet Archive...${NC}"
echo

# Small sound effects from slack_sfx collection (all CC0)
download_file "https://archive.org/download/slack_sfx/animal_stick.ogg" "animal_stick.ogg"
download_file "https://archive.org/download/slack_sfx/flitterbug.ogg" "flitterbug.ogg"
download_file "https://archive.org/download/slack_sfx/complete_quest_requirement.ogg" "notification.ogg"

# Nature/animal sounds
download_file "https://archive.org/download/creative-dice-roll-sounds/bird1.ogg" "bird.ogg"

# Music samples (larger files for testing streaming)
download_file "https://archive.org/download/MarchForHonor/Distant_Wonders.ogg" "music_distant.ogg"
download_file "https://archive.org/download/MarchForHonor/Meme_Medley.ogg" "music_meme.ogg"

echo
echo "Downloaded files:"
ls -lh assets/audio/ 2>/dev/null || echo "No files downloaded"

echo
echo -e "${YELLOW}Checking for FMOD SDK...${NC}"

# Check if FMOD SDK path is set
if [ -z "$FMOD_SDK_DIR" ]; then
    echo -e "${RED}❌ FMOD_SDK_DIR not set${NC}"
    echo
    echo "Please download FMOD Engine SDK from: https://www.fmod.com/download"
    echo "Then set the environment variable:"
    echo "  export FMOD_SDK_DIR=/path/to/fmod/sdk"
    echo
    echo "Example:"
    echo "  export FMOD_SDK_DIR=/home/\$USER/fmod/20309"
else
    echo -e "${GREEN}✓ FMOD_SDK_DIR is set to: $FMOD_SDK_DIR${NC}"

    # Check if the libraries exist
    if [ -d "$FMOD_SDK_DIR/api/core/lib/x86_64" ]; then
        echo -e "${GREEN}✓ FMOD libraries found${NC}"
    else
        echo -e "${RED}✗ FMOD libraries not found at expected location${NC}"
        echo "  Expected: $FMOD_SDK_DIR/api/core/lib/x86_64"
    fi
fi

echo
echo -e "${GREEN}Setup complete!${NC}"
echo
echo "To run demos:"
echo "  chmod +x run_demos.sh"
echo "  ./run_demos.sh <example_name>"
echo
echo "Available examples:"
echo "  harness_demo        - Non-interactive FMOD feature demos"
echo "  interactive_harness - Interactive 3D audio testing"
echo "  play_sound <file>   - Play an audio file"
echo "  verify_fmod         - Verify FMOD is working"
echo "  quick_test          - Run comprehensive test"
echo
echo "Example:"
echo "  ./run_demos.sh play_sound assets/audio/bird.ogg"