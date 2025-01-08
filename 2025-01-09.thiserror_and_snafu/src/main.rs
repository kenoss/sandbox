mod using_snafu {
    use snafu::prelude::*;
    use std::path::PathBuf;

    #[derive(Debug, Snafu)]
    pub(crate) enum Error {
        #[snafu(display("Unable to read configuration from {}", path.display()))]
        ReadConfiguration {
            source: std::io::Error,
            path: PathBuf,
        },

        #[snafu(display("Unable to write result to {}", path.display()))]
        WriteResult {
            source: std::io::Error,
            path: PathBuf,
        },
    }

    type Result<T, E = Error> = std::result::Result<T, E>;

    pub(crate) fn process_data() -> Result<()> {
        let path = "config.toml";
        let configuration =
            std::fs::read_to_string(path).context(ReadConfigurationSnafu { path })?;
        let path = unpack_config(&configuration);
        std::fs::write(&path, b"My complex calculation").context(WriteResultSnafu { path })?;
        Ok(())
    }

    fn unpack_config(_data: &str) -> &str {
        "/some/path/that/does/not/exist"
    }
}

mod using_thiserror {
    use std::path::PathBuf;

    #[derive(Debug, thiserror::Error)]
    pub(crate) enum Error {
        #[error("Unable to read configuration from {}", path.display())]
        ReadConfiguration {
            source: std::io::Error,
            path: PathBuf,
        },

        #[error("Unable to write result to {}", path.display())]
        WriteResult {
            source: std::io::Error,
            path: PathBuf,
        },
    }

    type Result<T, E = Error> = std::result::Result<T, E>;

    pub(crate) fn process_data() -> Result<()> {
        let path = PathBuf::from("config.toml");
        let configuration = std::fs::read_to_string(path.clone())
            .map_err(|source| Error::ReadConfiguration { source, path })?;
        let path = PathBuf::from(unpack_config(&configuration));
        std::fs::write(&path, b"My complex calculation")
            .map_err(|source| Error::WriteResult { source, path })?;
        Ok(())
    }

    fn unpack_config(_data: &str) -> &str {
        "/some/path/that/does/not/exist"
    }
}

fn main() {
    match using_snafu::process_data() {
        Ok(()) => {}
        Err(e) => {
            println!("{:?}", e);
        }
    }

    match using_thiserror::process_data() {
        Ok(()) => {}
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
