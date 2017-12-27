extern crate tle;

#[test]
fn parse_iss_tle() {
    let s = String::from("ISS (ZARYA)\n1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927\n2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537");
    println!("iss tle is:\n{}\n", s);
    let tle = tle::parse_tle(&s);
    println!("{:?}", tle);

    // line 0
    assert_eq!(tle.name, "ISS (ZARYA)");

    // line 1
    assert_eq!(tle.satellite_number, 25544);
    assert_eq!(tle.classification, 'U');
    assert_eq!(tle.international_designator, "98067A");
    assert_eq!(tle.date, "08264.51782528");
    assert_eq!(tle.first_derivative_mean_motion, -0.00002182 * 2.0);
    assert_eq!(tle.second_derivative_mean_motion, 0.0);
    assert_eq!(tle.bstar, -0.11606e-4 / 6.0);
    assert_eq!(tle.element_set_number, 292);

    // line 2
    assert_eq!(tle.inclination, 51.6416);
    assert_eq!(tle.right_ascension, 247.4627);
    assert_eq!(tle.eccentricity, 0.0006703);
    assert_eq!(tle.argument_of_perigee, 130.5360);
    assert_eq!(tle.mean_anomaly, 325.0288);
    assert_eq!(tle.mean_motion, 15.72125391);
    assert_eq!(tle.revolution_number, 56353);
}
