# rsomics-bed-unique

Remove duplicate BED intervals, keeping the first occurrence of each unique (chrom, start, end) triple.

## Usage

```sh
rsomics-bed-unique [INPUT]
rsomics-bed-unique intervals.bed
cat intervals.bed | rsomics-bed-unique
```

## Origin

Independent Rust implementation.

License: MIT OR Apache-2.0.
