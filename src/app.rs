use std::time::Duration;
use color_eyre::Result;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Style, Color, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, HighlightSpacing, Padding, Paragraph, Row, StatefulWidget, Table, TableState, Widget, Wrap};
use ratatui::{DefaultTerminal, Frame};
use crossterm::event::{Event, EventStream, KeyCode};
use tokio;
use tokio_stream::StreamExt;

#[derive(Debug, Default)]
pub struct App {
    should_quit: bool,
}


impl App {
  const FRAMES_PER_SECOND: f32 = 60.0;

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
        let mut interval = tokio::time::interval(period);
        let mut events = EventStream::new();

        while !self.should_quit {
            tokio::select! {
                _ = interval.tick() => { terminal.draw(|frame| self.render(frame))?; },
                Some(Ok(event)) = events.next() => self.handle_event(&event),
            }
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let main_layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [title_area, body_area] = frame.area().layout(&main_layout);
        let title = Line::from("帝国时代2 修改器").centered().bold();
        frame.render_widget(title, title_area);

        let body_layout = Layout::vertical([Constraint::Max(8), Constraint::Fill(1)]);
        let [help_area, game_area] = body_layout.areas(body_area);

        // 使用说明
        let help_block = Block::bordered().padding(Padding::new(2, 2, 1, 1)).title("使用说明");
        let info_area = help_block.inner(help_area);
        frame.render_widget(help_block, help_area);
        Paragraph::new(vec![
          Line::raw("按 Q 或 Esc 退出修改器。"),
          Line::raw("程序会自动修改游戏的四项资源，以及人口上限。"),
        ])
          .wrap(Wrap{ trim: true })
          .render(info_area, frame.buffer_mut());

        // 游戏信息
        let game_block = Block::bordered().padding(Padding::new(2, 2, 1, 1)).title("游戏信息");

        let main_area = game_block.inner(game_area);
        frame.render_widget(game_block, game_area);

        self.render_game_info(main_area, frame);
    }

    fn render_game_info(&self, area: Rect, frame: &mut Frame) {
        // frame.render_widget(Span::styled("游戏未运行！", Style::new().fg(Color::Red)), main_area);
        let rows = vec![
            Line::raw("资源"),
            Line::raw("木材 99999"),
            Line::raw("食物 99999"),
            Line::raw("黄金 99999"),
            Line::raw("石料 99999"),
            Line::raw("人口上限 无限制"),
        ];
        Paragraph::new(rows)
          .wrap(Wrap{ trim: true })
          .render(area, frame.buffer_mut());
    }

    fn handle_event(&mut self, event: &Event) {
        if let Some(key) = event.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                KeyCode::Esc | KeyCode::Esc => self.should_quit = true,
                _ => {}
            }
        }
    }  
}