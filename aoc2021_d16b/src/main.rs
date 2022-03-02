// AoC 2021 day 16 rewrite to use Result<> instead of panic'ing

use hex::FromHex;

// the raw data from the input hex string along with a pointers to the next bit to process
#[derive(Debug)]
struct BinaryRep {
    raw: Vec<u8>,
    rawidx: usize,
    inneridx: u8,
}

impl BinaryRep {
    fn new(s: &str) -> Result<Self, String> {
        match Vec::from_hex(s) {
            Ok(raw) => Ok(Self {
                raw,
                rawidx: 0,
                inneridx: 8,
            }),
            Err(e) => Err(format!("{}", e)),
        }
    }

    // chunk-wise iterator
    fn get_next_chunk(&mut self, n: usize) -> Result<u64, String> {
        if std::mem::size_of::<u64>() * 8 < n {
            return Err("chunk size too large".to_string());
        }

        let mut retval: u64 = 0;
        for _ in 0..n {
            if let Some(nv) = self.next() {
                retval <<= 1;
                retval += nv;
            } else {
                return Err("unexpected end of data".to_string());
            }
        }

        Ok(retval)
    }
}

// bit-wise iterator
impl Iterator for BinaryRep {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
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

// operator types
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

fn convert_type(tval: u8) -> Result<OpType, String> {
    match tval {
        0 => Ok(OpType::Sum),
        1 => Ok(OpType::Prod),
        2 => Ok(OpType::Min),
        3 => Ok(OpType::Max),
        4 => Ok(OpType::Literal),
        5 => Ok(OpType::Greater),
        6 => Ok(OpType::Less),
        7 => Ok(OpType::Equal),
        _ => Err("bad operation type".to_string()),
    }
}

#[derive(Debug)]
enum PacketType {
    Literal(u64),                // stores value in packet
    Operator(OpType, Vec<Packet>), // stores Operator type and sub-packets
}

// packet: with version, bit length and the packet type (see above)
#[derive(Debug)]
struct Packet {
    version: u8,
    bitc: u64,
    ptype: PacketType,
}

impl Packet {
    fn new(version: u8, optype: OpType, br: &mut BinaryRep) -> Result<Self, String> {
        if optype == OpType::Literal {
            let (val, bitc) = parse_literal(br)?;
            Ok(Self {
                version,
                bitc,
                ptype: PacketType::Literal(val),
            })
        } else {
            let (vecp, bitc) = parse_operator(br)?;
            Ok(Self {
                version,
                bitc,
                ptype: PacketType::Operator(optype, vecp),
            })
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
    fn eval(&self) -> Result<u64, String> {
        let mut retval = 0;

        match &self.ptype {
            PacketType::Literal(val) => Ok(*val),
            PacketType::Operator(o, v) => match o {
                OpType::Sum => {
                    for p in v.iter() {
                        retval += p.eval()?;
                    }
                    Ok(retval)
                }
                OpType::Prod => {
                    retval = 1;
                    for p in v.iter() {
                        retval *= p.eval()?;
                    }
                    Ok(retval)
                }
                OpType::Min => {
                    retval = u64::MAX;
                    for p in v.iter() {
                        let pval = p.eval()?;
                        if pval < retval {
                            retval = pval;
                        }
                    }
                    Ok(retval)
                }
                OpType::Max => {
                    for p in v.iter() {
                        let pval = p.eval()?;
                        if pval > retval {
                            retval = pval;
                        }
                    }
                    Ok(retval)
                }
                OpType::Greater => {
                    if v[0].eval()? > v[1].eval()? {
                        retval = 1;
                    }
                    Ok(retval)
                }
                OpType::Less => {
                    if v[0].eval()? < v[1].eval()? {
                        retval = 1;
                    }
                    Ok(retval)
                }
                OpType::Equal => {
                    if v[0].eval()? == v[1].eval()? {
                        retval = 1;
                    }
                    Ok(retval)
                }
                OpType::Literal => Err("illegal Literal optype".to_string()),
            },
        }
    }
}

// read a literal value from BinaryRep, returning (the value, the number of bits in the literal)
fn parse_literal(br: &mut BinaryRep) -> Result<(u64, u64), String> {
    let mut val: u64 = 0;
    let mut bitcount: u64 = 6;
    loop {
        let subval = br.get_next_chunk(5)?;
        bitcount += 5;
        if subval < 16 {
            val = val * 16 + subval;
            break;
        } else {
            val = val * 16 + subval - 16
        }
    }

    Ok((val, bitcount))
}

// read an operator from BinaryRep, returning (vector affected packets, number of bits) 
fn parse_operator(br: &mut BinaryRep) -> Result<(Vec<Packet>, u64), String> {
    let mut retval: Vec<Packet> = vec![];
    let mut bitcount: u64 = 6;

    let ltype = br.get_next_chunk(1)?;
    if ltype == 0 {
        let mut bits_left = br.get_next_chunk(15)?;
        bitcount += 16 + bits_left;
        while bits_left > 0 {
            let ver = br.get_next_chunk(3)? as u8;
            let typ = br.get_next_chunk(3)? as u8;
            let packet = Packet::new(ver, convert_type(typ)?, br)?;
            bits_left -= packet.bitc; // should never become < 0
            retval.push(packet);
        }
    } else {
        let pkts_left = br.get_next_chunk(11)?;
        bitcount += 12;
        for _ in 0..pkts_left {
            let ver = br.get_next_chunk(3)? as u8;
            let typ = br.get_next_chunk(3)? as u8;
            let packet = Packet::new(ver, convert_type(typ)?, br)?;
            bitcount += packet.bitc;
            retval.push(packet);
        }
    }

    Ok((retval, bitcount))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = std::io::stdin();
    let mut inputstr = String::new();

    while reader.read_line(&mut inputstr)? != 0 {
        let mut bin = BinaryRep::new(inputstr.trim())?;

        let ver = bin.get_next_chunk(3)? as u8;
        let typ = bin.get_next_chunk(3)? as u8;
        let packet = Packet::new(ver, convert_type(typ)?, &mut bin)?;

        // each line is a single packet
        println!("aoc16a: {}", packet.sum_versions());
        println!("aoc16b: {}", packet.eval()?);

        inputstr.clear();
    }

    Ok(())
}
