use nu_plugin::Plugin;

pub mod from;
pub mod to;

pub struct Bio;

impl Plugin for Bio {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![
            Box::new(from::bam::command_bam()),
            Box::new(from::bam::command_sam()),
            Box::new(from::bcf::bcf()),
            Box::new(from::bcf::bcf_gz()),
            Box::new(from::bcf::vcf()),
            Box::new(from::bcf::vcf_gz()),
            Box::new(from::bed::Command),
            Box::new(from::cram::Command),
            Box::new(from::fasta::command_fasta()),
            Box::new(from::fasta::command_fa()),
            Box::new(from::fasta::command_fasta_gz()),
            Box::new(from::fasta::command_fa_gz()),
            Box::new(from::fastq::Command::fastq()),
            Box::new(from::fastq::Command::fq()),
            Box::new(from::fastq::Command::fastq_gz()),
            Box::new(from::fastq::Command::fq_gz()),
            Box::new(from::gfa::gfa()),
            Box::new(from::gfa::gfa_gz()),
            Box::new(from::gff::Command),
            Box::new(to::fasta::Command),
            Box::new(to::fastq::Command),
        ]
    }
}
