#[derive(Debug)]
pub enum Command {
    Stop,
    Status,
}
#[derive(Debug)]
pub enum CommandResponse {
    Stop(StopResponse),
    Status(StatusResponse),
}

#[derive(Debug)]
pub enum StopResponse {
    Stopped,
}

#[derive(Debug)]
pub enum StatusResponse {
    Stable,
    Unstable,
}
