use rand::Rng;
use ratatui::{
    layout::Alignment,
    prelude::{Constraint, Direction, Layout},
    style::{Color, Style},
    symbols,
    widgets::{
        canvas::{Canvas, Circle, MapResolution, Rectangle},
        Block, BorderType, Borders, Paragraph,
    },
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let commands = format!(
        "
        \n Welcome to Suparimoto \n
        \n Press Left Button to move left \n
        \n Press Right Button to move right \n
        \n Press Ctrl-c or q to exit \n
        "
    );

    // Splits main window into top & bottom
    let div = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.size());

    // splits bottom window into 3 horizontal rows
    let child = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(div[1]);

    let game_commands = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(child[1]);

    let p1 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(child[0]);

    let p2 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(child[2]);

    let main_title = Canvas::default()
        .block(
            Block::default()
                .title(" Suparimoto ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().bg(Color::LightBlue))
                .borders(Borders::ALL),
        )
        .marker(symbols::Marker::HalfBlock)
        .background_color(Color::LightGreen)
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|ctx| {
            ctx.draw(&Circle {
                x: app.movable_object.x,
                y: app.movable_object.y,
                radius: app.movable_object.radius,
                color: app.movable_object.color,
            });
        });

    let sub_title = Block::default()
        .title(" Game Status ")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .borders(Borders::ALL);

    let p1_left_child = Canvas::default()
        .block(
            Block::default()
                .title(" Right")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        )
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|ctx| {
            ctx.draw(&Circle {
                x: app.player_one.x,
                y: app.player_one.y,
                radius: app.player_one.radius,
                color: app.player_one.left,
            });
        });

    let p1_right_child = Canvas::default()
        .block(
            Block::default()
                .title(" Left ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        )
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|ctx| {
            ctx.draw(&Circle {
                x: app.player_one.x,
                y: app.player_one.y,
                radius: app.player_one.radius,
                color: app.player_one.right,
            });
        });

    let middle_child = Paragraph::new(commands)
        .block(
            Block::default()
                .title(" Commands ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        )
        .alignment(Alignment::Center);

    let p2_left_child = Canvas::default()
        .block(
            Block::default()
                .title(" Up ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        )
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|ctx| {
            ctx.draw(&Circle {
                x: app.player_two.x,
                y: app.player_two.y,
                radius: app.player_two.radius,
                color: app.player_two.left,
            });
        });

    let p2_right_child = Canvas::default()
        .block(
            Block::default()
                .title(" Down ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        )
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|ctx| {
            ctx.draw(&Circle {
                x: app.player_two.x,
                y: app.player_two.y,
                radius: app.player_two.radius,
                color: app.player_two.right,
            });
        });

    let middle_child_score = Paragraph::new(format!("\n Score: {}", app.score))
        .block(
            Block::default()
                .title(" Game Score ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        )
        .alignment(Alignment::Center);

    let middle_child_debug = Paragraph::new(format!(
        "\n Object: {}x{},  (+): {}x{}, (-): {}x{}",
        app.movable_object.x,
        app.movable_object.y,
        app.positive_obstacle.x,
        app.positive_obstacle.y,
        app.negative_obstacle.x,
        app.negative_obstacle.y
    ))
    .block(
        Block::default()
            .title(" Game Debug ")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL),
    )
    .alignment(Alignment::Center);

    // Two Sections
    frame.render_widget(main_title, div[0]);
    frame.render_widget(sub_title, div[1]);

    // Player 1
    frame.render_widget(p1_left_child, p2[0]);
    frame.render_widget(p1_right_child, p2[1]);

    // Middle Child
    frame.render_widget(middle_child, game_commands[0]);
    frame.render_widget(middle_child_score, game_commands[1]);
    frame.render_widget(middle_child_debug, game_commands[2]);

    // Player 2
    frame.render_widget(p2_left_child, p1[0]);
    frame.render_widget(p2_right_child, p1[1]);

    // Game Objects spawn randomly
    let random_item = Canvas::default()
        // .block(
        //     Block::default()
        //         .title(" Random Item ")
        //         .title_alignment(Alignment::Center)
        //         .border_type(BorderType::Rounded)
        //         .borders(Borders::ALL),
        // )
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|ctx| {
            ctx.draw(&Rectangle {
                x: app.positive_obstacle.x,
                y: app.positive_obstacle.y,
                width: app.positive_obstacle.width,
                height: app.positive_obstacle.height,
                color: app.positive_obstacle.color,
            });
        });

    frame.render_widget(random_item, div[0]);

    // Game Objects spawn randomly
    let random_item = Canvas::default()
        .block(
            Block::default()
                .title(" Suparimoto ")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL),
        )
        .x_bounds([-180.0, 180.0])
        .y_bounds([-90.0, 90.0])
        .paint(|ctx| {
            ctx.draw(&Rectangle {
                x: app.negative_obstacle.x,
                y: app.negative_obstacle.y,
                width: app.negative_obstacle.width,
                height: app.negative_obstacle.height,
                color: app.negative_obstacle.color,
            });
        });
    frame.render_widget(random_item, div[0]);
}
