/// The CRAM format
use noodles::cram;
use noodles::sam;
use nu_plugin::EvaluatedCall;
use nu_protocol::{record, LabeledError, Record, Value};

use crate::bio_format::bam::{create_record_values, parse_header, BAM_COLUMNS};
// TODO: also allow the reference to be passed, so we can view the alignment sequences?

/// Parse a CRAM file into a nushell structure.
pub fn from_cram_inner(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    // match on file type
    let stream = match input {
        Value::Binary { val, .. } => val,
        other => {
            return Err(LabeledError::new(format!(
                "requires binary input, got {}",
                other.get_type()
            ))
            .with_label("Input should be binary.", call.head))
        }
    };

    let mut reader = cram::Reader::new(stream.as_slice());

    match reader.read_file_definition() {
        Ok(_) => (),
        Err(e) => {
            return Err(LabeledError::new(format!("cause of failure: {}", e))
                .with_label("Could not read CRAM file definition.", call.head))
        }
    };

    let header: sam::Header = match reader.read_file_header() {
        Ok(s) => s,
        Err(e) => {
            return Err(LabeledError::new(format!("cause of failure: {}", e))
                .with_label("CRAM file header reading failed.", call.head))
        }
    };

    let header_nuon = parse_header(call, &header);

    let mut value_records = Vec::new();

    while let Some(container) = reader.read_data_container().unwrap() {
        for slice in container.slices() {
            // FIXME: if these unwraps in this section are converted to match LabeledErrors as
            // above, some of my test crams hang indefinitely in parsing(?)
            let records = slice.records(container.compression_header()).unwrap();

            for r in records {
                let r = r.try_into_alignment_record(&header).unwrap();
                let vec_vals = create_record_values(call, r);

                let records_inner =
                    Record::from_iter(BAM_COLUMNS.iter().map(|e| e.to_string()).zip(vec_vals));

                value_records.push(Value::record(records_inner, call.head))
            }
        }
    }

    Ok(Value::record(
        record! {
            "header" => header_nuon,
            "body" => Value::list(value_records, call.head)
        },
        call.head,
    ))
}
