mod colors;

use std::io::{BufRead, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();

    let stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stdout = stdout.lock();

    let palette = colors::discrete::dark2();
    let mut line = String::new();
    let mut colorspec = ColorSpec::new();

    loop {
        line.clear();
        if stdin.read_line(&mut line)? == 0 {
            break;
        }

        let tokens = match unquote::tokenize(&line) {
            Ok(tokens) => tokens,
            Err(_) => {
                // The line contains invalid data, probably an unescaped quotation mark
                continue;
            }
        };

        for i in 0..tokens.len() {
            let color = palette.nth(i);
            let rgb = color.into_components();

            colorspec.set_fg(Some(Color::Rgb(rgb.0, rgb.1, rgb.2)));
            stdout.set_color(&colorspec)?;
            stdout.write_all(tokens[i].as_bytes())?;

            let _ = if i != tokens.len() - 1 {
                stdout.write(" ".as_bytes())?
            } else {
                stdout.write("\n".as_bytes())?
            };
        }
    }

    Ok(())
}
