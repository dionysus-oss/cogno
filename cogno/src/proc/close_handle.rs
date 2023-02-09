use std::process::Child;

/// Used by the test harness. Not for direct use.
///
/// This is a newtype wrapper around [`std::process::Child`] and implements the [`Drop`] trait.
/// It sends a kill signal to the child process when the handle is dropped.
pub struct CloseHandle(pub Child);

impl CloseHandle {
    pub fn new(child: Child) -> Self {
        tracing::info!("taking responsibility for closing process [{}]", child.id());
        CloseHandle(child)
    }

    pub fn command(&mut self) -> &mut Child {
        &mut self.0
    }
}

impl Drop for CloseHandle {
    fn drop(&mut self) {
        tracing::info!("waiting for process [{}] to finish", self.0.id());
        match self.0.kill() {
            Ok(()) => {
                tracing::info!("process closed successfully");
            },
            Err(e) => {
                tracing::error!("closing process failed - {}", e);
            }
        }
    }
}
