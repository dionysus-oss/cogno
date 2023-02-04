use cogno::{should_eq, TestController};
use std::panic::catch_unwind;
use std::sync::{Arc, Mutex};
use std::{panic, thread};

fn main() {
    let mut recorder = Arc::new(Mutex::new(TestController::new()));

    let recorder_panic_ref = recorder.clone();
    panic::set_hook(Box::new(move |info| {
        let mut recorder_handle = recorder_panic_ref.lock().unwrap();
        recorder_handle.set_panic_info(info.to_string());
    }));

    model_test(&mut recorder);

    println!("Results: {:?}", recorder.lock().unwrap());
}

fn model_test(recorder: &mut Arc<Mutex<TestController>>) {
    recorder.lock().unwrap().register("model_test");

    let recorder_thread_ref = recorder.clone();
    let result = thread::Builder::new()
        .name("model_test".to_string())
        .spawn(move || {
            catch_unwind(move || {
                should_eq!(recorder_thread_ref, "rfc_1234_sec_8.1", 'a', 'a');

                should_eq!(recorder_thread_ref, "rfc_1234_sec_8.2", 'a', 'b');
            })
        })
        .unwrap()
        .join()
        .unwrap();

    match result {
        Ok(_) => {
            recorder.lock().unwrap().complete();
        }
        _ => {}
    };
}
