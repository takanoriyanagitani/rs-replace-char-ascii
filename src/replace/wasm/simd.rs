use std::arch::wasm32;

use std::io;

use std::io::BufRead;
use std::io::Read;

use std::io::BufWriter;
use std::io::Write;

use wasm32::v128;

pub fn replace_v(original: v128, replace: v128, bools: v128) -> v128 {
    wasm32::v128_bitselect(replace, original, bools)
}

pub fn write_replaced<W>(
    original: &[u8],
    before: v128,
    after: v128,
    b: u8,
    a: u8,
    wtr: &mut W,
) -> Result<(), io::Error>
where
    W: FnMut(&[u8]) -> Result<(), io::Error>,
{
    let mut chunks = original.chunks_exact(16);
    for chunk in chunks.by_ref() {
        let ptr: *const u8 = chunk.as_ptr();
        let vp: *const v128 = ptr as *const v128;

        #[allow(unsafe_code)]
        let vr: &v128 = unsafe { vp.as_ref().unwrap() };

        let input: v128 = *vr;
        let bools: v128 = wasm32::u8x16_eq(input, before);
        let replaced: v128 = replace_v(input, after, bools);

        let outs: &[v128] = &[replaced];
        let outp: *const v128 = outs.as_ptr();
        let outu: *const u128 = outp as *const u128;

        #[allow(unsafe_code)]
        let outr: &u128 = unsafe { outu.as_ref().unwrap() };

        let output: u128 = *outr;
        let out: [u8; 16] = output.to_le_bytes();
        wtr(&out)?;
    }

    let remainder: &[u8] = chunks.remainder();
    for u in remainder {
        let out: u8 = match b.eq(u) {
            true => a,
            _ => *u,
        };
        wtr(&[out])?;
    }
    Ok(())
}

pub const BUFSIZ: usize = 16384;

pub fn rdr2replaced2writer<R, W>(
    mut rdr: R,
    before: u8,
    after: u8,
    mut writer: W,
) -> Result<(), io::Error>
where
    R: Read,
    W: FnMut(&[u8]) -> Result<(), io::Error>,
{
    let vb: v128 = wasm32::u8x16_splat(before);
    let va: v128 = wasm32::u8x16_splat(after);

    loop {
        let mut buf: [u8; BUFSIZ] = [0; BUFSIZ];
        let mut ms: &mut [u8] = &mut buf;

        let mut taken = rdr.by_ref().take(BUFSIZ as u64);
        let cnt: u64 = io::copy(&mut taken, &mut ms)?;
        if 0 == cnt {
            return Ok(());
        }

        let s: &[u8] = &buf[..(cnt as usize)];
        write_replaced(s, vb, va, before, after, &mut writer)?;
    }
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
    let wtr = |replaced: &[u8]| writer.write_all(replaced);
    rdr2replaced2writer(rdr, before, after, wtr)?;
    writer.flush()
}

pub fn stdin2replaced2stdout(before: u8, after: u8) -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();

    let bw = BufWriter::new(&mut ol);
    reader2replaced2writer(io::stdin().lock(), before, after, bw)?;

    ol.flush()
}
