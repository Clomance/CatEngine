use super::{
    Track,
    MonoTrack,
};

pub struct TrackChannel{
    track:MonoTrack,
    volume:f32,
}

pub struct TrackChannels{
    channels:Vec<TrackChannel>,
}