use std::io::{self, Write};
use std::process::Command;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers, KeyEventKind},
    style::Print,
    terminal::{self, ClearType},
    ExecutableCommand, QueueableCommand,
};

// ========== EDITOR ==========

struct Editor {
    lines: Vec<String>,
    cursor_x: usize,
    cursor_y: usize,
    filename: String,
    scroll_y: usize,
    modified: bool,
}

impl Editor {
    fn new(filename: &str) -> Self {
        let lines = if std::path::Path::new(filename).exists() {
            std::fs::read_to_string(filename)
                .unwrap_or_default()
                .lines()
                .map(|l| l.to_string())
                .collect()
        } else {
            vec![String::new()]
        };
        Editor {
            lines: if lines.is_empty() { vec![String::new()] } else { lines },
            cursor_x: 0,
            cursor_y: 0,
            filename: filename.to_string(),
            scroll_y: 0,
            modified: false,
        }
    }

    fn save(&mut self) {
        let content = self.lines.join("\n");
        match std::fs::write(&self.filename, content) {
            Ok(_) => self.modified = false,
            Err(e) => eprintln!("Save error: {}", e),
        }
    }

    fn draw(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        let (width, height) = terminal::size()?;
        let (width, height) = (width as usize, height as usize);

        stdout.queue(terminal::Clear(ClearType::All))?;
        stdout.queue(cursor::Hide)?;

        for screen_row in 0..height.saturating_sub(2) {
            let file_row = screen_row + self.scroll_y;
            stdout.queue(cursor::MoveTo(0, screen_row as u16))?;
            if file_row < self.lines.len() {
                let line = &self.lines[file_row];
                let display = if line.len() > width { &line[..width] } else { line.as_str() };
                stdout.queue(Print(display))?;
            } else {
                stdout.queue(Print("~"))?;
            }
        }

        let modified_str = if self.modified { " [+]" } else { "" };
        let status = format!(
            " RSH EDIT | {}{} | Ln {}/{} | Ctrl+S: Save | Ctrl+Q: Quit ",
            self.filename, modified_str,
            self.cursor_y + 1, self.lines.len()
        );
        let status_padded = format!("{:width$}", status, width = width);

        stdout.queue(cursor::MoveTo(0, (height - 2) as u16))?;
        stdout.queue(crossterm::style::SetBackgroundColor(crossterm::style::Color::DarkGrey))?;
        stdout.queue(crossterm::style::SetForegroundColor(crossterm::style::Color::White))?;
        stdout.queue(Print(&status_padded[..width.min(status_padded.len())]))?;
        stdout.queue(crossterm::style::ResetColor)?;

        let screen_y = self.cursor_y.saturating_sub(self.scroll_y);
        stdout.queue(cursor::MoveTo(self.cursor_x as u16, screen_y as u16))?;
        stdout.queue(cursor::Show)?;
        stdout.flush()?;
        Ok(())
    }

