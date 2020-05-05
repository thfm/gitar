use gitar::{standard_tuning, FretboardDiagram, Luthier, Note};
use structopt::StructOpt;

#[derive(StructOpt)]
enum Opt {
    /// Finds the occurences of the given note on a guitar.
    Find {
        note: Note,
        /// The number of frets on the guitar.
        #[structopt(short = "f", long = "frets", default_value = "21")]
        num_frets: usize,
        /// The tuning configuration of the guitar.
        #[structopt(short = "t", long = "tuning")]
        tuning: Option<Vec<Note>>,
    },
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Find {
            note,
            num_frets,
            tuning,
        } => {
            // Uses standard tuning if there was no given tuning (or if the given
            // tuning was invalid)
            let tuning = tuning.unwrap_or_else(standard_tuning);

            let luthier = Luthier::new(num_frets).string(tuning);
            let guitar = luthier.build();

            let locations = guitar.locations(note);
            match locations.len() {
                0 => {
                    println!("No occurences.");
                    return Ok(());
                }
                1 => println!("1 occurence:"),
                n => println!("{} occurences:", n),
            }

            println!("{}", FretboardDiagram::new(&guitar, locations));
        }
    }

    Ok(())
}
