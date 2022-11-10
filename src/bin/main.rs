// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use gitremind::{find_config_dir, Config, Error};
use std::{fs::File, thread, time::Duration};

fn main() -> Result<(), Error> {
    if let Some(dir) = find_config_dir()? {
        let file = File::open(dir)?;
        if let Some(config) = Config::parse(file)? {
            prompt(&config)?;
        }
    }

    Ok(())
}

fn prompt(config: &Config) -> Result<(), Error> {
    if let Some(mut stderr) = term::stderr() {
        let color = stderr.supports_color();
        let dur = Duration::from_secs(1);
        for d in (0..config.timeout()).rev() {
            if color {
                stderr.fg(term::color::BRIGHT_YELLOW)?;
            }
            writeln!(stderr, "{}", config.message())?;

            if color {
                stderr.fg(term::color::BRIGHT_BLACK)?;
            }
            write!(
                stderr,
                "Completing action in {} seconds. Press Ctrl+C to cancel.",
                d + 1,
            )?;

            if color {
                stderr.reset()?;
            }

            thread::sleep(dur);

            // Erase the prompt.
            stderr.carriage_return()?;
            stderr.delete_line()?;

            // Erase the message.
            stderr.cursor_up()?;
            stderr.carriage_return()?;
        }
    }

    Ok(())
}
