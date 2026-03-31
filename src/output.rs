use colored::Colorize;



pub fn display_results(input: &[(u16, bool)]) {
    let mut open_ports: Vec<u16> = Vec::new();

    for port in input {
        if port.1 {
            open_ports.push(port.0);
        }
    }

    println!("{}{}{}", "[".truecolor(223, 93, 108), "Open Ports".truecolor(92, 170, 180), "]".truecolor(223, 93, 108));

    if open_ports.is_empty() {
        println!("{}", "No open ports found.".red());
    } else {
        for port in open_ports {
            println!("{}", port.to_string().green());
        }
    }

}
