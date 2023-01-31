const COGNO_DEBUG: Option<&str> = option_env!("COGNO_DEBUG");

pub fn debug_enabled() -> bool {
    Some("true") == COGNO_DEBUG
}