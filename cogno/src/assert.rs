/// Assert that two values _MUST_ be equal.
///
/// The macro accepts an assertion identifier and two values to compare
///
/// ```
/// must_eq!("my_rfc_1234_section_1_2_my_requirement", 'a', 'a');
/// ```
///
/// The assertion identifier must be unique within the test. It is desirable that it is also
/// globally unique but this is not a hard requirement.
///
/// _Note_ The equality check must not panic. If you need to `unwrap` etc, do it before calling the assertion
/// and only assert expressions which are not expected to panic.
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

/// Assert that two values _MUST NOT_ be equal.
///
/// The macro accepts an assertion identifier and two values to compare
///
/// ```
/// must_not_eq!("my_rfc_1234_section_1_2_my_requirement", 'a', 'b');
/// ```
///
/// The assertion identifier must be unique within the test. It is desirable that it is also
/// globally unique but this is not a hard requirement.
///
/// _Note_ The equality check must not panic. If you need to `unwrap` etc, do it before calling the assertion
/// and only assert expressions which are not expected to panic.
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

/// Assert that two values _SHOULD_ be equal.
///
/// The macro accepts an assertion identifier and two values to compare
///
/// ```
/// should_eq!("my_rfc_1234_section_1_2_my_requirement", 'a', 'a');
/// ```
///
/// The assertion identifier must be unique within the test. It is desirable that it is also
/// globally unique but this is not a hard requirement.
///
/// _Note_ The equality check must not panic. If you need to `unwrap` etc, do it before calling the assertion
/// and only assert expressions which are not expected to panic.
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

/// Assert that two values _SHOULD NOT_ be equal.
///
/// The macro accepts an assertion identifier and two values to compare
///
/// ```
/// should_not_eq!("my_rfc_1234_section_1_2_my_requirement", 'a', 'b');
/// ```
///
/// The assertion identifier must be unique within the test. It is desirable that it is also
/// globally unique but this is not a hard requirement.
///
/// _Note_ The equality check must not panic. If you need to `unwrap` etc, do it before calling the assertion
/// and only assert expressions which are not expected to panic.
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

/// Assert that two values _MAY_ be equal.
///
/// The macro accepts an assertion identifier and two values to compare
///
/// ```
/// may_eq!("my_rfc_1234_section_1_2_my_requirement", 'a', 'a');
/// ```
///
/// The assertion identifier must be unique within the test. It is desirable that it is also
/// globally unique but this is not a hard requirement.
///
/// _Note_ The equality check must not panic. If you need to `unwrap` etc, do it before calling the assertion
/// and only assert expressions which are not expected to panic.
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
