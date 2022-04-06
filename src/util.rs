#[macro_export]
macro_rules! fail {
    ( $y:expr ) => {{
        return Err($y.into());
    }};
}

#[macro_export]
macro_rules! ensure {
    ( $x:expr, $y:expr $(,)? ) => {{
        if !$x {
            $crate::fail!($y);
        }
    }};
}
