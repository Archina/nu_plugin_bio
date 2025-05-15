//             PluginSignature::build("from fastq.gz")
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
// Run 
//             "from fastq.gz" => self.from_fastq(call, input, Compression::Gzipped),