    fn run(&mut self) {
        let mut stdout = io::stdout();
        terminal::enable_raw_mode().unwrap();
        stdout.execute(terminal::EnterAlternateScreen).unwrap();

        loop {
            self.draw(&mut stdout).unwrap();
            if let Ok(Event::Key(key)) = event::read() {
                // FIX: ignoruj Release i Repeat eventy, tylko Press
                if key.kind != KeyEventKind::Press { continue; }

                match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('q')) => break,
                    (KeyModifiers::CONTROL, KeyCode::Char('s')) => self.save(),

                    (_, KeyCode::Up) => {
                        if self.cursor_y > 0 {
                            self.cursor_y -= 1;
                            self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
                            if self.cursor_y < self.scroll_y { self.scroll_y -= 1; }
                        }
                    }
                    (_, KeyCode::Down) => {
                        if self.cursor_y < self.lines.len() - 1 {
                            self.cursor_y += 1;
                            self.cursor_x = self.cursor_x.min(self.lines[self.cursor_y].len());
                            let (_, h) = terminal::size().unwrap_or((80, 24));
                            if self.cursor_y >= self.scroll_y + h as usize - 2 {
                                self.scroll_y += 1;
                            }
                        }
                    }
                    (_, KeyCode::Left) => {
                        if self.cursor_x > 0 {
                            self.cursor_x -= 1;
                        } else if self.cursor_y > 0 {
                            self.cursor_y -= 1;
                            self.cursor_x = self.lines[self.cursor_y].len();
                        }
                    }
                    (_, KeyCode::Right) => {
                        if self.cursor_x < self.lines[self.cursor_y].len() {
                            self.cursor_x += 1;
                        } else if self.cursor_y < self.lines.len() - 1 {
                            self.cursor_y += 1;
                            self.cursor_x = 0;
                        }
                    }
                    (_, KeyCode::Home) => self.cursor_x = 0,
                    (_, KeyCode::End)  => self.cursor_x = self.lines[self.cursor_y].len(),

                    (_, KeyCode::Enter) => {
                        let rest = self.lines[self.cursor_y].split_off(self.cursor_x);
                        self.cursor_y += 1;
                        self.lines.insert(self.cursor_y, rest);
                        self.cursor_x = 0;
                        self.modified = true;
                    }
                    (_, KeyCode::Backspace) => {
                        if self.cursor_x > 0 {
                            self.lines[self.cursor_y].remove(self.cursor_x - 1);
                            self.cursor_x -= 1;
                            self.modified = true;
                        } else if self.cursor_y > 0 {
                            let line = self.lines.remove(self.cursor_y);
                            self.cursor_y -= 1;
                            self.cursor_x = self.lines[self.cursor_y].len();
                            self.lines[self.cursor_y].push_str(&line);
                            self.modified = true;
                        }
                    }
                    (_, KeyCode::Delete) => {
                        if self.cursor_x < self.lines[self.cursor_y].len() {
                            self.lines[self.cursor_y].remove(self.cursor_x);
                            self.modified = true;
                        } else if self.cursor_y < self.lines.len() - 1 {
                            let next = self.lines.remove(self.cursor_y + 1);
                            self.lines[self.cursor_y].push_str(&next);
                            self.modified = true;
                        }
                    }
                    (_, KeyCode::Tab) => {
                        self.lines[self.cursor_y].insert_str(self.cursor_x, "    ");
                        self.cursor_x += 4;
                        self.modified = true;
                    }
                    (_, KeyCode::Char(c)) => {
                        self.lines[self.cursor_y].insert(self.cursor_x, c);
                        self.cursor_x += 1;
                        self.modified = true;
                    }
                    _ => {}
                }
            }
        }

        stdout.execute(terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}

// ========== SHELL ==========

fn greeting() {
    println!("Welcome to RustSH v0.3");
    println!("Type '--help' for available commands");
}

fn help_function() {
    println!("RustSH Help:");
    println!("  exit / quit        ==> Exits RustSH");
    println!("  --help             ==> Shows this help");
    println!("  ls [path]          ==> List directory");
    println!("  cd <path>          ==> Change directory");
    println!("  mk <file>          ==> Create empty file");
    println!("  mkdir <dir>        ==> Create directory");
    println!("  edit <file>        ==> Built-in editor (Ctrl+S save, Ctrl+Q quit)");
    println!("  run <file>         ==> Auto-detect and run (.py .rs .js .c .cpp .sh)");
    println!("  apt install <pkg>  ==> Install via cargo");
    println!("  apt rustc          ==> Update rustc");
}

fn ls_function(path: &str) {
    let target = if path.is_empty() { "." } else { path };
    let entries = match std::fs::read_dir(target) {
        Ok(e) => e,
        Err(_) => { println!("ls: cannot open '{}'", target); return; }
    };
    for entry in entries {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        if file_type.is_dir() {
            println!("[DIR]  {}", entry.file_name().to_string_lossy());
        } else {
            println!("[FILE] {}", entry.file_name().to_string_lossy());
        }
    }
}

