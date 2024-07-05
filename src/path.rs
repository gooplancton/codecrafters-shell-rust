use std::{ffi::OsString, fs, io::Error};

fn list_executables_in_dir(path: &str) -> Result<Vec<String>, Error> {
    let read_dir = fs::read_dir(path)?;
    let executables = read_dir
        .filter(|entry| entry.as_ref().map(|entry| entry.metadata().map(|meta| meta.is_file()).unwrap_or(false)).unwrap_or(false))
        .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
        .collect();
        
    return Ok(executables);
}

pub fn list_executables_in_path(path: &str) -> Vec<String> {
    path.split(":")    
        .filter_map(|dir| list_executables_in_dir(dir).ok())
        .flatten()
        .collect()
}
