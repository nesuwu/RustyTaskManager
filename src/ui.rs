use crate::app::{App, AppMode, AppError};
use std::io;

use crossterm::event::{self, Event, KeyCode};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear},
    Terminal,
};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    // Load tasks and handle potential errors
    handle_app_error(app.load_tasks())?;

    let mut show_confirm = false;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(
                            if matches!(app.mode, AppMode::InputMode | AppMode::EditMode) {
                                3
                            } else {
                                0
                            },
                        ),
                        Constraint::Percentage(100),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let items: Vec<ListItem> = app
                .tasks
                .iter()
                .map(|task| {
                    let (status, style) = if task.done {
                        ("✅ ", Style::default().fg(Color::Green))
                    } else {
                        ("❌ ", Style::default())
                    };
                    ListItem::new(Spans::from(vec![
                        Span::styled(status, style),
                        Span::raw(task.name.clone()),
                    ]))
                })
                .collect();

            let tasks = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Tasks"))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                );

            f.render_stateful_widget(tasks, chunks[1], &mut app.state);

            if matches!(app.mode, AppMode::InputMode | AppMode::EditMode) {
                let input = Paragraph::new(app.input.as_ref())
                    .style(Style::default().fg(Color::Yellow))
                    .block(Block::default().borders(Borders::ALL).title("Input"));
                f.render_widget(input, chunks[0]);
            }

            if show_confirm {
                let area = centered_rect(60, 20, f.size());
                let confirm_block = Block::default().title("Confirm Delete").borders(Borders::ALL);
                let paragraph = Paragraph::new("Are you sure you want to delete this task?\n(y)es / (n)o")
                    .block(confirm_block)
                    .alignment(tui::layout::Alignment::Center);
                f.render_widget(Clear, area);
                f.render_widget(paragraph, area);
            }
        })?;

        match event::read()? {
            Event::Key(key) => {
                if show_confirm {
                    match key.code {
                        KeyCode::Char('y') => {
                            app.delete_task();
                            handle_app_error(app.save_tasks())?;
                            show_confirm = false;
                        }
                        KeyCode::Char('n') => {
                            show_confirm = false;
                        }
                        _ => {}
                    }
                } else {
                    match app.mode {
                        AppMode::TaskMode => match key.code {
                            KeyCode::Tab => {
                                app.mode = AppMode::InputMode;
                            }
                            KeyCode::Down => {
                                app.next_task();
                            }
                            KeyCode::Up => {
                                app.previous_task();
                            }
                            KeyCode::Enter => {
                                app.toggle_task();
                                handle_app_error(app.save_tasks())?;
                            },
                            KeyCode::Char('m') => {
                                app.modify_index = app.state.selected();
                                app.mode = AppMode::EditMode;
                            },
                            KeyCode::Char('x') => {
                                show_confirm = true;
                            }
                            KeyCode::Char('q') => {
                                break;
                            }
                            _ => {}
                        },
                        AppMode::EditMode => match key.code {
                            KeyCode::Enter => {
                                if !app.input.is_empty() {
                                    app.modify_task(app.input.clone());
                                    app.input.clear();
                                    handle_app_error(app.save_tasks())?;
                                }
                                app.mode = AppMode::TaskMode;
                            }
                            KeyCode::Char(c) => app.input.push(c),
                            KeyCode::Backspace => { app.input.pop(); }
                            KeyCode::Esc => {
                                app.mode = AppMode::TaskMode;
                            }
                            _ => {}
                        },
                        AppMode::InputMode => match key.code {
                            KeyCode::Enter => {
                                if !app.input.is_empty() {
                                    app.add_task(app.input.clone());
                                    app.input.clear();
                                    handle_app_error(app.save_tasks())?;
                                }
                                app.mode = AppMode::TaskMode;
                            }
                            KeyCode::Char(c) => app.input.push(c),
                            KeyCode::Backspace => { app.input.pop(); }
                            KeyCode::Esc => {
                                app.mode = AppMode::TaskMode;
                            }
                            _ => {}
                        },
                    }
                }
            }
            Event::Mouse(_) => {
                // Ignore mouse events
            }
            _ => {}
        }
    }

    Ok(())
}

// Helper function to handle app errors
fn handle_app_error(result: Result<(), AppError>) -> io::Result<()> {
    if let Err(e) = result {
        return Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
    }
    Ok(())
}

// Function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: tui::layout::Rect) -> tui::layout::Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}