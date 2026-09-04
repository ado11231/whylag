use crate::{
    command, ping,
    platform::{self, Network},
};
use std::{
    net::IpAddr,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, SyncSender},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

pub enum Update {
    Ping {
        router: Option<IpAddr>,
        router_ping: Option<f64>,
        internet_ping: Option<f64>,
    },
    Network(Network),
    Dns(bool),
}

pub struct Helpers {
    stop: Arc<AtomicBool>,
    threads: Vec<JoinHandle<()>>,
}

impl Helpers {
    pub fn start(target: IpAddr) -> (Self, Receiver<Update>) {
        let (sender, receiver) = mpsc::sync_channel(32);
        let stop = Arc::new(AtomicBool::new(false));
        let mut threads = Vec::new();
        for (interval, task) in [(1, 0), (5, 1), (10, 2)] {
            let sender = sender.clone();
            let stop = Arc::clone(&stop);
            threads.push(thread::spawn(move || {
                run(sender, stop, Duration::from_secs(interval), task, target)
            }));
        }
        (Self { stop, threads }, receiver)
    }
}

impl Drop for Helpers {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        for thread in self.threads.drain(..) {
            let _ = thread.join();
        }
    }
}

fn run(
    sender: SyncSender<Update>,
    stop: Arc<AtomicBool>,
    interval: Duration,
    task: u8,
    target: IpAddr,
) {
    while !stop.load(Ordering::Relaxed) {
        let started = Instant::now();
        let update = match task {
            0 => {
                let router = platform::get_router();
                let (router_ping, internet_ping) = thread::scope(|scope| {
                    let internet = scope.spawn(|| ping::ping(target));
                    let local = router.and_then(ping::ping);
                    (local, internet.join().ok().flatten())
                });
                Update::Ping {
                    router,
                    router_ping,
                    internet_ping,
                }
            }
            1 => Update::Network(platform::read_network()),
            _ => Update::Dns(check_dns()),
        };
        // A full queue must not prevent shutdown.
        let _ = sender.try_send(update);
        while started.elapsed() < interval && !stop.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(50));
        }
    }
}

fn check_dns() -> bool {
    #[cfg(target_os = "macos")]
    let result = command::output(
        "dscacheutil",
        &["-q", "host", "-a", "name", "example.com"],
        Duration::from_secs(3),
    );
    #[cfg(target_os = "linux")]
    let result = command::output("getent", &["ahosts", "example.com"], Duration::from_secs(3));
    result.is_ok_and(|text| !text.trim().is_empty())
}
