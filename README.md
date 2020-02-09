# Rust Project Group C - Music Entertainment for Friends (MEFF)

A distributed key-value database storing audio files for sharing and streaming music.



Our database is distributed in an interconnected peer to peer network. Storage resources are shared between peers. Every user that
wants to be part of the network to access files must also provide storage to the database.

The graphic illustrates the structure of the network, which is fully meshed.
This is optimal for smaller communities that want to privately share their music through a decentralised system.

![kvd-rust](https://user-images.githubusercontent.com/12140441/72470038-8f9f3880-37e0-11ea-8175-ed0e9f52fc50.png)

One instance of the program is visualised by the dotted rectangle. The software itself is split between the database, which handles instructions from the
user and listens to updates from the network. The user interaction, which currently happens in a shell,
is decoupled from the database thread and could be exchanged with a GUI.

Users will be able to choose what to do with the music by either downloading it to their drive or streaming it through the network.

## Usage

There are two ways to use the MEFF-Application:

#### Shell-App

If you want to create a new peer network use the following command:

`cargo run <your name> <port>`

If you want to join an existing peer network, you need to know the IP address of one peer including the port.
Use following command to join:

`cargo run <your name> <port> <ip-address>`

The application can now be used with following commands: 

- `help` - get the instruction menu
- `status` - show current state of peer
- `push [mp3 name] [direction to mp3]` - add mp3 to database
- `get [mp3 name]` - download the mp3 file from database
- `remove [mp3 name]` - deletes mp3 file from database
- `exit` - exit network

#### GUI-App

To make the app easier to use we have created a GUI implemented with the gtk library.
Downloading, streaming or playing the music can now be controlled by a simple user interface.

Prerequisite: for using the GUI-App the gtk library must be installed

    brew install gtk-3


Similar to the shell app, the GUI can be started either by joining an existing network or creating a new network.

The interface is divided into two parts:
- The right side contains a list of your own songs. These can be deleted, played, paused and stopped. 
- On the left side, there are commands concerning the other peers in the network.
    - The download button saves a specific song from other peers in your database. 
    - The streaming button plays the song from others.
    

<img src="https://fabianfrey.de/meff.png" width="320" />


## Crates used:

    clap
    
    get_if_addrs
    
    serde
    
    serde_json
    
    local_ipaddress
    
    rand
    
    prettytable-rs
    
    colored
    
    log
   
    log4rs
    
    gtk
    
    gio
    
    gdk
    
    glib-sys 
    
    glib

## Participants
- Marcel Baur
- Fabian Frey
- Franziska Lang
- Elena Liebl

