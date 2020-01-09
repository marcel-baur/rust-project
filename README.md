# Rust Project Group C

A distributed key-value database storing audio files for sharing and streaming music

## How to build project

If you want create a new peer network use the following command:

`cargo run <your name> <port>`

If you want to join an existing peer network, you need to know the IP address of one peer including the port.
Use following command to join:

`cargo run <your name> <port> <ip-address>`

## Usage
- `help` - get the instruction menu 
- `push [mp3 name] [direction to mp3]` - add mp3 to database
- `get [mp3 name]` - get mp3 file from database
- `stream [mp3 name]` - get mp3 stream from database
- `remove [mp3 name]` - deletes mp3 file from database
- `exit - exit network`

## What is already working?
- Creates a peer when starting programm
- Peer creates network if needed
- Peer joins network if network exists
- Handshake (exchanging network table) between peers
- Reading mp3 file and convert it to byte array
- Sending mp3 files through network and saving to database
- Find the peer which stores a certain mp3 file

## What are the next steps?
- Bugfixing: Redundancy
- Proper error handling
- Heartbeat: Checking if peers are still online
- Streaming

## Participants
- Marcel Baur
- Fabian Frey
- Franziska Lang
- Elena Liebl

