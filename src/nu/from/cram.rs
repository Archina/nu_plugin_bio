use crate::bio::from_cram;
use nu_plugin::SimplePluginCommand;
use nu_protocol::{Signature, Type};

pub struct Command;

impl SimplePluginCommand for Command {
    type Plugin = crate::Bio;

    fn name(&self) -> &str {
        "from cram"
    }

    fn description(&self) -> &str {
        "Parse a CRAM file into SAM output.\nReturns a record containing the header and the body of the CRAM file."
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
        from_cram(call, input)
    }
}
