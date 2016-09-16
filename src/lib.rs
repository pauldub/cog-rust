extern crate rustc_serialize;

use std::env;
use std::str::FromStr;
use std::collections::HashMap;

use rustc_serialize::json;

pub type Args = Vec<String>;
pub type Opts = HashMap<String, String>;

// TODO(pauldub): Maybe this should be named `environment` as in the command's environment?
#[derive(Debug)]
pub struct Command {
  bundle: String,
  name: String,
  args: Args,
  opts: Opts,
}

#[derive(PartialEq, Debug)]
pub enum Error {
  InvalidBundle,
  VarError(env::VarError)
}

impl From<env::VarError> for Error {
  fn from(e: env::VarError) -> Error {
    Error::VarError(e)
  }
}

impl Command {
  pub fn new() -> Result<Self, Error> {
    Ok(Command{
      bundle: env::var("COG_BUNDLE").expect("missing bundle"),
      name: env::var("COG_COMMAND").expect("missing command name"),
      args: Command::parse_args(),
      opts: Command::parse_opts(),
    })
  }

  fn parse_args() -> Vec<String> {
    let argc = env::var_os("COG_ARGC");

    if argc.is_none() {
      return vec![];
    }

    let argc = match usize::from_str(argc.unwrap().to_str().unwrap_or_default()) {
      Err(_) => return vec![],
      Ok(n) => n
    };

    let mut args = vec![];

    for i in 0..argc {
      let key = format!("COG_ARGV_{}", i);
      match env::var(key) {
        Err(_) => return vec![],
        Ok(v) => args.push(v)
      };
    }

    args
  }

  fn parse_opts() -> HashMap<String, String> {
    let opts = env::var("COG_OPTS");
    if opts.is_err() {
      return HashMap::new()
    }

    let mut options = HashMap::<String, String>::new();
    for opt in opts.unwrap().split(",") {
      let key = format!("COG_OPT_{}", opt.to_uppercase());
      match env::var(key) {
        Err(_) => return HashMap::new(),
        Ok(v) => options.insert(String::from(opt), v)
      };
    }

    options
  }
}

pub struct Bundle<'a> {
  name: String,
  commands: HashMap<String, Box<&'a Fn(Args, Opts)>>,
}

impl<'a> Bundle<'a> {
  pub fn new(name: &str) -> Self {
    Bundle{
      name: String::from(name),
      commands: HashMap::new(),
    }
  }

  pub fn command(&mut self, name: &str, f: &'a Fn(Args, Opts)) -> &mut Self {
    self.commands.insert(String::from(name), Box::new(f));
    self
  }

  pub fn run(&self) -> Result<(), Error> {
    let cmd = Command::new().expect("failed to parse cog command");
    if cmd.bundle != self.name {
      return Err(Error::InvalidBundle)
    }


    let f = self.commands.get(&cmd.name).expect("command not found");
    f(cmd.args, cmd.opts);
    Ok(())
  }
}

pub fn write(msg: &str) {
  println!("{}", msg)
}

pub fn json<T>(msg: &T)
  where T: rustc_serialize::Encodable {
  write("JSON");
  write(&json::encode(msg).unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
