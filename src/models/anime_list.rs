use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

#[derive(FromPrimitive, ToPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum AnimeList {
    MyAnimeList,
    AniList,
}
