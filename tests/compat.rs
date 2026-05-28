use rsomics_bed_unique::unique;
use std::io::Cursor;

#[test]
fn deduplicates_by_first_three_fields() {
    let bed = "chr1\t0\t100\tgene1\nchr1\t0\t100\tgene2\nchr1\t100\t200\n";
    let mut out = Vec::new();
    let (total, kept) = unique(Cursor::new(bed), &mut out).unwrap();
    assert_eq!(total, 3);
    assert_eq!(kept, 2);
    let s = String::from_utf8(out).unwrap();
    let lines: Vec<&str> = s.lines().collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "chr1\t0\t100\tgene1"); // first occurrence kept
    assert_eq!(lines[1], "chr1\t100\t200");
}

#[test]
fn all_unique_input_unchanged() {
    let bed = "chr1\t0\t100\nchr1\t100\t200\nchr2\t0\t50\n";
    let mut out = Vec::new();
    let (total, kept) = unique(Cursor::new(bed), &mut out).unwrap();
    assert_eq!(total, 3);
    assert_eq!(kept, 3);
}

#[test]
fn empty_input() {
    let mut out = Vec::new();
    let (total, kept) = unique(Cursor::new(""), &mut out).unwrap();
    assert_eq!(total, 0);
    assert_eq!(kept, 0);
}

#[test]
fn comment_lines_pass_through() {
    let bed = "# header\nchr1\t0\t100\nchr1\t0\t100\n";
    let mut out = Vec::new();
    unique(Cursor::new(bed), &mut out).unwrap();
    let s = String::from_utf8(out).unwrap();
    let lines: Vec<&str> = s.lines().collect();
    assert_eq!(lines[0], "# header");
    assert_eq!(lines.len(), 2); // header + 1 unique record
}

#[test]
fn different_chroms_not_duplicates() {
    let bed = "chr1\t0\t100\nchr2\t0\t100\n";
    let mut out = Vec::new();
    let (total, kept) = unique(Cursor::new(bed), &mut out).unwrap();
    assert_eq!(total, 2);
    assert_eq!(kept, 2);
}
