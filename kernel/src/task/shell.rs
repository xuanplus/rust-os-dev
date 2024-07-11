use crate::print;
use crate::println;
use alloc::vec::Vec;
use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::task::AtomicWaker;
use futures_util::Stream;
use futures_util::StreamExt;

static CHAR_QUEUE: OnceCell<ArrayQueue<char>> = OnceCell::uninit();

pub(crate) fn add_char(c: char) {
    if let Ok(queue) = CHAR_QUEUE.try_get() {
        if let Err(_) = queue.push(c) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}

pub struct CharStream {
    _private: (),
}

impl CharStream {
    pub fn new() -> Self {
        CHAR_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("CharStream::new should only be called once");
        CharStream { _private: () }
    }
}

impl Stream for CharStream {
    type Item = char;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<char>> {
        let queue = CHAR_QUEUE.try_get().expect("char queue not initialized");

        if let Some(c) = queue.pop() {
            return Poll::Ready(Some(c));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}

static WAKER: AtomicWaker = AtomicWaker::new();

pub async fn shell() {
    let mut chars: CharStream = CharStream::new();
    let mut string: Vec<char> = Vec::new();

    print!(">>>");

    while let Some(c) = chars.next().await {
        if c == '\n' {
            use alloc::string::String;
            let s: String = string.iter().collect();
            print!("\ninput: {}\n>>>", s);
            string.clear();
        } else if c == '\u{8}' {
            string.pop();
        } else {
            string.push(c);
            print!("{}", c);
        }
    }
}
