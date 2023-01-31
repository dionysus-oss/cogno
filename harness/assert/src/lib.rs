#[macro_export]
macro_rules! must_eq {
    ( $recorder_thread_ref:ident, $id:literal, $expected:expr, $actual:expr ) => {
        {
            $recorder_thread_ref.lock().unwrap().must_eq($id, $expected, $actual);
        }
    }
}

#[macro_export]
macro_rules! must_not_eq {
    ( $recorder_thread_ref:ident, $id:literal, $expected:expr, $actual:expr ) => {
        {
            $recorder_thread_ref.lock().unwrap().must_not_eq($id, $expected, $actual);
        }
    }
}

#[macro_export]
macro_rules! should_eq {
    ( $recorder_thread_ref:ident, $id:literal, $expected:expr, $actual:expr ) => {
        {
            $recorder_thread_ref.lock().unwrap().should_eq($id, $expected, $actual);
        }
    }
}

#[macro_export]
macro_rules! should_not_eq {
    ( $recorder_thread_ref:ident, $id:literal, $expected:expr, $actual:expr ) => {
        {
            $recorder_thread_ref.lock().unwrap().should_not_eq($id, $expected, $actual);
        }
    }
}

#[macro_export]
macro_rules! may_eq {
    ( $recorder_thread_ref:ident, $id:literal, $expected:expr, $actual:expr ) => {
        {
            $recorder_thread_ref.lock().unwrap().may_eq($id, $expected, $actual);
        }
    }
}
