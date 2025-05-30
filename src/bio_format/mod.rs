pub use nu_protocol::{Span, Value};
/// SAM + BAM parsing facility.
pub mod bam;
/// BCF + VCF parsing facility.
pub mod bcf;
/// BED parsing facility
pub mod bed;
/// CRAM parsing facility.
pub mod cram;
/// Fasta parsing facility.
pub mod fasta;
/// GFA parsing utility
pub mod gfa;
/// GFF(3) parsing facility
pub mod gff;

/// Compression enum
#[derive(PartialEq)]
pub enum Compression {
    Uncompressed,
    Gzipped,
}

pub trait SpanExt {
    fn with_string<S: ToString>(&self, s: S) -> Value;
    fn with_string_or<S: ToString>(&self, s: Option<S>, default: &str) -> Value;
    fn with_string_from_utf8(&self, s: &[u8]) -> Value;
}

impl SpanExt for Span {
    fn with_string<S: ToString>(&self, s: S) -> Value {
        Value::string(s.to_string(), *self)
    }

    fn with_string_or<S: ToString>(&self, s: Option<S>, default: &str) -> Value {
        Value::string(s.map(|s| s.to_string()).unwrap_or(default.into()), *self)
    }
    fn with_string_from_utf8(&self, s: &[u8]) -> Value {
        // TODO: remove this unwrap
        self.with_string(std::str::from_utf8(s).unwrap())
    }
}
