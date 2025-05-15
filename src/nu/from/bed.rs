use nu_plugin::SimplePluginCommand;
use nu_protocol::{Signature, Type};

use crate::bio::from_bed;
pub struct Command;

impl SimplePluginCommand for Command{
	type Plugin = crate::Bio;

	fn name(&self) -> &str {
		"from bed"
	}

	fn description(&self) -> &str {
		"Parse a BED file."
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
		from_bed(call, input)
	}
}