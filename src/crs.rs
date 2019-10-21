// Each coordinate system is defined by the following:
// - Its measurement framework, which is either geographic (in which spherical coordinates are measured from the earth's center)
//         or planimetric (in which the earth's coordinates are projected onto a two-dimensional planar surface)
// - Units of measurement (typically feet or meters for projected coordinate systems or decimal degrees for latitude-longitude)
// - The definition of the map projection for projected coordinate systems
// - Other measurement system properties such as a spheroid of reference, a datum, one or more standard parallels,
//          a central meridian, and possible shifts in the x- and y-directions
#![allow(dead_code)]

enum MeasurementFramework {
    GEOGRAPHIC,  // Spherical coordinates are measured from the earth's center
    PLANIMETRIC, // Earth's coordinates are projected onto a two-dimensional planar surface
}

// enum MeasurementUnit {
//     FEET,    // Used for Projected Coordinate System
//     METERS,  // Used for Projected Coordinate System
//     DECIMAL, //Used for LAT LONG,
//     DEGREE
// }

// enum Datum {
//     OSGB36,    
//     WGS84, 
// }

enum Projection {
    TransverseMercator,
}

pub struct Authority {
    name: String,
    code: String, 
}

pub struct Spheroid {
    name: String,
    authority: Authority,
    // Paramets //
}

pub struct Datum {
    name: String,
    authority: Authority,
    spheroid: Spheroid
}

pub struct PrimeMeridian {
    name: String,
    authority: Authority,
    difference: f64
}

pub struct MeasurementUnit {
    name: String,
    authority: Authority
    // Params:
}
pub struct GeographicCRS {
    name: String,
    datum: Datum,
    prime_meridian: PrimeMeridian,
    unit: MeasurementUnit,
    authority: Authority
}


// pub struct Crs {
//     name: String,
//     epsg_code: String,
//     datum: Datum,
//     projection: Projection,
//     framework: MeasurementFramework,
//     unit: MeasurementUnit,
//     precision: f64,
//     extent: Vec<f64>
// }
pub struct ProjectedCRSAxis {
    East: String,
    North: String
}


pub struct ProjectedCRS {
    name: String,
    geo_crs: GeographicCRS,
    unit: MeasurementUnit,
    projection: Projection,
    authority: Authority,
    axis: ProjectedCRSAxis
}

// For Projections
pub struct ProjectedCRSParameters {
    latitude_of_origin: f64,
    central_meridian: f64,
    scale_factor: f64,
    false_easting: i32,
    false_northing: i32,
}

// PROJCS["OSGB 1936 / British National Grid",
//     GEOGCS["OSGB 1936",
//         DATUM["OSGB_1936",
//             SPHEROID["Airy 1830",6377563.396,299.3249646,
//                 AUTHORITY["EPSG","7001"]],
//             AUTHORITY["EPSG","6277"]],
//         PRIMEM["Greenwich",0,
//             AUTHORITY["EPSG","8901"]],
//         UNIT["degree",0.01745329251994328,
//             AUTHORITY["EPSG","9122"]],
//         AUTHORITY["EPSG","4277"]],
//     UNIT["metre",1,
//         AUTHORITY["EPSG","9001"]],
//     PROJECTION["Transverse_Mercator"],
//     PARAMETER["latitude_of_origin",49],
//     PARAMETER["central_meridian",-2],
//     PARAMETER["scale_factor",0.9996012717],
//     PARAMETER["false_easting",400000],
//     PARAMETER["false_northing",-100000],
//     AUTHORITY["EPSG","27700"],
//     AXIS["Easting",EAST],
//     AXIS["Northing",NORTH]]

// PROJCS["OSGB 1936 / British National Grid",GEOGCS["OSGB 1936",DATUM["D_OSGB_1936",SPHEROID["Airy_1830",6377563.396,299.3249646]],PRIMEM["Greenwich",0],UNIT["Degree",0.017453292519943295]],PROJECTION["Transverse_Mercator"],PARAMETER["latitude_of_origin",49],PARAMETER["central_meridian",-2],PARAMETER["scale_factor",0.9996012717],PARAMETER["false_easting",400000],PARAMETER["false_northing",-100000],UNIT["Meter",1]]