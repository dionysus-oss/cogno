use std::process::Child;

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
        match self.0.wait() {
            Ok(exit_code) => {
                if exit_code.success() {
                    tracing::info!("process closed successfully")
                } else {
                    panic!("failed to close process")
                }
            },
            Err(e) => {
                tracing::error!("closing process failed - {}", e);
            }
        }
    }
}
