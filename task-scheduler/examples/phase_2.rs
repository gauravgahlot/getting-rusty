/// Phase 2 —  Multi-Consumer with crossbeam-channel
///
/// Swap std::sync::mpsc for crossbeam-channel.
/// Now multiple dispatcher threads can compete for jobs from the same channel, giving you a true MPMC setup
use std::thread;
use std::time;

use crossbeam_channel as crossbeam;
use tracing::info;

use task_scheduler::job::{Job, JobType};

fn main() {
    tracing_subscriber::fmt::init();

    let (tx, rx) = crossbeam::unbounded::<Job>();
    let mut producer_handles = vec![];

    // start multiple producers
    for i in 1..=3 {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            for i in 1..=10 {
                let job_type = match i % 3 {
                    0 => JobType::Metric,
                    1 => JobType::HTTPRequest,
                    _ => JobType::Reconcile,
                };

                let job = Job::new(i, job_type);
                info!(id = job.id, payload = %job.payload, "producer: sending job");

                tx.send(job).unwrap();
                thread::sleep(time::Duration::from_millis(20));
            }

            info!("producer-{i}: done, dropping tx");
        });

        producer_handles.push(handle);
    }
    drop(tx);

    let mut consumer_handles = vec![];
    // start multiple consumers
    for i in 1..5 {
        let rx = rx.clone();
        let handle = thread::spawn(move || {
            for job in rx {
                info!(id = job.id, payload = %job.payload, "consumer-{i}: received job");
            }
        });

        consumer_handles.push(handle);
    }

    // join on the producer threads
    for handle in producer_handles {
        handle.join().unwrap();
    }

    // join on the consumer threads
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    info!("consumer channels closed, exiting");
}
