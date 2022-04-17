use std::ops::Mul;

#[derive(PartialEq, Debug)]
pub enum Sentence {
    VHW(VesselInfo),
    SOG(f64),
}

#[derive(Debug, PartialEq)]
pub struct VesselInfo {
    pub heading_true: f64,
    pub heading_magnetic: f64,
}

impl Default for VesselInfo {
    fn default() -> Self {
        Self {
            heading_true: 0.0,
            heading_magnetic: 0.0,
        }
    }
}

impl TryFrom<&str> for Sentence {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let sentence_type = get_sentence_type(value)?;
        let info: Vec<&str> = value.split(',').collect();
        return match sentence_type {
            "VHW" => {
                // $YDVHW,34.2,T,31.4,M,0.0,N,0.0,K,*67
                Ok(Sentence::VHW(VesselInfo {
                    heading_true: info.get(1).ok_or(())?.parse().map_err(|_| ())?,
                    heading_magnetic: info.get(3).ok_or(())?.parse().map_err(|_| ())?,
                }))
            }
            "VTG" => {
                // $YDVTG,360.0,T,356.9,M,0.0,N,0.0,K,A*25
                Ok(Sentence::SOG(
                    info.get(5).ok_or(())?.parse().map_err(|_| ())?,
                ))
            }
            &_ => Err(()),
        };
    }
}

impl VesselInfo {
    /// format: $--VHW,x.x,T,x.x,M,x.x,N,x.x,K*hh<CR><LF>
    pub fn to_modified(&self, sog: f64, id: &str) -> String {
        let sentence = format!(
            "${}VHW,{:.1},T,{:.1},M,{:.1},N,{:.1},K",
            id,
            self.heading_true,
            self.heading_magnetic,
            sog,
            sog.mul(1.852)
        );
        format!("{}*{:02X?}", sentence, calc_checksum(&sentence))
    }
}

fn calc_checksum(sentence: &str) -> u8 {
    let mut sum: u8 = 0;
    for c in sentence.chars().skip(1) {
        sum ^= c as u8;
    }
    sum
}

fn get_sentence_type(data: &str) -> Result<&str, ()> {
    return if let Some(i) = data.find(',') {
        Ok(&data[3..i])
    } else {
        Err(())
    };
}

#[cfg(test)]
mod tests {
    use crate::nmea::{calc_checksum, get_sentence_type, Sentence, VesselInfo};

    #[test]
    fn checksum() {
        assert_eq!(
            "57",
            format!("{:02X?}", calc_checksum("$VWVHW,0,T,0,M,10,N,20,K"))
        )
    }

    #[test]
    fn modified() {
        let sentence = Sentence::try_from("$xxVHW,0,T,0,M,0,N,0,K*xx").unwrap();
        if let Sentence::VHW(vhw) = sentence {
            assert_eq!("$yyVHW,0,T,0,M,10,N,20,K*56", vhw.to_modified(10.0, "yy"))
        } else {
            assert!(false);
        }
    }

    #[test]
    fn get_valid_sentence_type() {
        let t = get_sentence_type("$VWVHW, , , , ,0.0,N,0.0,K*4D<0D><0A>");
        assert_eq!(Ok("VHW"), t);
    }

    #[test]
    fn parse_sentence() {
        let t = Sentence::try_from("$YDVHW,49.8,T,47.0,M,0.0,N,0.0,K,*62");
        assert_eq!(
            Ok(Sentence::VHW(VesselInfo {
                heading_true: 49.8,
                heading_magnetic: 47.0,
            })),
            t
        );
    }
}
