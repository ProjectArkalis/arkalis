use bitflags::bitflags;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq)]
pub struct Genre(u64);

bitflags! {
    impl Genre: u64 {
        const Unknown = 0;
        const Action = 1;
        const Comedy = 2;
        const Horror = 4;
        const Sports = 8;
        const Adventure = 16;
        const Drama = 32;
        const Mystery = 64;
        const Supernatural = 128;
        const AvantGarde = 256;
        const Fantasy = 512;
        const Romance = 1024;
        const Suspense = 2048;
        const AwardWinning = 4096;
        const GirlsLove = 8192;
        const SciFi = 16384;
        const BoysLove = 32768;
        const Gourmet = 65536;
        const SliceOfLife = 131072;
        const Ecchi = 262144;
        const Erotica = 524288;
        const Hentai = 1048576;
    }
}

impl Display for Genre {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Genre::Action => "Ação",
            Genre::Comedy => "Comédia",
            Genre::Horror => "Terror",
            Genre::Sports => "Esportes",
            Genre::Adventure => "Aventura",
            Genre::Drama => "Drama",
            Genre::Mystery => "Misterio",
            Genre::Supernatural => "Sobrenatural",
            Genre::AvantGarde => "Vanguarda",
            Genre::Fantasy => "Fantasia",
            Genre::Romance => "Romance",
            Genre::Suspense => "Suspense",
            Genre::AwardWinning => "Premiados",
            Genre::GirlsLove => "Amor entre garotas",
            Genre::SciFi => "Sci-Fi",
            Genre::BoysLove => "Amor entre garotos",
            Genre::Gourmet => "Gourmet",
            Genre::SliceOfLife => "Slice of Life",
            Genre::Ecchi => "Ecchi",
            Genre::Erotica => "Erotica",
            Genre::Hentai => "Hentai",
            Genre::Unknown => "Desconhecido",
            _ => "Genero Invalido",
        };

        write!(f, "{name}")
    }
}
