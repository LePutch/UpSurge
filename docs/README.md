# UpSurge User Guide

## 1. Introduction

Based on TakTuk : https://taktuk.gitlabpages.inria.fr/documentation.html,

UpSurge is a tool that allows you to execute commands on multiple remote machines.

## 2. Running UpSurge

UpSurge can be run in two different modes : bash mode and interactive mode. In both ways, it's working with a control network that manages the remote machines.

### 2.1. Bash mode

Bash mode is the default mode of UpSurge. It allows you to execute a command on multiple remote machines with a single command line. 

Here is the list of options available in bash mode :


```rust
Options:

-c, --connexion USER@IP:PORT - connect to a remote machine defined by USER@IP:PORT

-C, --connexions “FILE” - connect to all machines in FILE, USER@IP:PORT per line

-k, --command “COMMAND” - execute COMMAND on remote machine

-K, --commands “FILE”  - execute all commands in FILE, one command per line

-h, --help - printing commands

```

You can then run the following command to execute the command `ls` on the remote machine `
` :

```bash
$ cargo run -- -c remoteMachine@165.165.165.165:8080 -k "ls"
```

Or for multiple commands on multiple machines :


```bash
$ cargo run -- -C machines.txt -K commands.txt
```

With `machines.txt` containing :

```bash
remoteMachine1@165.165.165.165:8080
remoteMachine2@165.165.165.165:8080
remoteMachine3@165.165.165.165:8080
remoteMachine4@165.165.165.165:8080
...
```

And `commands.txt` containing :

```bash
ls
mkdir test
cd test
touch test.txt
```

### 2.2. Interactive mode

Interactive mode is a more advanced mode of UpSurge. This mode modifies the way you interact with the control network. It allows you to create groups of machines and execute commands on them.

Here is the list of commands available in interactive mode :

Basic commands —
```bash

/exit - Exit the shell
/help - Print commands
```

Network commands —
```bash
/network - Show network info
/add <user> <ip> <port> - Add a machine to the network  
/remove <id> - Remove a machine from the network 
/connect <id> - Connect to a machine 
/disconnect <id>- Disconnect from a machine 
/exec <id> <command> - Execute a command on a machine 
```

Group commands —

```bash
/create_group <name : 'g' + 'name' <id1> <id2>
/remove_group <name>

/exec 0 'echo hello' : single machine command
/exec [0,1,2] 'echo hello' : multiple machine command
/exec g MyGroup 'echo hello' : group command
```
