extern crate ffmpeg_next as ffmpeg;

use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;

pub fn extract<P>(path: &P) -> Result<(), ffmpeg::Error>
where
    P: AsRef<std::path::Path> + ?Sized,
{
    ffmpeg::init().unwrap();

    if let Ok(mut ictx) = input(path) {
        let input = ictx.streams().best(Type::Video).ok_or(ffmpeg::Error::StreamNotFound)?;
        let video_stream_index = input.index();

        let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
        let mut decoder = context_decoder.decoder().video()?;

        let mut scaler = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            Flags::BILINEAR,
        )?;

        // initialize a lance tensor

        let mut receive_and_process_decoded_frames = |decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error>{
            let mut decoded = Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                let mut scaled = Video::empty();
                scaler.run(&decoded, &mut scaled)?;
                // append to lance tensor
            }

            Ok(())
        };

        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                decoder.send_packet(&packet)?;
                receive_and_process_decoded_frames(&mut decoder)?;
            }
        }

        decoder.send_eof()?;
        receive_and_process_decoded_frames(&mut decoder)?;
    }

    Ok(())
}