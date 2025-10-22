#[derive(Debug)]
pub struct Packet {
    pub id: u64,
    pub from: String,
    pub dest: String,
}

//
// a line from tshark looks something like this
// `  id time from → dest protocol ? fromport destport payload...`
//

pub fn parse(string: String) -> Packet {
    let (id, rest) = string.split_at(6);
    let id: u64 = id.trim().parse().unwrap();

    let mut rest = rest.split('\t');

    let time = rest.next().unwrap();

    let from = rest.next().unwrap().trim();
    let _ = rest.next();
    let dest = rest.next().unwrap().trim();

    return Packet {
        id: id,
        from: from.into(),
        dest: dest.into(),
    };
}
