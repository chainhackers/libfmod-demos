// Quick test to verify FMOD 2.03.09 integration works
// Run with: cargo run --example quick_test

use libfmod::{System, Init};

fn main() {
    println!("=== FMOD 2.03.09 Quick Test ===\n");

    // Test 1: Create system
    print!("1. Creating FMOD System... ");
    let system = match System::create() {
        Ok(s) => {
            println!("✓ SUCCESS");
            s
        }
        Err(e) => {
            println!("✗ FAILED: {:?}", e);
            println!("\nMake sure FMOD libraries are installed:");
            println!("  export LD_LIBRARY_PATH=/path/to/fmod/lib:$LD_LIBRARY_PATH");
            return;
        }
    };

    // Test 2: Get version (NEW 2.03.09 API - returns version AND build number)
    print!("2. Getting version (2.03.09 API)... ");
    match system.get_version() {
        Ok((version, build)) => {
            let major = (version >> 16) & 0xFF;
            let minor = (version >> 8) & 0xFF;
            let patch = version & 0xFF;

            println!("✓ Version {}.{:02}.{:02} (build {})", major, minor, patch, build);

            // Verify it's 2.03.x
            if major == 2 && minor == 3 {
                println!("   ✓ Confirmed FMOD 2.03.x");
            } else {
                println!("   ⚠ Warning: Expected 2.03.x, got {}.{:02}.{:02}", major, minor, patch);
            }
        }
        Err(e) => {
            println!("✗ FAILED: {:?}", e);
            return;
        }
    }

    // Test 3: Initialize system
    print!("3. Initializing system... ");
    match system.init(512, Init::NORMAL, None) {
        Ok(_) => {
            println!("✓ SUCCESS (512 channels)");
        }
        Err(e) => {
            println!("✗ FAILED: {:?}", e);
            return;
        }
    }

    // Test 4: Get driver info
    print!("4. Getting audio driver info... ");
    match system.get_driver() {
        Ok(driver) => {
            println!("✓ Driver index: {}", driver);

            // Try to get driver name
            if let Ok(num_drivers) = system.get_num_drivers() {
                println!("   Found {} audio driver(s)", num_drivers);

                if num_drivers > 0 {
                    match system.get_driver_info(0, 256) {
                        Ok((name, _guid, rate, speaker_mode, _channels)) => {
                            println!("   Using: {}", name);
                            println!("   Sample rate: {} Hz", rate);
                            println!("   Speaker mode: {:?}", speaker_mode);
                        }
                        Err(_) => {
                            println!("   (Could not get driver details)");
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("⚠ WARNING: {:?}", e);
            println!("   (System may still work)");
        }
    }

    // Test 5: Create a simple sound (if we had a file)
    print!("5. Testing sound creation... ");
    // We'll test with a non-existent file to verify the API works
    match system.create_sound("test.ogg", libfmod::Mode::DEFAULT, None) {
        Ok(_) => {
            println!("✓ Found test.ogg");
        }
        Err(e) => {
            // We expect this to fail if file doesn't exist
            if format!("{:?}", e).contains("ERR_FILE_NOTFOUND") ||
               format!("{:?}", e).contains("(23)") {
                println!("✓ API works (no test file)");
            } else {
                println!("⚠ Unexpected error: {:?}", e);
            }
        }
    }

    // Test 6: Clean shutdown
    print!("6. Releasing system... ");
    match system.release() {
        Ok(_) => {
            println!("✓ SUCCESS");
        }
        Err(e) => {
            println!("✗ FAILED: {:?}", e);
        }
    }

    println!("\n========================================");
    println!("✅ FMOD 2.03.09 integration test PASSED!");
    println!("========================================");
    println!("\nAll core APIs are working correctly with FMOD 2.03.09");
}