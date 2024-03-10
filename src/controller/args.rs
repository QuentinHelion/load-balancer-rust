use clap::{arg, Parser};

#[derive(Parser, Debug)]
#[command(long_about = None)]
pub struct Arguments {
    /// Load balancer ip address
    #[arg(short, long, required = true)]
    pub load_balancer_ip: String,

    /// Health check path
    #[arg(short = 'p', required = true)]
    pub health_check_path: Option<String>,

    /// Health check interval
    #[arg(short = 'i', required = true)]
    pub health_check_interval: Option<u64>,

    /// upstream_servers_ips
    #[arg(
        required = true,
        short,
        long,
        value_delimiter = ',',
        value_name = "UPSTREAM_SERVERS"
    )]
    pub bind: Vec<String>,

    /// Sliding window size in seconds
    #[arg(short = 's', required = true)]
    pub window_size_secs: u64, // Accept as integer

    #[arg(short = 'r', required = true)]
    pub max_requests: u32,
}

pub fn parse_arguments() -> Arguments {
    Arguments::parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arguments() {
        // Simulate command-line arguments
        let args = vec![
            "program_name",
            "--load-balancer-ip",
            "192.168.1.196:5555",
            "-b",
            "192.168.1.192:2222,192.168.1.190:8080,192.168.1.191:3333",
            "-p",
            "/health-check",
            "-i",
            "3",
            "-s",
            "3",
            "-r",
            "5",
        ];

        // Parse the arguments
        let parsed_args = Arguments::parse_from(args.iter());

        // Expected values
        let expected_args = Arguments {
            load_balancer_ip: "192.168.1.196:5555".to_string(),
            health_check_path: Some("/health-check".to_string()),
            health_check_interval: Some(3),
            bind: vec![
                "192.168.1.192:2222".to_string(),
                "192.168.1.190:8080".to_string(),
                "192.168.1.191:3333".to_string(),
            ],
            window_size_secs: 3,
            max_requests: 5,
        };

        // Verify that the parsed arguments match the expected values
        assert_eq!(parsed_args.load_balancer_ip, expected_args.load_balancer_ip);
        assert_eq!(
            parsed_args.health_check_path,
            expected_args.health_check_path
        );
        assert_eq!(
            parsed_args.health_check_interval,
            expected_args.health_check_interval
        );
        assert_eq!(parsed_args.bind, expected_args.bind);
        assert_eq!(parsed_args.window_size_secs, expected_args.window_size_secs);
        assert_eq!(parsed_args.max_requests, expected_args.max_requests);
    }
}
