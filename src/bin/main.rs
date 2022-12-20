// Copyright 2022 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use console::{Color, Style, Term};
use gitremind::{find_config_dir, Config, Error};
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

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
    let mut stderr = Term::stderr();
    if !stderr.is_term() {
        return Ok(());
    }

    let clone = stderr.clone();
    #[allow(unused_must_use)]
    ctrlc::set_handler(move || {
        clone.clear_line();
        std::process::exit(1);
    })?;

    let message_style = Style::new().for_stderr().fg(Color::Yellow).bright();
    let prompt_style = Style::new().for_stderr().fg(Color::Black).bright();

    let dur = Duration::from_secs(1);
    let mut d = config.timeout();
    while d != 0 {
        let message = message_style.apply_to(config.message()).to_string();
        let prompt = prompt_style
            .apply_to(format!(
                "Completing action in {} seconds. Press Ctrl+C to cancel.",
                d,
            ))
            .to_string();

        writeln!(stderr, "{}", message.as_str())?;
        write!(stderr, "{}", prompt.as_str())?;
        stderr.flush()?;

        thread::sleep(dur);

        d -= 1;
        if d > 0 {
            stderr.clear_last_lines(1)?;
        } else {
            stderr.clear_line()?;
        }
    }

    Ok(())
}
