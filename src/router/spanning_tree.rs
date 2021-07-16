use async_std::{
    channel::{self, Receiver, Sender},
    prelude::*,
    task,
};
use std::{sync::mpsc::channel, time::Duration};
use chrono::DateTime;

use super::Router;

const ANNOUNCEMENT_INTERVAL: u32 = 15 * 60; // 15 minutes
const ANNOUNCEMENT_TIMEOUT: u64 = ANNOUNCEMENT_INTERVAL * 2;

struct RootAnnouncementWithTime {
    annc: SwitchAnnouncement,
    at: DateTime,
}

type SwitchPortID = u64;
pub struct SpanningTree<'r, F>
where
  F: Fn(SwitchPortID, &[SwitchPortID])
{
    router: &'r Router,
    root: Option<RootAnnouncementWithTime>,
    root_reset: (Sender<()>, Receiver<()>),
    parent: Arc<AtomicU64>,
    coords: Vec<AtomicU64>
    announce_interval: Receiver<()>,
    callback: F,
}

impl<F> SpanningTree<'_, F>
where
  F: Fn(SwitchPortID, &[SwitchPortID])
{
    pub fn new(r: &Router, callback: F) -> SpanningTree {
        let (int_tx, int_rx) = channel::bounded(2);
        let mut s = SpanningTree {
            router: r,
            root: None,
            root_reset: channel::unbounded(),
            parent: Arc<AtomicU64>,
            coords: Vec::new(),
            announce_interval: int_rx,
            callback,
        };

        s.become_root();
        task::spawn(async {
            loop {
                task::sleep(Duration::from_secs(ANNOUNCEMENT_TIMEOUT));
                int_tx.send(());
            }
        });
        task::spawn(s.worker_for_root());
        task::spawn(s.worker_for_announcement());
        s
    }

    async fn worker_for_announcement(&self) {
        loop {
            task::sleep(Duration::from_secs(ANNOUNCEMENT_INTERVAL));
            self.advertise();
            self.root_reset.0.send(());
        }
    }

    async fn worker_for_root(&self) {
        loop {
            match self.announce_interval.recv().await {
                Ok(_) => {
                    if !self.is_root() {
                        self.select_new_parent();
                    }
                },
                Err(_) => return,
            }
    }

    fn become_root(&self) {
      let mut m_parent = self.parent.clone();
      m_parent.store(0);
      let new_coords: Vec<AtomicU64> = Vec::new();
      if new_coords != self.coords {
        task::spawn(
          async { self.callback(0, Vec::new()); }
      }
      self.advertise();
    }
}
