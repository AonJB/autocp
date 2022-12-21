use std::env;
use std::thread;
use std::time::Duration;

use std::fs::File;

use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{self, Read, Write};

fn print_usage() {
    println!("Usage:");
    println!("  autocp [source_path] [destination_path]");
}

fn main() -> io::Result<()> {
    use std::process::{exit};

    let mut args = env::args();
    args.next().unwrap();
    let source_path = match args.next() {
        Some(path) => path,
        None => {
            eprintln!("Missing source file path");
            print_usage();
            exit(1);
        }
    };
    let destination_path = match args.next() {
        Some(path) => path,
        None => {
            eprintln!("Missing destination file path");
            print_usage();
            exit(1);
        }
    };

    // Get the source_path and the destination_path from the user
    /*
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    print!("Source path>");
    stdout.flush()?;
    stdin.read_line(&mut source_path)?;
    print!("\nDestination path>");
    stdout.flush()?;
    stdin.read_line(&mut destination_path)?;

    source_path = source_path.replace("\n", "");
    destination_path = destination_path.replace("\n", "");
    */

    // Construct Termion handles for raw standard input and output
    let stdout = io::stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    // The length of the file when it was last copied
    let mut last_len: u64 = 0;

    write!(stdout,
           "{}{}AutoCP: copying from {} to {}\n\r",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           source_path,
           destination_path)
        .unwrap();
    stdout.flush().unwrap();

    loop {

        let b = stdin.next();
        if let Some(Ok(b'q')) = b {
            write!(stdout, "{}{}Stopping AutoCP\n\r",
                   termion::clear::All,
                   termion::cursor::Goto(1, 1))
                .unwrap();
            stdout.flush().unwrap();
            break;
        }

        let mut source = File::open(&source_path)?;
        let len = source.metadata()?.len();
        // Write only if the current length is higher than the previous one (security in case the
        // original file was altered)
        if len > last_len {
            let mut destination = File::create(&destination_path)?;
            io::copy(&mut source, &mut destination)?;
            last_len = len;
        }
        
        thread::sleep(Duration::from_secs(10));
    }
    Ok(())
}