fn cd_function(path: &str) {
    if path.is_empty() { println!("cd: missing argument"); return; }
    if std::env::set_current_dir(path).is_err() {
        println!("cd: cannot find '{}'", path);
    }
}

fn mk_function(filename: &str) {
    if filename.is_empty() { println!("mk: missing filename"); return; }
    match std::fs::File::create(filename) {
        Ok(_) => println!("Created '{}'", filename),
        Err(e) => println!("mk: error: {}", e),
    }
}

fn mkdir_function(dirname: &str) {
    if dirname.is_empty() { println!("mkdir: missing name"); return; }
    match std::fs::create_dir_all(dirname) {
        Ok(_) => println!("Created directory '{}'", dirname),
        Err(e) => println!("mkdir: error: {}", e),
    }
}

fn edit_function(filename: &str) {
    if filename.is_empty() { println!("edit: missing filename"); return; }
    let mut editor = Editor::new(filename);
    editor.run();
}

fn run_function(filename: &str) {
    if filename.is_empty() { println!("run: missing filename"); return; }

    let ext = std::path::Path::new(filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match ext {
        "py" => {
            Command::new("python").arg(filename).status()
                .or_else(|_| Command::new("python3").arg(filename).status())
                .map_err(|_| println!("run: python not found")).ok();
        }
        "rs" => {
            println!("run: kompiluje {}...", filename);
            let out = filename.trim_end_matches(".rs");
            if Command::new("rustc").args([filename, "-o", out]).status().is_ok() {
                Command::new(format!("./{}", out)).status().ok();
            }
        }
        "js" => {
            Command::new("node").arg(filename).status()
                .map_err(|_| println!("run: node not found")).ok();
        }
        "c" => {
            println!("run: kompiluje {}...", filename);
            let out = filename.trim_end_matches(".c");
            if Command::new("gcc").args([filename, "-o", out]).status().is_ok() {
                Command::new(format!("./{}", out)).status().ok();
            }
        }
        "cpp" => {
            println!("run: kompiluje {}...", filename);
            let out = filename.trim_end_matches(".cpp");
            if Command::new("g++").args([filename, "-o", out]).status().is_ok() {
                Command::new(format!("./{}", out)).status().ok();
            }
        }
        "sh" => {
            Command::new("bash").arg(filename).status()
                .map_err(|_| println!("run: bash not found")).ok();
        }
        _ => println!("run: nieznane rozszerzenie '.{}', obsługiwane: py rs js c cpp sh", ext),
    }
}

fn apt_function(args: &[&str]) {
    if args.is_empty() { println!("apt: missing subcommand"); return; }
    match args[0] {
        "install" => {
            if args.len() < 2 { println!("apt install: missing package name"); return; }
            println!("Installing '{}' via cargo...", args[1]);
            Command::new("cargo").args(["install", args[1]]).status().ok();
        }
        "rustc" => {
            println!("Updating rustc...");
            Command::new("rustup").args(["update", "stable"]).status().ok();
        }
        _ => println!("apt: unknown subcommand '{}'", args[0]),
    }
}

fn main() {
    greeting();
    loop {
        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "?".to_string());

        print!("RSH {} >>> ", cwd);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() { continue; }

        match parts[0] {
            "exit" | "quit" => { println!("Exiting RustSH..."); break; }
            "--help"  => help_function(),
            "ls"      => ls_function(&parts[1..].join(" ")),
            "cd"      => cd_function(&parts[1..].join(" ")),
            "mk"      => mk_function(&parts[1..].join(" ")),
            "mkdir"   => mkdir_function(&parts[1..].join(" ")),
            "edit"    => edit_function(&parts[1..].join(" ")),
            "run"     => run_function(&parts[1..].join(" ")),
            "apt"     => apt_function(&parts[1..]),
            _ => {
                if Command::new(parts[0]).args(&parts[1..]).status().is_err() {
                    println!("RSH: command not found: '{}'", parts[0]);
                }
            }
        }
    }
}