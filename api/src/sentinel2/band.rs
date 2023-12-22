use std::fmt::Debug;

#[derive(Debug)]
pub enum Band {
    B01,
    B02,
    B03,
    B04,
    B05,
    B06,
    B07,
    B08,
    B08a,
    B09,
    B10,
    B11,
    B12,
}
impl TryFrom<&str> for Band {

    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "1" => Ok(Band::B01),
            "2" => Ok(Band::B02),
            "3" => Ok(Band::B03),
            "4" => Ok(Band::B04),
            "5" => Ok(Band::B05),
            "6" => Ok(Band::B06),
            "7" => Ok(Band::B07),
            "8" => Ok(Band::B08),
            "8a" => Ok(Band::B08a),
            "9" => Ok(Band::B09),
            "10" => Ok(Band::B10),
            "11" => Ok(Band::B11),
            "12" => Ok(Band::B12),
            _ => Err(()),
        }
    }
}


pub struct BandIdentifier<'req> {
    pub image_id: &'req str,
    pub band: Band,
}
impl<'req> BandIdentifier<'req> {
    pub fn new(image_id: &'req str, band: Band) -> Self {
        Self{
            image_id, band
        }
    }
}