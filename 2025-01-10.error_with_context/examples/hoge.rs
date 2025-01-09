mod hoge_lib {
    use error_with_context::ErrorContextI;

    #[derive(Debug, thiserror::Error)]
    pub enum Error<C>
    where
        C: ErrorContextI,
    {
        #[error("hoge: {source}")]
        Hoge { source: std::io::Error, context: C },
        #[error("fuga: {source}")]
        Fuga { source: std::io::Error, context: C },
    }

    pub fn hoge<C>() -> Result<(), Error<C>>
    where
        C: ErrorContextI,
    {
        let _file = std::fs::File::open("/hoge").map_err(|e| Error::Hoge {
            context: C::new_error_context(&e),
            source: e,
        })?;
        Ok(())
    }

    // pub fn hoge2<C>() -> Result<(), Error<C>> {
    //     hoge().context()?;
    //     Ok(())
    // }

    pub fn fuga<C>() -> Result<(), Error<C>>
    where
        C: ErrorContextI,
    {
        let _file = std::fs::File::open("/fuga").map_err(|e| Error::Fuga {
            context: C::new_error_context(&e),
            source: e,
        })?;
        Ok(())
    }
}

mod hoge_app {
    use super::hoge_lib;
    use error_with_context::ErrorContextI;

    #[derive(Debug)]
    struct ErrorContext {
        location: &'static core::panic::Location<'static>,
    }

    impl ErrorContextI for ErrorContext {
        #[inline]
        #[track_caller]
        fn new_error_context(_e: &dyn std::error::Error) -> Self {
            Self {
                location: core::panic::Location::caller(),
            }
        }
    }

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("hoge_lib: {source}")]
        HogeLib {
            source: hoge_lib::Error<ErrorContext>,
            context: ErrorContext,
        },
        #[error("mine: {source}")]
        Mine {
            source: std::io::Error,
            context: ErrorContext,
        },
    }

    impl From<hoge_lib::Error<ErrorContext>> for Error {
        fn from(source: hoge_lib::Error<ErrorContext>) -> Error {
            Error::HogeLib {
                context: ErrorContext::new_error_context(&source),
                source,
            }
        }
    }

    pub fn main(i: usize) -> Result<(), Error> {
        // match i {
        //     0 => hoge_lib::hoge().context()?,
        //     1 => hoge_lib::hoge2().context()?,
        //     2 => hoge_lib::fuga().context()?,
        //     3 => std::fs::File::open("/foo").map_err(|e| Error::Mine { source: e, context: ErrorContext::new_error_context(&e) })?;
        //     _ => unreachable!(),
        // }
        match i {
            0 => hoge_lib::hoge::<ErrorContext>()?,
            // 1 => hoge_lib::hoge2()?,
            1 => (),
            2 => hoge_lib::fuga::<ErrorContext>()?,
            3 => {
                std::fs::File::open("/foo").map_err(|e| Error::Mine {
                    context: ErrorContext::new_error_context(&e),
                    source: e,
                })?;
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}

fn main() {
    for i in 0..4 {
        dbg!(hoge_app::main(i));
    }
}
