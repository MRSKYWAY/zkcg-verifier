pub mod engine;
pub mod policy;
pub mod proof;
pub mod storage;
pub mod backend;
pub mod backend_stub;


#[cfg(feature = "zk-halo2")]
pub mod backend_halo2;

#[cfg(feature = "zk-halo2")]
pub use backend_halo2::Halo2Backend;

#[cfg(feature = "zk-vm")]
pub mod backend_zkvm;

#[cfg(test)]
mod tests;

#[cfg(all(test, feature = "zk-halo2"))]
mod tests_halo2;

#[cfg(all(test, feature = "zk-vm"))]
mod tests_zkvm;

#[cfg(all(feature = "zk-halo2", feature = "zk-vm"))]
mod tests_equivalence;