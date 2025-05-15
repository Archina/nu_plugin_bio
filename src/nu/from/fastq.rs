use nu_plugin::SimplePluginCommand;
use nu_protocol::{Signature, Type, Value};

use crate::bio_format::fasta::from_fastq_inner;

pub struct Command;

impl SimplePluginCommand for Command {
    type Plugin = crate::Bio;

    fn name(&self) -> &str {
        "from fastq"
    }

    fn description(&self) -> &str {
        "Parse a fastq file.\nReturns a table of ID's and sequences."
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
            let gz = crate::bio_format::Compression::Uncompressed;
            let value_records = from_fastq_inner(call, input, gz)?;
            Ok(Value::list(value_records, call.head))
        }
    }
}
