#[cfg(test)]
macro_rules! when {
    ( $x:expr ) => {
        if !$x {
            return TestResult::discard();
        }
    };
}

#[cfg(test)]
macro_rules! then {
    ( $x:expr ) => {
        TestResult::from_bool($x)
    };
}
