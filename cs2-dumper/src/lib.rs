use std::{path::PathBuf, str::FromStr};

use analysis::AnalysisResult;
use derive_builder::Builder;


pub mod analysis;
pub mod error;
mod mem;

mod source2;

use error::Result;
use log::info;
use memflow::{os::{Os, Process}, plugins::{ConnectorArgs, IntoProcessInstance, IntoProcessInstanceArcBox, Inventory, LibArc, OsArgs}};





#[derive(Builder)]
pub struct Scanner{
        /// The name of the memflow connector to use.
        connector: Option<String>,

        /// Additional arguments to pass to the memflow connector.
        connector_args: Option<String>,

        /// The output directory to write the generated files to.

        /// The name of the game process.
        process_name: String,

        /// Increase logging verbosity. Can be specified multiple times.
        verbose: u8,
}


impl Scanner{
    pub fn new()->Scanner{
        ScannerBuilder::default().build().unwrap()
    }
    pub fn run(self) -> Result<(AnalysisResult,IntoProcessInstanceArcBox<'static>)> {
        let conn_args = self
        .connector_args
        .map(|s| ConnectorArgs::from_str(&s).expect("unable to parse connector arguments"))
        .unwrap_or_default();

        let os = if let Some(conn) = self.connector {
            let inventory = Inventory::scan();

            inventory
                .builder()
                .connector(&conn)
                .args(conn_args)
                .os("win32")
                .build()?
        } else {
            // Fallback to the native OS layer if no connector name was specified.
            memflow_native::create_os(&OsArgs::default(), LibArc::default())?
        };

        let mut process = os.into_process_by_name(&self.process_name)?;

        let result = analysis::analyze_all(&mut process)?;

        


        return Ok((result,process));
    }
}
