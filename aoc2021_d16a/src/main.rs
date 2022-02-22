// AoC 2021 day 16

use hex::FromHex;

#[derive(Debug)]
struct BinaryRep {
    raw: Vec<u8>,
    rawidx: usize,
    inneridx: u8,
}

impl BinaryRep {
    fn new(s: &str) -> Self {
        Self {
            raw: Vec::from_hex(s).expect("Invalid Hex String"),
            rawidx: 0,
            inneridx: 8,
        }
    }

    fn get_next_chunk(&mut self, n: usize) -> Option<usize> {
        let mut retval = 0;

        for _ in 0..n {
            let nv = self.next();
            nv?; // return None if nv done
            retval <<= 1;
            retval += nv.unwrap();
        }

        Some(retval)
    }
}

// bit-wise iterator
impl Iterator for BinaryRep {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.rawidx >= self.raw.len() {
            None
        } else {
            let set = self.raw[self.rawidx] & 1 << (self.inneridx - 1) > 0;
            self.inneridx -= 1;
            if self.inneridx == 0 {
                self.rawidx += 1;
                self.inneridx = 8;
            }
            if set {
                Some(1)
            } else {
                Some(0)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum OpType {
    Sum,
    Prod,
    Min,
    Max,
    Literal,
    Greater,
    Less,
    Equal,
}

fn convert_type(tval: u8) -> OpType {
    match tval {
        0 => OpType::Sum,
        1 => OpType::Prod,
        2 => OpType::Min,
        3 => OpType::Max,
        4 => OpType::Literal,
        5 => OpType::Greater,
        6 => OpType::Less,
        7 => OpType::Equal,
        _ => panic!("bad operation type"),
    }
}

#[derive(Debug)]
enum PacketType {
    Literal(usize),                // stores value in packet
    Operator(OpType, Vec<Packet>), // stores Operator type and sub-packets
}

#[derive(Debug)]
struct Packet {
    version: u8,
    bitc: usize,
    ptype: PacketType,
}

impl Packet {
    fn new(version: u8, optype: OpType, br: &mut BinaryRep) -> Self {
        let (pv, bitc) = if optype == OpType::Literal {
            let (val, len) = parse_literal(br);
            (PacketType::Literal(val), len)
        } else {
            let (vecp, len) = parse_operator(br);
            (PacketType::Operator(optype, vecp), len)
        };
        Self {
            version,
            bitc,
            ptype: pv,
        }
    }

    // part one - sum all the versions
    fn sum_versions(&self) -> usize {
        let mut retval = self.version as usize;

        if let PacketType::Operator(_, v) = &self.ptype {
            for p in v.iter() {
                retval += p.sum_versions();
            }
        }

        retval
    }

    // part two - evaluate the packet expression
    fn eval(&self) -> usize {
        let mut retval = 0;

        match &self.ptype {
            PacketType::Literal(val) => retval = *val,
            PacketType::Operator(o, v) => match o {
                OpType::Sum => {
                    for p in v.iter() {
                        retval += p.eval();
                    }
                }
                OpType::Prod => {
                    retval = 1;
                    for p in v.iter() {
                        retval *= p.eval();
                    }
                }
                OpType::Min => {
                    retval = usize::MAX;
                    for p in v.iter() {
                        let pval = p.eval();
                        if pval < retval {
                            retval = pval;
                        }
                    }
                }
                OpType::Max => {
                    for p in v.iter() {
                        let pval = p.eval();
                        if pval > retval {
                            retval = pval;
                        }
                    }
                }
                OpType::Greater => {
                    if v[0].eval() > v[1].eval() {
                        retval = 1;
                    }
                }
                OpType::Less => {
                    if v[0].eval() < v[1].eval() {
                        retval = 1;
                    }
                }
                OpType::Equal => {
                    if v[0].eval() == v[1].eval() {
                        retval = 1;
                    }
                }
                OpType::Literal => panic!("illegal Literal optype"),
            },
        }

        retval
    }
}

// read a literal value from BinaryRep, returning (the value, the number of bits in the literal)
fn parse_literal(br: &mut BinaryRep) -> (usize, usize) {
    let mut val: usize = 0;
    let mut bitcount: usize = 6;
    loop {
        let subval = br.get_next_chunk(5).expect("bad literal val");
        bitcount += 5;
        if subval < 16 {
            val = val * 16 + subval;
            break;
        } else {
            val = val * 16 + subval - 16
        }
    }

    (val, bitcount)
}

fn parse_operator(br: &mut BinaryRep) -> (Vec<Packet>, usize) {
    let mut retval: Vec<Packet> = vec![];
    let mut bitcount: usize = 6;

    let ltype = br.get_next_chunk(1).expect("bad length type");
    if ltype == 0 {
        let mut bits_left = br.get_next_chunk(15).expect("bad bit length");
        bitcount += 16 + bits_left;
        while bits_left > 0 {
            let ver = br.get_next_chunk(3).expect("bad version found") as u8;
            let typ = br.get_next_chunk(3).expect("bad type found") as u8;
            let packet = Packet::new(ver, convert_type(typ), br);
            bits_left -= packet.bitc;  // should never become < 0
            retval.push(packet);
        }
    } else {
        let pkts_left = br.get_next_chunk(11).expect("bad packet number");
        bitcount += 12;
        for _ in 0..pkts_left {
            let ver = br.get_next_chunk(3).expect("bad version found") as u8;
            let typ = br.get_next_chunk(3).expect("bad type found") as u8;
            let packet = Packet::new(ver, convert_type(typ), br);
            bitcount += packet.bitc;
            retval.push(packet);
        }
    }

    (retval, bitcount)
}

fn main() -> std::io::Result<()> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();

    while reader.read_line(&mut inputstr)? != 0 {
        let mut bin = BinaryRep::new(inputstr.trim());

        let topver = bin.get_next_chunk(3).expect("bad initial version found") as u8;
        let toptype = bin.get_next_chunk(3).expect("bad initial type found") as u8;
        let packet = Packet::new(topver, convert_type(toptype), &mut bin);

        // each line is a single packet
        println!("aoc16a: {}", packet.sum_versions());
        println!("aoc16b: {}", packet.eval());

        inputstr.clear();
    }

    Ok(())
}
