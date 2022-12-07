pub mod serial;
pub mod logger;
// everyone that will import the log module will have direct access to the logging functions
pub use log::{debug, error, info, trace, warn};
