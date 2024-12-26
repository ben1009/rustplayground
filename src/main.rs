// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

fn main() {}

use std::iter;

#[inline(never)]
pub fn mismatch(xs: &[u8], ys: &[u8]) -> usize {
    xs.iter().zip(ys).take_while(|(x, y)| x == y).count()
}

#[cfg(target_arch = "x86_64")]
#[inline(never)]
pub fn mismatch_simd(xs: &[u8], ys: &[u8]) -> usize {
    let l = xs.len().min(ys.len());
    let mut xs = &xs[..l];
    let mut ys = &ys[..l];
    let mut off = 0;

    unsafe {
        use std::arch::x86_64::*;

        let zero = _mm256_setzero_si256();
        while xs.len() >= 32 {
            let x = _mm256_loadu_si256(xs.as_ptr() as _);
            let y = _mm256_loadu_si256(ys.as_ptr() as _);

            let r = _mm256_xor_si256(x, y);
            let r = _mm256_cmpeq_epi8(r, zero);
            let r = _mm256_movemask_epi8(r);
            if r.trailing_ones() < 32 {
                return off + r.trailing_ones() as usize;
            }

            xs = &xs[32..];
            ys = &ys[32..];
            off += 32;
        }
    }
    off + mismatch(xs, ys)
}

#[inline(never)]
pub fn mismatch_chunked(xs: &[u8], ys: &[u8]) -> usize {
    // https://users.rust-lang.org/t/how-to-find-common-prefix-of-two-byte-slices-effectively/25815/5
    fn inner<const N: usize>(xs: &[u8], ys: &[u8]) -> usize {
        let off = iter::zip(xs.chunks_exact(N), ys.chunks_exact(N))
            .take_while(|(x, y)| x == y)
            .count()
            * N;
        off + iter::zip(&xs[off..], &ys[off..])
            .take_while(|(x, y)| x == y)
            .count()
    }

    inner::<128>(xs, ys)
}

#[test]
fn bench() {
    fn bench_mismatch(name: &str, f: fn(&[u8], &[u8]) -> usize) {
        let n = 500_000;
        let m = 500;
        let mut xs = "Hello, world".repeat(n).into_bytes();
        let mut ys = xs.clone();
        xs.push(b'x');
        ys.extend(b"ijk");

        let t = std::time::Instant::now();
        let mut res = 0;
        for _ in 0..m {
            res += f(&xs, &ys);
        }
        eprintln!("{name:10} {:0.2?}", t.elapsed());
        assert_eq!(res, 3000000000);
    }

    bench_mismatch("naive", mismatch);
    bench_mismatch("simd ", mismatch_simd);
    #[cfg(target_arch = "x86_64")]
    bench_mismatch("chunk ", mismatch_chunked);
}
