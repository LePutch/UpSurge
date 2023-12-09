use std::io;

use crate::upsurge::ConnexionInfo::ConnexionInfo;
use openssh::{KnownHosts, Session};

pub struct Requester;

// Implementation of async Requester
impl Requester {
    pub async fn connect(infos: &ConnexionInfo) -> Result<Session, io::Error> {
        let res = Session::connect(infos.ssh_format(), KnownHosts::Strict).await;
        match res {
            Err(_e) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Session creation failed",
                ))
            }
            Ok(sess) => return Ok(sess),
        }
    }

    pub async fn make_request(sess: &Session, command: &str) -> Result<String, io::Error> {
        let out = sess.shell(command).output().await;
        match out {
            Err(_e) => return Err(io::Error::new(io::ErrorKind::Other, "No session")),
            Ok(outt) => return Ok(String::from_utf8(outt.stdout).unwrap()),
        }
    }

    pub async fn close(sess: Session) -> Result<(), io::Error> {
        sess.close()
            .await
            .map_err(|_e| io::Error::new(io::ErrorKind::Other, "Error while closing session"))?;
        Ok(())
    }
}
