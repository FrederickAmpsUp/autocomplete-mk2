use std::io::{self, Write};
use crossterm::{
    event::{self, Event, KeyCode},
    style::Print,
    terminal::{self, ClearType},
    style::{SetForegroundColor, Color}
};

pub fn run_ui(tree: &crate::tree::Tree) -> anyhow::Result<()> {
    use crossterm::{queue, cursor::MoveToColumn};

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;

    let mut buffer = String::new();

    loop {
        queue!(
            stdout,
            MoveToColumn(0),
            terminal::Clear(ClearType::FromCursorDown),
        )?;

        print!("> {}", buffer);

        let last_word = if buffer.chars().last().unwrap_or(' ').is_whitespace() {
            ""
        } else {
            buffer.split_whitespace().last().unwrap_or("")
        };

        let mut suggestion = String::new();

        let mut suffixes = vec![];
        for ctx_len in last_word.len().max(4)..8 {
            for s in tree.query_suffixes(&buffer[buffer.len()-ctx_len.min(buffer.len())..]) {
                suffixes.push(s);
            }
        }
        suffixes.sort_by_key(|&(_, freq)| std::cmp::Reverse(freq));
        suffixes.retain(|(s, _)| {
            let trimmed = s.trim();
            !trimmed.is_empty()
        });

        // top suggestion inline
        if let Some((top, _freq)) = suffixes.get(0) {
            let sugg = if top.len() == 0 || top.chars().nth(0).unwrap().is_whitespace() {
                ""
            } else {
                top.split_whitespace().collect::<Vec<&str>>().get(0).map_or("", |v| v)
            };
            queue!(
                stdout,
                SetForegroundColor(Color::Grey),
                Print(sugg),
                crossterm::cursor::MoveLeft(sugg.len() as u16),
                SetForegroundColor(Color::White)
            )?;
            suggestion = sugg.to_string();
        }

        for (i, (suf, _freq)) in suffixes.iter().skip(1).take(4).enumerate() {
            queue!(
                stdout,
                crossterm::cursor::MoveDown((i + 1) as u16),
                terminal::Clear(ClearType::CurrentLine),
                Print(suf),
                crossterm::cursor::MoveLeft(suf.len() as u16),
                crossterm::cursor::MoveUp((i + 1) as u16)
            )?;
        }

        stdout.flush()?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(c) => buffer.push(c),
                    KeyCode::Backspace => { buffer.pop(); }
                    KeyCode::Enter => { buffer.clear(); }
                    KeyCode::Tab => { buffer.push_str(&suggestion); buffer.push(' '); }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    println!();
    Ok(())
}

