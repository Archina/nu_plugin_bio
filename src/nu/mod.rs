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
            Box::new(from::fasta::Command::fasta_gz()),
            Box::new(from::fasta::Command::fa_gz()),
        ]
    }
}
