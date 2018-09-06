pub mod exec;
pub mod report;

#[doc(hidden)]
#[macro_export]
macro_rules! __bench {
    (levels $($level:expr),*; $($name:expr => $target:expr),*) => {{
        let res = $crate::report::write(
            &mut ::std::io::stdout(),
            &($($target),*),
            &[$($level,)*],
            &[$($name,)*]
        );
        res.expect("fatal error on writing to stdout")
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __find_lf {
    (levels $($level:expr),*;) => {
        __bench!(levels $($level),*; "lockfree" => ());
    };

    (
        levels $($level:expr),*;
        $name:expr => $target:expr $(,$names:expr => $targets:expr)*
    ) => {{
        if $name == "lockfree" {
            __bench!(levels $($level),*; $name => $target);
        } else {
            __find_lf!(levels $($level),*; $($names => $targets),*);
        }
    }};
}

#[macro_export]
macro_rules! bench {
    (levels $($level:expr),*; $($name:expr => $target:expr),*) => {{
        if ::std::env::var("PROFILING").ok().map_or(false, |x| x == "1") {
            __find_lf!(levels $($level),*; $($name => $target),*);
        } else {
            __bench!(levels $($level),*; $($name => $target),*);
        }
    }};

    (levels $($level:expr,)*; $($tok:tt)*) => {
        bench!(levels $($level),*; $($tok)*);
    };

    (levels $($level:expr),*; $($name:expr => $target:expr,)*) => {
        bench!(levels $($level),*; $($name => $target),*);
    };
}
