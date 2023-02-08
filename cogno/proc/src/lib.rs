pub mod close_handle;

#[macro_export]
macro_rules! defer_close {
    ( $child:ident ) => {{
        cogno::close_handle::CloseHandle::new($child)
    }};
}
