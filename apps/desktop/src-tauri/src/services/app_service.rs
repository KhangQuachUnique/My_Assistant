use std::{
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

pub fn open_app(app: &str) -> Result<String, String> {
    let target = resolve_app_target(app)?;

    match &target {
        AppTarget::Path(path) => open_shell_target(path)?,
        AppTarget::Command(command) => {
            Command::new(command)
                .spawn()
                .map_err(|err| format!("Cannot open app '{}': {}", app, err))?;
        }
    }

    Ok(target.describe())
}

enum AppTarget {
    Path(PathBuf),
    Command(String),
}

impl AppTarget {
    fn describe(&self) -> String {
        match self {
            AppTarget::Path(path) => path.display().to_string(),
            AppTarget::Command(command) => command.clone(),
        }
    }
}

fn resolve_app_target(app: &str) -> Result<AppTarget, String> {
    let app = app.trim();

    if app.is_empty() {
        return Err("Missing app name".to_string());
    }

    let input_path = PathBuf::from(app);
    if input_path.exists() {
        return Ok(AppTarget::Path(input_path));
    }

    if let Some(shortcut) = find_shortcut(app) {
        return Ok(AppTarget::Path(shortcut));
    }

    Ok(AppTarget::Command(resolve_command_alias(app)))
}

fn find_shortcut(app: &str) -> Option<PathBuf> {
    let query = normalize_name(app);
    let mut fallback_match = None;

    for directory in shortcut_directories() {
        for shortcut in collect_shortcuts(&directory) {
            let Some(stem) = shortcut.file_stem().and_then(OsStr::to_str) else {
                continue;
            };

            let normalized_stem = normalize_name(stem);

            if normalized_stem == query {
                return Some(shortcut);
            }

            if fallback_match.is_none() && normalized_stem.contains(&query) {
                fallback_match = Some(shortcut);
            }
        }
    }

    fallback_match
}

fn shortcut_directories() -> Vec<PathBuf> {
    let mut directories = Vec::new();

    if let Some(user_profile) = env_path("USERPROFILE") {
        directories.push(user_profile.join("Desktop"));
        directories.push(user_profile.join("AppData/Roaming/Microsoft/Windows/Start Menu/Programs"));
    }

    if let Some(app_data) = env_path("APPDATA") {
        directories.push(app_data.join("Microsoft/Windows/Start Menu/Programs"));
    }

    if let Some(program_data) = env_path("PROGRAMDATA") {
        directories.push(program_data.join("Microsoft/Windows/Start Menu/Programs"));
    }

    if let Some(public_dir) = env_path("PUBLIC") {
        directories.push(public_dir.join("Desktop"));
    }

    directories
}

fn collect_shortcuts(directory: &Path) -> Vec<PathBuf> {
    let mut shortcuts = Vec::new();
    collect_shortcuts_inner(directory, &mut shortcuts, 0);
    shortcuts
}

fn collect_shortcuts_inner(directory: &Path, shortcuts: &mut Vec<PathBuf>, depth: usize) {
    if depth > 5 {
        return;
    }

    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            collect_shortcuts_inner(&path, shortcuts, depth + 1);
            continue;
        }

        if is_shortcut_file(&path) {
            shortcuts.push(path);
        }
    }
}

fn is_shortcut_file(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .map(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "lnk" | "url" | "appref-ms"
            )
        })
        .unwrap_or(false)
}

fn open_shell_target(path: &Path) -> Result<(), String> {
    Command::new("cmd")
        .args(["/C", "start", ""])
        .arg(path)
        .spawn()
        .map_err(|err| format!("Cannot open shortcut '{}': {}", path.display(), err))?;

    Ok(())
}

fn resolve_command_alias(app: &str) -> String {
    match normalize_name(app).as_str() {
        "calculator" => "calc".to_string(),
        "note" | "notepad" => "notepad".to_string(),
        "paint" => "mspaint".to_string(),
        "vscode" | "visualstudiocode" => "code".to_string(),
        _ => app.to_string(),
    }
}

fn env_path(key: &str) -> Option<PathBuf> {
    env::var_os(key).map(PathBuf::from)
}

fn normalize_name(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}
