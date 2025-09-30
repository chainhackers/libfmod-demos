use std::env;

/// Get the FMOD SDK directory from environment variable
pub fn get_fmod_sdk_dir() -> Result<String, String> {
    env::var("FMOD_SDK_DIR")
        .map_err(|_| "FMOD_SDK_DIR environment variable not set. Please set it to your FMOD SDK path.".to_string())
}

/// Get the path to FMOD Studio example banks
pub fn get_example_banks_dir() -> Result<String, String> {
    Ok(format!("{}/api/studio/examples/media", get_fmod_sdk_dir()?))
}

/// Get the path to a specific example bank file
pub fn get_example_bank_path(bank_name: &str) -> Result<String, String> {
    Ok(format!("{}/{}", get_example_banks_dir()?, bank_name))
}