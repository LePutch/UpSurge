pub mod cli;
pub mod interactiv;
pub mod test;
pub mod upsurge;
use crate::upsurge::ConnexionInfo::ConnexionInfo;
use crate::upsurge::ControlNetwork::ControlNetwork;
use futures::executor::block_on;
//use std::io::{self, Write};

#[tokio::main]
async fn main() {
    //block_on(runtest());
    //block_on(interactiv::interactiv::Interactiv::run());
    let args = std::env::args();
    if args.len() > 1 {
        block_on(cli::cli::run());
    } else {
        block_on(interactiv::interactiv::Interactiv::run());
    }
}

//main for tests in src.test.test.rs
async fn runtest() {
    let mut network = ControlNetwork::default();
    let conn_info = ConnexionInfo::new(
        "u872994712".to_string(),
        "185.224.138.127".to_string(),
        65002,
    ); //
    network.add_machine(conn_info);
    network.update_session(0).await;
    println!("{:?}", network.get_network_state().await.unwrap());
    let res = network.exec("echo hello", 0).await;
    match res {
        Ok(e) => println!("{}", e),
        Err(e) => println!("{}", e),
    }
    println!("{:?}", network.get_network_state().await.unwrap());
    network.close_all_session().await.unwrap();
    println!("{:?}", network.get_network_state().await.unwrap());
    network.remove_machine(0).await.unwrap();
    println!("{:?}", network.get_network_state().await.unwrap());
    /*
    test::test::test_add_machine().await;
    test::test::test_update_session().await;
    test::test::test_remove_machine().await;
    test::test::test_close_session().await;
    test::test::test_close_all_session().await;
    test::test::test_exec().await;
    */
}
