extern crate libc;

use libc::funcs::posix88::unistd;
use std::c_str;
use std::cmp;
use std::os;
use std::ptr;
use std::rand::Rng;


fn main() {
    let version = "0.1.0";
    match parse_args(os::args().as_slice().tail()) {
        Err(Usage) => {
            os::set_exit_status(1);
            print_usage(version);
        }
        Err(Help) =>
            print_usage(version),
        Err(Version) =>
            print_version(version),
        Ok(opts) => {
            if whack(&opts) {
                os::set_exit_status(1);
            } else {
                opts.exe.map_or((), |exe| {
                    execvp_easy(exe.name, exe.args)
                });
            }
        }
    }
}

type ArgParse<T> = Result<T, Exit>;

#[deriving(Show)]
struct Options<'a> {
    chance: int,
    exe: Option<Exe<'a>>,
}

#[deriving(Show)]
struct Exe<'a> {
    name: &'a str,
    args: &'a[String],
}

#[deriving(Show)]
enum Exit {
    Usage,
    Help,
    Version,
}

fn parse_args(args: &[String]) -> ArgParse<Options> {
    args.uncons().map_or(Err(Usage), |(s, args)| {
        match s.as_slice() {
            "--help"    | "-h" => Err(Help),
            "--version" | "-v" => Err(Version),
            "--chance"  | "-c" => {
                args.uncons().map_or(Err(Usage), |(s, args)| {
                    from_str(s.as_slice()).map_or(Err(Usage), |val| {
                        args.skip("--").uncons().map_or(
                            Ok(Options {
                                chance: val,
                                exe: None
                            }), |(exe, args)| {
                                Ok(Options {
                                    chance: val,
                                    exe: Some(Exe {
                                        name: exe.as_slice(),
                                        args: args,
                                    })
                                })
                            })
                        })
                    })
                }
            _ => Err(Usage),
        }
    })
}

trait Uncons<'a, A> { fn uncons(&self) -> Option<(&'a A, &'a [A])>; }
trait Skip<'a, A, B> { fn skip(&self, x: A) -> &'a [B]; }

impl<'a, A> Uncons<'a, A> for &'a [A] {
    fn uncons(&self) -> Option<(&'a A, &'a [A])> {
        self.head().map(|x| (x, self.tail()))
    }
}

impl<'a > Skip<'a, &'a str, String> for &'a [String] {
    fn skip(&self, x: &'a str) -> &'a [String] {
        self.uncons().map_or(*self, |(y, ys)| { if x == y.as_slice() { ys } else { *self } })
    }
}

fn whack(opts: &Options) -> bool {
    std::rand::task_rng().gen_range(0, 100) > cmp::min(cmp::max(opts.chance, 0), 100)
}

fn execvp_easy(name: &str, args: &[String]) {
    let c_name: c_str::CString = name.to_c_str();
    let c_args: Vec<c_str::CString> = args.iter().map(|tmp| tmp.to_c_str()).collect();
    with_argv(&c_name, c_args.as_slice(), proc(c_argv) -> () unsafe {
        unistd::execvp(*c_argv, c_argv);
        fail!("executing {}: {}", name, os::last_os_error());
    });
}

fn with_argv<T>(prog: &c_str::CString, args: &[c_str::CString], f: proc(*mut*const libc::c_char) -> T) -> T {
    let mut ptrs: Vec<*const libc::c_char> = Vec::with_capacity(args.len() + 1);

    ptrs.push(prog.as_ptr());
    ptrs.extend(args.iter().map(|tmp| tmp.as_ptr()));
    ptrs.push(ptr::null());

    f(ptrs.as_mut_ptr())
}

fn print_usage(version: &str) {
    print_version(version);
    println!("\n\
        Usage: whacky (-c PERCENTAGE|--chance PERCENTAGE) [[--] COMMAND]\n\
          whacky, yet another randomly failing program\n\
        \n\
        Available options:\n\
          -h,--help           show this help text\n\
          -c,--chance         set success chance");
}

fn print_version(version: &str) {
    println!("whacky {}", version);
}
