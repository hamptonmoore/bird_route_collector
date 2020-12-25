use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
extern crate serde_json;
extern crate serde;

#[derive(serde::Serialize)]
#[derive(Clone)]
enum IpAddrVersion {
    V6,
}

#[derive(serde::Serialize)]
#[derive(Clone)]
struct Route {
    version: IpAddrVersion,
    address: String,
}

#[derive(serde::Serialize)]
struct AutonomousNetwork {
    asn: String,
    routes: Vec<Route>,
    children: HashMap<String, AutonomousNetwork>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);

    let mut current_route: Route = Route {
        version: IpAddrVersion::V6,
        address: "::/0".to_string()
    };
    let mut route_table = AutonomousNetwork {
        asn: "root".to_string(),
        routes: vec![],
        children: HashMap::new()
    };

    for wrapped_line in reader.lines() {
        let line = wrapped_line.expect("issue reading line");
        let mut line_split = line.split_whitespace();

        if !line.starts_with("	") {
            if !line.starts_with("                     ") {
                // new route
                let ip = line_split.nth(0).unwrap();
                current_route = Route {
                    version: IpAddrVersion::V6,
                    address: ip.to_string()
                };
            }
        } else {
            if line_split.next().unwrap() == "BGP.as_path:" {
                let mut location: &mut AutonomousNetwork = &mut route_table;

                for asn in line_split {
                    if !location.children.contains_key(asn) {
                       location.children.insert(asn.to_string(), AutonomousNetwork {
                           asn: asn.to_string(),
                           routes: vec![],
                           children: HashMap::new()
                       });
                    }

                    location = location.children.get_mut(asn).unwrap();
                }

                location.routes.push(current_route.clone());
            }
        }
    }

    println!("{}", serde_json::to_string(&route_table).expect("Oop"));

}
