use getopts::{Fail, Options};
use png::{Decoder, DecodingError};
use std::{
    env,
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    fs::{self, OpenOptions},
    io::Error as IoError,
};

fn print_usage(opts: Options) {
    let brief = format!(
        "{}\nOutput name will be set to FILE.dat\n\nUsage: {} FILE [options]",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_NAME")
    );
    print!("{}", opts.usage(&brief));
}

fn main() -> Result<(), Error> {
    let args = env::args().collect::<Vec<_>>();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");

    let matches = opts.parse(&args[1..])?;

    if matches.opt_present("h") {
        print_usage(opts);

        return Ok(());
    }

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(opts);

        return Ok(());
    };

    let decoder = Decoder::new(OpenOptions::new().read(true).open(input.clone())?);
    let (info, mut reader) = decoder.read_info()?;
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf)?;
    fs::write(format!("{}.dat", input), buf)?;

    Ok(())
}

#[derive(Debug)]
enum Error {
    Getopts(Fail),
    Io(IoError),
    Png(DecodingError),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::Getopts(fa) => write!(f, "{:?}", fa),
            Self::Io(ioe) => write!(f, "{:?}", ioe),
            Self::Png(de) => write!(f, "{:?}", de),
        }
    }
}
impl From<Fail> for Error {
    fn from(src: Fail) -> Self {
        Self::Getopts(src)
    }
}
impl From<IoError> for Error {
    fn from(src: IoError) -> Self {
        Self::Io(src)
    }
}
impl From<DecodingError> for Error {
    fn from(src: DecodingError) -> Self {
        Self::Png(src)
    }
}
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Getopts(f) => Some(f),
            Self::Io(ioe) => Some(ioe),
            Self::Png(de) => Some(de),
        }
    }
}
