//! Open Graph Protocol Music Object Type
//!
//! Implementation of the music object type from the Open Graph protocol.
//!
//! # References
//! - https://ogp.me/#type_music

pub struct Album {
    pub song: Box<Song>,
}

pub struct Song {
    /// `music:duration` Open Graph's tag.
    ///
    /// The song's length in seconds.
    duration: u64,
    /// `music:album` Open Graph's tag.
    ///
    /// The album this song is from.
    albums: Vec<Album>,
    disc: u32,
    track: u32,
    musician: u32,
}

/// In order for your object to be represented within the graph, you need to
/// specify its type. This is done using the og:type property.
pub enum Music {
    Song,
    Album,
    Playlist,
    RadioStation,
}
