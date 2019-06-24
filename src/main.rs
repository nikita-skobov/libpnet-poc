use pnet::datalink::{self, NetworkInterface, Config, ChannelType};

use std::env;
use std::process;
use std::io::{self, Write};

fn main() {
    use pnet::datalink::Channel::Ethernet;

    for interface in datalink::interfaces() {
        println!("{}", interface);
    }

    let iface_name = match env::args().nth(1) {
        Some(n) => n,
        None => {
            writeln!(io::stderr(), "USAGE: packetdump <NETWORK INTERFACE>").unwrap();
            process::exit(1);
        }
    };
    let interface_names_match = |iface: &NetworkInterface| iface.name == iface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap();

    let custom_config = Config {
      read_buffer_size: 32768 * 4,
      write_buffer_size: 32768 * 4,
      read_timeout: None, // linux only
      write_timeout: None, // linux only
      bpf_fd_attempts: 1000, // linux only
      linux_fanout: None, // linux only
      channel_type: ChannelType::Layer2,
    };

    // let default_config = Default::default();

    // Create a channel to receive on
    let (_, mut rx) = match datalink::channel(&interface, custom_config) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("packetdump: unhandled channel type: {}"),
        Err(e) => panic!("packetdump: unable to create channel: {}", e),
    };

    let mut packet_count = 0;
    let mut total_byte_count = 0;

    loop {
        match rx.next() {
            Ok(packet) => {
              let some_ip = [3, 220, 46, 57];
              let mut match_index = 0;
              let max_match_index = 3;

              for byte in packet {
                if *byte == some_ip[match_index] {
                  match_index += 1;
                } else {
                  match_index = 0;
                }

                if match_index > max_match_index {
                  packet_count += 1;
                  total_byte_count += packet.len();
                  println!("{:?} - NUM_PACKETS: {}, PACKET_LEN: {}, TOTAL_BYTES: {}", some_ip, packet_count, packet.len(), total_byte_count);
                  break;
                }
              }
            }
            Err(e) => panic!("packetdump: unable to receive packet: {}", e),
        }
    }
}
