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
}

pub fn parse_arguments() -> Arguments {
    Arguments::parse()
}
