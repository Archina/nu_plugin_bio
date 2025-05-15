use nu_plugin::SimplePluginCommand;
use nu_protocol::{Signature, Type};

use crate::bio::{from_bam, from_sam};

pub struct Command{
	name: String,
	description: String,
	run: Box<dyn Sync + Fn(&nu_plugin::EvaluatedCall, &nu_protocol::Value) -> Result<nu_protocol::Value, nu_protocol::LabeledError>>
}

impl Command {
	fn new(file: &str, run: Box<dyn Sync + Fn(&nu_plugin::EvaluatedCall, &nu_protocol::Value) -> Result<nu_protocol::Value, nu_protocol::LabeledError>>) -> Self {
		let upper = file.to_uppercase();
		Self{
			name: format!("from {}", file.to_lowercase()),
			description: format!("Parse a {upper} file.\nReturns a record containing the header and the body of the {upper} file."),
			run
		}
	}

    pub fn sam() -> Self {
        Self::new("sam", Box::new(|call, input| from_sam(call, input)))
    }
    pub fn bam() -> Self {
        Self::new("bam", Box::new(|call, input| from_bam(call, input)))
    }
}

impl SimplePluginCommand for Command{
	type Plugin = crate::Bio;

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
	}

	fn run(
		&self,
		_plugin: &Self::Plugin,
		_engine: &nu_plugin::EngineInterface,
		call: &nu_plugin::EvaluatedCall,
		input: &nu_protocol::Value,
	) -> Result<nu_protocol::Value, nu_protocol::LabeledError> {
		(self.run)(call, input)
	}
}
