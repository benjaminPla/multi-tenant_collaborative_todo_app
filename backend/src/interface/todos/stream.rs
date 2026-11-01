use crate::{
    domain::todos::{
        ports::{TodoFeed, TodoFeedItem},
        TodoEventKind,
    },
    interface::todos::dto::TodoEventResponse,
};
use axum::response::sse::{Event, Sse};
use std::{convert::Infallible, time::Duration};
use tokio::{
    sync::{watch, OwnedSemaphorePermit},
    time::{interval, Instant},
};
use tokio_stream::{
    wrappers::{IntervalStream, WatchStream},
    Stream,
    StreamExt,
};

const KEEP_ALIVE: Duration = Duration::from_secs(15);
const LIFETIME:   Duration = Duration::from_secs(300);

enum Signal {
    Feed(TodoFeedItem),
    Stop,
    Tick,
}

pub fn sse(
    feed:     TodoFeed,
    permit:   OwnedSemaphorePermit,
    shutdown: watch::Receiver<bool>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    Sse::new(events(feed, permit, shutdown))
}

// the permit lives as long as the stream, capping open connections
fn events(
    feed:     TodoFeed,
    permit:   OwnedSemaphorePermit,
    shutdown: watch::Receiver<bool>,
) -> impl Stream<Item = Result<Event, Infallible>> {
    let deadline = Instant::now() + LIFETIME;
    let ticks    = IntervalStream::new(interval(KEEP_ALIVE)).map(|_| Signal::Tick);
    let stop     = WatchStream::new(shutdown).filter(|stopping| *stopping).map(|_| Signal::Stop);
    feed.map(Signal::Feed)
        .merge(ticks)
        .merge(stop)
        .take_while(move |signal| !matches!(signal, Signal::Stop) && Instant::now() < deadline)
        .map(move |signal| {
            let _permit = &permit;
            Ok(event(signal))
        })
}

fn event(signal: Signal) -> Event {
    match signal {
        Signal::Feed(TodoFeedItem::Event(todo_event)) => {
            let name = match todo_event.kind {
                TodoEventKind::Created => "todo.created",
                TodoEventKind::Deleted => "todo.deleted",
                TodoEventKind::Updated => "todo.updated",
            };
            Event::default()
                .event(name)
                .json_data(TodoEventResponse::from(todo_event))
                .unwrap_or_else(|_| resync())
        }
        Signal::Feed(TodoFeedItem::Lagged) => resync(),
        Signal::Stop | Signal::Tick        => Event::default().comment("keep-alive"),
    }
}

fn resync() -> Event {
    Event::default().event("resync").data("{}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Semaphore;

    #[tokio::test]
    async fn ends_on_shutdown() {
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        let permit                     = Arc::new(Semaphore::new(1)).try_acquire_owned().unwrap();
        let mut stream                 = Box::pin(events(Box::pin(tokio_stream::pending()), permit, shutdown_rx));
        assert!(stream.next().await.is_some());
        shutdown_tx.send(true).unwrap();
        assert!(stream.next().await.is_none());
    }
}
