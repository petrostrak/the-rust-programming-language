// Bidirectional thread communication
use crossbeam_channel::unbounded;
use std::thread;

enum WorkerMsg {
    PrintData(String),
    Sum(i64, i64),
    Quit,
}

enum MainMsg {
    SumResult(i64),
    WorkerQuit,
}

fn main() {
    let (worker_tx, worker_rx) = unbounded::<WorkerMsg>();
    let (main_tx, main_rx) = unbounded::<MainMsg>();

    let worker = thread::spawn(move || loop {
        match worker_rx.recv() {
            Ok(msg) => match msg {
                WorkerMsg::PrintData(d) => println!("Worker:{}", d),
                WorkerMsg::Sum(lhs, rhs) => {
                    println!("Worker summing...");
                    main_tx.send(MainMsg::SumResult(lhs + rhs));
                    ()
                }
                WorkerMsg::Quit => {
                    println!("Worker: thread terminating");
                    main_tx.send(MainMsg::WorkerQuit);
                    break;
                }
            },
            Err(e) => {
                println!("disconnected");
                main_tx.try_send(MainMsg::WorkerQuit);
                break;
            }
        }
    });

    worker_tx
        .send(WorkerMsg::PrintData("hello from main".to_owned()))
        .unwrap();
    worker_tx.send(WorkerMsg::Sum(10, 10)).unwrap();
    worker_tx.send(WorkerMsg::Quit).unwrap();

    while let Ok(msg) = main_rx.recv() {
        match msg {
            MainMsg::SumResult(rslt) => println!("Main: {}", rslt),
            MainMsg::WorkerQuit => println!("Main: worker terminated"),
        }
    }

    worker.join().unwrap();
}
