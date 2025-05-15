use nu_plugin::SimplePluginCommand;
use nu_protocol::{Signature, Type, Value};

use crate::bio_format::{fasta::from_fastq_inner, Compression};

enum File {
    Fastq,
    Fq,
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                File::Fastq => "fastq",
                File::Fq => "fq",
            }
        )
    }
}

pub struct Command {
    name: String,
    description: String,
    compression: Compression,
}

impl SimplePluginCommand for Command {
    type Plugin = crate::nu::Bio;

    fn name(&self) -> &str {
        &self.name
        // "from fastq"
    }

    fn description(&self) -> &str {
        &self.description
        // "Parse a fastq file.\nReturns a table of ID's and sequences."
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build(<Self as SimplePluginCommand>::name(self))
            .input_output_types(vec![(Type::String, Type::table())])
            .category(nu_protocol::Category::Formats)
            .switch(
                "description",
                "parse the fastq header description",
                Some('d'),
            )
            .switch(
                "quality-scores",
                "parse the fastq quality scores",
                Some('q'),
            )
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        input: &Value,
    ) -> Result<Value, nu_protocol::LabeledError> {
        {
            let value_records = from_fastq_inner(call, input, &self.compression)?;
            Ok(Value::list(value_records, call.head))
        }
    }
}

//             PluginSignature::build("from fq")
//                 .usage("Parse a fastq file.\nReturns a table of ID's and sequences.")
//             "from fq" => self.from_fastq(call, input, Compression::Uncompressed),

//             PluginSignature::build("from fastq.gz")
//                 .usage("Parse a gzipped fastq file.\nReturns a table of ID's and sequences.")
// Run
//             "from fastq.gz" => self.from_fastq(call, input, Compression::Gzipped),

//             PluginSignature::build("from fq.gz")
//                 .usage("Parse a gzipped fastq file.\nReturns a table of ID's and sequences.")
//                 .category(Category::Experimental),

//             "from fq.gz" => self.from_fastq(call, input, Compression::Gzipped),
