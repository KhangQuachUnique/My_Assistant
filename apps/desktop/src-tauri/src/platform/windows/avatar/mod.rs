use std::{
    path::{Path, PathBuf},
    process::{Child, Command},
    sync::Mutex,
};

use crate::modules::avatar::{AvatarError, AvatarProcessRunner};

const AVATAR_EXE_RESOURCE_PATH: &[&str] = &["resources", "modules", "avatar", "avatar.exe"];

#[derive(Debug)]
pub struct WindowsAvatarProcessRunner {
    executable_path: PathBuf,
    arguments: Vec<String>,
    child: Mutex<Option<Child>>,
}

impl WindowsAvatarProcessRunner {
    pub fn new() -> Self {
        Self::with_executable_path(resolve_avatar_executable_path())
    }

    pub fn with_executable_path(executable_path: PathBuf) -> Self {
        Self {
            executable_path,
            arguments: Vec::new(),
            child: Mutex::new(None),
        }
    }

    #[cfg(test)]
    fn with_executable_path_and_arguments(
        executable_path: PathBuf,
        arguments: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            executable_path,
            arguments: arguments.into_iter().map(Into::into).collect(),
            child: Mutex::new(None),
        }
    }

    fn lock_child(&self) -> Result<std::sync::MutexGuard<'_, Option<Child>>, AvatarError> {
        self.child
            .lock()
            .map_err(|_| AvatarError::ProcessStatusUnavailable(String::from("lock poisoned")))
    }

    fn refresh_child_state(child: &mut Option<Child>) -> Result<bool, AvatarError> {
        let Some(process) = child.as_mut() else {
            return Ok(false);
        };

        match process.try_wait() {
            Ok(Some(_status)) => {
                *child = None;
                Ok(false)
            }
            Ok(None) => Ok(true),
            Err(error) => Err(AvatarError::ProcessStatusUnavailable(error.to_string())),
        }
    }

    #[cfg(test)]
    fn child_id(&self) -> Result<Option<u32>, AvatarError> {
        let mut child = self.lock_child()?;

        if !Self::refresh_child_state(&mut child)? {
            return Ok(None);
        }

        Ok(child.as_ref().map(Child::id))
    }
}

impl Default for WindowsAvatarProcessRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl AvatarProcessRunner for WindowsAvatarProcessRunner {
    fn start(&self) -> Result<(), AvatarError> {
        if !self.executable_path.is_file() {
            return Err(AvatarError::ExeNotFound(self.executable_path.clone()));
        }

        let mut child = self.lock_child()?;

        if Self::refresh_child_state(&mut child)? {
            return Ok(());
        }

        let process = Command::new(&self.executable_path)
            .args(&self.arguments)
            .spawn()
            .map_err(|error| AvatarError::ProcessStartFailed(error.to_string()))?;

        *child = Some(process);

        Ok(())
    }

    fn stop(&self) -> Result<(), AvatarError> {
        let mut child = self.lock_child()?;

        if !Self::refresh_child_state(&mut child)? {
            return Ok(());
        }

        if let Some(mut process) = child.take() {
            process
                .kill()
                .map_err(|error| AvatarError::ProcessStopFailed(error.to_string()))?;
            process
                .wait()
                .map_err(|error| AvatarError::ProcessStopFailed(error.to_string()))?;
        }

        Ok(())
    }

    fn is_running(&self) -> Result<bool, AvatarError> {
        let mut child = self.lock_child()?;

        Self::refresh_child_state(&mut child)
    }
}

fn resolve_avatar_executable_path() -> PathBuf {
    let manifest_resource = resource_path_from(Path::new(env!("CARGO_MANIFEST_DIR")));

    if manifest_resource.is_file() {
        return manifest_resource;
    }

    std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(resource_path_from))
        .unwrap_or(manifest_resource)
}

fn resource_path_from(base_path: &Path) -> PathBuf {
    AVATAR_EXE_RESOURCE_PATH
        .iter()
        .fold(base_path.to_path_buf(), |path, segment| path.join(segment))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn powershell_path() -> PathBuf {
        PathBuf::from(r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe")
    }

    #[test]
    fn stop_is_safe_when_process_was_not_started() {
        let runner = WindowsAvatarProcessRunner::with_executable_path(PathBuf::from("missing.exe"));

        runner.stop().expect("stop without process should be safe");
        assert!(!runner
            .is_running()
            .expect("status should be readable after stop"));
    }

    #[test]
    fn start_reports_missing_executable() {
        let missing_path = PathBuf::from("definitely-missing-avatar.exe");
        let runner = WindowsAvatarProcessRunner::with_executable_path(missing_path.clone());

        let result = runner.start();

        assert!(matches!(result, Err(AvatarError::ExeNotFound(path)) if path == missing_path));
    }

    #[test]
    fn duplicate_start_does_not_spawn_second_process() {
        let runner = WindowsAvatarProcessRunner::with_executable_path_and_arguments(
            powershell_path(),
            ["-NoProfile", "-Command", "Start-Sleep -Seconds 10"],
        );

        runner.start().expect("first start should spawn process");
        let first_process_id = runner
            .child_id()
            .expect("child id should be readable")
            .expect("process should be running after first start");

        runner.start().expect("second start should be safe");
        let second_process_id = runner
            .child_id()
            .expect("child id should be readable")
            .expect("process should still be running after second start");

        runner.stop().expect("process should stop");

        assert_eq!(first_process_id, second_process_id);
        assert!(!runner
            .is_running()
            .expect("status should be readable after stop"));
    }
}
