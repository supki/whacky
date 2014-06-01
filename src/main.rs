extern crate libc;

use libc::funcs::posix88::unistd::execvp;
use std::c_str::CString;
use std::os::last_os_error;
use std::ptr;
use std::rand::Rng;


fn main() {
    let version = "0.1.0";
    match parse_args(&std::os::args()) {
        Err(Usage) => {
            std::os::set_exit_status(1);
            print_usage(version);
        }
        Err(Help) =>
            print_usage(version),
        Err(Version) =>
            print_version(version),
        Ok(opts) => {
            if whack(&opts) {
                std::os::set_exit_status(1);
            } else {
                opts.exe.map_or((), |exe| {
                    execvp_easy(&exe.name.to_string(), exe.args)
                });
            }
        }
    }
}

type ArgParse<T> = Result<T, Usage>;

#[deriving(Show)]
struct Options {
    chance: int,
    exe: Option<Exe>,
}

#[deriving(Show)]
struct Exe {
    name: String,
    args: Box<Vec<String>>,
}

#[deriving(Show)]
enum Usage {
    Usage,
    Help,
    Version,
}

trait Uncons<'a, T> {
    fn uncons(&self) -> Option<(&'a T, &'a [T])>;
}

impl<'a, T> Uncons<'a, T> for &'a [T] {
    fn uncons(&self) -> Option<(&'a T, &'a [T])> {
        self.head().map(|x| (x, self.tail()))
    }
}

fn parse_args(args: &Vec<String>) -> ArgParse<Options> {
    args.as_slice().tail().uncons().map_or(Err(Usage), |(s, args)| {
        if ["--help".to_string(), "-h".to_string()].contains(s) {
            Err(Help)
        } else if ["--version".to_string(), "-v".to_string()].contains(s) {
            Err(Version)
        } else if ["--chance".to_string(), "-c".to_string()].contains(s) {
            args.uncons().map_or(Err(Usage), |(s, args)| {
                from_str::<int>(s.as_slice()).map_or(Err(Usage), |val| {
                    args.uncons().map_or(Ok(Options { chance: val, exe: None }), |(exe, args)| {
                        if "--".to_string() == exe.to_string() {
                            args.uncons().map_or(Ok(Options { chance: val, exe: None }), |(exe, args)| {
                                Ok(Options {
                                    chance: val,
                                    exe: Some(Exe {
                                        name: exe.to_string(),
                                        args: box std::vec::Vec::from_slice(args),
                                    })
                                })
                            })
                        } else {
                            Ok(Options {
                                chance: val,
                                exe: Some(Exe {
                                    name: exe.to_string(),
                                    args: box std::vec::Vec::from_slice(args),
                                })
                            })
                        }
                    })
                })
            })
        } else {
            Err(Usage)
        }
    })
}

fn whack(opts: &Options) -> bool {
    std::rand::task_rng().gen_range(0, 100) > std::cmp::min(std::cmp::max(opts.chance, 0), 100)
}

fn execvp_easy(name: &String, args: &Vec<String>) {
    let c_name: CString = name.to_c_str();
    let c_args: Vec<CString> = args.iter().map(|tmp| tmp.to_c_str()).collect();
    with_argv(&c_name, c_args.as_slice(), proc(c_argv) -> () unsafe {
        execvp(*c_argv, c_argv);
        fail!("executing {}: {}", name, std::os::last_os_error());
    });
}

fn with_argv<T>(prog: &CString, args: &[CString], f: proc(**libc::c_char) -> T) -> T {
    let mut ptrs: Vec<*libc::c_char> = Vec::with_capacity(args.len() + 1);

    ptrs.push(prog.with_ref(|buf| buf));
    ptrs.extend(args.iter().map(|tmp| tmp.with_ref(|buf| buf)));
    ptrs.push(ptr::null());

    f(ptrs.as_ptr())
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
