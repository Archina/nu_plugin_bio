//             PluginSignature::build("from fasta.gz")
//                 .usage("Parse a gzipped fasta file.\nReturns a table of ID's and sequences.")
//                 .switch(
//                     "description",
//                     "parse the fasta header description",
//                     Some('d'),
//                 )
//                 .category(Category::Experimental),

// Run
//             "from fasta.gz" => self.from_fasta(call, input, Compression::Gzipped),