use std::comm::{channel, Sender, Receiver};

fn plus_one(sender: &Sender<int>, receiver: &Receiver<int>) {
    let mut value: int;
    loop {
        value = receiver.recv();
        sender.send(value + 1);
        if value == 0 { break; }
    }
}

fn main () {
    let (fromParentSender, fromParentReceiver) = channel();
    let (fromChildSender, fromChildReceiver) = channel();

    spawn(proc() {
        plus_one(&fromChildSender, &fromParentReceiver);
    });

    fromParentSender.send(22);
    fromParentSender.send(23);
    fromParentSender.send(24);
    fromParentSender.send(25);

    for _ in range(0u, 4) {
        let answer = fromChildReceiver.recv();
        println!("{:s}", answer.to_str());
        fromParentSender.send(0);
    }
}
