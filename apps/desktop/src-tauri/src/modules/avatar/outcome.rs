#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarStartOutcome {
    Started,
    AlreadyRunning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarStopOutcome {
    Stopped,
    AlreadyStopped,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarRestartOutcome {
    Restarted,
    Started,
}
