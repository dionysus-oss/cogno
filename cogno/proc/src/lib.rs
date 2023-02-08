/// Used by the test harness. Not for direct use.
pub mod close_handle;

/// Ensure that a [`std::process::Child`] is killed at the end of a test.
///
/// If the child process has not completed or been stopped otherwise, then an attempt will be made
/// to kill the child before continuing to the next test.
///
/// Example:
/// ```
/// #[macro_use]
/// extern crate cogno;
/// use std::process::Command;
///
/// let child = Command::new("ls").arg("-al").spawn().unwrap();
/// let mut close_handle = defer_close!(child);
/// ```
#[macro_export]
macro_rules! defer_close {
    ( $child:ident ) => {{
        cogno::close_handle::CloseHandle::new($child)
    }};
}
