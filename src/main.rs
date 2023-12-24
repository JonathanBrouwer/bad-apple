use crate::event_loop::run_event_loop;
use crate::mp4::bad_apple_frames;

mod mp4;
mod event_loop;

const FRAME_RATE: usize = 30;
const HEIGHT: usize = 360;
const WIDTH: usize = 480;

fn main() {
    let mut frames = bad_apple_frames();

    run_event_loop(|output_frame| {
        let input_frame = frames.next().unwrap();

        for (inp, outp) in input_frame.data(0).chunks(3).zip(output_frame.chunks_exact_mut(4)) {
            outp.copy_from_slice(&[inp[0],inp[1],inp[2],255]);
        }
    });
}