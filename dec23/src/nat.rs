use std::{
    collections::HashMap,
    sync::mpsc::{channel, Sender},
    time::Duration,
};

use crate::nic::run_nic;
use crate::Packet;

pub fn run_nat(program: &String, stop_at_first: bool) -> i128 {
    let mut last_packet_to_255 = None;
    let mut last_y_to_0 = None;

    let (pkt_q_tx, pkt_q_rx) = channel::<Packet>();

    let nics = (0..50)
        .map(|id| (id, run_nic(id, &program, pkt_q_tx.clone())))
        .collect::<HashMap<i128, Sender<Packet>>>();

    loop {
        while let Some(p) = pkt_q_rx.recv_timeout(Duration::from_secs(1)).ok() {
            if p.addr == 255 {
                if stop_at_first {
                    return p.y;
                }
                println!("Storing ({},{})", p.x, p.y);
                last_packet_to_255 = Some(p);
            } else {
                if p.addr == 0 {
                    println!(
                        "About to send y={} to 0; previous was {:?}",
                        p.y, last_y_to_0
                    );
                    if last_y_to_0.map(|y| y == p.y).unwrap_or(false) {
                        println!("Repeated y value!");
                        return p.y;
                    }
                    println!("Sending y={} to 0", p.y);
                    last_y_to_0 = Some(p.y);
                }
                nics.get(&p.addr).unwrap().send(p).unwrap();
            }
        }

        // the loop above should empty the packet queue; whenever we get here, all nics are idling
        assert!(
            last_packet_to_255.is_some(),
            "all idling, but no packet sent to 255 yet!"
        );
        let p = last_packet_to_255.unwrap();
        println!(
            "About to send y={} to 0; previous was {:?}",
            p.y, last_y_to_0
        );
        if last_y_to_0.map(|y| y == p.y).unwrap_or(false) {
            println!("Repeated y value!");
            return p.y;
        }
        println!("Sending y={} to 0", p.y);
        last_y_to_0 = Some(p.y);
        nics.get(&0).unwrap().send(p).unwrap();
    }
}
