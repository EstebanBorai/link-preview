//! Open Graph Protocol Music Object Type
//!
//! Implementation of the music object type from the Open Graph protocol.
//!
//! # References
//! - https://ogp.me/#type_music
use super::profile::Profile;

pub struct Album {
    pub song: Option<Box<Song>>,
}

pub struct Song {
    /// `music:duration` Open Graph's tag.
    ///
    /// The song's length in seconds.
    duration: Option<u64>,
    /// `music:album` Open Graph's tag.
    ///
    /// The album this song is from.
    albums: Option<Vec<Album>>,
    /// `music:album` Open Graph's tag.
    ///
    /// The album this song is from.
    disc: Option<u32>,
    /// `music:album` Open Graph's tag.
    ///
    /// The album this song is from.
    track: Option<u32>,
    /// `music:album` Open Graph's tag.
    ///
    /// The album this song is from.
    musician: Option<Profile>,
}
