// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod types;

use alloc::sync::Arc;
use std::{
    collections::HashMap,
    fmt::{Debug, Formatter, Result},
};

pub use self::types::{WalletEvent, WalletEventType};

type Handler<T> = Arc<dyn Fn(&T) + Send + Sync + 'static>;

pub struct EventEmitter {
    handlers: HashMap<WalletEventType, Vec<Handler<WalletEvent>>>,
}

impl EventEmitter {
    /// Creates a new instance of `EventEmitter`.
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Registers function `handler` as a listener for a `WalletEventType`. There may be
    /// multiple listeners for a single event.
    pub fn on<F>(&mut self, events: impl IntoIterator<Item = WalletEventType>, handler: F)
    where
        F: Fn(&WalletEvent) + 'static + Send + Sync,
    {
        let mut events = events.into_iter().peekable();
        let handler = Arc::new(handler);
        // if no event is provided the handler is registered for all event types
        if events.peek().is_none() {
            // we could use a crate like strum or a macro to iterate over all values, but not sure if it's worth it
            for event_type in [
                WalletEventType::NewOutput,
                WalletEventType::SpentOutput,
                WalletEventType::TransactionInclusion,
                WalletEventType::TransactionProgress,
                #[cfg(feature = "ledger_nano")]
                WalletEventType::LedgerAddressGeneration,
            ] {
                self.handlers.entry(event_type).or_default().push(handler.clone());
            }
        }
        for event in events {
            self.handlers.entry(event).or_default().push(handler.clone());
        }
    }

    /// Removes handlers for each given `WalletEventType`.
    /// If no `WalletEventType` is given, handlers will be removed for all event types.
    pub fn clear(&mut self, events: impl IntoIterator<Item = WalletEventType>) {
        let mut events = events.into_iter().peekable();
        // if no event is provided handlers are removed for all event types
        if events.peek().is_none() {
            self.handlers.clear();
        }
        for event in events {
            self.handlers.remove(&event);
        }
    }

    /// Invokes all listeners of `event`, passing a reference to `payload` as an
    /// argument to each of them.
    pub fn emit(&self, event: WalletEvent) {
        let event_type = match &event {
            WalletEvent::NewOutput(_) => WalletEventType::NewOutput,
            WalletEvent::SpentOutput(_) => WalletEventType::SpentOutput,
            WalletEvent::TransactionInclusion(_) => WalletEventType::TransactionInclusion,
            WalletEvent::TransactionProgress(_) => WalletEventType::TransactionProgress,
            #[cfg(feature = "ledger_nano")]
            WalletEvent::LedgerAddressGeneration(_) => WalletEventType::LedgerAddressGeneration,
        };
        if let Some(handlers) = self.handlers.get(&event_type) {
            for handler in handlers {
                handler(&event);
            }
        }
    }
}

impl Default for EventEmitter {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for EventEmitter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "event_types_with_handlers: {:?}",
            self.handlers.keys().collect::<Vec<&WalletEventType>>()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{
        str::FromStr,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
    };

    use pretty_assertions::assert_eq;

    use super::{
        types::{TransactionInclusionEvent, TransactionProgressEvent, WalletEvent, WalletEventType},
        EventEmitter,
    };
    use crate::{types::block::payload::signed_transaction::TransactionId, wallet::types::InclusionState};

    #[test]
    fn events() {
        let mut emitter = EventEmitter::new();
        let event_counter = Arc::new(AtomicUsize::new(0));

        // single event
        emitter.on([WalletEventType::TransactionInclusion], |_name| {
            // println!("TransactionInclusion: {:?}", name);
        });

        // listen to two events
        emitter.on(
            [
                WalletEventType::TransactionProgress,
                WalletEventType::TransactionInclusion,
            ],
            move |_name| {
                // println!("TransactionProgress or TransactionInclusion: {:?}", name);
            },
        );

        // listen to all events
        let event_counter_clone = Arc::clone(&event_counter);
        emitter.on([], move |_name| {
            // println!("Any event: {:?}", name);
            event_counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // emit events
        emitter.emit(WalletEvent::TransactionProgress(
            TransactionProgressEvent::BuildingTransaction,
        ));
        emitter.emit(WalletEvent::TransactionInclusion(TransactionInclusionEvent {
            transaction_id: TransactionId::from_str(
                "0x2289d9981fb23cc5f4f6c2742685eeb480f8476089888aa886a18232bad8198900000000",
            )
            .expect("invalid tx id"),
            inclusion_state: InclusionState::Confirmed,
        }));

        assert_eq!(2, event_counter.load(Ordering::SeqCst));

        // remove handlers of single event
        emitter.clear([WalletEventType::TransactionProgress]);
        // emit event of removed type
        emitter.emit(WalletEvent::TransactionProgress(
            TransactionProgressEvent::BuildingTransaction,
        ));

        assert_eq!(2, event_counter.load(Ordering::SeqCst));

        // remove handlers of all events
        emitter.clear([]);
        // emit events
        emitter.emit(WalletEvent::TransactionProgress(
            TransactionProgressEvent::BuildingTransaction,
        ));
        emitter.emit(WalletEvent::TransactionInclusion(TransactionInclusionEvent {
            transaction_id: TransactionId::from_str(
                "0x2289d9981fb23cc5f4f6c2742685eeb480f8476089888aa886a18232bad8198900000000",
            )
            .expect("invalid tx id"),
            inclusion_state: InclusionState::Confirmed,
        }));
        assert_eq!(2, event_counter.load(Ordering::SeqCst));

        // listen to a single event
        let event_counter_clone = Arc::clone(&event_counter);
        emitter.on([WalletEventType::TransactionProgress], move |_name| {
            // println!("Any event: {:?}", name);
            event_counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        for _ in 0..1_000_000 {
            emitter.emit(WalletEvent::TransactionProgress(
                TransactionProgressEvent::BuildingTransaction,
            ));
        }
        assert_eq!(1_000_002, event_counter.load(Ordering::SeqCst));
    }
}
