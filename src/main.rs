use std::{collections::LinkedList, io::{stdout, Write}, process, sync::{Arc, Mutex}, thread, time::Duration};
use crossterm::event::{self, Event, KeyCode};
use rand::{rngs::ThreadRng, Rng};

struct Snake{
	body: LinkedList<(u32, u32)>,
	n: u32,
	dir: char
}

impl Snake{
	fn new(n: u32) -> Snake {
		Snake { 
			body: [(n-1, (n+1)/2)].iter().cloned().collect(),
			n: n, 
			dir: 'w' 
		}
	}
	fn next(&mut self, map: &mut Map) -> bool {
		let (mut x, mut y) = self.body.front().unwrap().clone();
		match self.dir {
			'w' => x -= 1,
			's' => x += 1,
			'a' => y -= 1,
			'd' => y += 1,
			_ => ()
		}
		if x <= 1 || x >= self.n || y <= 1 || y >= self.n || self.body.contains(&(x, y)) {
			return false;
		}
		let got = map.eat(x as usize, y as usize);
		if got {
			self.body.push_front((x, y));
			write!(stdout(), "\x1B[{};{}H*", x, y).unwrap();
		} else {
			let (tx, ty) = self.body.pop_back().unwrap();
			write!(stdout(), "\x1B[{};{}H ", tx, ty).unwrap();
			self.body.push_front((x, y));
			write!(stdout(), "\x1B[{};{}H*", x, y).unwrap();
		}
		stdout().flush().unwrap();
		true
	}
	fn set_dir(&mut self, c: char) {
		self.dir = c;
	}
}

struct Map{
	fruits: Vec<Vec<bool>>,
	n: u32
}
impl Map{
	fn new(n: u32) -> Map {
		Map::init_map(n);
		Map { 
			fruits: vec![vec![false; n as usize]; n as usize], 
			n: n
		}
	}
	fn init_map(n: u32) {
		for i in 0..n {
			for j in 0..n {
				if i == 0 || i == n-1 || j == 0 || j == n-1 {
					print!("#");
				} else {
					print!(" ");
				}
			}
			println!();
		}
		stdout().flush().unwrap();
	}
	fn gen_fruit(&mut self, rng: &mut ThreadRng) {
		let x = rng.gen_range(1..self.n-1);
		let y = rng.gen_range(1..self.n-1);
		self.fruits[x as usize][y as usize] = true;
		write!(stdout(), "\x1B[{};{}H+", x+1, y+1).unwrap();
	}
	fn eat(&mut self, mut x: usize, mut y: usize) -> bool {
		x -= 1;
		y -= 1;
		let ret = self.fruits[x][y];
		self.fruits[x][y] = false;
		ret
	}
}

fn main() {
    let mut stdout = stdout();

	//raw mode
	crossterm::terminal::enable_raw_mode().unwrap();
    // 进入 alternate screen buffer
    write!(stdout, "\x1B[?1049h").unwrap();
    // 隐藏光标
    write!(stdout, "\x1B[?25l").unwrap();

    // 清屏并显示自定义界面
    write!(stdout, "\x1B[2J\x1B[1;1H").unwrap();

	let mut snake = Arc::new(Mutex::new(Snake::new(30)));
	let mut map = Map::new(30);
	let mut last_char = None;

	thread::spawn({
		let snake = Arc::clone(&snake);
		move ||{
			let mut rng = rand::thread_rng();
			let mut cnt = 5 * 1000 / 250;
			loop {
				cnt -= 1;
				if cnt <= 0 {
					map.gen_fruit(&mut rng);
					cnt = 5 * 1000 / 250;
				}
				let live = snake.lock().unwrap().next(&mut map);
				if !live {
					process::exit(0);
				}
				thread::sleep(Duration::from_millis(250));
			}
		}
	});

	loop {
		// 等待最多 100ms 看是否有事件
		if event::poll(Duration::from_millis(250)).unwrap() {
			// 有事件，读取它
			if let event::Event::Key(key_event) = event::read().unwrap() {
				if let event::KeyCode::Char(c) = key_event.code {
					// 只处理字符输入
					if last_char != Some(c) {
						snake.lock().unwrap().set_dir(c);
						last_char = Some(c);
					}
				} else if let event::KeyCode::Esc = key_event.code {
					break;
				}
			}
		} else {
			// 可选：超时后重置 last_char，允许再次识别相同字符
			last_char = None;
		}
	}


    // 恢复终端
    write!(stdout, "\x1B[?25h").unwrap();     // 显示光标
    write!(stdout, "\x1B[?1049l").unwrap();   // 返回主屏幕

    stdout.flush().unwrap();
}