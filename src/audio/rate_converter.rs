use super::{
    PlayingTrack,
    SampleTransform
};

/// Taken from rodio and modified (I will create my own later)
/// 
/// Iterator that converts from a certain sample rate to another.
pub struct RateConverter{
    /// We convert chunks of `from` samples into chunks of `to` samples.
    from: u32,
    /// We convert chunks of `from` samples into chunks of `to` samples.
    to: u32,
    /// One sample per channel, extracted from `input`.
    current_frame: Vec<f32>,
    /// Position of `current_sample` modulo `from`.
    current_frame_pos_in_chunk: u32,
    /// The samples right after `current_sample` (one per channel), extracted from `input`.
    next_frame: Vec<f32>,
    /// The position of the next sample that the iterator should return, modulo `to`.
    /// This counter is incremented (modulo `to`) every time the iterator is called.
    next_output_frame_pos_in_chunk: u32,
}

impl RateConverter{
    ///
    ///
    /// # Panic
    ///
    /// Panicks if `from` or `to` are equal to 0.
    ///
    pub fn new(
        from:u32,
        to:u32,
        track:&mut PlayingTrack,
    )->RateConverter{
        assert!(from >= 1);
        assert!(to >= 1);

        // finding greatest common divisor
        let gcd = {
            fn gcd(a: u32, b: u32) -> u32 {
                if b == 0 {
                    a
                } else {
                    gcd(b, a % b)
                }
            }

            gcd(from, to)
        };

        let channels=track.channels as usize;

        let mut first_samples=Vec::with_capacity(channels);
        let mut next_samples=Vec::with_capacity(channels);

        if from!=to{
            track.next(&mut first_samples);
            track.next(&mut next_samples);
        }

        RateConverter {
            from: from / gcd,
            to: to / gcd,
            current_frame_pos_in_chunk: 0,
            next_output_frame_pos_in_chunk: 0,
            current_frame:first_samples,
            next_frame:next_samples,
        }
    }

    fn next_input_frame(&mut self,track:&mut PlayingTrack){
        self.current_frame_pos_in_chunk += 1;

        std::mem::swap(&mut self.current_frame, &mut self.next_frame);
        self.next_frame.clear();

        track.next(&mut self.next_frame);
    }

    pub fn next(&mut self,track:&mut PlayingTrack,frame:&mut Vec<f32>){
        if self.from==self.to{
            return track.next(frame)
        }

        // The frame we are going to return from this function will be a linear interpolation
        // between `self.current_frame` and `self.next_frame`.

        if self.next_output_frame_pos_in_chunk == self.to {
            // If we jump to the next frame, we reset the whole state.
            self.next_output_frame_pos_in_chunk = 0;

            self.next_input_frame(track);
            while self.current_frame_pos_in_chunk != self.from {
                self.next_input_frame(track);
            }
            self.current_frame_pos_in_chunk = 0;
        } else {
            // Finding the position of the first sample of the linear interpolation.
            let req_left_sample =
                (self.from * self.next_output_frame_pos_in_chunk / self.to) % self.from;

            // Advancing `self.current_frame`, `self.next_frame` and
            // `self.current_frame_pos_in_chunk` until the latter variable
            // matches `req_left_sample`.
            while self.current_frame_pos_in_chunk != req_left_sample {
                self.next_input_frame(track);
                debug_assert!(self.current_frame_pos_in_chunk < self.from);
            }
        }

        // Merging `self.current_frame` and `self.next_frame` into `self.output_buffer`.
        // Note that `self.output_buffer` can be truncated if there is not enough data in
        // `self.next_frame`.
        //let mut result = None;
        let numerator = (self.from * self.next_output_frame_pos_in_chunk) % self.to;
        for (off, (cur, next)) in self.current_frame
            .iter()
            .zip(self.next_frame.iter())
            .enumerate()
        {
            let sample = SampleTransform::lerp(cur.clone(), next.clone(), numerator, self.to);

            frame.push(sample);
        }

        // Incrementing the counter for the next iteration.
        self.next_output_frame_pos_in_chunk += 1;
    }
}