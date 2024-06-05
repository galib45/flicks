use std::io::{stdout, Write};
use std::time::Duration;
use std::error::Error;
use crossterm::{
	QueueableCommand, 
	cursor::{MoveTo}, 
	style::{Stylize, Print, PrintStyledContent},
	terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
	event::{
		poll, read, Event, KeyCode, KeyEventKind, 
		EnableMouseCapture, DisableMouseCapture,
		MouseEventKind, MouseButton
	}
};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

fn prev(current:usize, total:usize) -> usize {
	(current + total - 1) % total
}

fn next(current:usize, total:usize) -> usize {
	(current + 1) % total
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    stdout
    	.queue(EnterAlternateScreen)?
    	.queue(EnableMouseCapture)?;
    
	let mut selected = 0;
    let items = vec![
            "Game of Thrones",
            "Breaking Bad",
            "The Sopranos",
            "Friends",
            "The Office (US)",
            "Stranger Things",
            "The Crown",
            "The Mandalorian",
            "Chernobyl",
            "The Witcher",
        ];
    let mut display_items = items.clone();
	let mut num_of_items = display_items.len();

	let matcher = SkimMatcherV2::default();
	
	let mut prompt = String::new();
	loop {
		if poll(Duration::from_millis(500))? {
			match read()? {
				Event::Key(event) => {
					if event.kind == KeyEventKind::Press {
						match event.code {
							KeyCode::Char(ch) => {
								prompt.push(ch); selected = 0;
							},
							KeyCode::Backspace => {
								prompt.pop(); selected = 0;
							}
							KeyCode::Esc => {
								stdout
									.queue(LeaveAlternateScreen)?
									.queue(DisableMouseCapture)?;
								break;
							},
							KeyCode::Up | KeyCode::Left => {
								selected = prev(selected, num_of_items);
							},
							KeyCode::Down | KeyCode::Right => {
								selected = next(selected, num_of_items);
							},
							KeyCode::Enter => {
								stdout
									.queue(LeaveAlternateScreen)?
									.queue(DisableMouseCapture)?;
								println!("{}", display_items[selected]);
								break;
							},
							
							_ => {}
						}
					}
				},
				Event::Mouse(event) => {
					if event.kind == MouseEventKind::Down(MouseButton::Left)
						&& event.row < num_of_items as u16 +1 { // +1 for the row of prompt
							selected = event.row as usize - 1;
					}	
				},
				_ => {}
			}
		}
		stdout
			.queue(Clear(ClearType::All))?
			.queue(MoveTo(0, 0))?
			.queue(PrintStyledContent(format!("> {}", prompt).green().bold()))?;
		let mut index = 0;
		let mut item;
		display_items = items.clone();
		if !prompt.is_empty() {
			display_items.retain(
				|&s| matcher.fuzzy_match(
					s.to_lowercase().as_str(), prompt.as_str().to_lowercase().as_str()
				).unwrap_or_default() != 0
			);
			display_items.sort_by_key(
				|s| -matcher.fuzzy_match(s, &prompt).unwrap_or_default()
			);
		}
		num_of_items = display_items.len();
		while index < num_of_items {
			item = display_items[index];
			stdout
				.queue(MoveTo(0, index as u16 + 1))?
				.queue(PrintStyledContent(" ".on_dark_grey()))?;
			if index == selected {
				stdout
					.queue(PrintStyledContent(" ".on_dark_grey()))?
					.queue(PrintStyledContent(
						item.white().on_dark_grey()
					))?;
			} else {
	    		stdout.queue(Print(format!(" {}", item)))?;
			}
			index += 1;
		}
		stdout.queue(MoveTo(prompt.len() as u16 + 2, 0))?;
		stdout.flush()?;
    }
    Ok(())
}
