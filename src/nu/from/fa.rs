use nu_plugin::SimplePluginCommand;
use nu_protocol::{Signature, Span, Type, Value};

use crate::bio_format::fasta::from_fasta_inner;

pub struct Command;

impl SimplePluginCommand for Command {
    type Plugin = crate::nu::Bio;

    fn name(&self) -> &str {
        "from fa"
    }

    fn description(&self) -> &str {
        "Parse text as .fa file and create a table of ID's and sequences."
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
        from_fasta_inner(call, input, crate::bio_format::Compression::Uncompressed)
            .map(|list| Value::list(list, Span::unknown()))
    }
}
