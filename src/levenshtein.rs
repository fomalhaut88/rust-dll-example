use std::{cmp, slice};


#[unsafe(export_name="levenshtein_distance")]
pub extern "C" fn distance(n1: usize, p1: *const u8, 
                       n2: usize, p2: *const u8) -> usize {
    let s1 = unsafe { slice::from_raw_parts(p1, n1) };
    let s2 = unsafe { slice::from_raw_parts(p2, n2) };
    let lcs = lcs_solve(s1, s2);
    cmp::max(n1, n2) - lcs
}


pub fn lcs_solve<T: PartialEq>(s1: &[T], s2: &[T]) -> usize {
    let n1 = s1.len();
    let n2 = s2.len();

    let mut row = vec![0usize; n1];

    for i2 in 0..n2 {
        let mut prev = 0;

        for i1 in 0..n1 {
            let prev_new = row[i1];

            if s1[i1] == s2[i2] {
                row[i1] = prev + 1;
            } else if i1 > 0 {
                if row[i1] < row[i1 - 1] {
                    row[i1] = row[i1 - 1];
                }
            }

            prev = prev_new;
        }
    }

    row[n1 - 1]
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_lcs_solve() {
        let s1 = "lewenstein";
        let s2 = "levenshtein";

        assert_eq!(
            lcs_solve(s1.as_bytes(), s2.as_bytes()), 
            9
        );
    }

    #[test]
    fn test_distance() {
        let s1 = "lewenstein";
        let s2 = "levenshtein";
        assert_eq!(
            distance(s1.len(), s1.as_ptr(), 
                     s2.len(), s2.as_ptr()), 
            2
        );
    }

    #[bench]
    fn bench_lcs_solve(bencher: &mut Bencher) {
        let s1 = "lewenstein";
        let s2 = "levenshtein";

        bencher.iter(|| {
            lcs_solve(s1.as_bytes(), s2.as_bytes());
        });
    }

    #[bench]
    fn bench_distance(bencher: &mut Bencher) {
        let s1 = "lewenstein";
        let s2 = "levenshtein";

        bencher.iter(|| {
            distance(s1.len(), s1.as_ptr(), 
                     s2.len(), s2.as_ptr());
        });
    }
}
