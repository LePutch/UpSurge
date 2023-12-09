use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::Path;

use crate::upsurge::ConnexionInfo::ConnexionInfo;
use crate::upsurge::ControlNetwork::ControlNetwork;
//use crate::upsurge::UpSurge::UpSurge;

fn treat_output(output: String) {
    write_output(output);
}

fn write_output(output: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("output.log")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", output) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn load_inputs<'a>(
    connexions_path: &str,
    commands_path: &str,
) -> (Vec<ConnexionInfo>, Vec<String>) {
    // Create the connexions vector and commands vector
    let mut connexions = Vec::new();
    let mut commands = Vec::new();

    if let Ok(lines) = read_lines(connexions_path.to_string()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(infos) = line {
                let mut split = infos.split_whitespace();
                let user = split.next().unwrap();
                let ip = split.next().unwrap();
                let port = split.next().unwrap().parse::<i32>().unwrap();
                let con = ConnexionInfo::new(user.to_string(), ip.to_string(), port);
                connexions.push(con);
            }
        }
    }
    if let Ok(lines) = read_lines(commands_path.to_string()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(command) = line {
                commands.push(command);
            }
        }
    }
    return (connexions, commands);
}

/*
pub async fn test_process_all_async(test_number: usize) {
    let connexions_path = "./tests_config/connexions/".to_string() + &test_number.to_string();
    let commands_path = "./tests_config/commands/".to_string() + &test_number.to_string();
    let (connexions, commands) = load_inputs(&connexions_path, &commands_path);

    let tt = UpSurge::default();

    let res = tt
        .process_all_async(&connexions, &commands, treat_output)
        .await;
    match res {
        Err(e) => println!("{}", e),
        _ => (),
    }

}
*/

//test ControlNetwork.rs, ControlNetwork section

pub async fn test_add_machine() {
    //addin two machines
    let mut network = ControlNetwork::default();
    let conn_info = ConnexionInfo::new("user1".to_string(), "127.0.0.1".to_string(), 22);
    let conn_info2 = ConnexionInfo::new("user2".to_string(), "127.0.0.;".to_string(), 22);
    network.add_machine(conn_info);
    network.add_machine(conn_info2);
    assert_eq!(network.get_connexions().len(), 2);
}

pub async fn test_update_session() {
    let mut network = ControlNetwork::default();
    let conn_info = ConnexionInfo::new(
        "u872994712".to_string(),
        "185.224.138.127".to_string(),
        65002,
    ); //
    network.add_machine(conn_info);
    network.update_session(0).await;
    assert!(network.get_connexions()[0].0.is_some());
}

pub async fn test_remove_machine() {
    let mut network = ControlNetwork::default();
    let conn_info = ConnexionInfo::new(
        "u872994712".to_string(),
        "185.224.138.127".to_string(),
        65002,
    ); //
    network.add_machine(conn_info);
    let res = network.remove_machine(0).await;
    match res {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
    assert_eq!(network.get_connexions().len(), 0);
}

pub async fn test_close_session() {
    let mut network = ControlNetwork::default();
    let conn_info = ConnexionInfo::new(
        "u872994712".to_string(),
        "185.224.138.127".to_string(),
        65002,
    ); //
    network.add_machine(conn_info);
    network.update_session(0).await;
    network.close_session(0).await.unwrap();
    assert!(network.get_connexions()[0].0.is_none());
}

pub async fn test_close_all_session() {
    let mut network = ControlNetwork::default();
    let conn_info1 = ConnexionInfo::new(
        "u872994712".to_string(),
        "185.224.138.127".to_string(),
        65002,
    ); //
    let conn_info2 = ConnexionInfo::new(
        "u872994712".to_string(),
        "185.224.138.127".to_string(),
        65002,
    ); //
    network.add_machine(conn_info1);
    network.add_machine(conn_info2);
    network.update_session(0).await;
    network.update_session(1).await;
    network.close_all_session().await.unwrap();
    assert!(network.get_connexions()[0].0.is_none());
    assert!(network.get_connexions()[1].0.is_none());
}

pub async fn test_exec() {
    let mut network = ControlNetwork::default();
    let conn_info = ConnexionInfo::new(
        "u872994712".to_string(),
        "185.224.138.127".to_string(),
        65002,
    ); //
    network.add_machine(conn_info);
    network.update_session(0).await;
    let res = network.exec("echo hello", 0).await;
    assert!(res.is_ok());
}
