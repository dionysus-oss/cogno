#[macro_export]
macro_rules! must_eq {
    ( $controller_thread_ref:ident, $id:literal, $expected:expr, $actual:expr ) => {{
        let assert_result = $controller_thread_ref
            .lock()
            .unwrap()
            .must_eq($id, $expected, $actual);
        // Must not panic when the mutex lock is held
        assert_result.unwrap();
    }};
}

#[macro_export]
macro_rules! must_not_eq {
    ( $controller_thread_ref:ident, $id:literal, $expected:expr, $actual:expr ) => {{
        let assert_result = $controller_thread_ref
            .lock()
            .unwrap()
            .must_not_eq($id, $expected, $actual);
        // Must not panic when the mutex lock is held
        assert_result.unwrap();
    }};
}

#[macro_export]
macro_rules! should_eq {
    ( $controller_thread_ref:ident, $id:literal, $expected:expr, $actual:expr ) => {{
        let assert_result = $controller_thread_ref
            .lock()
            .unwrap()
            .should_eq($id, $expected, $actual);
        // Must not panic when the mutex lock is held
        assert_result.unwrap();
    }};
}

#[macro_export]
macro_rules! should_not_eq {
    ( $controller_thread_ref:ident, $id:literal, $expected:expr, $actual:expr ) => {{
        let assert_result = $controller_thread_ref
            .lock()
            .unwrap()
            .should_not_eq($id, $expected, $actual);
        // Must not panic when the mutex lock is held
        assert_result.unwrap();
    }};
}

#[macro_export]
macro_rules! may_eq {
    ( $controller_thread_ref:ident, $id:literal, $expected:expr, $actual:expr ) => {{
        let assert_result = $controller_thread_ref
            .lock()
            .unwrap()
            .may_eq($id, $expected, $actual);
        // Must not panic when the mutex lock is held
        assert_result.unwrap();
    }};
}
