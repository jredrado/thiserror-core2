#![cfg_attr(thiserror_nightly_testing, feature(backtrace))]

use thiserror_core2::Error;

#[derive(Error, Debug)]
#[error("...")]
pub struct Inner;

#[cfg(thiserror_nightly_testing)]
#[derive(Error, Debug)]
#[error("...")]
pub struct InnerBacktrace {
    backtrace: core2::backtrace::Backtrace,
}

#[cfg(thiserror_nightly_testing)]
pub mod structs {
    use super::{Inner, InnerBacktrace};
    use core2::backtrace::Backtrace;
    use core2::error::Error;
    use core2::sync::Arc;
    use thiserror_core2::Error;

    #[derive(Error, Debug)]
    #[error("...")]
    pub struct PlainBacktrace {
        backtrace: Backtrace,
    }

    #[derive(Error, Debug)]
    #[error("...")]
    pub struct ExplicitBacktrace {
        #[backtrace]
        backtrace: Backtrace,
    }

    #[derive(Error, Debug)]
    #[error("...")]
    pub struct OptBacktrace {
        #[backtrace]
        backtrace: Option<Backtrace>,
    }

    #[derive(Error, Debug)]
    #[error("...")]
    pub struct ArcBacktrace {
        #[backtrace]
        backtrace: Arc<Backtrace>,
    }

    #[derive(Error, Debug)]
    #[error("...")]
    pub struct BacktraceFrom {
        #[from]
        source: Inner,
        #[backtrace]
        backtrace: Backtrace,
    }

    #[derive(Error, Debug)]
    #[error("...")]
    pub struct CombinedBacktraceFrom {
        #[from]
        #[backtrace]
        source: InnerBacktrace,
    }

    #[derive(Error, Debug)]
    #[error("...")]
    pub struct OptBacktraceFrom {
        #[from]
        source: Inner,
        #[backtrace]
        backtrace: Option<Backtrace>,
    }

    #[derive(Error, Debug)]
    #[error("...")]
    pub struct ArcBacktraceFrom {
        #[from]
        source: Inner,
        #[backtrace]
        backtrace: Arc<Backtrace>,
    }

    #[test]
    fn test_backtrace() {
        let error = PlainBacktrace {
            backtrace: Backtrace::capture(),
        };
        assert!(error.backtrace().is_some());

        let error = ExplicitBacktrace {
            backtrace: Backtrace::capture(),
        };
        assert!(error.backtrace().is_some());

        let error = OptBacktrace {
            backtrace: Some(Backtrace::capture()),
        };
        assert!(error.backtrace().is_some());

        let error = ArcBacktrace {
            backtrace: Arc::new(Backtrace::capture()),
        };
        assert!(error.backtrace().is_some());

        let error = BacktraceFrom::from(Inner);
        assert!(error.backtrace().is_some());

        let error = CombinedBacktraceFrom::from(InnerBacktrace {
            backtrace: Backtrace::capture(),
        });
        assert!(error.backtrace().is_some());

        let error = OptBacktraceFrom::from(Inner);
        assert!(error.backtrace().is_some());

        let error = ArcBacktraceFrom::from(Inner);
        assert!(error.backtrace().is_some());
    }
}

#[cfg(thiserror_nightly_testing)]
pub mod enums {
    use super::{Inner, InnerBacktrace};
    use core2::backtrace::Backtrace;
    use core2::error::Error;
    use core2::sync::Arc;
    use thiserror_core2::Error;

    #[derive(Error, Debug)]
    pub enum PlainBacktrace {
        #[error("...")]
        Test { backtrace: Backtrace },
    }

    #[derive(Error, Debug)]
    pub enum ExplicitBacktrace {
        #[error("...")]
        Test {
            #[backtrace]
            backtrace: Backtrace,
        },
    }

    #[derive(Error, Debug)]
    pub enum OptBacktrace {
        #[error("...")]
        Test {
            #[backtrace]
            backtrace: Option<Backtrace>,
        },
    }

    #[derive(Error, Debug)]
    pub enum ArcBacktrace {
        #[error("...")]
        Test {
            #[backtrace]
            backtrace: Arc<Backtrace>,
        },
    }

    #[derive(Error, Debug)]
    pub enum BacktraceFrom {
        #[error("...")]
        Test {
            #[from]
            source: Inner,
            #[backtrace]
            backtrace: Backtrace,
        },
    }

    #[derive(Error, Debug)]
    pub enum CombinedBacktraceFrom {
        #[error("...")]
        Test {
            #[from]
            #[backtrace]
            source: InnerBacktrace,
        },
    }

    #[derive(Error, Debug)]
    pub enum OptBacktraceFrom {
        #[error("...")]
        Test {
            #[from]
            source: Inner,
            #[backtrace]
            backtrace: Option<Backtrace>,
        },
    }

    #[derive(Error, Debug)]
    pub enum ArcBacktraceFrom {
        #[error("...")]
        Test {
            #[from]
            source: Inner,
            #[backtrace]
            backtrace: Arc<Backtrace>,
        },
    }

    #[test]
    fn test_backtrace() {
        let error = PlainBacktrace::Test {
            backtrace: Backtrace::capture(),
        };
        assert!(error.backtrace().is_some());

        let error = ExplicitBacktrace::Test {
            backtrace: Backtrace::capture(),
        };
        assert!(error.backtrace().is_some());

        let error = OptBacktrace::Test {
            backtrace: Some(Backtrace::capture()),
        };
        assert!(error.backtrace().is_some());

        let error = ArcBacktrace::Test {
            backtrace: Arc::new(Backtrace::capture()),
        };
        assert!(error.backtrace().is_some());

        let error = BacktraceFrom::from(Inner);
        assert!(error.backtrace().is_some());

        let error = CombinedBacktraceFrom::from(InnerBacktrace {
            backtrace: Backtrace::capture(),
        });
        assert!(error.backtrace().is_some());

        let error = OptBacktraceFrom::from(Inner);
        assert!(error.backtrace().is_some());

        let error = ArcBacktraceFrom::from(Inner);
        assert!(error.backtrace().is_some());
    }
}

#[test]
#[cfg_attr(not(thiserror_nightly_testing), ignore)]
fn test_backtrace() {}
