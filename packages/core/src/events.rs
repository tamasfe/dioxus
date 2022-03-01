//! Internal and external event system
//!
//!
//! This is all kinda WIP, but the bones are there.

use crate::{ElementId, ScopeId};
use std::{any::Any, cell::Cell, fmt::Debug, sync::Arc};

pub(crate) struct BubbleState {
    pub canceled: Cell<bool>,
}

impl BubbleState {
    pub fn new() -> Self {
        Self {
            canceled: Cell::new(false),
        }
    }
}

/// User Events are events that are shuttled from the renderer into the VirtualDom through the scheduler channel.
///
/// These events will be passed to the appropriate Element given by `mounted_dom_id` and then bubbled up through the tree
/// where each listener is checked and fired if the event name matches.
///
/// It is the expectation that the event name matches the corresponding event listener, otherwise Dioxus will panic in
/// attempting to downcast the event data.
///
/// Because Event Data is sent across threads, it must be `Send + Sync`. We are hoping to lift the `Sync` restriction but
/// `Send` will not be lifted. The entire `UserEvent` must also be `Send + Sync` due to its use in the scheduler channel.
///
/// # Example
/// ```rust, ignore
/// fn App(cx: Scope) -> Element {
///     rsx!(cx, div {
///         onclick: move |_| println!("Clicked!")
///     })
/// }
///
/// let mut dom = VirtualDom::new(App);
/// let mut scheduler = dom.get_scheduler_channel();
/// scheduler.unbounded_send(SchedulerMsg::UiEvent(
///     UserEvent {
///         scope_id: None,
///         priority: EventPriority::Medium,
///         name: "click",
///         element: Some(ElementId(0)),
///         data: Arc::new(ClickEvent { .. })
///     }
/// )).unwrap();
/// ```
#[derive(Debug)]
pub struct UserEvent {
    /// The originator of the event trigger if available
    pub scope_id: Option<ScopeId>,

    /// The priority of the event to be scheduled around ongoing work
    pub priority: EventPriority,

    /// The optional real node associated with the trigger
    pub element: Option<ElementId>,

    /// The event type IE "onclick" or "onmouseover"
    pub name: &'static str,

    /// The event data to be passed onto the event handler
    pub data: Arc<dyn UiEvent>,
}

/// Priority of Event Triggers.
///
/// Internally, Dioxus will abort work that's taking too long if new, more important work arrives. Unlike React, Dioxus
/// won't be afraid to pause work or flush changes to the RealDOM. This is called "cooperative scheduling". Some Renderers
/// implement this form of scheduling internally, however Dioxus will perform its own scheduling as well.
///
/// The ultimate goal of the scheduler is to manage latency of changes, prioritizing "flashier" changes over "subtler" changes.
///
/// React has a 5-tier priority system. However, they break things into "Continuous" and "Discrete" priority. For now,
/// we keep it simple, and just use a 3-tier priority system.
///
/// - NoPriority = 0
/// - LowPriority = 1
/// - NormalPriority = 2
/// - UserBlocking = 3
/// - HighPriority = 4
/// - ImmediatePriority = 5
///
/// We still have a concept of discrete vs continuous though - discrete events won't be batched, but continuous events will.
/// This means that multiple "scroll" events will be processed in a single frame, but multiple "click" events will be
/// flushed before proceeding. Multiple discrete events is highly unlikely, though.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum EventPriority {
    /// Work that must be completed during the EventHandler phase.
    ///
    /// Currently this is reserved for controlled inputs.
    Immediate = 3,

    /// "High Priority" work will not interrupt other high priority work, but will interrupt medium and low priority work.
    ///
    /// This is typically reserved for things like user interaction.
    ///
    /// React calls these "discrete" events, but with an extra category of "user-blocking" (Immediate).
    High = 2,

    /// "Medium priority" work is generated by page events not triggered by the user. These types of events are less important
    /// than "High Priority" events and will take precedence over low priority events.
    ///
    /// This is typically reserved for VirtualEvents that are not related to keyboard or mouse input.
    ///
    /// React calls these "continuous" events (e.g. mouse move, mouse wheel, touch move, etc).
    Medium = 1,

    /// "Low Priority" work will always be preempted unless the work is significantly delayed, in which case it will be
    /// advanced to the front of the work queue until completed.
    ///
    /// The primary user of Low Priority work is the asynchronous work system (Suspense).
    ///
    /// This is considered "idle" work or "background" work.
    Low = 0,
}

pub struct AnyEvent {
    pub(crate) data: Arc<dyn UiEvent>,
}

impl AnyEvent {
    pub fn downcast<T: Send + Sync + 'static>(&self) -> Option<&T> {
        (&self.data as &dyn Any).downcast_ref::<T>()
    }
}

pub trait UiEvent: Send + Sync + 'static + Any + Debug {
    fn bubble_state(&self) -> bool;
    fn cancel_bubble(&self);
}

impl UiEvent for () {
    fn bubble_state(&self) -> bool {
        false
    }

    fn cancel_bubble(&self) {}
}
