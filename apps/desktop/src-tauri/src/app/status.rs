#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplicationStatus {
    Created,
    Initializing,
    Ready,
    Running,
    Stopping,
    Stopped,
}
