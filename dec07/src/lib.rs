use intcode::Builder;
use permutohedron::Heap;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;

pub fn solve_a(input: &String) -> i128 {
    find_best_config([0, 1, 2, 3, 4], |s| run_without_feedback(&input, s))
}

pub fn solve_b(input: &String) -> i128 {
    find_best_config([5, 6, 7, 8, 9], |s| run_with_feedback(&input, s))
}

fn find_best_config<F: Fn([i128; 5]) -> i128>(mut settings: [i128; 5], runner: F) -> i128 {
    let permutations = Heap::new(&mut settings);

    let mut max_output = 0;
    for permutation in permutations {
        let output = runner(permutation);
        if output > max_output {
            max_output = output
        }
    }

    max_output
}

fn run_without_feedback(input: &String, settings: [i128; 5]) -> i128 {
    let (i, o, q) = create_amp_array(input, settings);
    i.send(0).expect("Failed to send initialization message.");

    for _ in q.iter() {
        // wait for the system to complete
    }

    if let Some(signal) = o.iter().next() {
        signal
    } else {
        panic!("Failed to get a final result from settings {:?}", settings);
    }
}

fn run_with_feedback(input: &String, settings: [i128; 5]) -> i128 {
    let (i, o, q) = create_amp_array(input, settings);
    i.send(0).expect("Failed to send initialization message.");

    let (tx, rx) = channel();
    feed_back(o, i, tx, q);

    if let Some(signal) = rx.iter().next() {
        signal
    } else {
        panic!("Failed to get a final result from settings {:?}", settings);
    }
}

fn create_amp_array(
    input: &String,
    settings: [i128; 5],
) -> (Sender<i128>, Receiver<i128>, Receiver<i128>) {
    let (ain_tx, ain_rx) = channel();
    let (aout_tx, aout_rx) = channel();
    let (bin_tx, bin_rx) = channel();
    let (a_quit_tx, a_quit_rx) = channel();
    let (bc_tx, bc_rx) = channel();
    let (cd_tx, cd_rx) = channel();
    let (de_tx, de_rx) = channel();
    let (eout_tx, eout_rx) = channel();
    ain_tx
        .send(settings[0])
        .expect("Failed to set phase on amplifier A");
    bin_tx
        .send(settings[1])
        .expect("Failed to set phase on amplifier B");
    bc_tx
        .send(settings[2])
        .expect("Failed to set phase on amplifier C");
    cd_tx
        .send(settings[3])
        .expect("Failed to set phase on amplifier D");
    de_tx
        .send(settings[4])
        .expect("Failed to set phase on amplifier E");
    forward_with_end_signal(aout_rx, bin_tx, a_quit_tx);
    Builder::new().parse(input).run(ain_rx, aout_tx);
    Builder::new().parse(input).run(bin_rx, bc_tx);
    Builder::new().parse(input).run(bc_rx, cd_tx);
    Builder::new().parse(input).run(cd_rx, de_tx);
    Builder::new().parse(input).run(de_rx, eout_tx);
    (ain_tx, eout_rx, a_quit_rx)
}

fn forward_with_end_signal(from: Receiver<i128>, to: Sender<i128>, end_signal: Sender<i128>) {
    thread::spawn(move || {
        for msg in from.iter() {
            to.send(msg).expect("Failed to forward message");
        }
        end_signal
            .send(std::i128::MIN)
            .expect("Failed to send quit signal");
    });
}

fn feed_back(
    from: Receiver<i128>,
    to: Sender<i128>,
    alternate: Sender<i128>,
    end_signal: Receiver<i128>,
) {
    thread::spawn(move || {
        for msg in from.iter() {
            match end_signal.try_recv() {
                Ok(_) => alternate
                    .send(msg)
                    .expect("Failed to forward message to alternate receiver"),
                Err(TryRecvError::Empty) => to.send(msg).expect("Failed to forward message"),
                Err(TryRecvError::Disconnected) => panic!("End signal disconnected"),
            }
        }
    });
}
