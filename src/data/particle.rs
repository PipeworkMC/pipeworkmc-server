use serde::Serialize as Ser;


#[derive(Ser, Debug)]
#[serde(tag = "type")]
pub enum Particle {

}
