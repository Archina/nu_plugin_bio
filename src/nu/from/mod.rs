use crate::bio_format::Compression;

pub mod bam;
pub mod bcf;
pub mod bed;
pub mod cram;
pub mod fasta;
pub mod fastq;
pub mod gfa;
pub mod gff;

fn file_extension_from(displayable: &dyn std::fmt::Display, c: &Compression) -> String {
    format!(".{}", file_name_from(displayable, c))
}

fn file_name_from(displayable: &dyn std::fmt::Display, c: &Compression) -> String {
    match c {
        Compression::Uncompressed => format!("{displayable}",),
        Compression::Gzipped => format!("{displayable}.gz",),
    }
}
