use nickel_lang_core::repl::query_print;

use crate::{
    cli::GlobalOptions,
    customize::NoCustomizeMode,
    error::{CliResult, ResultErrorExt, Warning},
    input::{InputOptions, Prepare},
};

#[derive(clap::Parser, Debug)]
pub struct QueryCommand {
    /// A field path to inspect. If omitted, we query the top level value.
    #[arg(long, short)]
    pub path: Option<String>,

    #[arg(long)]
    pub doc: bool,

    #[arg(long)]
    pub contract: bool,

    #[arg(long = "type")]
    pub typ: bool,

    #[arg(long)]
    pub default: bool,

    #[arg(long)]
    pub value: bool,

    #[command(flatten)]
    pub inputs: InputOptions<NoCustomizeMode>,
}

impl QueryCommand {
    fn attributes_specified(&self) -> bool {
        self.doc || self.contract || self.typ || self.default || self.value
    }

    fn query_attributes(&self) -> query_print::Attributes {
        // Use a default selection of attributes if no option is specified
        if !self.attributes_specified() {
            query_print::Attributes::default()
        } else {
            query_print::Attributes {
                doc: self.doc,
                contract: self.contract,
                typ: self.typ,
                default: self.default,
                value: self.value,
            }
        }
    }

    pub fn run(mut self, global: GlobalOptions) -> CliResult<()> {
        let mut program = self.inputs.prepare(&global)?;

        if self.path.is_none() {
            program.report(Warning::EmptyQueryPath)
        }

        let found = program
            .query(std::mem::take(&mut self.path))
            .map(|field| {
                query_print::write_query_result(
                    &mut std::io::stdout(),
                    &field,
                    self.query_attributes(),
                )
                .unwrap()
            })
            .report_with_program(program)?;

        if !found {
            eprintln!("No metadata found for this field.")
        }

        Ok(())
    }
}
