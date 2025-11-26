use crate::network::Packet;

pub fn parse(string: String) -> Result<Option<Packet>, String> {
    let mut split = string.split("\t");
    let _id = split.next();
    let time = split.next().unwrap().to_owned().replace(' ', "");
    let Ok(from) = split.next().unwrap().replace(' ', "").parse() else {
        // TODO this is for non ip things like "broadcast", and "Intel_91:0d:66" (mac i think)
        return Ok(None);
    };
    let _arrow = split.next();
    let Ok(dest) = split.next().unwrap().replace(' ', "").parse() else {
        return Ok(None);
    };
    // let protocol = split.next();

    return Ok(Some(Packet {
        timestamp: time,
        from: from,
        dest: dest,
    }));
}
