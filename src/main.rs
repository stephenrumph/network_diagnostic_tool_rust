use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};
use std::thread;

/// Adds color to terminal output for better readability.
fn colorize(text: &str, color: &str) -> String {
    let color_code = match color {
        "red" => "\x1b[31m",
        "green" => "\x1b[32m",
        "yellow" => "\x1b[33m",
        "blue" => "\x1b[34m",
        "cyan" => "\x1b[36m",
        _ => "\x1b[0m",
    };
    format!("{}{}{}", color_code, text, "\x1b[0m")
}

/// Executes a shell command and prints the result.
fn run_command(command: &str, args: &[&str], description: &str) {
    println!("🔹 {}", colorize(description, "blue"));
    let output = Command::new(command).args(args).output();

    match output {
        Ok(result) => {
            if result.status.success() {
                println!("✅ {}\n{}", colorize("[SUCCESS]", "green"), String::from_utf8_lossy(&result.stdout));
            } else {
                println!("❌ {}\n{}", colorize("[ERROR]", "red"), String::from_utf8_lossy(&result.stderr));
            }
        }
        Err(e) => println!("❌ {} {}", colorize("[ERROR]", "red"), e),
    }
    thread::sleep(Duration::from_secs(1));
}

/// Runs basic network tests.
fn network_test() {
    println!("\n🌐 {} Running Network Diagnostics...\n", colorize("[INFO]", "blue"));

    run_command("ping", &["-c", "4", "8.8.8.8"], "Pinging Google DNS Server (8.8.8.8)");
    run_command("curl", &["ifconfig.me"], "Fetching Public IP Address");
    run_command("sh", &["-c", "ifconfig -a | grep 'inet '"], "Fetching Private IP Address");
    run_command("sh", &["-c", "netstat -an | grep 'ESTABLISHED'"], "Checking Open Listening Ports");
    run_command("sh", &["-c", "traceroute google.com"], "Running Traceroute to Google");
    run_command("netstat", &["-rn", "-f", "inet"], "Displaying Routing Table");

    println!("🌍 {}\n", colorize("[INFO] Network tests completed.", "blue"));
}

/// Captures network packets using `tcpdump` while visiting websites.
fn capture_traffic(interface: &str, port: &str, max_packets: usize, timeout_secs: u64) {
    println!("\n📡 {} Capturing {} packets on {} (port {})\n",
        colorize("[INFO]", "blue"), max_packets, colorize(interface, "cyan"), colorize(port, "cyan"));

    // Spawn tcpdump process
    let mut child = Command::new("tcpdump")
        .args(&["-i", interface, "port", port, "-c", &max_packets.to_string(), "-nn", "-vvv"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start tcpdump");

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);
    let start_time = Instant::now();
    let mut packet_count = 0;

    // Start a separate thread for visiting websites while capturing traffic
    let site_thread = thread::spawn(|| visit_websites());

    println!("\n🌍 {} Visiting Websites While Capturing Traffic...\n", colorize("[INFO]", "blue"));

    println!(
        "{:<20} {:<20} {:<10} {:<40}",
        colorize("Timestamp", "yellow"),
        colorize("Source", "cyan"),
        colorize("Protocol", "blue"),
        colorize("Info", "green")
    );
    println!("{}", "-".repeat(90));

    for line in reader.lines() {
        match line {
            Ok(packet) => {
                if let Some((timestamp, src, protocol, info)) = parse_packet(&packet) {
                    println!(
                        "{:<20} {:<20} {:<10} {:<40}",
                        colorize(&timestamp, "yellow"),
                        colorize(&src, "cyan"),
                        colorize(&protocol, "blue"),
                        colorize(&info, "green")
                    );
                }
                packet_count += 1;
            }
            Err(e) => {
                println!("❌ {} Error reading packet: {}", colorize("[ERROR]", "red"), e);
                break;
            }
        }

        if packet_count >= max_packets || start_time.elapsed() >= Duration::from_secs(timeout_secs) {
            println!("\n⏳ {} Stopping capture after {} packets or {} seconds.",
                     colorize("[TIMEOUT]", "yellow"), packet_count, timeout_secs);
            break;
        }
    }

    // Ensure tcpdump exits cleanly
    let _ = child.kill();
    let _ = site_thread.join();

    println!("\n📊 {} Summary: Captured {} packets.\n", colorize("[SUMMARY]", "blue"), packet_count);
}

/// Visits multiple websites in parallel while traffic is being captured.
fn visit_websites() {
    let sites = vec![
        ("https://www.google.com/search?q=network+diagnostics", "Google"),
        ("http://www.microsoft.com", "Microsoft"),
        ("http://www.amazon.com.au", "Amazon"),
        ("http://www.facebook.com", "Facebook"),
        ("https://www.youtube.com", "YouTube"),
        ("http://www.apple.com", "Apple"),
        ("http://www.github.com", "GitHub"),
        ("http://www.linkedin.com", "LinkedIn"),
        ("http://www.reddit.com", "Reddit"),
        ("http://www.twitter.com", "Twitter"),
        ("http://www.wikipedia.org", "Wikipedia"),
        ("http://www.instagram.com", "Instagram"),
        ("http://www.netflix.com", "Netflix"),
        ("http://www.spotify.com", "Spotify"),
        ("http://www.stackoverflow.com", "StackOverflow"),
        ("http://www.medium.com", "Medium"),
        ("http://www.quora.com", "Quora"),
        ("http://www.udemy.com", "Udemy"),
        ("http://www.coursera.org", "Coursera"),
        ("http://www.khanacademy.org", "Khan Academy"),
    ];

    for (url, name) in &sites {
        let result = Command::new("curl").args(&["-I", url]).output();
        match result {
            Ok(response) => {
                if response.status.success() {
                    println!("✅ {} Visited: {}", colorize("[SUCCESS]", "green"), colorize(name, "cyan"));
                } else {
                    println!("❌ {} Failed to visit {}", colorize("[ERROR]", "red"), name);
                }
            }
            Err(e) => println!("❌ {} {}", colorize("[ERROR]", "red"), e),
        }
    }
}

/// Parses a `tcpdump` packet line into structured fields.
fn parse_packet(packet: &str) -> Option<(String, String, String, String)> {
    let parts: Vec<&str> = packet.split_whitespace().collect();
    if parts.len() < 6 { return None; }

    Some((
        parts[0].to_string(), // Timestamp
        parts[2].to_string(), // Source IP
        parts[4].to_string(), // Protocol
        parts[5..].join(" "), // Packet details
    ))
}

/// **Main function: Runs network tests and captures traffic.**
fn main() {
    network_test();
    capture_traffic("en0", "53", 10, 1); // Capture packets while visiting sites
}
