mod format;
mod generate;

use std::{
    fmt,
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::Path,
};

use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use generate::generate_bindings;

use crate::format::Formatter;

const BSDS: [(&str, &str); 5] = [
    (
        "apple.rs",
        "https://raw.githubusercontent.com/apple-oss-distributions/xnu/main/bsd/sys/errno.h",
    ),
    (
        "freebsd.rs",
        "https://raw.githubusercontent.com/freebsd/freebsd-src/main/sys/sys/errno.h",
    ),
    (
        "dragonflybsd.rs",
        "https://raw.githubusercontent.com/DragonFlyBSD/DragonFlyBSD/master/sys/sys/errno.h",
    ),
    (
        "openbsd.rs",
        "https://raw.githubusercontent.com/openbsd/src/master/sys/sys/errno.h",
    ),
    (
        "netbsd.rs",
        "https://raw.githubusercontent.com/NetBSD/src/trunk/sys/sys/errno.h",
    ),
];

fn main() -> Result<()> {
    color_eyre::install()?;

    let outdir = {
        let mut args = std::env::args();
        if args.len() != 2 {
            bail!("Please provide the output directory");
        }
        _ = args.next();
        args.next().unwrap()
    };

    let formatter = Formatter::new()?;

    for (file, url) in BSDS {
        let bindings = generate_bindings(url)?;

        write_if_ne(
            Path::new(&outdir).to_path_buf().join("src").join(file),
            &formatter,
            &bindings,
        )?;
    }

    Ok(())
}

fn write_if_ne<P, B>(path: P, formatter: &Formatter, content: B) -> Result<()>
where
    P: AsRef<Path>,
    B: fmt::Display,
{
    let content = formatter.format(content)?;
    let path = path.as_ref();

    match slurp(path) {
        Err(err) if err.kind() != std::io::ErrorKind::NotFound => {
            return Err(err).wrap_err("Failed to write results")
        }
        Ok(buf) if content.as_bytes() == buf => return Ok(()),
        _ => (),
    }

    let mut f = File::create(path).wrap_err("Failed to write results")?;
    std::io::Write::write_all(&mut f, content.as_bytes()).wrap_err("Failed to write results")
}

pub fn slurp<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let len = f.seek(SeekFrom::End(0))? as usize;
    f.seek(SeekFrom::Start(0))?;

    let mut buf = Vec::<u8>::with_capacity(len);
    unsafe {
        f.read_exact(std::slice::from_raw_parts_mut(buf.as_mut_ptr(), len))?;
        buf.set_len(len);
    }

    Ok(buf)
}
