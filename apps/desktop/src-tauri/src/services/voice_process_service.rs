use std::{
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
    thread,
};

use serde_json::json;
use tauri::{AppHandle, Emitter};

pub struct VoiceProcessService {
    child: Arc<Mutex<Option<Child>>>,
}

impl VoiceProcessService {
    pub fn new() -> Self {
        Self {
            child: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(&self, app: AppHandle) -> Result<(), String> {
        let mut child_guard = self.child.lock().map_err(|err| err.to_string())?;

        if let Some(child) = child_guard.as_mut() {
            if child.try_wait().map_err(|err| err.to_string())?.is_none() {
                return Ok(());
            }

            *child_guard = None;
        }

        let project_root = project_root();
        let python_path = resolve_python_path(&project_root);
        let worker_path = project_root.join("services").join("voice").join("voice_worker.py");

        if !worker_path.exists() {
            return Err(format!("Voice worker does not exist: {}", worker_path.display()));
        }

        let mut child = Command::new(&python_path)
            .arg(&worker_path)
            .current_dir(&project_root)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| {
                format!(
                    "Cannot start voice worker with '{}': {}",
                    python_path.display(),
                    err
                )
            })?;

        let stdout = child
            .stdout
            .take()
            .ok_or("Cannot read voice worker stdout")?;
        let stderr = child
            .stderr
            .take()
            .ok_or("Cannot read voice worker stderr")?;

        let stdout_app = app.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);

            for line in reader.lines().map_while(Result::ok) {
                if let Ok(event) = serde_json::from_str::<serde_json::Value>(&line) {
                    let _ = stdout_app.emit("voice_event", event);
                } else {
                    let _ = stdout_app.emit(
                        "voice_event",
                        json!({
                            "type": "worker_output",
                            "message": line,
                        }),
                    );
                }
            }
        });

        thread::spawn(move || {
            let reader = BufReader::new(stderr);

            for line in reader.lines().map_while(Result::ok) {
                let _ = app.emit(
                    "voice_event",
                    json!({
                        "type": "worker_error",
                        "message": line,
                    }),
                );
            }
        });

        *child_guard = Some(child);

        Ok(())
    }

    pub fn stop(&self) -> Result<(), String> {
        let mut child_guard = self.child.lock().map_err(|err| err.to_string())?;

        if let Some(mut child) = child_guard.take() {
            child.kill().map_err(|err| err.to_string())?;
            let _ = child.wait();
        }

        Ok(())
    }
}

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")))
        .to_path_buf()
}

fn resolve_python_path(project_root: &Path) -> PathBuf {
    let venv_python = project_root.join(".venv").join("Scripts").join("python.exe");

    if venv_python.exists() {
        return venv_python;
    }

    PathBuf::from("python")
}
