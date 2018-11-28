use std::time::Duration;
use std::sync::{Arc, Mutex};

pub struct Reflector<T> {
    /// Name of the resource
    pub name: String
    /// Group of the resource
    pub group: String,

    /// The destination to sync up with the watch source
    pub store: Arc<Mutex<HashMap<String, T>>>

    // Resource version token last seen when doing a sync with kube
    lastResourceVersion: String

    // Period between one watch ending and the next beginning
    period: Duration

    // Period between full syncing the full state
    resyncPeriod: Duration,

    // TODO: something to watch and list?
}

impl Reflector<T> {
    pub fn new(name: &str, group: &str, period: Duration) -> Self {
        // TODO: do initial LIST operation
        // store found resourceVersion along with initial state
        // set up actor mechanics to resync every n seconds
        Relefctor {
            name, group, period,
            resyncPeriod: Duration::new(1800, 0), // half hour
        }
    }
}

use actix::prelude::*;

impl Actor for Reflector<T> {
    type Context = Context<Self>;
}
