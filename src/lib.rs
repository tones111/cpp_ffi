#![allow(clippy::missing_safety_doc)]

use std::{
    ptr::NonNull,
    sync::mpsc::{channel, sync_channel, Sender, SyncSender},
    thread::{self, JoinHandle},
};

struct Model {
    tx: Option<SyncSender<Request>>,
    iot: Option<JoinHandle<()>>,
}

#[derive(Debug)]
enum Request {
    JOB1(Sender<()>),
    End(Sender<()>),
    Stop,
}

#[no_mangle]
unsafe extern "C" fn model__new() -> *mut Model {
    Box::into_raw(Box::new(Model {
        tx: None,
        iot: None,
    }))
}

#[no_mangle]
extern "C" fn model__drop(ptr: Option<NonNull<Model>>) {
    ptr.map(NonNull::as_ptr)
        .map(|ptr| unsafe { Box::from_raw(ptr) });
}

#[no_mangle]
unsafe extern "C" fn model__serve(ptr: Option<NonNull<Model>>) {
    let mut ptr = ptr.unwrap();
    let model = ptr.as_mut();

    let (tx, rx) = sync_channel(50);
    let mut iot_sender = Some(tx.clone());
    model.tx = Some(tx);

    model.iot = Some(thread::spawn(move || {
        for job in rx.iter() {
            match job {
                Request::JOB1(tx) => {
                    if let Some(iot_sender) = iot_sender.as_ref() {
                        let _ = iot_sender.send(Request::End(tx));
                    }
                }
                Request::End(tx) => {
                    let _ = tx.send(());
                }
                Request::Stop => {
                    iot_sender = None;
                }
            }
        }
    }));
}

#[no_mangle]
unsafe extern "C" fn model__stop(ptr: Option<NonNull<Model>>) {
    let mut ptr = ptr.unwrap();
    let model = ptr.as_mut();

    if let Some(tx) = model.tx.as_ref() {
        let _ = tx.send(Request::Stop);
    }
    model.tx.take();
    model.iot.take().unwrap().join().unwrap();
}

#[no_mangle]
extern "C" fn model__new_sender(ptr: Option<NonNull<Model>>) -> *mut SyncSender<Request> {
    let ptr = ptr.unwrap();
    let model = unsafe { ptr.as_ref() };
    Box::into_raw(Box::new(model.tx.as_ref().unwrap().clone()))
}

#[no_mangle]
extern "C" fn sender__drop(ptr: Option<NonNull<SyncSender<Request>>>) {
    ptr.map(NonNull::as_ptr)
        .map(|ptr| unsafe { Box::from_raw(ptr) });
}

#[no_mangle]
extern "C" fn sender__send_job(ptr: Option<NonNull<SyncSender<Request>>>) -> u8 {
    let ptr = ptr.unwrap();
    let sender = unsafe { ptr.as_ref() };

    let (tx, rx) = channel();
    let mut status = 0;
    match sender.send(Request::JOB1(tx)) {
        Ok(()) => match rx.recv() {
            Ok(_) => status = 1,
            Err(e) => println!("error receiving response {}", e),
        },
        Err(e) => println!("send err: {:?}", e),
    }
    status
}
