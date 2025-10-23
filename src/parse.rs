use serde_json::{self, Value};

#[derive(Debug)]
pub struct Packet {
    pub timestamp: String,
    pub from: String,
    pub dest: String,
}

pub fn parse(string: String) -> Result<Packet, String> {
    let json: Value = serde_json::from_str(&string).expect("invalid json");

    // `tshark -T ek` prints two jsons to stdout, one with index, and one with values, for now ignore index
    if json.get("index").is_some() {
        return Err("not an error TODO".to_owned());
    }

    if let Some(packet) = temp(json) {
        return Ok(packet);
    }

    return Err("got no packet".to_owned());
}

fn temp(json: Value) -> Option<Packet> {
    return Some(Packet {
        timestamp: json.get("timestamp")?.to_string(),
        from: json
            .get("layers")?
            .get("eth")?
            .get("eth_eth_dst")?
            .to_string(),
        dest: json
            .get("layers")?
            .get("eth")?
            .get("eth_eth_src")?
            .to_string(),
    });
}
