use std::{
    process::Command,
    net::{SocketAddr, UdpSocket},
    time::Duration,
    thread::sleep,
};

/// Example of a process pair.
///
/// The program knows of two valid addresses, and will try to aquire one.
/// If the program manages to bind to an address, it can assume that the
/// other address is its peer.
/// A newly spawned process will assume the role of the backup process, and
/// will try to receive and back up data from the primary process.
/// If this fails, it will assume the role of the primary process and spawn
/// a backup.
/// If the backup dies, it will spawn a new backup.
fn main() {
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 3400)),
        SocketAddr::from(([127, 0, 0, 1], 3401)),
    ];
    let socket = UdpSocket::bind(&addrs[..]).expect("couldn't bind to address");
    socket.set_read_timeout(Some(Duration::from_millis(1000))).expect("couldn't set timeout");

    let peer_addr = match socket.local_addr().unwrap() == addrs[0] {
        true => addrs[1],
        false => addrs[0],
    };

    socket.connect(peer_addr).expect("couldn't connect to peer");

    let mut buffer = [0; 1];
    let mut i: u8 = 0;

    println!("-------- BACKUP PROCESS ---------");
    loop {
        match socket.recv(&mut buffer) {
            Ok(_) => {
                i = buffer[0];
            },
            Err(_) => {
                println!("Counting from {}", i);
                Command::new("gnome-terminal")
                    .arg("--")
                    .arg("./phoenix")
                    .spawn()
                    .expect("failed to start backup process");
                socket.connect(peer_addr).expect("couldn't connect to peer");
                loop {
                    if let Err(_) = socket.send(&[i]) {
                        Command::new("gnome-terminal")
                            .arg("--")
                            .arg("./phoenix")
                            .spawn()
                            .expect("failed to start backup process");
                        socket.connect(peer_addr).expect("couldn't connect to peer");
                    }
                    println!("{}", i);
                    i = i + 1;
                    sleep(Duration::from_millis(500));
                }
            },
        }
    }
}
