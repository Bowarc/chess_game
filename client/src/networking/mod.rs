/*
    unlike the server, this application need to run fast, so der/serialisation need to be done in a diffrent thread
    So we need to make something like that
       client - client proxy thread - server


*/

mod client;
mod stats;

pub use client::Client;
pub use stats::NetworkStats;
