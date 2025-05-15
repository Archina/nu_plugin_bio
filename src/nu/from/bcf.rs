use crate::{
    bio::{from_bcf, from_vcf},
    bio_format::Compression,
};
use nu_plugin::SimplePluginCommand;
use nu_protocol::{Signature, Type};

pub fn bcf() -> Command {
    Command::new(
        "bcf",
        Compression::Uncompressed,
        Box::new(|call, input, c| from_bcf(call, input, c)),
    )
}

pub fn bcf_gz() -> Command {
    Command::new(
        "bcf",
        Compression::Gzipped,
        Box::new(|call, input, c| from_bcf(call, input, c)),
    )
}

pub fn vcf() -> Command {
    Command::new(
        "vcf",
        Compression::Uncompressed,
        Box::new(|call, input, c| from_vcf(call, input, c)),
    )
}

pub fn vcf_gz() -> Command {
    Command::new(
        "vcf",
        Compression::Gzipped,
        Box::new(|call, input, c| from_vcf(call, input, c)),
    )
}

pub struct Command {
    name: String,
    description: String,
    compression: Compression,
    runner: Box<
        dyn Sync
            + Fn(
                &nu_plugin::EvaluatedCall,
                &nu_protocol::Value,
                &Compression,
            ) -> Result<nu_protocol::Value, nu_protocol::LabeledError>,
    >,
}

impl Command {
    fn new(
        filename: &str,
        compression: Compression,
        runner: Box<
            dyn Sync
                + Fn(
                    &nu_plugin::EvaluatedCall,
                    &nu_protocol::Value,
                    &Compression,
                ) -> Result<nu_protocol::Value, nu_protocol::LabeledError>,
        >,
    ) -> Self {
        let uppercase = filename.to_uppercase();
        Self {
			name: format!("from {}", super::file_name_from(&filename.to_string(), &compression)),
			description: match &compression {
				Compression::Uncompressed => format!("Parse a {uppercase} file.\nReturns a record containing the header and the body of the {uppercase} file."),
				Compression::Gzipped => format!("Parse a gzipped {uppercase} file.\nReturns a record containing the header and the body of the {uppercase} file."),
			},
			runner,
			compression,
		}
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
        (self.runner)(call, input, &self.compression)
    }
}

//             PluginSignature::build("from bcf")
//                 .usage("Parse a BCF file.\nReturns a record containing the header and the body of the BCF file.")
//                 .category(Category::Experimental),
//             PluginSignature::build("from bcf.gz")
//                 .usage("Parse a gzipped BCF file.\nReturns a record containing the header and the body of the BCF file.")
//                 .category(Category::Experimental),

//             "from bcf" => self.from_bcf(call, input, Compression::Uncompressed),
//             "from bcf.gz" => self.from_bcf(call, input, Compression::Gzipped),

//             PluginSignature::build("from vcf")
//                 .usage("Parse a VCF file.\nReturns a record containing the header and the body of the VCF file.")
//                 .category(Category::Experimental),
//             PluginSignature::build("from vcf.gz")
//                 .usage("Parse a gzipped VCF file.\nReturns a record containing the header and the body of the VCF file.")
//                 .category(Category::Experimental),

//             "from vcf" => self.from_vcf(call, input, Compression::Uncompressed),
//             "from vcf.gz" => self.from_vcf(call, input, Compression::Gzipped),
