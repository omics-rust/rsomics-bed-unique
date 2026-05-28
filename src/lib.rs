use rsomics_common::{Result, RsomicsError};
use std::collections::HashSet;
use std::io::{BufRead, BufWriter, Write};

/// Remove duplicate BED intervals, keeping the first occurrence.
///
/// Two records are considered duplicates when their first three fields
/// (chrom, start, end) are identical. Extra columns are preserved on the
/// kept record. Comment and blank lines are passed through unchanged.
///
/// Returns `(total_records, kept_records)`.
pub fn unique<R: BufRead, W: Write>(reader: R, output: W) -> Result<(u64, u64)> {
    let mut out = BufWriter::with_capacity(64 * 1024, output);
    let mut seen: HashSet<String> = HashSet::new();
    let mut total: u64 = 0;
    let mut kept: u64 = 0;

    for line in reader.lines() {
        let line = line.map_err(RsomicsError::Io)?;
        if line.starts_with('#') || line.is_empty() {
            writeln!(out, "{line}").map_err(RsomicsError::Io)?;
            continue;
        }
        total += 1;
        let key = line.split('\t').take(3).collect::<Vec<_>>().join("\t");
        if seen.insert(key) {
            writeln!(out, "{line}").map_err(RsomicsError::Io)?;
            kept += 1;
        }
    }

    out.flush().map_err(RsomicsError::Io)?;
    Ok((total, kept))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn removes_exact_duplicates() {
        let bed = "chr1\t0\t100\nchr1\t0\t100\nchr1\t100\t200\n";
        let mut out = Vec::new();
        let (total, kept) = unique(Cursor::new(bed), &mut out).unwrap();
        assert_eq!(total, 3);
        assert_eq!(kept, 2);
        let s = String::from_utf8(out).unwrap();
        let lines: Vec<&str> = s.lines().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "chr1\t0\t100");
        assert_eq!(lines[1], "chr1\t100\t200");
    }

    #[test]
    fn preserves_extra_columns_on_first_occurrence() {
        let bed = "chr1\t0\t100\tgene1\nchr1\t0\t100\tgene2\n";
        let mut out = Vec::new();
        unique(Cursor::new(bed), &mut out).unwrap();
        let s = String::from_utf8(out).unwrap();
        assert_eq!(s.trim(), "chr1\t0\t100\tgene1");
    }

    #[test]
    fn headers_pass_through() {
        let bed = "# comment\nchr1\t0\t100\n";
        let mut out = Vec::new();
        let (total, kept) = unique(Cursor::new(bed), &mut out).unwrap();
        assert_eq!(total, 1);
        assert_eq!(kept, 1);
        let s = String::from_utf8(out).unwrap();
        let lines: Vec<&str> = s.lines().collect();
        assert_eq!(lines[0], "# comment");
        assert_eq!(lines[1], "chr1\t0\t100");
    }

    #[test]
    fn no_duplicates_returns_all() {
        let bed = "chr1\t0\t100\nchr1\t100\t200\nchr2\t0\t50\n";
        let mut out = Vec::new();
        let (total, kept) = unique(Cursor::new(bed), &mut out).unwrap();
        assert_eq!(total, 3);
        assert_eq!(kept, 3);
    }
}
