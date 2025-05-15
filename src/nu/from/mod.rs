use crate::bio_format::Compression;

pub mod bam;
pub mod bed;
pub mod fasta;
pub mod fastq;

fn file_extension_from(displayable: &dyn std::fmt::Display, c: &Compression) -> String {
    format!(".{}", file_name_from(displayable, c))
}

fn file_name_from(displayable: &dyn std::fmt::Display, c: &Compression) -> String {
    match c {
        Compression::Uncompressed => format!("{displayable}",),
        Compression::Gzipped => format!("{displayable}.gz",),
    }
}
