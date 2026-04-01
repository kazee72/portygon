use colored::Colorize;



pub fn display_results(input: &[(u16, Option<String>)]) {
    let mut open_ports: Vec<(u16, String)> = Vec::new();

    for port in input {
        if let Some(banner) = &port.1 {
            open_ports.push((port.0, banner.trim().to_string()));
        }
    }

    println!("{}{}{}", "[".truecolor(223, 93, 108), "Open Ports".truecolor(92, 170, 180), "]".truecolor(223, 93, 108));

    if open_ports.is_empty() {
        println!("{}", "No open ports found.".red());
    } else {
        for port in open_ports {
            println!("{}: {}", port.0.to_string().green(), if port.1.is_empty() { "No banner".truecolor(92, 170, 180) } else { port.1.truecolor(92, 170, 180)});
        }
    }

}
