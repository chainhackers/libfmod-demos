// Play a sound file with FMOD 2.03.09
// Run with: cargo run --example play_sound [path/to/sound.wav/mp3/ogg]

use libfmod::{System, Init, Mode, TimeUnit};
use std::{env, thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    println!("\n🎵 FMOD 2.03.09 Sound Player\n");

    // Check for sound file argument
    if args.len() < 2 {
        println!("Usage: {} <path/to/sound/file>", args[0]);
        println!("\nSupported formats: WAV, MP3, OGG, FLAC, etc.");
        println!("\nExample:");
        println!("  cargo run --example play_sound /usr/share/sounds/freedesktop/stereo/bell.oga");
        println!("  cargo run --example play_sound ~/Music/song.mp3");

        // Try to find a system sound to suggest
        let test_sounds = vec![
            "/usr/share/sounds/freedesktop/stereo/bell.oga",
            "/usr/share/sounds/freedesktop/stereo/complete.oga",
            "/usr/share/sounds/freedesktop/stereo/message.oga",
            "/usr/share/sounds/ubuntu/stereo/bell.ogg",
            "/usr/share/sounds/gnome/default/alerts/drip.ogg",
        ];

        println!("\nLooking for system sounds...");
        for sound in &test_sounds {
            if std::path::Path::new(sound).exists() {
                println!("  Found: {}", sound);
                println!("  Try: cargo run --example play_sound {}", sound);
                break;
            }
        }

        return Ok(());
    }

    let sound_file = &args[1];

    // Check if file exists
    if !std::path::Path::new(sound_file).exists() {
        println!("❌ File not found: {}", sound_file);
        return Ok(());
    }

    // Create FMOD system
    println!("Initializing FMOD 2.03.09...");
    let system = System::create()?;

    // Get and display version
    let (version, build) = system.get_version()?;
    let major = (version >> 16) & 0xFF;
    let minor = (version >> 8) & 0xFF;
    let patch = version & 0xFF;
    println!("✅ FMOD {}.{:02}.{:02} (build {})", major, minor, patch, build);

    // Initialize system
    system.init(512, Init::NORMAL, None)?;
    println!("✅ System initialized");

    // Create sound
    println!("\nLoading: {}", sound_file);
    let sound = match system.create_sound(sound_file, Mode::DEFAULT, None) {
        Ok(s) => {
            println!("✅ Sound loaded successfully");
            s
        }
        Err(e) => {
            println!("❌ Failed to load sound: {:?}", e);
            system.release()?;
            return Ok(());
        }
    };

    // Get sound info
    if let Ok(length) = sound.get_length(TimeUnit::MS) {
        let seconds = length as f32 / 1000.0;
        println!("ℹ️  Duration: {:.1} seconds", seconds);
    }

    // Play sound
    println!("\n▶️  Playing sound...");
    let channel = system.play_sound(sound, None, false)?;

    // Show volume control hint
    println!("   Use your system volume control if needed");
    println!("   Press Ctrl+C to stop\n");

    // Wait for sound to finish
    let mut position = 0u32;
    let mut is_playing = true;

    while is_playing {
        // Update system
        system.update()?;

        // Check if still playing
        is_playing = channel.is_playing().unwrap_or(false);

        if is_playing {
            // Get playback position
            if let Ok(pos) = channel.get_position(TimeUnit::MS) {
                let new_pos = pos / 1000; // Convert to seconds
                if new_pos != position {
                    position = new_pos;
                    print!("\r   Playing... {} seconds", position);
                    use std::io::{self, Write};
                    io::stdout().flush()?;
                }
            }

            // Small delay
            thread::sleep(Duration::from_millis(50));
        }
    }

    println!("\n\n✅ Playback complete!");

    // Cleanup
    sound.release()?;
    system.release()?;
    println!("✅ System released");

    println!("\n🎉 Success! FMOD 2.03.09 audio playback works!");

    Ok(())
}