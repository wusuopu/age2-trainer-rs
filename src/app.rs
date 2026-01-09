use std::time::Duration;
use color_eyre::Result;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Padding, Paragraph, Widget, Wrap};
use ratatui::{DefaultTerminal, Frame};
use crossterm::event::{Event, EventStream, KeyCode};
use tokio;
use tokio_stream::StreamExt;

pub struct App {
    should_quit: bool,
    winapi: crate::winapi::WinApi,
    manager: crate::winapi::process::ProcessManager,
}


impl App {
    const FRAMES_PER_SECOND: f32 = 60.0;
    pub fn new() -> Self {
        Self {
            should_quit: false,
            winapi: crate::winapi::WinApi::new(),
            manager: crate::winapi::process::ProcessManager::default(),
        }
    }

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

    fn render(&mut self, frame: &mut Frame) {
        let main_layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]);
        let [title_area, body_area] = frame.area().layout(&main_layout);
        let title = Line::from("帝国时代2 修改器 v2026.01.09").centered().bold();
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

    fn render_game_info(&mut self, area: Rect, frame: &mut Frame) {
        if self.winapi.game_process.is_none() {
          let game_info = self.manager.find_game_process(&self.winapi.psapi_lib, &self.winapi.kernel32_lib);
          if game_info.is_none() {
              frame.render_widget(Span::styled("游戏程序未运行！", Style::new().fg(Color::Red)), area);
              return;
          }
          let mut game_process = game_info.unwrap();
          if let Err(_) = self.winapi.set_game_process(game_process) {
              frame.render_widget(Span::styled("游戏程序未运行！", Style::new().fg(Color::Red)), area);
              return;
          }
        }

        // println!("name: {}, pid: {}", self.winapi.game_process.as_ref().unwrap().name, self.winapi.game_process.as_ref().unwrap().pid);

        let ret = self.winapi.read_game_info();
        if let Err(err) = ret {
            frame.render_widget(Span::styled(err.to_string(), Style::new().fg(Color::Red)), area);
            return;
        }
        let info = ret.unwrap();
        if !info.is_running {
            frame.render_widget(Span::styled("还未开始游戏！", Style::new().fg(Color::Yellow)), area);
            return;
        }

        if let Err(_) = self.winapi.write_game_info() {
          // 忽略错误
        }

        let rows = vec![
            Line::raw(format!("游戏进程: {}", info.pid)),
            Line::raw(format!("木材: {}", info.wood)),
            Line::raw(format!("食物 {}", info.food)),
            Line::raw(format!("黄金 {}", info.gold)),
            Line::raw(format!("石料 {}", info.stone)),
            Line::raw(format!("人口上限 无限制")),
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