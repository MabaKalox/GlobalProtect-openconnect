use anyhow::{ensure, Context, Result};
use gpapi::GP_CALLBACK_PORT_FILENAME;
use std::{env, env::temp_dir, io::Write, net::TcpStream};

// TODO - use from crate
fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  ensure!(args.len() == 2, "usage: {} token", args[0]);

  let port_file = temp_dir().join(GP_CALLBACK_PORT_FILENAME);

  let port: u16 = std::fs::read_to_string(&port_file)
    .context(format!("reading {}", port_file.display()))?
    .parse()
    .context("parsing port number")?;

  let mut stream = TcpStream::connect(("127.0.0.1", port)).context("connecting to token socket")?;

  stream
    .write_all(args[1].as_bytes())
    .context("writing token to socket")?;

  Ok(())
}
