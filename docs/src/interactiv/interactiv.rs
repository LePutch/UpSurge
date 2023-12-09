//interactive module for add, remove, connect, send command, etc...

use crate::upsurge::ConnexionInfo::ConnexionInfo;
use crate::upsurge::ControlNetwork::ControlNetwork;
use std::io::{self, Write};

pub struct Interactiv;

impl Interactiv {
    pub async fn run() {
        // Run interactively
        let mut network = ControlNetwork::new(Vec::new());
        println!("Enter a command or '/help' for help\n");
        Self::loop_parse(&mut network).await;
    }

    pub async fn loop_parse(network: &mut ControlNetwork) {
        loop {
            print!("int $ ");
            let inputs = Self::parsing_line();
            //declare a resultat variable to return
            let mut res = Ok(());
            match inputs[0].as_str() {
                "/help" => Self::printhelp(),
                "/network" => Self::show_network(network, inputs).await,
                "/exit" => break,

                "/add" => res = Self::add_machine(network, inputs), //println!("Add machine"),

                "/remove" => res = Self::remove_machine(network, inputs).await, //println!("Remove machine"),
                "/connect" => res = Self::connect_machine(network, inputs).await, //println!("Connect to a machine"),
                "/disconnect" => res = Self::disconnect_machine(network, inputs).await, //println!("Disconnect from a machine"),

                "/exec" => res = Self::exec_command(network, inputs).await, //println!("Execute a command on a machine"),

                _ => println!("Unknown command\n"),
            }

            //check if there is an error
            if res.is_err() {
                println!("Error: {}", res.err().unwrap());
            }
        }
    }

    fn printhelp() {
        println!("/help - Show this help");
        println!("/exit - Exit the shell\n");
        println!(" --- Network commands --- ");
        println!("/network - Show network info");
        println!("/add - Add a machine to the network : /add <user> <ip> <port>");
        println!("/remove - Remove a machine from the network: /remove <id>");
        println!("/connect - Connect to a machine : /connect <id>");
        println!("/disconnect - Disconnect from a machine : /disconnect <id>");
        println!("/exec - Execute a command on a machine : /exec <id> <command>");
    }

    pub fn parsing_line() -> Vec<String> {
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let inputs = input.split_whitespace();

        let res: Vec<String> = inputs.map(|s| s.to_string()).collect();
        return res;
    }

    pub fn add_machine(network: &mut ControlNetwork, inputs: Vec<String>) -> Result<(), io::Error> {
        // print!("Currently in add machine with {:?} arguments\n", inputs);
        //retrieve the arguments
        let user = inputs[1].clone();
        let ip = inputs[2].clone();
        let port = inputs[3].clone();
        let int_port = port.parse::<i32>();
        let prt = match int_port {
            Ok(int_port) => int_port,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Port not valid\n")),
        };
        let conn_info = ConnexionInfo::new(user, ip, prt);
        network.add_machine(conn_info);
        return Ok(());
    }

    pub async fn remove_machine(
        network: &mut ControlNetwork,
        inputs: Vec<String>,
    ) -> Result<(), io::Error> {
        // print!("Currently in remove machine with {:?} arguments\n", inputs);
        //retrieve the arguments (id)
        let id = inputs[1].clone();
        let int_id = id.parse::<usize>();
        let id = match int_id {
            Ok(int_id) => int_id,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Id not valid\n")),
        };
        return network.remove_machine(id).await;
    }

    pub async fn connect_machine(
        network: &mut ControlNetwork,
        inputs: Vec<String>,
    ) -> Result<(), io::Error> {
        // print!("Currently in connect machine with {:?} arguments\n", inputs);
        //retrieve the arguments (id)
        let id = inputs[1].clone();
        let int_id = id.parse::<usize>();
        let id = match int_id {
            Ok(int_id) => int_id,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Id not valid\n")),
        };
        network.update_session(id).await;
        return Ok(());
    }

    pub async fn disconnect_machine(
        network: &mut ControlNetwork,
        inputs: Vec<String>,
    ) -> Result<(), io::Error> {
        // print!(
        //     "Currently in disconnect machine with {:?} arguments\n",
        //     inputs
        // );
        //retrieve the arguments (id)
        let id = inputs[1].clone();
        let int_id = id.parse::<usize>();
        let id = match int_id {
            Ok(int_id) => int_id,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Id not valid\n")),
        };

        return network.close_session(id).await;
        //veridy the machine is connected
        /*
        let connected = network.is_connected(id).await;
        match connected {
            Ok(_) => {
                network.close_session(id).await;
                return Ok(());
            },
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "not connected\n",
                ))
            }
        }
        */
    }

    pub async fn show_network(network: &ControlNetwork, inputs: Vec<String>) {
        // print!("Currently in show network with {:?} arguments\n", inputs);
        let states = network.get_network_state().await;
        //print ,etwork stats like this for each node:
        //id: 0, user@ip:port, Alive/dead

        //for looop on states.length
        match states {
            Ok(states) => {
                for i in 0..states.len() {
                    println!("id: {}, {}", i, states[i]);
                }
            }
            Err(_) => {
                println!("Error while getting network state");
            }
        }
    }

    pub async fn exec_command(
        network: &mut ControlNetwork,
        inputs: Vec<String>,
    ) -> Result<(), io::Error> {
        // print!("Currently in exec command with {:?} arguments\n", inputs);
        //retrieve the arguments (id)
        let id = inputs[1].clone();
        let int_id = id.parse::<usize>();
        let id = match int_id {
            Ok(int_id) => int_id,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Id not valid\n")),
        };
        //join inputs slice [2::end] with ' ' to get the command
        let command = &inputs[2..].join(" ");
        let res = network.exec(command, id).await;
        match res {
            Ok(_) => {
                //println!("{}", res);
                return Ok(());
            }
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Error while executing command\n",
                ))
            }
        }
    }

    /*
    fn from_formatted(&self, format: &str) -> Result<Arc<dyn ClientData>, Error> {
        let re = Regex::new(r"(?P<user>\S+)@(?P<ip>\S+):(?P<port>\d+)");
        match re {
            Ok(reg) => match reg.captures(format) {},
        }
    }
    */
}
