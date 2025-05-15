use super::{file_extension_from, file_name_from};
use crate::{bio::from_fasta, bio_format::Compression};
use nu_plugin::SimplePluginCommand;
use nu_protocol::{Signature, Type, Value};

enum File {
    Fasta,
    Fa,
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                File::Fasta => "fasta",
                File::Fa => "fa",
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
            name: format!("from {}", file_name_from(&f, &c)),
            description: if c == Compression::Gzipped {
                format!(
                    "Parse a gzipped {} file.\nReturns a table of ID's and sequences.",
                    file_extension_from(&f, &c)
                )
            } else {
                format!(
                    "Parse text as {} file and create a table of ID's and sequences.",
                    file_extension_from(&f, &c)
                )
            },
            compression: c,
        }
    }
}

pub fn command_fasta() -> Command {
    Command::new(File::Fasta, Compression::Uncompressed)
}

pub fn command_fa() -> Command {
    Command::new(File::Fa, Compression::Uncompressed)
}

pub fn command_fasta_gz() -> Command {
    Command::new(File::Fasta, Compression::Gzipped)
}

pub fn command_fa_gz() -> Command {
    Command::new(File::Fa, Compression::Gzipped)
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
            .switch(
                "description",
                "parse the fasta header description",
                Some('d'),
            )
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: &Value,
    ) -> Result<Value, nu_protocol::LabeledError> {
        from_fasta(call, input, &self.compression)
    }
}
