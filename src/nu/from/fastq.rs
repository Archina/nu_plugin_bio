use nu_plugin::SimplePluginCommand;
use nu_protocol::{Signature, Type, Value};

use crate::{bio::from_fastq, bio_format::Compression};

enum File {
    Fastq,
    Fq,
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                File::Fastq => "fastq",
                File::Fq => "fq",
            }
        )
    }
}

pub struct Command {
    name: String,
    description: String,
    compression: Compression,
}

impl Command {
    fn new(f: File, c: Compression) -> Self {
        Self {
            name: format!("from {}", super::file_name_from(&f, &c)),
            description: if c == Compression::Gzipped {
                format!(
                    "Parse a gzipped {} file.\nReturns a table of ID's and sequences.",
                    super::file_extension_from(&f, &c)
                )
            } else {
                format!(
                    "Parse a {} file.\nReturns a table of ID's and sequences.",
                    super::file_extension_from(&f, &c)
                )
            },
            compression: c,
        }
    }

    pub fn fastq() -> Self {
        Self::new(File::Fastq, Compression::Uncompressed)
    }

    pub fn fq() -> Self {
        Self::new(File::Fq, Compression::Uncompressed)
    }

    pub fn fastq_gz() -> Self {
        Self::new(File::Fastq, Compression::Gzipped)
    }

    pub fn fq_gz() -> Self {
        Self::new(File::Fq, Compression::Gzipped)
    }
}

impl SimplePluginCommand for Command {
    type Plugin = crate::nu::Bio;

    fn name(&self) -> &str {
        &self.name
        // "from fastq"
    }

    fn description(&self) -> &str {
        &self.description
        // "Parse a fastq file.\nReturns a table of ID's and sequences."
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(<Self as SimplePluginCommand>::name(self))
            .input_output_types(vec![(Type::String, Type::table())])
            .category(nu_protocol::Category::Formats)
            .switch(
                "description",
                "parse the fastq header description",
                Some('d'),
            )
            .switch(
                "quality-scores",
                "parse the fastq quality scores",
                Some('q'),
            )
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: &Value,
    ) -> Result<Value, nu_protocol::LabeledError> {
        {
            from_fastq(call, input, &self.compression)
        }
    }
}
