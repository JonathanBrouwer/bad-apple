use ffmpeg_next::format::{input, Pixel};
use ffmpeg_next::media::Type;
use ffmpeg_next::software::scaling::{context::Context, flag::Flags};
use ffmpeg_next::util::frame::video::Video;
use std::path::Path;
use ffmpeg_next::{decoder, Error, Packet};

pub struct FrameIter<PacketIter: Iterator<Item=Packet>> {
    packets: PacketIter,
    decoder: decoder::Video,
    scaler: Context,
}

impl<PacketIter: Iterator<Item=Packet>> Iterator for FrameIter<PacketIter> {
    type Item = Video;

    fn next(&mut self) -> Option<Self::Item> {
        let mut decoded = Video::empty();

        loop {
            match self.decoder.receive_frame(&mut decoded) {
                Ok(_) => {
                    let mut frame = Video::empty();
                    self.scaler.run(&decoded, &mut frame).unwrap();
                    return Some(frame)
                },
                // No more packets to process
                Err(Error::Other { errno: 11 }) => {
                    match self.packets.next() {
                        None => return None,
                        Some(packet) => {
                            self.decoder.send_packet(&packet).unwrap();
                        }
                    }
                }
                Err(e) => panic!("{}", e),
            }

        }
    }
}

pub fn bad_apple_frames() -> impl Iterator<Item=Video> {
    ffmpeg_next::init().unwrap();

    let mut ictx = input(&Path::new("./resources/bad_apple.mp4")).unwrap();

    let input = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg_next::Error::StreamNotFound).unwrap();
    let video_stream_index = input.index();

    let context_decoder = ffmpeg_next::codec::context::Context::from_parameters(input.parameters()).unwrap();
    let decoder = context_decoder.decoder().video().unwrap();

    let scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    ).unwrap();

    FrameIter {
        packets: ictx.packets().filter(|(stream, _)| stream.index() == video_stream_index).map(|(_, packet)| packet).collect::<Vec<_>>().into_iter(),
        decoder,
        scaler,
    }
}