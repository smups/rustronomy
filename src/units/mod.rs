mod units;
/*
    This module contains type definitions for astronomical units to be
    used by other modules in this program. All units have float64 data
    containers.

    The units listed here are the IAU regognized units for astronomy
*/

// (1) SI Base units
/*
    The SI base untis cannnot be cast to eachother. They *can*, however,
    be multiplied/divided/raised to a power using operator
*/

//meters
pub struct m {
    val: f64
}

//kilogrammes
pub struct kg {
    val: f64
}

//seconds
pub struct s {
    val: f64
}

//radians
pub struct rad {
    val: f64
}

//steradians
pub struct sr {
    val: f64
}

//Kelvin
pub struct K {
    val: f64
}
//Ampere
pub struct A {
    val: f64
}
//Mole
pub struct mol {
    val: f64
}

//candela
pub struct cd {
    val: f64
}