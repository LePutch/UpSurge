use crate::upsurge::ConnexionInfo::ConnexionInfo;
use crate::upsurge::SshRequester;
use openssh::Session;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

/*
    The machine IDs are represented by their index in the vector
*/
#[derive(Default)]
pub struct ControlNetwork {
    connexions: Vec<(Option<Session>, ConnexionInfo)>,
    log_files: HashMap<usize, File>,
}

impl ControlNetwork {
    pub fn new(infos: Vec<(Option<Session>, ConnexionInfo)>) -> Self {
        ControlNetwork {
            connexions: infos,
            log_files: HashMap::new(),
        }
    }

    pub fn add_machine(&mut self, con: ConnexionInfo) {
        // add a machine to the network by adding a connexion info into the vector and put None to the session
        self.connexions.push((None, con));
        // create a file with
    }

    pub async fn update_session(&mut self, id: usize) {
        // update the session of the machine with the given id
        let session = &self.connexions[id].0;
        let con = self.connexions[id].1.clone();
        if session.is_none() {
            // if the session is null, we create a new one with SSHRequester
            let new_session = SshRequester::Requester::connect(&con).await;
            match new_session {
                Ok(sess) => self.connexions[id].0 = Some(sess),
                Err(_e) => {
                    println!("Can't connect to the machine id={}", id);
                }
            }
        } else {
            // if the session is not null, we check if it is still connected
            let res = self.is_connected(id).await;
            match res {
                Ok(_e) => (),
                Err(_e) => {
                    // if the session is not connected, we create a new one
                    let new_session = SshRequester::Requester::connect(&con).await;
                    match new_session {
                        Ok(sess) => self.connexions[id].0 = Some(sess),
                        Err(_e) => {
                            println!("Can't connect to the machine id={}", id);
                        }
                    }
                }
            }
        }
    }

    pub async fn remove_machine(&mut self, id: usize) -> Result<(), io::Error> {
        //checking if the id is in the vector
        if id >= self.connexions.len() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "id isn't in the vector of connexions",
            ));
        }
        // remove a machine from the network by removing the connexion info from the vector
        let connected = self.is_connected(id).await;
        match connected {
            Ok(_e) => {
                let res = self.close_session(id).await;
                match res {
                    Ok(_e) => {
                        self.connexions.remove(id);
                        return Ok(());
                    }
                    Err(_e) => {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "Session is not connected",
                        ))
                    }
                }
            }

            Err(_e) => {
                self.connexions.remove(id);

                return Ok(());
            }
        }
        /*
        let res = self.close_session(id).await;
        match res {
            Ok(_e) => {
                self.connexions.remove(id);
                Ok(())
            }
            Err(_e) => Err(io::Error::new(
                io::ErrorKind::Other,
                "Session is not connected",
            )),
        }
        */
    }

    pub async fn close_session(&mut self, id: usize) -> Result<(), io::Error> {
        // close the session of the machine with the given id
        let session = &mut self.connexions[id].0;
        match session.take() {
            None => Err(io::Error::new(io::ErrorKind::Other, "No Session")),
            Some(sess) => {
                let res = SshRequester::Requester::close(sess).await;
                match res {
                    Ok(_e) => Ok(()),
                    Err(_e) => {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "Session is not connected",
                        ))
                    }
                }
            }
        }
    }

    pub async fn close_all_session(&mut self) -> Result<(), io::Error> {
        // close all the sessions of the network
        for i in 0..self.connexions.len() {
            let res = self.close_session(i).await;
            match res {
                Ok(_) => (),
                Err(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Can't close all sessions",
                    ))
                }
            }
        }
        Ok(())
    }

    pub async fn is_connected(&self, id: usize) -> Result<(), io::Error> {
        let session = &self.connexions[id].0;
        //checking the session with check method
        match session {
            None => return Err(io::Error::new(io::ErrorKind::Other, "No Session")),
            Some(sess) => {
                let res = sess.check().await;
                match res {
                    Ok(_e) => Ok(()),
                    Err(_e) => {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "Session is not connected",
                        ))
                    }
                }
            }
        }
    }

    pub async fn exec(&mut self, command: &str, id: usize) -> Result<String, io::Error> {
        // Get the session for the given machine ID
        let session = &self.connexions[id].0;
        match session {
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "No Session in exec function",
                ))
            }
            Some(sess) => {
                // Get the log file for this machine
                let log_file = self.log_files.entry(id).or_insert_with(|| {
                    let mut file_path = PathBuf::new();
                    file_path.push("feedbacks");
                    std::fs::create_dir_all(&file_path).unwrap(); // Create the directory if it doesn't exist
                    file_path.push(format!("machine_{}.log", id));
                    File::create(file_path).unwrap()
                });

                // Print the machine connexion info
                writeln!(log_file, "    machine : {:?}", &self.connexions[id].1).unwrap();
                // Write the command to the log file
                writeln!(log_file, "    command : {}", command).unwrap();

                // Execute the command using SSH
                let res = SshRequester::Requester::make_request(sess, &command).await;
                match res {
                    Ok(e) => {
                        writeln!(log_file, ">>>>>>>>\n{}", e).unwrap();
                        Ok(e)
                    }
                    Err(_e) => {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "Error while executing the command in exec function of ControlNetwork",
                        ))
                    }
                }
            }
        }
    }

    /*
        Generates a recap on the network state :
        "
        - user@ip:port > Alive
        - user@ip:port > Dead
        ...
        "
    */
    pub async fn get_network_state(&self) -> Result<Vec<String>, io::Error> {
        let mut states = Vec::new();
        for i in 0..self.connexions.len() {
            let l = format!("{:?}", self.connexions.get(i).unwrap().1);
            let res = self.is_connected(i).await;
            match res {
                Ok(_) => states.push(format!("{l} > Alive")),
                Err(_) => states.push(format!("{l} > Dead")),
            }
        }
        Ok(states)
    }

    pub fn get_connexions(&self) -> &Vec<(Option<Session>, ConnexionInfo)> {
        &self.connexions
    }

    pub fn get_connexions_len(&self) -> usize {
        self.connexions.len()
    }
}
