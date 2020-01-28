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


## How to build the project

If you want create a new peer network use the following command:

`cargo run <your name> <port>`

If you want to join an existing peer network, you need to know the IP address of one peer including the port.
Use following command to join:

`cargo run <your name> <port> <ip-address>`

## Usage
- `help` - get the instruction menu
- `push [mp3 name] [direction to mp3]` - add mp3 to database
- `get [mp3 name]` - download the mp3 file from database
- `play [mp3 name]` - download the mp3 file from the database and play it
- `stream [mp3 name]` - get mp3 stream from database. The file won’t be downloaded, only streamed.
- `remove [mp3 name]` - deletes mp3 file from database
- `exit` - exit network

## What is already working?
- Creates a peer when starting program
- Peer creates network if needed
- Peer joins network if network exists
- Handshake (exchanging network table) between peers
- Reading mp3 file and convert it to byte array
- Sending mp3 files through network and saving to database
- Find the peer which stores a certain mp3 file
- Heartbeat
- Redundancy for saving files
- Get a file and play it
- Error handling has greatly improved

## What are the next steps?
- Some more error handling
- Load management
- Streaming
- Maybe a GUI
- Debugging and testing

## Participants
- Marcel Baur
- Fabian Frey
- Franziska Lang
- Elena Liebl

