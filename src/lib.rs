/*
TLE {
  name: 'ISS (ZARYA)',
  number: 25544,
  class: 'U',
  id: '98067A',
  date: Date<'2008-09-20T12:25:40.104Z'>,
  fdmm: -0.00002182,
  sdmm: 0,
  drag: -1.1606,
  ephemeris: 0,
  esn: 292,
  inclination: 51.6416,
  ascension: 247.4627,
  eccentricity: 0.0006703,
  perigee: 130.536,
  anomaly: 325.0288,
  motion: 15.721253915,
  revolution: 6353
}
 */

struct TLE {
    name: String,
    number: u32,
    class: String,
    id: String,
    date: String,
    fdmm: f64,
    dsmm: f64,
    drag: f64,
    ephemeris: u32,
    esn: u32,
    inclination: f64,
    ascension: f64,
    eccentricity: f64,
    perigee: f64,
    anomaly: f64,
    motion: f64,
    revolution: u32
}

pub fn parse_tle(str: String) -> TLE {
    TLE {
        name: 'wow'
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
