#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::create_md5;
    fn create_md5_test() -> String {
        let opt = create_md5::create_md5::Opt{
            input: PathBuf::from("src/main.rs"),
            output: "l".to_string(),
            t: "str".to_string(),
        };
        let res = create_md5::create_md5::create_md5(opt).unwrap();
        return res
    }

    #[test]
    fn it_works() {
        assert_eq!(create_md5_test(), "639fbc4ef05b315af92b4d836c31b023".to_string());
    }
}

pub mod proxy {
    extern crate yaml_rust;
    use yaml_rust::{YamlLoader, YamlEmitter};
    use structopt::StructOpt;
    use exitfailure::ExitFailure;
    use std::path::{Path, PathBuf};
    use std::io::{BufReader, Read};
    use std::fs::File;
    use failure::{Error, ResultExt, ensure};

    use std::io::prelude::*;
    use std::io::BufWriter;
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    #[derive(StructOpt)]
    pub enum Command {
        #[structopt(name = "server")]
        Server(Opt),

        #[structopt(name = "client")]
        Client(Opt),
    }

    #[derive(StructOpt)]
    pub struct Opt {
        /// Input string or file path
        #[structopt(short, long, parse(from_os_str), default_value = "./conf.yaml")]
        pub config: PathBuf,
    }

    #[derive(StructOpt)]
    #[structopt(name = "classify")]
    pub struct ApplicationArguments {
        #[structopt(subcommand)]
        pub command: Command,
    }

    pub fn server(opt: Opt) -> Result<String, ExitFailure> {
        let info = read_file(opt.config).unwrap();
        let docs = YamlLoader::load_from_str(info.as_str()).unwrap();
        let doc = &docs[0];
        let host = doc["host"].as_str().unwrap();
        let port = doc["port"].as_i64().unwrap();
        let addr = format!("{}:{}", host, port);
        server_socks5(addr);
        return Ok("server".to_string())
    }

    pub fn client(opt: Opt) -> Result<String, ExitFailure> {
        let info = read_file(opt.config).unwrap();
        let docs = YamlLoader::load_from_str(info.as_str()).unwrap();
        let doc = &docs[0];
        let host = doc["host"].as_str().unwrap();
        let port = doc["port"].as_i64().unwrap();
        let addr = format!("{}:{}", host, port);
        client_socks5(addr);
        Ok("client".to_string())
    }

    fn server_socks5(addr: String) {
        let l = TcpListener::bind(addr).unwrap();
        for stream in l.incoming() {
            thread::spawn(move || {
                let stream = stream.unwrap();
                let reader = BufReader::new(&stream);
                let mut writer = BufWriter::new(&stream);
                for line in reader.lines() {
                    let line = line.unwrap();
                    println!("{}", line);
                    if line == "ping".to_string() {
                        writer.write_all(b"pong\n").unwrap();
                        writer.flush().unwrap();
                    }
                }
            });
        }
    }

    fn client_socks5(addr: String) {
        let stream = TcpStream::connect(addr).unwrap();
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        writer.write_all(b"ping\n").unwrap();
        writer.flush().unwrap();
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        println!("{}", line);
    }

    fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
        let path = path.as_ref();
        ensure!(
        path.exists() && path.is_file(),
        "Path {:?} is not a file!",
        path
    );
        let file = File::open(path).with_context(|_| format!("Could not open file {:?}", path))?;
        let mut file = BufReader::new(file);
        let mut result = String::new();
        file.read_to_string(&mut result)
            .with_context(|_| format!("Could not read file {:?}", path))?;

        Ok(result)
    }
}



