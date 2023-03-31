// This crate is responsible for getting data from the actual RingEdge 2 Maimai Touchscreen COM and
// wrapping it in the way that Maimai DX (based on ALLs system) can read it.
//
// Since RingEdge 2 touch lacks some Touch areas that ALLs touch has, we basically map them to
// existing ones (see alls_touch_areas crate)
// So if you press, for example, B1 area in Maimai DX, it will also press E1 and E2 (which is is close to B1)

use std::io::{Read, Write};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::touch::alls::*;
use crate::touch::ringedge2::*;

mod alls;
mod ringedge2;

pub struct AllsMessageCmd {
    player_num: usize,
    cmd: AllsTouchMasterCommand,
}

pub fn spawn_thread(args: &super::Arguments) -> (JoinHandle<()>, JoinHandle<()>) {
    let (sender, receiver) = crossbeam_channel::bounded::<AllsMessageCmd>(10);

    let mut alls_p1_touch =
        Alls::new(args.alls_p1_touch_com.clone(), 115_200, 0, sender.clone()).unwrap();
    let mut alls_p2_touch =
        Alls::new(args.alls_p2_touch_com.clone(), 115_200, 1, sender.clone()).unwrap();

    let alls_p1_port = alls_p1_touch.port.try_clone().unwrap();
    let alls_p2_port = alls_p2_touch.port.try_clone().unwrap();

    let mut re2_touch =
        RingEdge2::new(args.re2_touch_com.clone(), 9600, alls_p1_port, alls_p2_port).unwrap();

    let alls_handle = thread::spawn(move || loop {
        // alls_p1_touch.read();
        // alls_p2_touch.read();
    });

    let re2_handle = thread::spawn(move || {
        let rcv = receiver.clone();
        loop {
            let timer = Instant::now();
            rcv.try_iter()
                .for_each(|c| re2_touch.parse_command_from_alls(c));
            re2_touch.read();
            let elapsed = timer.elapsed();
            println!("Time need for loop: {:?}", elapsed);
        }
    });
    (re2_handle, alls_handle)
}
