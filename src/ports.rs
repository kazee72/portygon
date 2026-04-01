/// Parses a port specification string into a list of port numbers.
///
/// Supports single ports, comma-separated lists, and ranges.
/// Invalid entries are silently skipped.
///
/// # Arguments
/// * `port_string` - Port specification (e.g. "80", "80,443", "1-1024", "80,443,8000-8080")
///
/// # Returns
/// A `Vec<u16>` of parsed port numbers
pub fn parse_ports(port_string: &str) -> Vec<u16> {

    let mut ports: Vec<u16> = Vec::new();

    // Remove all whitespace from input
    let port_string_clean: String = port_string.chars().filter(|c| !c.is_whitespace()).collect();

    let separated: Vec<&str> = port_string_clean.split(',').collect();

    for port in separated {
        let port_range: Vec<&str> = port.split('-').collect();
        // Handle port ranges (e.g. "80-443")
        if port_range.len() > 1 {
            if let Ok(port1) = port_range[0].parse::<u16>() {
                if let Ok(port2) = port_range[1].parse::<u16>() {
                    for i in port1..=port2 {
                        ports.push(i);
                    }
                }
            }
        // Handle single ports
        } else if let Ok(port) = port.parse::<u16>() {
            ports.push(port);
        }
    }

    ports

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_port() {
        let expected: Vec<u16> = vec![80];
        let output: Vec<u16> = parse_ports("80");

        assert_eq!(expected, output);
    }

    #[test]
    fn test_comma_separated() {
        let expected: Vec<u16> = vec![80, 1024, 3000];
        let output: Vec<u16> = parse_ports("80,1024,3000");

        assert_eq!(expected, output);
    }

    #[test]
    fn test_whitespace() {
        let expected: Vec<u16> = vec![80, 1024, 3000];
        let output: Vec<u16> = parse_ports("80, 1024,   3000 ");

        assert_eq!(expected, output);
    }

    #[test]
    fn test_port_range_only() {
        let expected: Vec<u16> = vec![80, 81, 82, 83, 84, 85, 86];
        let output: Vec<u16> = parse_ports("80-86");

        assert_eq!(expected, output);
    }

    #[test]
    fn test_port_range_and_comma_separated() {
        let expected: Vec<u16> = vec![80, 124, 800, 3000, 3001, 3002, 3003, 4500, 4678];
        let output: Vec<u16> = parse_ports("80,124,800,3000-3003,4500,4678");

        assert_eq!(expected, output);
    }

    #[test]
    fn test_invalid_input() {
        let expected: Vec<u16> = vec![];
        let output: Vec<u16> = parse_ports("kjhsd,!!-djfh,///,++");

        assert_eq!(expected, output);
    }

    #[test]
    fn test_empty_string() {
        let expected: Vec<u16> = vec![];
        let output: Vec<u16> = parse_ports("");

        assert_eq!(expected, output);
    }

    #[test]
    fn test_mixed() {
        let expected: Vec<u16> = vec![80, 124, 125, 126];
        let output: Vec<u16> = parse_ports("sdkhjds,80,124-126,swe+*/");

        assert_eq!(expected, output);
    }
}