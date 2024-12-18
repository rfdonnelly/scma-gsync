use hyper_rustls::HttpsConnector;
use hyper_util::client::legacy::connect::HttpConnector;

// For hyper connections
pub(crate) type Connector = HttpsConnector<HttpConnector>;

mod input;
mod model;
mod output;

pub use input::Web;
pub use model::{DateSelect, Event};
pub use output::{GAuth, GCal, GPpl};
