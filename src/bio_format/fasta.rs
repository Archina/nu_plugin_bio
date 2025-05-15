use std::io::{BufRead, BufReader};

use noodles::fasta::{
    record::{Definition as FastaDefinition, Record as FastaRecord, Sequence},
    Writer as FastaWriter,
};
use noodles::fastq::{
    record::{Definition as FastqDefinition, Record as FastqRecord},
    Writer as FastqWriter,
};
use noodles::{bgzf, fasta, fastq};
use nu_plugin::EvaluatedCall;
use nu_protocol::{LabeledError, Value};

use crate::bio_format::{Compression, SpanExt};

/// Compression status of a fastq reader.
enum FastqReader<'a> {
    Uncompressed(Box<fastq::Reader<&'a [u8]>>),
    Compressed(Box<fastq::Reader<BufReader<bgzf::Reader<&'a [u8]>>>>),
}

/// Compression status of a fasta reader.
enum FastaReader<'a> {
    Uncompressed(Box<fasta::Reader<&'a [u8]>>),
    Compressed(fasta::Reader<Box<bgzf::Reader<&'a [u8]>>>),
}

/// Iterate over the records of a reader that implements [`BufRead`].
fn iterate_fastq_records<R: BufRead>(
    mut reader: fastq::Reader<R>,
    call: &EvaluatedCall,
    value_records: &mut Vec<Value>,
    description: bool,
    quality_scores: bool,
    cols: Vec<String>,
) -> Result<(), LabeledError> {
    // iterate over the records.
    for record in reader.records() {
        let r = record.map_err(|e| {
            LabeledError::new(format!("cause of failure: {}", e))
                .with_label("Record reading failed.", call.head)
        })?;

        let mut vec_vals = Vec::new();
        vec_vals.push(call.head.with_string_from_utf8(r.name()));

        if description {
            vec_vals.push(call.head.with_string_from_utf8(r.description()));
        }

        if quality_scores {
            vec_vals.push(call.head.with_string_from_utf8(r.quality_scores()));
        }

        vec_vals.push(call.head.with_string_from_utf8(r.sequence()));

        let mut tmp_record = nu_protocol::Record::new();
        for (col, val) in cols.clone().iter().zip(vec_vals) {
            tmp_record.push(col, val);
        }
        value_records.push(Value::record(tmp_record, call.head))
    }

    Ok(())
}

pub fn from_fastq_inner(
    call: &EvaluatedCall,
    input: &Value,
    gz: &Compression,
) -> Result<Vec<Value>, LabeledError> {
    // parse description flag.
    let description = call.has_flag("description");
    let quality_scores = call.has_flag("quality-scores");

    let bytes = input.as_binary().map_err(|e| {
        LabeledError::new(format!("cause of failure: {}", e))
            .with_label("Value conversion to binary failed.", call.head)
    })?;

    let reader = match gz {
        Compression::Uncompressed => FastqReader::Uncompressed(Box::new(fastq::Reader::new(bytes))),
        Compression::Gzipped => {
            let gz = bgzf::Reader::new(bytes);
            FastqReader::Compressed(Box::new(fastq::Reader::new(BufReader::new(gz))))
        }
    };

    let cols = match (description.is_ok(), quality_scores.is_ok()) {
        (false, false) => vec!["id".to_string(), "sequence".to_string()],
        (true, false) => vec![
            "id".to_string(),
            "description".to_string(),
            "sequence".to_string(),
        ],
        (false, true) => vec![
            "id".to_string(),
            "quality_scores".to_string(),
            "sequence".to_string(),
        ],
        (true, true) => vec![
            "id".to_string(),
            "description".to_string(),
            "quality_scores".to_string(),
            "sequence".to_string(),
        ],
    };

    let mut value_records = Vec::new();

    match reader {
        FastqReader::Uncompressed(u) => iterate_fastq_records(
            *u,
            call,
            &mut value_records,
            description.is_ok(),
            quality_scores.is_ok(),
            cols,
        )?,
        FastqReader::Compressed(c) => iterate_fastq_records(
            *c,
            call,
            &mut value_records,
            description.is_ok(),
            quality_scores.is_ok(),
            cols,
        )?,
    };

    Ok(value_records)
}

fn iterate_fasta_records<R: BufRead>(
    mut reader: fasta::Reader<R>,
    call: &EvaluatedCall,
    value_records: &mut Vec<Value>,
    description: bool,
    cols: Vec<String>,
) -> Result<(), LabeledError> {
    // iterate over the records
    for record in reader.records() {
        let r = record.map_err(|e| {
            LabeledError::new(format!("cause of failure: {}", e))
                .with_label("Record reading failed.", call.head)
        })?;

        let mut vec_vals = Vec::new();

        vec_vals.push(call.head.with_string(r.name()));

        if description {
            vec_vals.push(call.head.with_string_or(r.description(), ""));
        }

        vec_vals.push(call.head.with_string_from_utf8(r.sequence().as_ref()));

        let mut tmp_record = nu_protocol::Record::new();
        for (col, val) in cols.clone().iter().zip(vec_vals) {
            tmp_record.push(col, val);
        }
        value_records.push(Value::record(tmp_record, call.head))
    }
    Ok(())
}

/// Parse a fasta file into a nushell structure.
pub fn from_fasta_inner(
    call: &EvaluatedCall,
    input: &Value,
    gz: &Compression,
) -> Result<Vec<Value>, LabeledError> {
    // parse description flag.
    let description = call.has_flag("description");

    let bytes = input.as_binary()?;

    let reader = match gz {
        Compression::Uncompressed => FastaReader::Uncompressed(Box::new(fasta::Reader::new(bytes))),
        Compression::Gzipped => {
            let gz = Box::new(bgzf::Reader::new(bytes));
            FastaReader::Compressed(fasta::Reader::new(gz))
        }
    };

    let cols = if description.is_ok() {
        vec![
            "id".to_string(),
            "description".to_string(),
            "sequence".to_string(),
        ]
    } else {
        vec!["id".to_string(), "sequence".to_string()]
    };

    let mut value_records = Vec::new();

    match reader {
        FastaReader::Uncompressed(u) => {
            iterate_fasta_records(*u, call, &mut value_records, description.is_ok(), cols)?
        }
        FastaReader::Compressed(c) => {
            iterate_fasta_records(c, call, &mut value_records, description.is_ok(), cols)?
        }
    };

    Ok(value_records)
}

/// Go from a parsed nuon fasta structure to a string to stdout
///
/// Note that this assumes that we are parsing fasta format specifically.
pub fn nuon_to_fasta(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let mut out = FastaWriter::new(Vec::new());

    if let Ok(list) = input.as_list() {
        for el in list {
            let inner = el.as_record()?;
            let mut vals = inner.values();

            let id = vals
                .next()
                .and_then(|v| v.as_str().ok())
                .unwrap_or_default()
                .to_string();

            // let id = vals.nth(0).map(|e| e.as_str().unwrap());
            let description = vals
                .next()
                .and_then(|e| e.as_str().ok())
                .map(|v| v.to_string());

            let sequence = vals.last().unwrap().as_str()?;

            let fa_def = FastaDefinition::new(id, description);
            let fa_seq = Sequence::from(sequence.as_bytes().iter().cloned().collect::<Vec<_>>());

            out.write_record(&FastaRecord::new(fa_def.clone(), fa_seq))
                .map_err(|err| {
                    LabeledError::new(err.to_string()).with_label(
                        format!("Error in writing record ({}) to fasta", fa_def),
                        call.head,
                    )
                })?;
        }
    }

    let bytes = out.get_ref();
    let out_final = String::from_utf8(bytes.clone()).map_err(|err| {
        LabeledError::new(err.to_string()).with_label("Can't format bytes as UTF-8", call.head)
    })?;

    Ok(Value::string(out_final, call.head))
}

pub fn nuon_to_fastq(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let mut out = FastqWriter::new(Vec::new());

    if let Ok(list) = input.as_list() {
        // we need to check the columns
        let first = list.first();
        let (description, quality) = match first {
            Some(e) => {
                let first_inner = e.as_record()?;
                let mut cols = first_inner.columns();
                (
                    cols.position(|c| *c == String::from("description"))
                        .is_some(),
                    cols.position(|c| *c == String::from("quality_scores")),
                )
            }
            None => {
                // what's the error?
                return Err(
                    LabeledError::new("There was no first value to call `to fastq` on")
                        .with_label("No value", call.head),
                );
            }
        };

        // if we don't have quality scores no point going further.
        if quality.is_none() {
            return Err(LabeledError::new("Consider using `to fasta` if you don't have any quality scores, or pass the -q option on a fastq").with_label("No quality scores", call.head));
        }

        for el in list {
            let inner = el.as_record()?;
            // we need to check the columns.
            let mut vals = inner.values().cloned();

            let id = vals
                .next()
                .and_then(|e| e.as_str().ok().map(|str| str.to_string()));

            let d = if description {
                vals.next()
                    .and_then(|e| e.as_str().ok().map(|str| str.to_string()))
            } else {
                None
            };
            let q = vals
                .next()
                .and_then(|e| e.as_str().ok().map(|str| str.to_string()));

            // let (d, q) = match (description, quality.is_some()) {
            //     (true, true) => {
            //         // we got both
            //         let d = vals
            //             .nth(1)
            //             .and_then(|e| e.as_str().ok().map(|str| str.to_string()));
            //         let q = vals
            //             .nth(2)
            //             .and_then(|e| e.as_str().ok().map(|str| str.to_string()));
            //         (d, q)
            //     }
            //     (false, true) => {
            //         let q = vals
            //             .nth(1)
            //             .and_then(|e| e.as_str().ok().map(|str| str.to_string()));
            //         (None, q)
            //     }
            //     _ => unreachable!(),
            // };

            let sequence = vals
                .last()
                .and_then(|v| v.as_str().ok().map(|str| str.to_string()))
                .unwrap_or_default();

            let fq_def = FastqDefinition::new(id.unwrap_or("".into()), d.unwrap_or("".into()));

            out.write_record(&FastqRecord::new(
                fq_def.clone(),
                sequence.as_bytes(),
                q.unwrap_or("".into()).as_bytes(),
            ))
            .map_err(|err| {
                LabeledError::new(err.to_string()).with_label(
                    format!("Error in writing record ({:?}) to fastq", fq_def),
                    call.head,
                )
            })?;
        }
    }

    let bytes = out.get_ref();
    let out_final = String::from_utf8(bytes.clone()).map_err(|err| {
        LabeledError::new(err.to_string()).with_label("Can't format bytes as UTF-8", call.head)
    })?;

    Ok(Value::string(out_final, call.head))
}
