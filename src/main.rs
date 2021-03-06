#![feature(collections, exit_status, libc)]

extern crate libc;
extern crate rand;

use libc::funcs::posix88::unistd;
use std::ffi::CString;
use std::cmp;
use std::env;
use std::io;
use std::ptr;
use rand::Rng;


static VERSION: &'static str = "0.1.0";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match parse_args(args.as_ref()) {
        Err(Exit::Usage) => {
            die_usage();
        }
        Err(Exit::Help) =>
            print_usage(),
        Err(Exit::Version) =>
            print_version(),
        Ok(opts) => {
            if whack(&opts) {
                env::set_exit_status(1);
            } else {
                opts.exe.map_or((), |exe| {
                    execvp(exe.name, exe.args)
                });
            }
        }
    }
}

type ArgParse<T> = Result<T, Exit>;

#[derive(Debug)]
struct Options<'a> {
    chance: i32,
    exe: Option<Exe<'a>>,
}

#[derive(Debug)]
struct Exe<'a> {
    name: &'a str,
    args: &'a[String],
}

#[derive(Debug)]
enum Exit {
    Usage,
    Help,
    Version,
}

fn parse_args(args: &[String]) -> ArgParse<Options> {
    args.uncons().map_or(Err(Exit::Usage), |(s, args)| {
        match s.as_ref() {
            "--help"    | "-h" => Err(Exit::Help),
            "--version" | "-v" => Err(Exit::Version),
            "--chance"  | "-c" => {
                args.uncons().map_or(Err(Exit::Usage), |(s, args)| {
                    s.parse().ok().map_or(Err(Exit::Usage), |val| {
                        args.skip("--").uncons().map_or(
                            Ok(Options {
                                chance: val,
                                exe: None
                            }), |(exe, args)| {
                                Ok(Options {
                                    chance: val,
                                    exe: Some(Exe {
                                        name: exe.as_ref(),
                                        args: args,
                                    })
                                })
                            })
                        })
                    })
                }
            _ => Err(Exit::Usage),
        }
    })
}

trait Uncons<'a, A> { fn uncons(&self) -> Option<(&'a A, &'a [A])>; }
trait Skip<'a, A, B> { fn skip(&self, x: A) -> &'a [B]; }

impl<'a, A> Uncons<'a, A> for &'a [A] {
    fn uncons(&self) -> Option<(&'a A, &'a [A])> {
        self.first().map(|x| (x, self.tail()))
    }
}

impl<'a > Skip<'a, &'a str, String> for &'a [String] {
    fn skip(&self, x: &'a str) -> &'a [String] {
        self.uncons().map_or(*self, |(y, ys)| {
            if x == AsRef::<str>::as_ref(y) { ys } else { *self }
        })
    }
}

fn whack(opts: &Options) -> bool {
    rand::thread_rng().gen_range(0, 100) > cmp::min(cmp::max(opts.chance, 0), 100)
}

fn execvp(name: &str, args: &[String]) {
    let c_name = CString::new(name).unwrap(); // should be safe, the string is genuine
    let c_args: Vec<CString> =
        args.iter().map(|tmp| { CString::new(AsRef::<str>::as_ref(tmp)).unwrap() }).collect(); // likewise

    let mut xs: Vec<*const libc::c_char> = Vec::with_capacity(args.len() + 2);

    xs.push(c_name.as_ptr());
    xs.extend(c_args.iter().map(|tmp| tmp.as_ptr()));
    xs.push(ptr::null());

    let c_arg0 = xs[0];
    let c_argv = xs.as_mut_ptr();

    unsafe {
        unistd::execvp(c_arg0, c_argv);
    };

    panic!("execvp(3) failed with: {}", io::Error::last_os_error())
}

fn die_usage() {
    env::set_exit_status(1);
    print_usage();
}

fn print_usage() {
    print_version();
    println!("\n\
        Usage: whacky (-c PERCENTAGE|--chance PERCENTAGE) [[--] COMMAND]\n\
          whacky, yet another randomly failing program\n\
        \n\
        Available options:\n\
          -h,--help           show this help text\n\
          -c,--chance         set success chance");
}

fn print_version() {
    println!("whacky {}", VERSION);
}
