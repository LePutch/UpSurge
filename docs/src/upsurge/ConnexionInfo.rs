use regex::Regex;

#[derive(Clone, Debug)]
pub struct ConnexionInfo {
    user: String,
    ip: String,
    port: i32,
}

impl ConnexionInfo {
    pub fn new(user: String, ip: String, port: i32) -> ConnexionInfo {
        ConnexionInfo { user, ip, port }
    }
    fn _debug_dump(&self) {
        println!("{:?}", self);
    }
    pub fn _empty() -> ConnexionInfo {
        ConnexionInfo {
            user: "".to_string(),
            ip: "".to_string(),
            port: -1,
        }
    }

    pub fn ssh_format(&self) -> String {
        format!("ssh://{}@{}:{}", self.user, self.ip, self.port)
    }

    pub fn from_formatted(connexion: String) -> Result<Self, &'static str> {
        let re = Regex::new(r"^(?P<user>[^@]+)@(?P<ip>[^:]+):(?P<port>\d+)$").unwrap();
        
        if let Some(captures) = re.captures(&connexion) {
            let user = captures.name("user").unwrap().as_str().to_owned();
            let ip = captures.name("ip").unwrap().as_str().to_owned();
            let port = captures.name("port").unwrap().as_str().parse::<i32>().unwrap();
            Ok(ConnexionInfo::new(user, ip, port))
        } else {
            Err("invalid connexion format")
        }
    }
}
