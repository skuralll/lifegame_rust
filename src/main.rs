//自作モジュール
mod game;

//外部ライブラリ
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};

// アプリケーション構造体
struct App {
    Board: game::Board,
}
impl App {
    fn new() -> App {
        App {
            Board: game::Board::new(10, 10),
        }
    }

    fn on_tick(&mut self) {
        // todo
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // ターミナルのセットアップ
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App構造体を作成してアプリケーションを実行
    let tick_rate = Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // 終了時ターミナルを元に戻す
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

//アプリケーションの起動
fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App, tick_rate: Duration) -> io::Result<()> {
    let mut last_tick = Instant::now();
    //メインループ
    loop {
        // 描画処理
        terminal.draw(|f| ui(f, &app))?;
        // ティックレートを取得
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        // イベント処理
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        // 一定時間ごとにon_tick()を呼び出す
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

// UIの描画処理
fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    let block = Block::default().style(Style::default());
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(100)
            ]
            .as_ref(),
        )
        .split(size);

    // 一文字ずつ装飾するサンプル
    let spans = Spans::from(vec![
        Span::styled("My", Style::default().fg(Color::Yellow)),
        Span::raw("text"),
    ]);

    let text = vec![
        spans,
        Spans::from("test line"),
    ];
    
    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };
    let paragraph = Paragraph::new(text.clone())
        .style(Style::default())
        .block(create_block("life game"))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, chunks[0]);
}