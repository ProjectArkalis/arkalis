use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
#[repr(u8)]
#[derive(FromPrimitive, ToPrimitive, Serialize, Deserialize)]
pub enum TitleType {
    Romaji,
    English,
    Portuguese,
    Native,
}
