use std::io;

use std::io::BufRead;

use std::io::BufWriter;
use std::io::Write;

pub fn replace(s: &mut [u8], before: u8, after: u8) {
    for u in s.iter_mut() {
        if before.eq(u) {
            *u = after;
        }
    }
}

pub fn lines2replaced2writer<I, W>(
    lines: I,
    before: u8,
    after: u8,
    mut writer: W,
) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<Vec<u8>, io::Error>>,
    W: FnMut(&[u8]) -> Result<(), io::Error>,
{
    for rline in lines {
        let mut line: Vec<u8> = rline?;
        replace(&mut line, before, after);
        writer(&line)?;
    }
    Ok(())
}

pub fn reader2replaced2writer<R, W>(
    rdr: R,
    before: u8,
    after: u8,
    mut writer: W,
) -> Result<(), io::Error>
where
    R: BufRead,
    W: Write,
{
    let lines = rdr.split(b'\n');
    let wtr = |replaced: &[u8]| {
        writer.write_all(replaced)?;
        writeln!(writer)
    };
    lines2replaced2writer(lines, before, after, wtr)?;
    writer.flush()
}

pub fn stdin2replaced2stdout(before: u8, after: u8) -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();

    let bw = BufWriter::new(&mut ol);
    reader2replaced2writer(io::stdin().lock(), before, after, bw)?;

    ol.flush()
}
