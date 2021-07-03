use pitch_calc::LetterOctave;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::option::Option;
use std::sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender};
use std::time::{Duration, Instant};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
#[allow(dead_code)]
pub enum Message {
    NoteOn(LetterOctave),
    NoteOff(LetterOctave),
    Stop,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Event {
    pub msg: Message,
    pub del: Duration,
}

#[derive(PartialEq, Eq)]
struct EventAbs {
    msg: Message,
    ins: Instant,
}

impl Ord for EventAbs {
    fn cmp(&self, other: &EventAbs) -> Ordering {
        // sooner events come first
        return other.ins.cmp(&self.ins);
    }
}

impl PartialOrd for EventAbs {
    fn partial_cmp(&self, other: &EventAbs) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
fn rel_to_abs(rel: Event) -> EventAbs {
    EventAbs {
        msg: rel.msg,
        ins: Instant::now() + rel.del,
    }
}

pub fn start() -> (Sender<Event>, Receiver<Message>) {
    let (msg_tx, msg_rx) = channel();
    let (event_tx, event_rx) = channel();

    std::thread::spawn(move || {
        // Heap of events that are awaiting processing. The soonest event that must be processed
        // sits at the top of the heap
        let mut heap = BinaryHeap::<EventAbs>::new();

        loop {
            // outer Maybe: Some(_) if there is an event, None if there are no events
            // inner Maybe: Some(t) if the event is going to happen in the future, after t time. None if it is in the past.
            let t_event = heap
                .peek()
                .map(|e| e.ins.checked_duration_since(Instant::now()));

            match t_event {
                // soonest event must trigger in the future
                Some(Some(t)) => match event_rx.recv_timeout(t) {
                    Ok(e) => heap.push(rel_to_abs(e)),
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => break,
                },
                // soonest event must trigger now
                Some(None) => {}
                // no events in heap
                None => match event_rx.recv() {
                    Ok(e) => heap.push(rel_to_abs(e)),
                    _ => break,
                },
            };

            // put any other events onto the heap
            for e in event_rx.try_iter() {
                heap.push(rel_to_abs(e));
            }

            // predicate which is true if the heap has events that must trigger, false if it is empty or
            // if all the remaining events are in the future
            while heap
                .peek()
                .map(|e| e.ins <= Instant::now())
                .unwrap_or(false)
            {
                // send the events that must fire through the message receiver
                msg_tx.send(heap.pop().unwrap().msg).unwrap();
            }
        }
    });

    (event_tx, msg_rx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare() {
        use pitch_calc::Letter::C;
        let first_event = EventAbs {
            msg: Message::NoteOn(LetterOctave(C, 4)),
            ins: Instant::now(),
        };
        let second_event = EventAbs {
            msg: Message::Stop,
            ins: Instant::now() + Duration::from_millis(200),
        };
        assert_eq!(first_event > second_event, true);
    }

    #[test]
    fn sequencer_basic() {
        use pitch_calc::Letter::C;

        let first_msg = Message::NoteOn(LetterOctave(C, 4));
        let first_event = Event {
            msg: first_msg,
            del: Duration::from_millis(0),
        };

        let second_msg = Message::Stop;
        let second_event = Event {
            msg: second_msg,
            del: Duration::from_millis(200),
        };

        let (tx, rx) = start();

        tx.send(first_event).unwrap();
        tx.send(second_event).unwrap();

        assert_eq!(rx.recv().unwrap(), first_msg);
        assert!(rx.try_recv().is_err());

        std::thread::sleep(Duration::from_millis(300));
        assert_eq!(rx.try_recv().unwrap(), second_msg);
    }

    #[test]
    fn sequencer_order_of_operations() {
        use pitch_calc::Letter::{A, B};
        let msg_1 = Message::NoteOn(LetterOctave(A, 4));
        let msg_2 = Message::NoteOn(LetterOctave(B, 4));
        let msg_3 = Message::NoteOff(LetterOctave(A, 4));
        let msg_4 = Message::NoteOff(LetterOctave(B, 4));

        let zero = Duration::from_millis(0);
        let del = Duration::from_millis(200);

        let ev_1 = Event {
            msg: msg_1,
            del: zero,
        };
        let ev_2 = Event {
            msg: msg_3,
            del: del,
        };
        let ev_3 = Event {
            msg: msg_2,
            del: zero,
        };
        let ev_4 = Event {
            msg: msg_4,
            del: del,
        };

        let (tx, rx) = start();
        tx.send(ev_1).unwrap();
        tx.send(ev_2).unwrap();

        std::thread::sleep(Duration::from_millis(10));
        tx.send(ev_3).unwrap();
        tx.send(ev_4).unwrap();

        assert_eq!(rx.recv().unwrap(), msg_1);
        assert_eq!(rx.recv().unwrap(), msg_2);
        assert_eq!(rx.recv().unwrap(), msg_3);
        assert_eq!(rx.recv().unwrap(), msg_4);
    }
}
