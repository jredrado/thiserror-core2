#![deny(clippy::all, clippy::pedantic)]

use anyhow::anyhow;
use core2::error::Error as _;
use core2::io;
use thiserror_core2::Error;

#[test]
fn test_transparent_struct() {
    #[derive(Error, Debug)]
    #[error(transparent)]
    struct Error(ErrorKind);

    #[derive(Error, Debug)]
    enum ErrorKind {
        #[error("E0")]
        E0,
        #[error("E1")]
        E1(#[from] io::Error),
    }

    let error = Error(ErrorKind::E0);
    assert_eq!("E0", error.to_string());
    assert!(error.source().is_none());

    let io = io::Error::new(io::ErrorKind::Other, "oh no!");
    let error = Error(ErrorKind::from(io));
    assert_eq!("E1", error.to_string());
    error.source().unwrap().downcast_ref::<io::Error>().unwrap();
}

#[test]
#[cfg(feature = "std")]
fn test_transparent_enum() {
    #[derive(Error, Debug)]
    enum Error {
        #[error("this failed")]
        This,
        #[error(transparent)]
        Other(anyhow::Error),
    }

    let error = Error::This;
    assert_eq!("this failed", error.to_string());

    let error = Error::Other(anyhow!("inner").context("outer"));
    assert_eq!("outer", error.to_string());
    assert_eq!("inner", error.source().unwrap().to_string());
}

#[test]
#[cfg(feature = "std")]
fn test_anyhow() {
    #[derive(Error, Debug)]
    #[error(transparent)]
    struct Any(#[from] anyhow::Error);

    let error = Any::from(anyhow!("inner").context("outer"));
    assert_eq!("outer", error.to_string());
    assert_eq!("inner", error.source().unwrap().to_string());
}

#[test]
fn test_non_static() {
    #[derive(Error, Debug)]
    #[error(transparent)]
    struct Error<'a> {
        inner: ErrorKind<'a>,
    }

    #[derive(Error, Debug)]
    enum ErrorKind<'a> {
        #[error("unexpected token: {:?}", token)]
        Unexpected { token: &'a str },
    }

    let error = Error {
        inner: ErrorKind::Unexpected { token: "error" },
    };
    assert_eq!("unexpected token: \"error\"", error.to_string());
    assert!(error.source().is_none());
}
