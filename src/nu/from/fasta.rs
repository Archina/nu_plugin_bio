use nu_plugin::SimplePluginCommand;
use nu_protocol::{Signature, Span, Type, Value};

use crate::bio_format::{fasta::from_fasta_inner, Compression};

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

fn file_extension_from(displayable: &dyn std::fmt::Display, c: &Compression) -> String {
    format!(".{}", file_name_from(displayable, c))
}

fn file_name_from(displayable: &dyn std::fmt::Display, c: &Compression) -> String {
    match c {
        Compression::Uncompressed => format!("{displayable}",),
        Compression::Gzipped => format!("{displayable}.gz",),
    }
}

pub struct Command {
    name: String,
    description: String,
    switch_description: String,
    compression: Compression,
}

impl Command {
    fn new(f: File, c: Compression) -> Self {
        Self {
            name: format!("from {}", file_name_from(&f, &c)),
            description: if c == Compression::Gzipped {
                format!(
                    "Parse a gzipped {} file and create a table of ID's and sequences.",
                    file_extension_from(&f, &c)
                )
            } else {
                format!(
                    "Parse text as {} file and create a table of ID's and sequences.",
                    file_extension_from(&f, &c)
                )
            },
            switch_description: format!("parse the {f} header description"),
            compression: c,
        }
    }

    pub fn fasta() -> Self {
        Self::new(File::Fasta, Compression::Uncompressed)
    }

    pub fn fa() -> Self {
        Self::new(File::Fa, Compression::Uncompressed)
    }

    pub fn fasta_gz() -> Self {
        Self::new(File::Fasta, Compression::Gzipped)
    }

    pub fn fa_gz() -> Self {
        Self::new(File::Fa, Compression::Gzipped)
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
