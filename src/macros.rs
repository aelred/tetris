macro_rules! when {
    ( $x:expr ) => {
        if !$x {
            return TestResult::discard();
        }
    };
}

macro_rules! then {
    ( $x:expr ) => {
        TestResult::from_bool($x)
    };
}
