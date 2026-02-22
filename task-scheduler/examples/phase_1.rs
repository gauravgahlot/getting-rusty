/// Phase 1 — Synchronous Foundation (std::sync::mpsc)
///
/// A single producer thread sends 100 Jobs down a std::sync::mpsc channel.
/// The main thread receives and prints each one.
///
/// Key things to observe:
///   1. tx can be cloned; rx cannot — try it and watch the compile error.
///   2. When the producer thread finishes, tx is dropped.
///      The main thread's for-loop detects the closed channel and exits cleanly.
///   3. send() blocks if the channel is full (unbounded here, so it won't).
///      try_send() is the non-blocking alternative — swap it in to experiment.
use std::thread;
use std::{sync::mpsc, time};

use tracing::info;

use task_scheduler::job::{Job, JobType};

fn main() {
    // Initialise tracing so info!() calls print to stdout.
    // Set RUST_LOG=info (or debug/warn) to control verbosity.
    tracing_subscriber::fmt::init();

    let (tx, rx) = mpsc::channel::<Job>();

    // start the producer thread
    thread::spawn(move || {
        for i in 1..=10 {
            let job_type = match i % 3 {
                0 => JobType::Metric,
                1 => JobType::HTTPRequest,
                _ => JobType::Reconcile,
            };

            let job = Job::new(i, job_type);
            info!(id = job.id, payload = %job.payload, "producer: sending job");

            // send() returns Err only if the receiver has been dropped.
            // Here we just unwrap — if rx is gone there's nothing useful to do.
            tx.send(job).unwrap();

            // Small sleep so the output is readable when running live.
            thread::sleep(time::Duration::from_millis(20));
        }

        // tx is dropped here when the closure ends.
        // The receiver will see the channel close after draining remaining items.
        info!("producer: done, dropping tx");
    });

    // main thread (consumer)
    // Iterating over rx blocks until the next item arrives.
    // The loop exits automatically when tx is dropped and the channel is empty.
    for job in rx {
        info!(id = job.id, payload = %job.payload, "consumer: received job");
    }

    info!("consumer: channel closed, exiting");
}
