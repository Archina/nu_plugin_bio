//             PluginSignature::build("from gfa")
//                 .usage("Parse a GFA file.\nReturns a record containing the header, segments, links, containments, and paths.")
//                 .category(Category::Experimental),
//             PluginSignature::build("from gfa.gz")
//                 .usage("Parse a gzipped GFA file.\nReturns a record containing the header, segments, links, containments, and paths.")
//                 .category(Category::Experimental),

//             "from gfa" => self.from_gfa(call, input, Compression::Uncompressed),
//             "from gfa.gz" => self.from_gfa(call, input, Compression::Gzipped),