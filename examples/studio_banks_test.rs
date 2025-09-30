// Test FMOD Studio bank loading and management with FMOD 2.03.09
// Run with: ./run_fmod.sh studio_banks_test

use libfmod::{Studio, StudioInit, Init, LoadBank};
use libfmod_demos;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎵 FMOD Studio Banks Test (2.03.09)\n");
    println!("====================================\n");

    // Initialize Studio System
    print!("Creating Studio System... ");
    let studio = Studio::create()?;
    println!("✓");

    print!("Initializing Studio... ");
    studio.initialize(1024, StudioInit::NORMAL, Init::NORMAL, None)?;
    println!("✓\n");

    // Define bank paths - using FMOD SDK examples
    let bank_dir = libfmod_demos::get_example_banks_dir()?;

    // Test 1: Load Master Banks
    println!("📦 TEST 1: Loading Master Banks");
    println!("--------------------------------");

    let master_path = format!("{}/Master.bank", bank_dir);
    let strings_path = format!("{}/Master.strings.bank", bank_dir);

    if !Path::new(&master_path).exists() {
        println!("⚠️  FMOD SDK banks not found at: {}", bank_dir);
        println!("   Using test banks instead...");

        // Fall back to test banks
        let master = studio.load_bank_file("./tests/data/Build/Desktop/Master.bank", LoadBank::NORMAL)?;
        println!("✓ Loaded test Master.bank");

        let strings = studio.load_bank_file("./tests/data/Build/Desktop/Master.strings.bank", LoadBank::NORMAL)?;
        println!("✓ Loaded test Master.strings.bank");

        master.unload()?;
        strings.unload()?;
    } else {
        let master = studio.load_bank_file(&master_path, LoadBank::NORMAL)?;
        println!("✓ Loaded Master.bank");

        let strings = studio.load_bank_file(&strings_path, LoadBank::NORMAL)?;
        println!("✓ Loaded Master.strings.bank");

        // Test 2: Load Additional Banks
        println!("\n📦 TEST 2: Loading Content Banks");
        println!("---------------------------------");

        let banks = vec![
            ("SFX.bank", "Sound Effects"),
            ("Music.bank", "Music"),
            ("Vehicles.bank", "Vehicles"),
            ("VO.bank", "Voice Over"),
        ];

        for (bank_name, description) in &banks {
            let path = format!("{}/{}", bank_dir, bank_name);
            match studio.load_bank_file(&path, LoadBank::NORMAL) {
                Ok(bank) => {
                    println!("✓ Loaded {} - {}", bank_name, description);

                    // Get bank info
                    if let Ok(count) = bank.get_event_count() {
                        println!("  → Contains {} events", count);
                    }

                    bank.unload()?;
                }
                Err(e) => {
                    println!("⚠️  Failed to load {}: {:?}", bank_name, e);
                }
            }
        }

        // Test 3: List Available Events
        println!("\n📦 TEST 3: Listing Available Events");
        println!("------------------------------------");

        // Reload SFX bank to list its events
        let sfx_path = format!("{}/SFX.bank", bank_dir);
        if let Ok(sfx) = studio.load_bank_file(&sfx_path, LoadBank::NORMAL) {
            if let Ok(count) = sfx.get_event_count() {
                println!("SFX.bank contains {} events:", count);

                if let Ok(events) = sfx.get_event_list(count) {
                    for (i, event) in events.iter().enumerate().take(5) {
                        if let Ok(path) = event.get_path() {
                            println!("  {}. {}", i + 1, path);
                        }
                    }
                    if count > 5 {
                        println!("  ... and {} more", count - 5);
                    }
                }
            }
            sfx.unload()?;
        }

        // Test 4: Load from Memory
        println!("\n📦 TEST 4: Loading Bank from Memory");
        println!("------------------------------------");

        let vo_path = format!("{}/VO.bank", bank_dir);
        if let Ok(vo_data) = std::fs::read(&vo_path) {
            let vo_bank = studio.load_bank_memory(&vo_data, LoadBank::NORMAL)?;
            println!("✓ Loaded VO.bank from memory ({} bytes)", vo_data.len());
            vo_bank.unload()?;
        }

        // Clean up
        strings.unload()?;
        master.unload()?;
    }

    // Test 5: Error Handling
    println!("\n📦 TEST 5: Error Handling");
    println!("-------------------------");

    match studio.load_bank_file("nonexistent.bank", LoadBank::NORMAL) {
        Ok(_) => println!("❌ Should have failed on missing bank"),
        Err(_) => println!("✓ Correctly handled missing bank"),
    }

    // Release Studio
    studio.release()?;

    println!("\n====================================");
    println!("✅ All bank tests completed!");
    println!("====================================\n");

    Ok(())
}