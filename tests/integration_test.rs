extern crate tle;

#[test]
fn parse_iss_tle() {
    let s = String::from("ISS (ZARYA)\n1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927\n2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537");
    println!("iss tle is:\n{}\n", s);
    let iss_tle = tle::parse_tle(&s);
    println!("{:?}", iss_tle);

    // line 0
    assert_eq!(iss_tle.name, "ISS (ZARYA)");

    // line 1
    assert_eq!(iss_tle.satellite_number, 25544);
    assert_eq!(iss_tle.classification, 'U');
    assert_eq!(iss_tle.international_designator, "98067A");
    assert_eq!(iss_tle.date, "08264.51782528");
    assert_eq!(iss_tle.first_derivative_mean_motion, -0.00002182 * 2.0);
    assert_eq!(iss_tle.second_derivative_mean_motion, 0.0);
    assert_eq!(iss_tle.bstar, -0.11606e-4);
    assert_eq!(iss_tle.element_set_number, 292);

    // line 2
    assert_eq!(iss_tle.inclination, 51.6416);
    assert_eq!(iss_tle.right_ascension, 247.4627);
    assert_eq!(iss_tle.eccentricity, 0.0006703);
    assert_eq!(iss_tle.argument_of_perigee, 130.5360);
    assert_eq!(iss_tle.mean_anomaly, 325.0288);
    assert_eq!(iss_tle.mean_motion, 15.72125391);
    assert_eq!(iss_tle.revolution_number, 56353);
}

#[test]
fn format_iss_tle() {
    let s = String::from("ISS (ZARYA)\n1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927\n2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537");
    let iss_tle = tle::parse_tle(&s);
    let s2 = format!("{}", iss_tle);
    println!("{}", s2);
    assert_eq!(s, s2);
}
