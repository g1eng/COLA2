use std::io::{stdin,BufRead};
use num_bigint::BigUint;
use num_traits::{Zero, One};
use clap::{App, Arg, ArgMatches};

struct Cola2 {
    debug: bool,
    serialize: bool,
    ret: BigUint,
    p: BigUint,
}


impl Cola2 {

    //opb is operation A for collats' map
    fn opa(&mut self, p :BigUint) -> BigUint {
        if self.serialize {
            print!("A");
        } else if self.debug {
            print!("{} -> ", p);
        }
        self.f(p / BigUint::from(2u8))
    }

    //opb is operation B for collats' map
    fn opb(&mut self, p :BigUint) -> BigUint {
        if self.serialize {
            print!("B");
        } else if self.debug {
            print!("{} -> ", p)
        }
        self.f((BigUint::from(3u8) * p) + BigUint::from(1u8))
    }

    fn f (&mut self, p :BigUint) -> BigUint {
        if p == BigUint::from(0u8) {
            panic!("undef");
        }
        if p == BigUint::from(1u8) {
            if self.serialize {
                println!("");
            } else if self.debug {
                println!("1 [len={}]", self.ret);
            } else {
                println!("{} {}", self.p, self.ret);
            }
            return BigUint::from(0u8)
        }
        self.ret = self.ret.clone() + BigUint::from(1u8);
        let zero :BigUint = Zero::zero();
        let one :BigUint = One::one();
        let modulo = p.clone() % BigUint::from(2u8);
        if modulo == zero {
            self.opa(p)
        } else if modulo == one {
            self.opb(p)
        } else {
            zero
        }
    }
    pub fn run (p :BigUint, debug :bool, serialize :bool) {
        let mut runner = Cola2{
            debug: debug,
            serialize: serialize,
            ret: BigUint::from(0u8),
            p: p.clone(),
        };
        runner.f(p);
    }
}

fn get_cli_matcher() -> ArgMatches<'static> {
    App::new("cola2")
        .version("0.1.0")
        .author("Nomura Suzume <suzume315@g00.g1e.org>")
        .arg(
            Arg::with_name("encode")
                .short("E")
                .help("encode output with CPD format")
                .required(false)
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .help("show verbose output")
                .takes_value(false)
                .required(false)
        )
        .arg(
            Arg::with_name("split")
                .short("s")
                .value_name("NUM")
                .takes_value(true)
                .help("split serialized octet with newline for specified blocks")
                .required(false)
        )
        .get_matches()
}

fn main() {
    let app = get_cli_matcher();

    let stdin = stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut numbers = line.split_whitespace().collect::<Vec<_>>();
        while let Some(n) = numbers.pop() {
            if let Ok(n) = n.parse::<BigUint>() {
                Cola2::run(n, app.is_present("debug"), app.is_present("encode"));
            }
        }
    }
}
