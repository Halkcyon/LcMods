mod get_drives;

use get_drives::get_drives;

use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use std::error::Error;
use std::io;
use std::path::PathBuf;
use zip::read::ZipArchive;

const MODS_ZIP: &[u8] = include_bytes!("mods.zip");
const LC_PATH: &str = "steamapps/common/Lethal Company";
const PATH_A: &str = "Program Files (x86)/Steam";
const PATH_B: &str = "Program Files/Steam";
const PATH_C: &str = "SteamLibrary";

fn main() -> Result<(), Box<dyn Error>> {
    let lc_paths = get_lc_paths();
    let lc_root = lc_paths.iter().find(|&it| it.exists());

    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let default_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let header = Paragraph::new("LcMods").block(default_block.clone());
    let footer = Paragraph::new("press (q) to exit").block(default_block.clone());

    if let Some(lc_root) = lc_root {
        let cursor = io::Cursor::new(MODS_ZIP);
        let mut zip = ZipArchive::new(cursor)?;
        if let Err(err) = zip.extract(lc_root) {
            let body = Paragraph::new(format!("Failed to expand mods to {lc_root:?}: {err}"));
            loop {
                terminal.draw(|f| {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(3),
                            Constraint::Min(1),
                            Constraint::Length(3),
                        ])
                        .split(f.size());

                    f.render_widget(header.clone(), chunks[0]);

                    let area = centered_rect(50, 50, chunks[1]);
                    f.render_widget(body.clone(), area);

                    f.render_widget(footer.clone(), chunks[2]);
                })?;
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Release {
                        continue;
                    }
                    if let KeyCode::Char('q') = key.code {
                        break;
                    }
                }
            }
        } else {
            let body = Paragraph::new(format!("Installed mods to {lc_root:?}"));
            loop {
                terminal.draw(|f| {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(3),
                            Constraint::Min(1),
                            Constraint::Length(3),
                        ])
                        .split(f.size());

                    f.render_widget(header.clone(), chunks[0]);

                    let area = centered_rect(50, 50, chunks[1]);
                    f.render_widget(body.clone(), area);

                    f.render_widget(footer.clone(), chunks[2]);
                })?;
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Release {
                        continue;
                    }
                    if let KeyCode::Char('q') = key.code {
                        break;
                    }
                }
            }
        }
    } else {
        let paths_tried = lc_paths.iter().map(|p| format!("-  {}", p.to_string_lossy())).collect::<Vec<String>>().join("\n");
        let body = Paragraph::new(format!("Unable to find Lethal Company installation. Paths attempted:\n{paths_tried}"));
        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(1),
                        Constraint::Length(3),
                    ])
                    .split(f.size());

                f.render_widget(header.clone(), chunks[0]);

                let area = centered_rect(50, 50, chunks[1]);
                f.render_widget(body.clone(), area);

                f.render_widget(footer.clone(), chunks[2]);
            })?;
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn get_lc_paths() -> Vec<PathBuf> {
    get_drives()
        .expect("unable to get drive letters")
        .iter()
        .flat_map(|drive| {
            [
                PathBuf::from(format!("{drive}:/{PATH_A}/{LC_PATH}")),
                PathBuf::from(format!("{drive}:/{PATH_B}/{LC_PATH}")),
                PathBuf::from(format!("{drive}:/{PATH_C}/{LC_PATH}")),
            ]
        })
        .collect()
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
