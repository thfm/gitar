use gitar::{standard_tuning, FretboardDiagram, Key, Luthier, Mode, Note};
use structopt::StructOpt;

#[derive(StructOpt)]
enum Opt {
    /// Finds the occurences of the given note on a guitar.
    Find {
        note: Note,
        /// The number of frets on the guitar.
        #[structopt(short = "f", long = "frets", default_value = "21")]
        num_frets: usize,
        /// A tuning configuration for the guitar.
        #[structopt(short = "t", long = "tuning")]
        tuning: Option<Vec<Note>>,
        /// The fret number of a capo.
        #[structopt(short = "c", long = "capo")]
        capo: Option<usize>,
    },
    /// Prints the notes in the given key.
    Notes {
        /// The root (starting) note of the key.
        root_note: Note,
        #[structopt(short = "m", long = "mode")]
        mode: Option<Mode>,
    },
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Find {
            note,
            num_frets,
            tuning,
            capo,
        } => {
            // Uses standard tuning if there was no given tuning (or if the given
            // tuning was invalid)
            let tuning = tuning.unwrap_or_else(standard_tuning);

            let capo = capo.unwrap_or(0);

            let luthier = Luthier::new(num_frets).string(tuning).add_capo(capo);
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
        Opt::Notes { root_note, mode } => {
            let mode = mode.unwrap_or(Mode::Ionian);
            let key = Key::new(root_note, mode);
            println!("{}", key);
        }
    }

    Ok(())
}
