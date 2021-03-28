use std::sync::mpsc::{channel, Sender, TryRecvError};

use super::Packet;
use intcode::Builder;

///
/// run_nic runs a NIC node with the provided id
///
/// Arguments:
/// * `id`: the id of this NIC
/// * `program`: the NIC software
/// * `pkt_out_tx`: a `Sender<Packet>` to which output packages will be sent
///
/// Returns: a `Sender<Packet>` to which to send packages as input to this NIC node
/// */
pub fn run_nic(id: i128, program: &String, pkt_out_tx: Sender<Packet>) -> Sender<Packet> {
    let (computer_in_tx, computer_out_rx) = Builder::new()
        .parse(&program)
        .input_hook(vec![104, INPUT_SENTINEL])
        .silent()
        .run();

    let (pkt_in_tx, pkt_in_rx) = channel::<Packet>();

    const INPUT_SENTINEL: i128 = -17;

    // first the NIC reads its id
    // this will also trigger the input hook; just ignore and send the id
    let _ = computer_out_rx.recv().unwrap();
    computer_in_tx.send(id).unwrap();

    std::thread::spawn(move || {
        loop {
            // the output signal is either a packet address, or the sentinel used to request input
            let addr = computer_out_rx.recv().unwrap();
            if addr == INPUT_SENTINEL {
                // if it was the input sentinel, the computer is asking for the next package
                match pkt_in_rx.try_recv() {
                    Ok(pkt) => {
                        assert!(pkt.addr == id || (pkt.addr == 255 && id == 0));
                        computer_in_tx.send(pkt.x).unwrap();
                        assert_eq!(INPUT_SENTINEL, computer_out_rx.recv().unwrap());
                        computer_in_tx.send(pkt.y).unwrap();
                    }
                    Err(TryRecvError::Empty) => computer_in_tx.send(-1).unwrap(),
                    Err(TryRecvError::Disconnected) => break,
                }
            } else {
                // if it wasn't the input sentinel, it was the address of an output packet
                // read x and y, then send the packet to the queue
                let x = computer_out_rx.recv().unwrap();
                let y = computer_out_rx.recv().unwrap();
                pkt_out_tx.send(Packet { addr, x, y }).unwrap();
            }
        }
    });

    pkt_in_tx
}
