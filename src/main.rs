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
	widgets::{Block, Borders, Paragraph, Tabs},
	Frame, Terminal,
};

enum Mode {
	Run = 0,
	Edit = 1,
}

// アプリケーション構造体
struct App<'a> {
	board: game::Board, // ゲーム盤
	pub titles: Vec<&'a str>, // モードのタイトルリスト
	pub mode: usize, // モード
	pub selected_cell: (i32, i32) // 選択されているセル
}
impl <'a> App<'a>  {
	fn new() -> App<'a> {
		App {
			titles: vec!["Run", "Edit"],
			board: game::Board::new(30, 30),
			mode: 0,
			selected_cell: (0, 0)
		}
	}

	fn on_tick(&mut self) {
		if self.mode == Mode::Run as usize {
			self.board.step();
		}
	}

	// モードを変更する
	pub fn mode_change(&mut self){
		self.mode = (self.mode + 1) % self.titles.len();
	}

	// カーソルを動かす
	pub fn move_cursor(&mut self, x: i32, y: i32){
		// Edit モード時
		if(self.mode == Mode::Edit as usize) {
			let new_selected = (self.selected_cell.0 + x, self.selected_cell.1 + y);
			let board_size = self.board.get_size();
			if 0 <= new_selected.0 && new_selected.0 < board_size.0 && 0 <= new_selected.1 && new_selected.1 < board_size.1{
				self.selected_cell = new_selected;
			}
		}
	}

	// カーソルで選択しているセルの生死を切り替える
	pub fn toggle_selected_cell(&mut self){
		self.board.toggle_cell(self.selected_cell.0, self.selected_cell.1);
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
		// イベント処理
		let timeout = tick_rate
		.checked_sub(last_tick.elapsed())
		.unwrap_or_else(|| Duration::from_secs(0));
		if crossterm::event::poll(timeout)? {
			if let Event::Key(key) = event::read()? {
				match key.code {
					// 終了
					KeyCode::Char('q') => return Ok(()),
					// モード切替
					KeyCode::Tab => app.mode_change(),
					// カーソル移動
					KeyCode::Right => app.move_cursor(1, 0),
					KeyCode::Left => app.move_cursor(-1, 0),
					KeyCode::Up => app.move_cursor(0, -1),
					KeyCode::Down => app.move_cursor(0, 1),
					// 決定
					KeyCode::Char(' ') => {
						if app.mode == Mode::Edit as usize {app.toggle_selected_cell();} // Editモード時: セルの生死を切り替える
					},
					_ => {}
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


	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints(
			[
				Constraint::Length(3),
				Constraint::Length(app.board.get_size().1 as u16 + 2),
				Constraint::Min(0),
			]
			.as_ref(),
		)
		.split(size);

	//モード切り替えタブ
	let titles = app
		.titles
		.iter()
		.map(|t| {
			Spans::from(vec![
				Span::styled(*t, Style::default().fg(Color::White)),
			])
		})
		.collect();
	let tabs = Tabs::new(titles)
		.block(Block::default().borders(Borders::ALL).title("Mode"))
		.select(app.mode)
		.style(Style::default().fg(Color::White))
		.highlight_style(
			Style::default()
				.add_modifier(Modifier::BOLD)
				.bg(Color::Yellow),
		);
	f.render_widget(tabs, chunks[0]);

	// ボードの描画
	let mut spanses: Vec<Spans> = Vec::new();
	let board_size = app.board.get_size();
	for y in 0..board_size.1 {
		let mut spans_raw: Vec<Span> = Vec::new();
		for x in 0..board_size.0 {
			//セルを描画する
			//セルの色を決定する
			let mut style = Style::default().bg(Color::Red);
			if let Some(cell) = app.board.get_cell(x, y) {
				if cell.is_alive() {
					style = style.bg(Color::LightYellow);		
				}else{
					style = style.bg(Color::DarkGray);
				}
			}
			// Editモードかつ セルが選択状態ならハイライトする
			if  app.mode == Mode::Edit as usize && (x, y) == app.selected_cell{
				style =style.bg(Color::LightMagenta);
			}
			// セルの表示を表示配列に追加する
			spans_raw.push(Span::styled("  ", style));
		}
		spanses.push(Spans::from(spans_raw));
	}
	let create_block = |title| {
		Block::default()
			.borders(Borders::ALL)
			.style(Style::default())
			.title(Span::styled(
				title,
				Style::default().add_modifier(Modifier::BOLD),
			))
	};
	let paragraph = Paragraph::new(spanses.clone())
		.style(Style::default())
		.block(create_block("Game Board"))
		.alignment(Alignment::Center);
	f.render_widget(paragraph, chunks[1]);

	// 操作方法の表示
    let mut usage_texts: Vec<Spans> = vec![
		Spans::from("Tab   : Change Mode"), // 共通
	];
	if app.mode == Mode::Edit as usize {
		usage_texts.push(Spans::from("←↑→↓  : Move Cursor")); // Editモード時
		usage_texts.push(Spans::from("Space : Toggle Cell Status"));
	}else if app.mode == Mode::Run as usize {
		// todo
	}

    let usage_paragraph = Paragraph::new(usage_texts.clone())
        .style(Style::default())
        .alignment(Alignment::Left);
    f.render_widget(usage_paragraph, chunks[2]);
}