use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>
}

struct FnDesc {
    func: for <'r> fn(&Command<'r>),
    cmd: &'static str,
}

static BUILTINS: &'static [FnDesc] = &{
    [
        FnDesc {
            func: cmd_echo,
            cmd: "echo"
        }
    ]
};

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args.as_slice()[0]
    }
}

fn cmd_echo(command: &Command) {
    let mut echo_phrases = command.args.iter();
    echo_phrases.next(); // throw away path

    for phrase in echo_phrases {
        kprint!("{} ", phrase);
    }

    kprintln!();
}

fn func_lookup(path: &str) -> Result<for <'r> fn(&Command<'r>),()> {
    for f in BUILTINS {
        if path == f.cmd {
            return Ok(f.func);
        }
    }
    Err(())
}

fn ext_exec(path: &str) -> Result<(),()> {
    kprintln!("unknown command: ${}", path);
    Ok(())
}

fn exec(command: &Command) {
    let path = command.path();

    match func_lookup(path) {
        Err(_) => { ext_exec(path).expect("Failed external program execute"); },
        Ok(builtin_func) => { builtin_func(command); }
    };
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    loop {
        kprint!("\r\n{}", prefix);
        let mut b_in = CONSOLE.lock().read_byte();

        let mut str_buf = [0u8; 512];
        let mut str_vec = StackVec::new(&mut str_buf);
        loop {
            if b_in == b'\r' || b_in == b'\n' { break; }
            if b_in == 8 || b_in == 127 {
                if !str_vec.is_empty(){
                    str_vec.pop().expect("Invalid backspace");
                    CONSOLE.lock().write_byte(8);
                    CONSOLE.lock().write_byte(b' ');
                    CONSOLE.lock().write_byte(8);
                }
            } else {
                kprint!("{}", b_in as char);
                str_vec.push(b_in).expect("Buffer overflow!");
            }

            b_in = CONSOLE.lock().read_byte();
        }

        let mut args_buf: [&str; 64] = [""; 64];
        let command_str = ::core::str::from_utf8(str_vec.as_slice()).expect("Couldn't stringify");
        let command = Command::parse(command_str, &mut args_buf);

        kprintln!();
        match command {
            Err(_) => {kprintln!("Parse Error!!"); continue; },
            Ok(unwrapped_cmd) =>  {
                kprintln!("path: {}", unwrapped_cmd.path());
                exec(&unwrapped_cmd); }
        }

    }
}
