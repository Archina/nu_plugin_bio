use crate::{bio::from_gfa, bio_format::Compression};
use nu_plugin::SimplePluginCommand;
use nu_protocol::{Signature, Type};

pub struct Command {
    name: String,
    description: String,
    compression: Compression,
}

pub fn gfa() -> Command {
    new(Compression::Uncompressed)
}
pub fn gfa_gz() -> Command {
    new(Compression::Gzipped)
}

fn new(compression: Compression) -> Command {
    Command{
		name: format!("from {}", super::file_name_from(&"gfa".to_string(), &compression)),
		description: match compression{
			Compression::Uncompressed => "Parse a GFA file.\nReturns a record containing the header, segments, links, containments, and paths.".into(),
			Compression::Gzipped => "Parse a gzipped GFA file.\nReturns a record containing the header, segments, links, containments, and paths.".into(),
		},
		compression,
	}
}

impl SimplePluginCommand for Command {
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
        from_gfa(call, input, &self.compression)
    }
}
