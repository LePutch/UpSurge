use crate::upsurge::Config::Config;
use crate::upsurge::SshRequester::Requester;
use crate::upsurge::ConnexionInfo::ConnexionInfo;
use futures::future::join_all;
use std::io;

#[derive(Default)]
pub struct SSHUpSurge {
    config: Config,
}

impl SSHUpSurge {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

// UpSurge with async functions, not threaded, SSH
impl SSHUpSurge {
    pub async fn process(
        &self,
        commands: Vec<String>,
        connexion: &ConnexionInfo,
        callback: fn(String) -> (),
    ) -> Result<(), io::Error> {
        
        let sess =
        match Requester::connect(&connexion).await {
            Ok(s) => s,
            Err(e) => {return Err(e);}
        };

        for command in commands {
            let output: Result<String, std::io::Error> = Requester::make_request(&sess, &command).await;
            match output {
                Err(e) => return Err(e),
                Ok(out) => {
                    callback(format!("{:?} > {} > {}", connexion, command, out));
                }
            }
        }

        let res_close = Requester::close(sess).await;
        match res_close {
            Err(e) => return Err(e),
            _ => return Ok(()),
        }
    }

    // Process all the connexions in the array with the given command
    // NO THREADS
    // Returns time to process all the connexions
    pub async fn process_all(
        &self,
        connexions: &Vec<ConnexionInfo>,
        commands: &Vec<String>,
        callback: fn(String) -> (),
    ) -> Result<(), io::Error> {
        let start_time = std::time::Instant::now();

        let mut futures = Vec::new();

        for connexion in connexions.iter() {
            let future = self.process(commands.clone(), &connexion, callback);
            futures.push(future);
        }

        // Wait for all the futures to finish
        join_all(futures).await;

        let end_time = std::time::Instant::now();
        let duration = end_time.duration_since(start_time);
        println!(
            "Time elapsed in process_all_async() for {} connexions is: {:?}",
            connexions.len(),
            duration
        );
        Ok(())
    }
}
