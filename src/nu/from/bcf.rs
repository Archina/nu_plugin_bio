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