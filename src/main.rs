use std::time::{Duration, Instant};
use crate::event_loop::run_event_loop;
use crate::mp4::bad_apple_frames;

mod mp4;
mod event_loop;
mod notes;

const FRAME_RATE: usize = 30;
const HEIGHT: usize = 360;
const WIDTH: usize = 480;

fn main() {
    let mut frames = bad_apple_frames();

    let mut next_frame = Instant::now();

    run_event_loop(|output_frame| {
        if next_frame > Instant::now() {
            return;
        }
        next_frame += Duration::from_micros((1000000 / FRAME_RATE) as u64);

        let input_frame = frames.next().unwrap();

        for (inp, outp) in input_frame.data(0).chunks(3).zip(output_frame.chunks_exact_mut(4)) {
            outp.copy_from_slice(&[inp[0],inp[1],inp[2],255]);
        }
    });
}
