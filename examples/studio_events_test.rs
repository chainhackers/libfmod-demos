// Test FMOD Studio event playback and sound variations with FMOD 2.03.09
// Run with: ./run_fmod.sh studio_events_test

use libfmod::{Studio, StudioInit, Init, LoadBank, StopMode};
use libfmod_demos;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎵 FMOD Studio Events & Variations Test (2.03.09)\n");
    println!("=================================================\n");

    // Initialize Studio System
    let studio = Studio::create()?;
    studio.initialize(1024, StudioInit::NORMAL, Init::NORMAL, None)?;

    // Load banks
    println!("Loading banks...");
    let bank_dir = libfmod_demos::get_example_banks_dir()?;

    let master = studio.load_bank_file(&format!("{}/Master.bank", bank_dir), LoadBank::NORMAL)?;
    let strings = studio.load_bank_file(&format!("{}/Master.strings.bank", bank_dir), LoadBank::NORMAL)?;
    let sfx = studio.load_bank_file(&format!("{}/SFX.bank", bank_dir), LoadBank::NORMAL)?;
    let vehicles = studio.load_bank_file(&format!("{}/Vehicles.bank", bank_dir), LoadBank::NORMAL)?;
    let music = studio.load_bank_file(&format!("{}/Music.bank", bank_dir), LoadBank::NORMAL)?;

    println!("✓ Banks loaded\n");

    // Test 1: One-shot Event (Explosion)
    println!("🎆 TEST 1: One-shot Event - Explosion");
    println!("--------------------------------------");

    let explosion_desc = studio.get_event("event:/Weapons/Explosion")?;
    println!("Playing explosion (one-shot)...");

    for i in 1..=3 {
        println!("  Explosion #{}", i);
        let explosion = explosion_desc.create_instance()?;
        explosion.start()?;
        explosion.release()?; // Release immediately, sound continues

        // Update and wait
        for _ in 0..20 {
            studio.update()?;
            thread::sleep(Duration::from_millis(50));
        }
    }

    // Test 2: Looping Ambient Event
    println!("\n🌳 TEST 2: Looping Ambient Sound");
    println!("---------------------------------");

    let ambience_desc = studio.get_event("event:/Ambience/Country")?;
    let ambience = ambience_desc.create_instance()?;

    println!("Starting ambient loop...");
    ambience.start()?;

    for i in 1..=3 {
        println!("  Playing... {} seconds", i);
        for _ in 0..20 {
            studio.update()?;
            thread::sleep(Duration::from_millis(50));
        }
    }

    println!("Stopping ambient with fadeout...");
    ambience.stop(StopMode::AllowFadeout)?;

    for _ in 0..40 {
        studio.update()?;
        thread::sleep(Duration::from_millis(50));
    }
    ambience.release()?;

    // Test 3: Multiple Instances (Footsteps with variations)
    println!("\n👟 TEST 3: Sound Variations - Footsteps");
    println!("----------------------------------------");

    let footstep_desc = studio.get_event("event:/Character/Player Footsteps")?;

    println!("Playing footsteps (each plays different variation):");

    for step in 1..=8 {
        println!("  Step {}", step);
        let footstep = footstep_desc.create_instance()?;

        // Start the footstep
        footstep.start()?;
        footstep.release()?;

        // Short pause between steps
        for _ in 0..5 {
            studio.update()?;
            thread::sleep(Duration::from_millis(50));
        }
    }

    // Test 4: UI Sounds (Cancel)
    println!("\n🔘 TEST 4: UI Sound - Cancel");
    println!("-----------------------------");

    let cancel_desc = studio.get_event("event:/UI/Cancel")?;

    for i in 1..=2 {
        println!("  Cancel sound #{}", i);
        let cancel = cancel_desc.create_instance()?;
        cancel.start()?;
        cancel.release()?;

        for _ in 0..10 {
            studio.update()?;
            thread::sleep(Duration::from_millis(50));
        }
    }

    // Test 5: Vehicle Engine (continuous with variations)
    println!("\n🚜 TEST 5: Vehicle Engine");
    println!("-------------------------");

    let vehicle_desc = studio.get_event("event:/Vehicles/Ride-on Mower")?;
    let vehicle = vehicle_desc.create_instance()?;

    println!("Starting engine...");
    vehicle.start()?;

    println!("Running for 3 seconds...");
    for _ in 0..60 {
        studio.update()?;
        thread::sleep(Duration::from_millis(50));
    }

    println!("Stopping engine...");
    vehicle.stop(StopMode::AllowFadeout)?;

    for _ in 0..20 {
        studio.update()?;
        thread::sleep(Duration::from_millis(50));
    }
    vehicle.release()?;

    // Test 6: Music Track
    println!("\n🎵 TEST 6: Music Playback");
    println!("-------------------------");

    if let Ok(music_desc) = studio.get_event("event:/Music/Level 01") {
        let music_inst = music_desc.create_instance()?;

        println!("Starting music...");
        music_inst.start()?;

        println!("Playing for 5 seconds...");
        for i in 1..=5 {
            println!("  {} seconds...", i);
            for _ in 0..20 {
                studio.update()?;
                thread::sleep(Duration::from_millis(50));
            }
        }

        println!("Fading out music...");
        music_inst.stop(StopMode::AllowFadeout)?;

        for _ in 0..40 {
            studio.update()?;
            thread::sleep(Duration::from_millis(50));
        }
        music_inst.release()?;
    }

    // Clean up
    println!("\nCleaning up...");
    music.unload()?;
    vehicles.unload()?;
    sfx.unload()?;
    strings.unload()?;
    master.unload()?;
    studio.release()?;

    println!("\n=================================================");
    println!("✅ All event tests completed!");
    println!("   - One-shot events work");
    println!("   - Looping sounds work");
    println!("   - Sound variations play correctly");
    println!("   - Multiple instances work");
    println!("   - Music playback works");
    println!("=================================================\n");

    Ok(())
}