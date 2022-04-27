use super::JobScheduler;
use crate::prelude::*;

#[derive(Default, Clone)]
pub struct FifoScheduler {
    queue: VecDeque<usize>,
    tid: usize,
    current: Option<usize>,
}

impl JobScheduler for FifoScheduler {
    type Handle = usize;

    fn add_new_with_priority(&mut self, _: isize) -> Option<Self::Handle> {
        let tid = self.tid;
        self.tid += 1;
        self.queue.push_front(tid);
        Some(tid)
    }

    fn set_priority(&self, _: Self::Handle, _: isize) {
        warn!("Priority has no effect in a FIFO cheduler",)
    }

    fn next(&mut self) -> Option<Self::Handle> {
        let next = self.queue.pop_front();
        self.current = next;
        next
    }

    fn complete(&mut self, job: Self::Handle) {
        self.current = None;
        self.queue.retain(|&j| j != job);
    }

    fn current(&self) -> Option<Self::Handle> {
        self.current
    }

    fn yld(&mut self, job: Self::Handle) {
        self.queue.retain(|&j| j != job);
        self.queue.push_back(job);
    }
}