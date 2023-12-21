use std::fmt::Debug;

#[derive(Debug)]
pub enum Band {
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B8a,
    B9,
    B10,
    B11,
    B12,
}
impl TryFrom<&str> for Band {

    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "1" => Ok(Band::B1),
            "2" => Ok(Band::B2),
            "3" => Ok(Band::B3),
            "4" => Ok(Band::B4),
            "5" => Ok(Band::B5),
            "6" => Ok(Band::B6),
            "7" => Ok(Band::B7),
            "8" => Ok(Band::B8),
            "8a" => Ok(Band::B8a),
            "9" => Ok(Band::B9),
            "10" => Ok(Band::B10),
            "11" => Ok(Band::B11),
            "12" => Ok(Band::B12),
            _ => Err(()),
        }
    }
}


pub struct BandIdentifier<'req> {
    image_id: &'req str,
    band: &'req Band,
}
impl<'req> BandIdentifier<'req> {
    pub fn new(image_id: &'req str, band: &'req Band) -> Self {
        Self{
            image_id, band
        }
    }
}