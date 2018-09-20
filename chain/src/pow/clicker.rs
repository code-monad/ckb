use super::PowEngine;
use core::header::{BlockNumber, Header, RawHeader, Seal};
use crossbeam_channel;

/// Clicker meant to serve as a pow engine Stub
///
/// - submit fake pow solution
/// - verify consistent return ture
pub struct Clicker {
    rx: crossbeam_channel::Receiver<u64>,
    tx: crossbeam_channel::Sender<u64>,
}

impl Default for Clicker {
    fn default() -> Self {
        Self::new()
    }
}

impl Clicker {
    pub fn new() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        Clicker { tx, rx }
    }

    pub fn submit(&self, nonce: u64) {
        self.tx.send(nonce)
    }
}

impl PowEngine for Clicker {
    fn init(&self, _number: BlockNumber) {}

    fn verify_header(&self, _header: &Header) -> bool {
        true
    }

    fn solve_header(&self, _header: &RawHeader, _nonce: u64) -> Option<Seal> {
        self.rx.recv().map(|nonce| Seal {
            nonce,
            proof: vec![],
        })
    }

    fn solve(&self, _number: BlockNumber, _message: &[u8]) -> Option<Vec<u8>> {
        unimplemented!();
    }

    fn verify(&self, _number: BlockNumber, _message: &[u8], _proof: &[u8]) -> bool {
        unimplemented!();
    }
}
