use std::io::{stdin,BufRead};
use clap::{App, Arg, ArgMatches};

struct Cola2 {
    debug: bool,
    serialize: bool,
    ret: u128,
    p: u128,
}

impl Cola2 {

    fn is_on_octet_boundary(&self) -> bool {
        self.ret % 8 == 0 && self.ret != 0
    }

    //opb is operation A for collats' map
    fn opa(&mut self, p :u128) -> u128 {
        if self.serialize {
            print!("A");
        } else if self.debug {
            print!("{} -> ", p);
        }
        self.f(p / 2)
    }

    //opb is operation B for collats' map
    fn opb(&mut self, p :u128) -> u128 {
        if self.serialize {
            print!("B");
        } else if self.debug {
            print!("{} -> ", p)
        }
        self.f(p * 3 + 1)
    }

    fn f (&mut self, p :u128) -> u128 {
        if p == 0 {
            panic!("undef");
        }
        if p == 1 {
            if self.serialize {
                println!("");
            } else if self.debug {
                println!("1 [len={}]", self.ret);
            } else {
                println!("{} {}", self.p, self.ret);
            }
            return 0
        }
        if self.is_on_octet_boundary() {
            if self.serialize {
                /*if self.ret % 64 == 0 {
                    println!("")
                }*/
                //print!(" ");
            } 
        }
        self.ret = self.ret + 1;
        match p % 2 {
            0 => self.opa(p),
            1 => self.opb(p),
            _ => 0,
        }
    }
    pub fn run (p :u128, debug :bool, serialize :bool) {
        let mut runner = Cola2{
            debug: debug,
            serialize: serialize,
            ret: 0,
            p: p,
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
            if let Ok(n) = n.parse::<u128>() {
                Cola2::run(n, app.is_present("debug"), app.is_present("encode"));
            }
        }
    }
}
