use std::fmt::{self, Formatter, Display};
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

    #[derive(Debug)]
    pub struct TLE {
        pub name: String,
        pub satellite_number: u32,
        pub classification: char,
        pub international_designator: String,
        // date: String,
        pub first_derivative_mean_motion: f64,
        pub second_derivative_mean_motion: f64,
        pub bstar: f64,
        pub element_set_number: u32,
        pub inclination: f64,
        pub right_ascension: f64,
        pub eccentricity: f64,
        pub argument_of_perigee: f64,
        pub mean_anomaly: f64,
        pub mean_motion: f64,
        pub revolution_number: u32
    }

    impl Display for TLE {
        fn fmt(&self, f:&mut Formatter) -> fmt::Result {
            write!(f, "{}\n{}", self.name, self.satellite_number)
        }
    }

    /*
    ISS (ZARYA)
    1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
    2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537
     */

    // "ISS (ZARYA)\n1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927\n2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537"
    pub fn parse_tle(tle_str: &str) -> TLE {
        // let name = match &tle_str[0..23].lines().next() {
        //     None => String::From("Unidentified"),
        //     Some(s) => String::From(s)
        // };
        let lines: Vec<&str> = tle_str.lines().collect();
        println!("{:?}", lines);

        // CHECK NUMBER OF lines
        // VERIFY THE CHECKSUMS
        // check that the two satellite_number values are the same

        // Name line, if exists
        let name = if lines.len() == 3 {
            String::from(lines[0])
        } else {
            String::from("")
        };

        // TLE line 1
        let line1 = if lines.len() == 3 {
            lines[1]
        } else {
            lines[0]
        };

        // TLE line 2
        let line2 = if lines.len() == 3 {
            lines[2]
        } else {
            lines[3]
        };

        /*
        ISS (ZARYA)
        1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
        2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537
         */
        let satellite_number = line1[2..7].parse().expect("Could not parse field 'number'");
        let classification: char = line1.chars().nth(7).unwrap();
        let international_designator = String::from(&line1[9..17]);
        let first_derivative_mean_motion = 0.0;
        let second_derivative_mean_motion = 0.0;
        let bstar = 0.0;
        let element_set_number = 0;

        let inclination = 0.0;
        let right_ascension = 0.0;
        let eccentricity = 0.0;
        let argument_of_perigee = 0.0;
        let mean_anomaly = 0.0;
        let mean_motion = 0.0;
        let revolution_number = 0;

        // count the lines, two line element sets are often three lines long...
        // let num_lines = lines.count();
        // println!("{}", num_lines);
        // let number : u32 = 0;//lines[1][2..6].parse();
        TLE {
            name,
            satellite_number,
            classification,
            international_designator,
            first_derivative_mean_motion,
            second_derivative_mean_motion,
            bstar,
            element_set_number,
            inclination,
            right_ascension,
            eccentricity,
            argument_of_perigee,
            mean_anomaly,
            mean_motion,
            revolution_number
        }
    }
