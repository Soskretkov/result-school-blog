mod creaters;
mod delete;
mod readers;
mod updaters;
pub use creaters::*;
pub use delete::*;
pub use readers::*;
pub use updaters::*;

const URL: &'static str = "http://localhost:3005";
