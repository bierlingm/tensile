use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::app::{App, InputMode, Screen};
use crate::engine::{PatternAnalyzer, TensionCalculator};

pub fn draw_dashboard(f: &mut Frame, app: &App) {
    let size = f.area();

    // Create main layout: header, body, footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .split(size);

    // Draw header
    draw_header(f, chunks[0], app);

    // Draw body based on current screen
    match app.screen {
        Screen::VisionList => draw_vision_list(f, chunks[1], app),
        Screen::VisionDetail => draw_vision_detail(f, chunks[1], app),
        Screen::ActionForm => draw_action_form(f, chunks[1], app),
        Screen::MetricsSummary => draw_metrics_summary(f, chunks[1], app),
    }

    // Draw footer
    draw_footer(f, chunks[2], app);
}

fn draw_header(f: &mut Frame, area: Rect, app: &App) {
    let title = format!(
        "   Tensile   [{}]  Total Visions: {}  Active: {}  Actions: {}",
        match app.screen {
            Screen::VisionList => "1",
            Screen::MetricsSummary => "2",
            Screen::ActionForm => "A",
            Screen::VisionDetail => "D",
        },
        app.db.visions.len(),
        app.get_active_visions_count(),
        app.get_total_actions(),
    );

    let header = Paragraph::new(title)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::BOTTOM));

    f.render_widget(header, area);
}

fn draw_footer(f: &mut Frame, area: Rect, app: &App) {
    let help_text = match app.input_mode {
        InputMode::Normal => "[1]List [2]Metrics [a]Action [n]New [↑↓]Select [q]Quit",
        InputMode::Editing => "[Enter]Save [Esc]Cancel",
    };

    let footer = Paragraph::new(help_text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::TOP));

    f.render_widget(footer, area);
}

fn draw_vision_list(f: &mut Frame, area: Rect, app: &App) {
    let visions: Vec<ListItem> = app
        .db
        .visions
        .iter()
        .enumerate()
        .map(|(_idx, vision)| {
            let is_selected = app.selected_vision == Some(vision.id);
            let marker = if is_selected { "> " } else { "  " };

            let state_icon = match vision.state {
                crate::models::VisionState::Achieved => "✓",
                crate::models::VisionState::Blocked => "⊗",
                crate::models::VisionState::InProgress => "→",
                crate::models::VisionState::Reassessed => "◌",
                crate::models::VisionState::Conceived => "○",
            };

            let actions_count = app
                .db
                .actions
                .iter()
                .filter(|a| a.vision_id == vision.id)
                .count();

            let text = format!(
                "{}{} {} | {} | {} actions",
                marker, state_icon, vision.title, vision.state, actions_count
            );

            let style = if is_selected {
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(text).style(style)
        })
        .collect();

    let visions_block = Block::default()
        .title("Visions")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Green));

    let list = List::new(visions)
        .block(visions_block)
        .style(Style::default());

    f.render_widget(list, area);
}

fn draw_vision_detail(f: &mut Frame, area: Rect, app: &App) {
    if let Some(vision) = app.get_selected_vision() {
        let mut detail_text = vec![
            Line::from(vec![Span::styled(
                format!("Vision: {}", vision.title),
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("State: "),
                Span::styled(vision.state.to_string(), Style::default().fg(Color::Yellow)),
            ]),
            Line::from(format!("Created: {}", vision.created_at.format("%Y-%m-%d"))),
            Line::from(format!(
                "Completed: {}",
                if vision.completed { "Yes" } else { "No" }
            )),
        ];

        if let Some(desc) = &vision.description {
            detail_text.push(Line::from(""));
            detail_text.push(Line::from(vec![Span::raw("Description: ")]));
            detail_text.push(Line::from(desc.to_string()));
        }

        // Add metrics
        detail_text.push(Line::from(""));
        if let Some(tension) = TensionCalculator::calculate_vision_tension(&app.db, vision.id) {
            detail_text.push(Line::from(vec![
                Span::raw("Tension: "),
                Span::styled(
                    format!("{:.1}%", tension.tension_score),
                    Style::default().fg(Color::Red),
                ),
            ]));
            detail_text.push(Line::from(format!("Days Active: {}", tension.days_active)));
            detail_text.push(Line::from(format!(
                "Actions Logged: {}",
                tension.action_count
            )));
        }

        // Add pattern
        if let Some(metrics) = PatternAnalyzer::get_detailed_metrics(&app.db, vision.id) {
            detail_text.push(Line::from(""));
            detail_text.push(Line::from(vec![
                Span::raw("Pattern: "),
                Span::styled(
                    format!("{:?}", metrics.pattern),
                    Style::default().fg(Color::Magenta),
                ),
            ]));
            detail_text.push(Line::from(format!(
                "Success Rate: {:.1}%",
                metrics.success_rate * 100.0
            )));
            detail_text.push(Line::from(format!(
                "Velocity: {:.2} actions/day",
                metrics.velocity
            )));
        }

        let detail_block = Block::default()
            .title("Vision Details")
            .borders(Borders::ALL);

        let paragraph = Paragraph::new(detail_text).block(detail_block);
        f.render_widget(paragraph, area);
    } else {
        let msg = Paragraph::new("No vision selected").block(
            Block::default()
                .title("Vision Details")
                .borders(Borders::ALL),
        );
        f.render_widget(msg, area);
    }
}

fn draw_action_form(f: &mut Frame, area: Rect, app: &App) {
    if app.selected_vision.is_none() {
        let msg = Paragraph::new("Please select a vision first (↑↓)")
            .block(Block::default().title("Log Action").borders(Borders::ALL));
        f.render_widget(msg, area);
        return;
    }

    let input_text = Line::from(vec![
        Span::raw("Action: "),
        Span::styled(&app.input_buffer, Style::default().fg(Color::Yellow)),
    ]);

    let form = Paragraph::new(input_text)
        .block(Block::default().title("Log Action").borders(Borders::ALL));

    f.render_widget(form, area);
}

fn draw_metrics_summary(f: &mut Frame, area: Rect, app: &App) {
    let tensions = TensionCalculator::calculate_all_tensions(&app.db);
    let sorted_tensions = TensionCalculator::sort_by_tension(tensions);

    let mut metrics_text = vec![
        Line::from(vec![Span::styled(
            "Metrics Summary",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(format!("Total Visions: {}", app.db.visions.len())),
        Line::from(format!(
            "Active Visions: {}",
            app.get_active_visions_count()
        )),
        Line::from(format!("Total Actions: {}", app.get_total_actions())),
        Line::from(format!(
            "Average Tension: {:.1}%",
            if sorted_tensions.is_empty() {
                0.0
            } else {
                sorted_tensions.iter().map(|t| t.tension_score).sum::<f32>()
                    / sorted_tensions.len() as f32
            }
        )),
    ];

    metrics_text.push(Line::from(""));
    metrics_text.push(Line::from(vec![Span::styled(
        "Priority Visions",
        Style::default().add_modifier(Modifier::BOLD),
    )]));

    for (rank, tension) in sorted_tensions.iter().take(5).enumerate() {
        let icon = match rank {
            0 => "🔴",
            1 => "🟠",
            2 => "🟡",
            _ => "  ",
        };
        metrics_text.push(Line::from(format!(
            "{} {}. {} ({:.0}%)",
            icon,
            rank + 1,
            tension.vision_title,
            tension.tension_score
        )));
    }

    let metrics_block = Block::default()
        .title("Metrics")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(metrics_text).block(metrics_block);
    f.render_widget(paragraph, area);
}
