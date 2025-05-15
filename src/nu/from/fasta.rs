use nu_plugin::SimplePluginCommand;
use nu_protocol::{Signature, Span, Type, Value};

use crate::bio_format::{fasta::from_fasta_inner, Compression};

enum File {
    Fasta,
    Fa,
    FastaGz,
    FaGz,
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                File::Fasta=>"fasta",
                File::Fa=>"fa",
                File::FastaGz => "fasta.gz",
                File::FaGz => "fa.gz",
            }
        )
    }
}

impl File{
    fn compression(&self) -> Compression{
        match self{
            File::Fasta | File::Fa => Compression::Uncompressed,
            File::FastaGz | File::FaGz => Compression::Gzipped,
        }
    }
}

pub struct Command {
    name: String,
    description: String,
    switch_description: String,
    compression: Compression,
}

impl Command {
    fn new(f: File) -> Self {
        Self {
            name: format!("from {f}"),
            description: if f.compression() == Compression::Gzipped {
                format!(
                    "Parse a gzipped .{f} file and create a table of ID's and sequences."
                )
            } else {
                format!(
                    "Parse text as .{f} file and create a table of ID's and sequences."
                )
            },
            switch_description: format!("parse the {f} header description"),
            compression: f.compression()
        }
    }

    pub fn fasta() -> Self {
        Self::new(File::Fasta)
    }

    pub fn fa() -> Self {
        Self::new(File::Fa)
    }

    pub fn fasta_gz() -> Self {
        Self::new(File::FastaGz)
    }

    pub fn fa_gz() -> Self {
        Self::new(File::FaGz)
    }
}

impl SimplePluginCommand for Command {
    type Plugin = crate::nu::Bio;

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(<Self as SimplePluginCommand>::name(self))
            .input_output_types(vec![(Type::String, Type::table())])
            .category(nu_protocol::Category::Formats)
            .switch("description", &self.switch_description, Some('d'))
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: &Value,
    ) -> Result<Value, nu_protocol::LabeledError> {
        from_fasta_inner(call, input, &self.compression)
            .map(|list| Value::list(list, Span::unknown()))
    }
}
