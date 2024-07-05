use std::{
    collections::HashMap, env, fs, io::Error
};

fn list_executables_in_dir(path: &str) -> Result<HashMap<String, String>, Error> {
    let read_dir = fs::read_dir(path)?;
    let entries: Vec<(String, String)> = read_dir
        .filter(|entry| {
            entry
                .as_ref()
                .map(|entry| entry.metadata().map(|meta| meta.is_file()).unwrap_or(false))
                .unwrap_or(false)
        })
        .map(|entry| {
            let entry = entry.unwrap();
            let executable = entry.file_name().to_string_lossy().to_string();
            let path = entry.path().to_string_lossy().to_string();

            (executable, path)
        })
        .collect();

    let executables = HashMap::from_iter(entries);

    return Ok(executables);
}

pub fn list_executables_in_path() -> HashMap<String, String> {
    let path = env::var("PATH").unwrap_or_default();

    path.split(":")
        .filter_map(|dir| list_executables_in_dir(dir).ok())
        .flatten()
        .collect()
}
