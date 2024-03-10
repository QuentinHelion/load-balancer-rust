mod response;
mod request;
mod interpreter;
mod controller;

use std::net::TcpListener;
use std::thread;
use log;


use controller::{handle_connection, parse_arguments, LoadBalancer};
use response::{generator,write_to_stream,make_http_error};
use interpreter::http_str2struct::HttpRequest;


#[tokio::main]
async fn main() {

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let args = parse_arguments();

    let load_balancer = LoadBalancer::new(
        args.load_balancer_ip.clone(),
        args.health_check_path
            .unwrap_or_else(|| "/health-check".to_string()), // Default health check path
        args.health_check_interval.unwrap_or(60), // Default health check interval of 60 seconds
        args.bind.clone(),
        args.window_size_secs.clone(),
        args.max_requests.clone(),
    );

    let listener = match TcpListener::bind(load_balancer.load_balancer_ip.clone()) {
        Ok(listener) => listener,
        Err(e) => {
            log::error!(
                "Failed to bind to address {}: {}",
                load_balancer.load_balancer_ip,
                e
            );
            return;
        }
    };


    let mut load_balancer_clone = load_balancer.clone();
    load_balancer_clone.start_health_check();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                log::info!("New client connection: {}", stream.peer_addr().unwrap());
                log::info!("Index : {:?}", load_balancer.last_selected_index);
                let mut load_balancer_clone2 = load_balancer.clone();
                thread::spawn(move || {
                    handle_connection(stream, &mut load_balancer_clone2);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }




    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    println!("Listening on 127.0.0.1:8080...");


    // while let Ok((stream, _)) = listener.accept() {
    //     let stream_buff_err = stream.try_clone().expect("Failed to clone stream");
    //     let stream_buff_res = stream.try_clone().expect("Failed to clone stream");
    //     let gen_resp = generator("200 OK", "text/plain", "Hello world ");
    //     task::spawn(async {
    //         let result = handle_client(stream);

    //         // Parse the HTTP request string
    //         match HttpRequest::from_string(result) {
    //             Ok(parsed_request) => {
    //                 println!("Parsed HTTP Request: {:#?}", parsed_request);
    //             }
    //             Err(err) => {
    //                 make_http_error(err, stream_buff_err);
    //                 eprintln!("Error parsing HTTP request: {}", err);
    //             }
    //         }
    //         write_to_stream(gen_resp, stream_buff_res);
    //     });
        
    // }
}
