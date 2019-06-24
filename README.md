# libpnet-poc

## Windows:

1. make sure you have npcap installed
2. git clone https://github.com/nikita-skobov/libpnet-poc.git
3. cd libpnet-poc/
4. change line 56 in src/main.rs to track a specific IP. by default it uses an IP that points to a server I set up, but in the future that server might be down, so you'll want to change the IP in that case.
5. cargo run
6. The previous command will build the dependencies, and then run the PoC, but it will exit initially because you must specify which interface to listen to. It will print out the interfaces so you can then pick which interface you want to use the next time you run it
7. cargo run -- "<INTERFACE NAME>"
8. open wireshark, select the same interface. add an `ip.addr == <THE IP IN src/main.rs>` filter to only track data to/from a specific ip.
9. make a request to the ip in src/main.rs, if the ip in the source code is still up, you can request `http://3.220.46.57/` for a small, static html page, or `http://3.220.46.57/profile.jpg` for a 2.1MB image.

## Linux:

1. make sure you have libcap installed
2. git clone https://github.com/nikita-skobov/libpnet-poc.git
3. cd libpnet-poc/
4. change line 56 in src/main.rs to track a specific IP. by default it uses an IP that points to a server I set up, but in the future that server might be down, so you'll want to change the IP in that case.
5. cargo run
6. The previous command will build the dependencies, and then run the PoC, but it will exit initially because you must specify which interface to listen to. It will print out the interfaces so you can then pick which interface you want to use the next time you run it
7. cargo run -- "<INTERFACE NAME>"
8. open wireshark, select the same interface. add an `ip.addr == <THE IP IN src/main.rs>` filter to only track data to/from a specific ip.
9. make a request to the ip in src/main.rs, if the ip in the source code is still up, you can request `http://3.220.46.57/` for a small, static html page, or `http://3.220.46.57/profile.jpg` for a 2.1MB image.
