// AoC 2021 day 16 rewrite to use Result<> instead of panic'ing

use hex::FromHex;

#[derive(Debug)]
struct BinaryRep {
    raw: Vec<u8>,
    rawidx: usize,
    inneridx: u8,
}

impl BinaryRep {
    fn new(s: &str) -> Result<Self, &str> {
        match Vec::from_hex(s) {
            Ok(raw) => Ok(Self {
                raw,
                rawidx: 0,
                inneridx: 8,
            }),
            Err(_) => Err("Invalid Hex string"),
        }
    }

    fn get_next_chunk(&mut self, n: usize) -> Result<usize, &str> {
        let mut retval = 0;
        for _ in 0..n {
            if let Some(nv) = self.next() {
                retval <<= 1;
                retval += nv;
            } else {
                return Err("unexpected end of data");
            }
        }

        Ok(retval)
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

fn convert_type(tval: u8) -> Result<OpType, &'static str> {
    match tval {
        0 => Ok(OpType::Sum),
        1 => Ok(OpType::Prod),
        2 => Ok(OpType::Min),
        3 => Ok(OpType::Max),
        4 => Ok(OpType::Literal),
        5 => Ok(OpType::Greater),
        6 => Ok(OpType::Less),
        7 => Ok(OpType::Equal),
        _ => Err("bad operation type"),
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
    fn new(version: u8, optype: OpType, br: &mut BinaryRep) -> Result<Self, &str> {
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
    fn eval(&self) -> Result<usize, &str> {
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
                    retval = usize::MAX;
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
                OpType::Literal => Err("illegal Literal optype"),
            },
        }
    }
}

// read a literal value from BinaryRep, returning (the value, the number of bits in the literal)
fn parse_literal(br: &mut BinaryRep) -> Result<(usize, usize), &str> {
    let mut val: usize = 0;
    let mut bitcount: usize = 6;
    loop {
        let subval = match br.get_next_chunk(5) {
            Ok(s) => s,
            Err(_) => return Err("bad literal subval"),
        };
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

fn parse_operator(br: &mut BinaryRep) -> Result<(Vec<Packet>, usize), &str> {
    let mut retval: Vec<Packet> = vec![];
    let mut bitcount: usize = 6;

    let ltype = match br.get_next_chunk(1) {
        Ok(s) => s,
        Err(_) => return Err("bad length type"),
    };
    if ltype == 0 {
        let mut bits_left = match br.get_next_chunk(15) {
            Ok(s) => s,
            Err(_) => return Err("bad type 0 length value"),
        };
        bitcount += 16 + bits_left;
        while bits_left > 0 {
            let ver = match br.get_next_chunk(3) {
                Ok(s) => s as u8,
                Err(_) => return Err("bad type 0 version"),
            };
            let typ = match br.get_next_chunk(3) {
                Ok(s) => s as u8,
                Err(_) => return Err("bad type 0 type"),
            };
            let packet = match Packet::new(ver, convert_type(typ)?, br) {
                Ok(s) => s,
                Err(_) => return Err("bad type 0 operator packet"),
            };
            bits_left -= packet.bitc; // should never become < 0
            retval.push(packet);
        }
    } else {
        let pkts_left = match br.get_next_chunk(11) {
            Ok(s) => s,
            Err(_) => return Err("bad type 1 length value"),
        };
        bitcount += 12;
        for _ in 0..pkts_left {
            let ver = match br.get_next_chunk(3) {
                Ok(s) => s as u8,
                Err(_) => return Err("bad type 1 version"),
            };
            let typ = match br.get_next_chunk(3) {
                Ok(s) => s as u8,
                Err(_) => return Err("bad type 1 type"),
            };
            let packet = match Packet::new(ver, convert_type(typ)?, br) {
                Ok(s) => s,
                Err(_) => return Err("bad type 1 operator packet"),
            };
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
