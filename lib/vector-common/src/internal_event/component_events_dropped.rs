use super::{Count, InternalEvent, InternalEventHandle, RegisterInternalEvent};
use metrics::{counter, Counter};

pub const INTENTIONAL: bool = true;
pub const UNINTENTIONAL: bool = false;

#[derive(Debug)]
pub struct ComponentEventsDropped<'a, const INTENTIONAL: bool> {
    pub count: usize,
    pub reason: &'a str,
}

impl<const INTENTIONAL: bool> InternalEvent for ComponentEventsDropped<'_, INTENTIONAL> {
    fn emit(self) {
        let count = self.count;
        self.register().emit(Count(count));
    }

    fn name(&self) -> Option<&'static str> {
        Some("ComponentEventsDropped")
    }
}

impl<'a, const INTENTIONAL: bool> From<&'a str> for ComponentEventsDropped<'a, INTENTIONAL> {
    fn from(reason: &'a str) -> Self {
        Self { count: 0, reason }
    }
}

impl<'a, const INTENTIONAL: bool> RegisterInternalEvent
    for ComponentEventsDropped<'a, INTENTIONAL>
{
    type Handle = DroppedHandle<'a, INTENTIONAL>;
    fn register(self) -> Self::Handle {
        Self::Handle {
            discarded_events: counter!(
                "component_discarded_events_total",
                "intentional" => if INTENTIONAL { "true" } else { "false" },
            ),
            reason: self.reason,
        }
    }
}

#[derive(Clone)]
pub struct DroppedHandle<'a, const INTENDED: bool> {
    discarded_events: Counter,
    reason: &'a str,
}

impl<const INTENDED: bool> InternalEventHandle for DroppedHandle<'_, INTENDED> {
    type Data = Count;
    fn emit(&self, data: Self::Data) {
        let message = "Events dropped";
        if INTENDED {
            debug!(
                message,
                intentional = INTENDED,
                count = data.0,
                reason = self.reason,
            );
        } else {
            error!(
                message,
                intentional = INTENDED,
                count = data.0,
                reason = self.reason,
            );
        }
        self.discarded_events.increment(data.0 as u64);
    }
}
