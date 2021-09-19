#![allow(clippy::missing_safety_doc)]

use std::{
    ptr::NonNull,
    sync::mpsc::{sync_channel, SyncSender},
    thread::{self, JoinHandle},
};

//#[repr(C)]
//struct Foo {
//    _data: [u8; 0], // Private to prevent creation
//    _marker: std::marker::PhantomData<(*mut u8, std::marker::PhantomPinned)>, // Prevent Send, Sync, Unpin
//}

struct Model {
    tx: Option<SyncSender<Request>>,
    iot: Option<JoinHandle<u64>>,
}

#[derive(Debug)]
enum Request {
    JOB1,
}

impl Drop for Model {
    fn drop(&mut self) {
        println!("dropping Model");
    }
}

#[no_mangle]
unsafe extern "C" fn model__new() -> *mut Model {
    let (tx, rx) = sync_channel(50);
    Box::into_raw(Box::new(Model {
        tx: Some(tx),
        iot: Some(thread::spawn(move || {
            let mut cnt = 0;
            for job in rx.iter() {
                match job {
                    Request::JOB1 => cnt += 1,
                }
            }
            cnt
        })),
    }))
}

#[no_mangle]
extern "C" fn model__drop(ptr: Option<NonNull<Model>>) {
    ptr.map(NonNull::as_ptr)
        .map(|ptr| unsafe { Box::from_raw(ptr) });
}

#[no_mangle]
extern "C" fn model__new_sender(ptr: Option<NonNull<Model>>) -> *mut SyncSender<Request> {
    let model = ptr.map(|ptr| unsafe { ptr.as_ref() }).unwrap();
    Box::into_raw(Box::new(model.tx.as_ref().unwrap().clone()))
}

#[no_mangle]
extern "C" fn sender__drop(ptr: Option<NonNull<SyncSender<Request>>>) {
    ptr.map(NonNull::as_ptr)
        .map(|ptr| unsafe { Box::from_raw(ptr) });
}

#[no_mangle]
extern "C" fn sender__send_job(ptr: Option<NonNull<SyncSender<Request>>>) -> u8 {
    let mut status = 0;
    if let Some(sender) = ptr.map(|ptr| unsafe { ptr.as_ref() }) {
        match sender.send(Request::JOB1) {
            Ok(()) => status = 1,
            Err(e) => println!("send err: {:?}", e),
        }
    }
    status
}

#[no_mangle]
unsafe extern "C" fn model__stop(ptr: Option<NonNull<Model>>) -> u64 {
    let mut res = 0;
    if let Some(model) = ptr.map(|mut ptr| unsafe { ptr.as_mut() }) {
        model.tx.take();
        res = model.iot.take().unwrap().join().unwrap_or_default();
    }
    res
}
