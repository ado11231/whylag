use std::{
    fs::{File, OpenOptions},
    io::{self, BufWriter, Write},
    net::IpAddr,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct Log(BufWriter<File>);
impl Log {
    pub fn create(path: &Path) -> io::Result<Self> {
        let mut log = Self(BufWriter::new(
            OpenOptions::new().write(true).create_new(true).open(path)?,
        ));
        writeln!(log.0, "unix_seconds,target,router,router_ms,internet_ms")?;
        log.0.flush()?;
        Ok(log)
    }
    pub fn record(
        &mut self,
        target: IpAddr,
        router: Option<IpAddr>,
        router_ping: Option<f64>,
        internet_ping: Option<f64>,
    ) -> io::Result<()> {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(io::Error::other)?
            .as_secs_f64();
        writeln!(
            self.0,
            "{seconds:.3},{target},{},{},{}",
            router.map_or(String::new(), |value| value.to_string()),
            number(router_ping),
            number(internet_ping)
        )?;
        self.0.flush()
    }
}
fn number(value: Option<f64>) -> String {
    value.map_or(String::new(), |value| format!("{value:.3}"))
}
