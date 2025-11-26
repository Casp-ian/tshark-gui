use serde_json::{self, Value};
use std::{net::IpAddr, str::FromStr};

use crate::network::Packet;

pub fn parse(string: String) -> Result<Option<Packet>, String> {
    let json: Value = serde_json::from_str(&string).expect("invalid json");

    // `tshark -T ek` prints two jsons to stdout, one with index, and one with values, for now ignore index
    if json.get("index").is_some() {
        return Ok(None);
    }

    let packet: Option<Packet> = extract(json);
    return Ok(packet);
}

fn extract(json: Value) -> Option<Packet> {
    let from = json["layers"]["ip"]["ip_ip_src"].as_str()?;
    let dest = json["layers"]["ip"]["ip_ip_dst"].as_str()?;

    return Some(Packet {
        timestamp: json["timestamp"].as_str()?.to_owned(),
        from: IpAddr::from_str(from).unwrap(),
        dest: IpAddr::from_str(dest).unwrap(),
    });
}
