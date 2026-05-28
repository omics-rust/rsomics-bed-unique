use clap::Parser;
use rsomics_bed_unique::unique;
use rsomics_common::{CommonFlags, Result, RsomicsError, Tool, ToolMeta};
use rsomics_help::{Example, FlagSpec, HelpSpec, Section};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;

pub const META: ToolMeta = ToolMeta {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
};

pub const HELP: HelpSpec = HelpSpec {
    name: META.name,
    version: META.version,
    tagline: "Remove duplicate BED intervals (same chrom, start, end).",
    origin: None,
    usage_lines: &["[OPTIONS] [INPUT]"],
    sections: &[Section {
        title: "OPTIONS",
        flags: &[FlagSpec {
            short: Some('h'),
            long: "help",
            aliases: &[],
            value: None,
            type_hint: Some("bool"),
            required: false,
            default: None,
            description: "Show this help",
            why_default: None,
        }],
    }],
    examples: &[
        Example {
            description: "Remove duplicate intervals from file",
            command: "rsomics-bed-unique intervals.bed",
        },
        Example {
            description: "Pipe from stdin",
            command: "cat intervals.bed | rsomics-bed-unique",
        },
    ],
    json_result_schema_doc: None,
};

#[derive(Parser, Debug)]
#[command(name = "rsomics-bed-unique", disable_help_flag = true)]
pub struct Cli {
    /// Input BED file (default: stdin)
    pub input: Option<PathBuf>,

    #[command(flatten)]
    pub common: CommonFlags,
}

impl Tool for Cli {
    fn meta() -> ToolMeta {
        META
    }
    fn common(&self) -> &CommonFlags {
        &self.common
    }

    fn execute(self) -> Result<()> {
        let stdout = io::stdout();
        let mut out = stdout.lock();
        match &self.input {
            Some(p) => {
                let reader = BufReader::new(File::open(p).map_err(RsomicsError::Io)?);
                unique(reader, &mut out)?;
            }
            None => {
                let stdin = io::stdin();
                unique(stdin.lock(), &mut out)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        super::Cli::command().debug_assert();
    }
}
