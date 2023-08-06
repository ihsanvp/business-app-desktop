use crate::constants;

use sha2::{digest::FixedOutput, Digest, Sha256, Sha512};
use std::{
    hash::{Hash, Hasher},
    io::{Read, Write},
    path::PathBuf,
};
use tauri::AppHandle;

struct FingerPrintHasher(Sha256);
struct DeviceHasher(Sha512);

impl FingerPrintHasher {
    fn new() -> Self {
        FingerPrintHasher(Sha256::new())
    }
}

impl DeviceHasher {
    fn new() -> Self {
        DeviceHasher(Sha512::new())
    }
}

impl Hasher for FingerPrintHasher {
    fn finish(&self) -> u64 {
        0
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }
}

impl Hasher for DeviceHasher {
    fn finish(&self) -> u64 {
        0
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }
}

pub fn get_device_hash(key: &String) -> String {
    let mut hasher = DeviceHasher::new();
    let fingerpint = get_device_fingerprint();

    constants::SECRET.hash(&mut hasher);
    fingerpint.hash(&mut hasher);
    key.hash(&mut hasher);

    let hash = hasher.0.finalize_fixed();
    format!("{:x}", hash)
}

pub fn get_device_fingerprint() -> String {
    let mut hasher = FingerPrintHasher::new();

    let platform = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let release = sys_info::os_release().unwrap();
    let total_memory = sys_info::mem_info().unwrap().total;
    let cpus = sys_info::cpu_num().unwrap();

    platform.hash(&mut hasher);
    arch.hash(&mut hasher);
    release.hash(&mut hasher);
    total_memory.hash(&mut hasher);
    cpus.hash(&mut hasher);

    let fingerpint = hasher.0.finalize_fixed();
    format!("{:x}", fingerpint)
}

pub fn get_hash_save_path(handle: &AppHandle) -> PathBuf {
    let base_dir = handle.path_resolver().app_config_dir().unwrap();
    let parent_dir = base_dir.join(".keys");
    let fingerpint = get_device_fingerprint();
    let file_path = parent_dir.join(format!("{}.bin", fingerpint));

    std::fs::create_dir_all(parent_dir).unwrap();
    file_path
}

pub fn get_key_save_path(handle: &AppHandle) -> PathBuf {
    let base_dir = handle.path_resolver().app_config_dir().unwrap();
    let parent_dir = base_dir.join(".keys");
    let file_path = parent_dir.join(format!("activation.bin"));

    std::fs::create_dir_all(parent_dir).unwrap();
    file_path
}

pub fn save_to_file(path: PathBuf, data: String) {
    let mut file = std::fs::File::create(path).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

pub fn read_from_file(path: PathBuf) -> std::io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer).to_string())
}

pub fn check_activation(handle: &AppHandle) -> bool {
    let hash_file = get_hash_save_path(handle);
    let key_file = get_key_save_path(handle);

    let saved_hash = read_from_file(hash_file);
    let saved_key = read_from_file(key_file);

    if saved_hash.is_ok() && saved_key.is_ok() {
        let saved_hash = saved_hash.unwrap();
        let saved_key = saved_key.unwrap();
        let device_hash = get_device_hash(&saved_key);
        return saved_hash == device_hash;
    }

    false
}
