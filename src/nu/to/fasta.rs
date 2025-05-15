use crate::bio::to_fasta;
use nu_plugin::SimplePluginCommand;
use nu_protocol::{Signature, Type};

pub struct Command;

impl SimplePluginCommand for Command {
    type Plugin = crate::Bio;

    fn name(&self) -> &str {
        "to fasta"
    }

    fn description(&self) -> &str {
        "Print a parsed fasta object to a string"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(<Self as SimplePluginCommand>::name(self))
            .input_output_types(vec![(Type::String, Type::table())])
            .category(nu_protocol::Category::Formats)
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: &nu_protocol::Value,
    ) -> Result<nu_protocol::Value, nu_protocol::LabeledError> {
        to_fasta(call, input)
    }
}
