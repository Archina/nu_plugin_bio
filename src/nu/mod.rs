use nu_plugin::Plugin;

pub mod from;

pub struct Bio;

impl Plugin for Bio {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(from::fasta::Command::fasta()),
            Box::new(from::fasta::Command::fa()),
        ]
    }
}

// impl Plugin for Bio {

//             PluginSignature::build("from fa.gz")
//                 .usage("Parse a gzipped fasta file.\nReturns a table of ID's and sequences.")
//                 .switch(
//                     "description",
//                     "parse the fasta header description",
//                     Some('d'),
//                 )
//                 .category(Category::Experimental),
//             PluginSignature::build("to fasta")
//                 .usage("Print a parsed fasta object to a string"),
//             PluginSignature::build("from fq")
//                 .usage("Parse a fastq file.\nReturns a table of ID's and sequences.")
//                 .switch(
//                     "description",
//                     "parse the fastq header description",
//                     Some('d'),
//                 )
//                 .switch(
//                     "quality-scores",
//                     "parse the fastq quality scores",
//                     Some('q'),
//                 )
//                 .category(Category::Experimental),
//             PluginSignature::build("from fq.gz")
//                 .usage("Parse a gzipped fastq file.\nReturns a table of ID's and sequences.")
//                 .switch(
//                     "description",
//                     "parse the fastq header description",
//                     Some('d'),
//                 )
//                 .switch(
//                     "quality-scores",
//                     "parse the fastq quality scores",
//                     Some('q'),
//                 )
//                 .category(Category::Experimental),
//             PluginSignature::build("to fastq")
//                 .usage("Print out a fastq from structured nuon"),
//             PluginSignature::build("from bam")
//                 .usage("Parse a BAM file.\nReturns a record containing the header and the body of the BAM file.")
//                 .category(Category::Experimental),
//             PluginSignature::build("from sam")
//                 .usage("Parse a SAM file.\nReturns a record containing the header and the body of the SAM file.")
//                 .category(Category::Experimental),
//             PluginSignature::build("from cram")
//                 .usage("Parse a CRAM file into SAM output.\nReturns a record containing the header and the body of the CRAM file.")
//                 .category(Category::Experimental),
//             PluginSignature::build("from bcf")
//                 .usage("Parse a BCF file.\nReturns a record containing the header and the body of the BCF file.")
//                 .category(Category::Experimental),
//             PluginSignature::build("from bcf.gz")
//                 .usage("Parse a gzipped BCF file.\nReturns a record containing the header and the body of the BCF file.")
//                 .category(Category::Experimental),
//             PluginSignature::build("from vcf")
//                 .usage("Parse a VCF file.\nReturns a record containing the header and the body of the VCF file.")
//                 .category(Category::Experimental),
//             PluginSignature::build("from vcf.gz")
//                 .usage("Parse a gzipped VCF file.\nReturns a record containing the header and the body of the VCF file.")
//                 .category(Category::Experimental),
//             PluginSignature::build("from gff")
//                 .usage("Parse a GFF file.\nReturns a table.")
//                 .category(Category::Experimental),
//             PluginSignature::build("from gfa")
//                 .usage("Parse a GFA file.\nReturns a record containing the header, segments, links, containments, and paths.")
//                 .category(Category::Experimental),
//             PluginSignature::build("from gfa.gz")
//                 .usage("Parse a gzipped GFA file.\nReturns a record containing the header, segments, links, containments, and paths.")
//                 .category(Category::Experimental),
//             PluginSignature::build("from bed")
//                 .usage("Parse a BED file.")
//                 .category(Category::Experimental)
//         ]
//     }

//     fn run(
//         &mut self,
//         name: &str,
//         call: &EvaluatedCall,
//         input: &Value,
//     ) -> Result<Value, LabeledError> {
//         match name {
//             "from fq" => self.from_fastq(call, input, Compression::Uncompressed),
//             "from fa.gz" => self.from_fasta(call, input, Compression::Gzipped),
//             "from fq.gz" => self.from_fastq(call, input, Compression::Gzipped),
//             "to fasta" => self.to_fasta(call, input),
//             "to fastq" => self.to_fastq(call, input),
//             "from bam" => self.from_bam(call, input),
//             "from sam" => self.from_sam(call, input),
//             "from cram" => self.from_cram(call, input),
//             "from bcf" => self.from_bcf(call, input, Compression::Uncompressed),
//             "from bcf.gz" => self.from_bcf(call, input, Compression::Gzipped),
//             "from vcf" => self.from_vcf(call, input, Compression::Uncompressed),
//             "from vcf.gz" => self.from_vcf(call, input, Compression::Gzipped),
//             "from gff" => self.from_gff(call, input),
//             "from gfa" => self.from_gfa(call, input, Compression::Uncompressed),
//             "from gfa.gz" => self.from_gfa(call, input, Compression::Gzipped),
//             "from bed" => self.from_bed(call, input.clone()),
//             _ => Err(LabeledError {
//                 label: "Plugin call with wrong name signature".into(),
//                 msg: "the signature used to call the plugin does not match any name in the plugin signature vector".into(),
//                 span: Some(call.head),
//             }),
//         }
//     }
// }
