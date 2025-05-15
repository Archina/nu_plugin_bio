use nu_plugin::SimplePluginCommand;
use nu_protocol::{Signature, Span, Type, Value};

use crate::bio_format::fasta::from_fasta_inner;

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
    switch_description: String,
}

impl Command {
    fn new(f: File) -> Self {
        Self {
            name: format!("from {f}"),
            description: format!(
                "Parse text as .{f} file and create a table of ID's and sequences."
            ),
            switch_description: format!("parse the {f} header description"),
        }
    }

    pub fn fasta() -> Self {
        Self::new(File::Fasta)
    }

    pub fn fa() -> Self {
        Self::new(File::Fa)
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
        from_fasta_inner(call, input, crate::bio_format::Compression::Uncompressed)
            .map(|list| Value::list(list, Span::unknown()))
    }
}
