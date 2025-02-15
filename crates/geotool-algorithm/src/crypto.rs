/// # References
/// - https://github.com/googollee/eviltransform/blob/master/rust/src/lib.rs
/// - https://github.com/billtian/wgtochina_lb-php/tree/master
/// - https://github.com/Leask/EvilTransform
use std::f64::consts::PI;
pub enum CryptoSpace {
    WGS84,
    GCJ02,
    BD09,
}

const EARTH_R: f64 = 6378137.0;
const _X_PI: f64 = PI * 3000.0 / 180.0;
const EE: f64 = 0.006_693_421_622_965_943;

fn transform(x: f64, y: f64) -> (f64, f64) {
    let xy = x * y;
    let abs_x = x.abs().sqrt();
    let x_pi = x * PI;
    let y_pi = y * PI;
    let d = 20.0 * (6.0 * x_pi).sin() + 20.0 * (2.0 * x_pi).sin();

    let mut lat = d;
    let mut lon = d;

    lat += 20.0 * (y_pi).sin() + 40.0 * (y_pi / 3.0).sin();
    lon += 20.0 * (x_pi).sin() + 40.0 * (x_pi / 3.0).sin();

    lat += 160.0 * (y_pi / 12.0).sin() + 320.0 * (y_pi / 30.0).sin();
    lon += 150.0 * (x_pi / 12.0).sin() + 300.0 * (x_pi / 30.0).sin();

    lat *= 2.0 / 3.0;
    lon *= 2.0 / 3.0;

    lat += -100.0 + 2.0 * x + 3.0 * y + 0.2 * y * y + 0.1 * xy + 0.2 * abs_x;
    lon += 300.0 + x + 2.0 * y + 0.1 * x * x + 0.1 * xy + 0.1 * abs_x;

    (lon, lat)
}

fn delta(lon: f64, lat: f64) -> (f64, f64) {
    let (d_lon, d_lat) = transform(lon - 105.0, lat - 35.0);
    let mut d_lat = d_lat;
    let mut d_lon = d_lon;
    let rad_lat = lat / 180.0 * PI;
    let mut magic = (rad_lat).sin();
    magic = 1.0 - EE * magic * magic;
    let sqrt_magic = (magic).sqrt();
    d_lat = (d_lat * 180.0) / ((EARTH_R * (1.0 - EE)) / (magic * sqrt_magic) * PI);
    d_lon = (d_lon * 180.0) / (EARTH_R / sqrt_magic * (rad_lat).cos() * PI);
    (d_lon, d_lat)
}

/// Converts coordinates from `BD09` to `GCJ02` coordinate system.
///
/// # Arguments
///
/// - `bd09_lon`: Longitude in `BD09` coordinate system.
/// - `bd09_lat`: Latitude in `BD09` coordinate system.
///
/// # Returns
///
/// A tuple `(lon, lat)` representing the coordinates in the `GCJ02` coordinate system:
/// - `lon`: Longitude in the `GCJ02` coordinate system.
/// - `lat`: Latitude in the `GCJ02` coordinate system.
///
/// # Example
/// ```
/// use float_cmp::assert_approx_eq;
/// let p = (121.10271732371203, 30.61484572185035);
/// let p = geotool_algorithm::bd09_to_gcj02(p.0, p.1);
/// assert_approx_eq!(f64, p.0, 121.09626935575027, epsilon = 1e-6);
/// assert_approx_eq!(f64, p.1, 30.608604331756705, epsilon = 1e-6);
/// ```
pub fn bd09_to_gcj02(bd09_lon: f64, bd09_lat: f64) -> (f64, f64) {
    let x = bd09_lon - 0.0065;
    let y = bd09_lat - 0.006;
    let z = (x * x + y * y).sqrt() - 0.00002 * (y * _X_PI).sin();
    let theta = y.atan2(x) - 0.000003 * (x * _X_PI).cos();
    let gcj02_lon = z * theta.cos();
    let gcj02_lat = z * theta.sin();
    (gcj02_lon, gcj02_lat)
}

/// Converts coordinates from `GCJ02` to `WGS84` coordinate system.
///
/// # Arguments
///
/// - `gcj02_lon`: Longitude in `GCJ02` coordinate system.
/// - `gcj02_lat`: Latitude in `GCJ02` coordinate system.
///
/// # Returns
///
/// A tuple `(lon, lat)` representing the coordinates in the `WGS84` coordinate system:
/// - `lon`: Longitude in the `WGS84` coordinate system.
/// - `lat`: Latitude in the `WGS84` coordinate system.
///
/// # Example
/// ```
/// use float_cmp::assert_approx_eq;
/// let p = (121.09626935575027, 30.608604331756705);
/// let p = geotool_algorithm::gcj02_to_wgs84(p.0, p.1);
/// assert_approx_eq!(f64, p.0, 121.0917077 , epsilon = 1e-5);
/// assert_approx_eq!(f64, p.1, 30.6107779 , epsilon = 1e-5);
/// ```
pub fn gcj02_to_wgs84(gcj02_lon: f64, gcj02_lat: f64) -> (f64, f64) {
    let (d_lon, d_lat) = delta(gcj02_lon, gcj02_lat);
    (gcj02_lon - d_lon, gcj02_lat - d_lat)
}

/// Converts coordinates from `BD09` to `WGS84` coordinate system.
///
/// # Arguments
///
/// - `bd09_lon`: Longitude in `BD09` coordinate system.
/// - `bd09_lat`: Latitude in `BD09` coordinate system.
///
/// # Returns
///
/// A tuple `(lon, lat)` representing the coordinates in the `WGS84` coordinate system:
/// - `lon`: Longitude in the `WGS84` coordinate system.
/// - `lat`: Latitude in the `WGS84` coordinate system.
///
/// # Example
/// ```
/// use float_cmp::assert_approx_eq;
/// let p = (121.10271691314193, 30.614836298418275);
/// let p = geotool_algorithm::bd09_to_wgs84(p.0, p.1);
/// assert_approx_eq!(f64, p.0, 121.09170577473259, epsilon = 1e-6);
/// assert_approx_eq!(f64, p.1, 30.610767662599578, epsilon = 1e-6);
/// ```
pub fn bd09_to_wgs84(bd09_lon: f64, bd09_lat: f64) -> (f64, f64) {
    let (gcj_lon, gcj_lat) = bd09_to_gcj02(bd09_lon, bd09_lat);
    gcj02_to_wgs84(gcj_lon, gcj_lat)
}

/// Converts coordinates from `GCJ02` to `BD09` coordinate system.
///
/// # Arguments
///
/// - `gcj02_lon`: Longitude in `GCJ02` coordinate system.
/// - `gcj02_lat`: Latitude in `GCJ02` coordinate system.
///
/// # Returns
///
/// A tuple `(lon, lat)` representing the coordinates in the `BD09` coordinate system:
/// - `lon`: Longitude in the `BD09` coordinate system.
/// - `lat`: Latitude in the `BD09` coordinate system.
///
/// # Example
/// ```
/// use float_cmp::assert_approx_eq;
/// let p = (121.09626935575027, 30.608604331756705);
/// let p = geotool_algorithm::gcj02_to_bd09(p.0, p.1);
/// assert_approx_eq!(f64, p.0, 121.10271732371203, epsilon = 1e-17);
/// assert_approx_eq!(f64, p.1, 30.61484572185035, epsilon = 1e-17);
/// ```
pub fn gcj02_to_bd09(gcj02_lon: f64, gcj02_lat: f64) -> (f64, f64) {
    let z = (gcj02_lon * gcj02_lon + gcj02_lat * gcj02_lat).sqrt()
        + 0.00002 * (gcj02_lat * _X_PI).sin();
    let theta = gcj02_lat.atan2(gcj02_lon) + 0.000003 * (gcj02_lon * _X_PI).cos();
    let bd09_lon = z * (theta).cos() + 0.0065;
    let bd09_lat = z * (theta).sin() + 0.006;
    (bd09_lon, bd09_lat)
}

/// Converts coordinates from `WGS84` to `GCJ02` coordinate system.
///
/// # Arguments
///
/// - `wgs84_lon`: Longitude in `WGS84` coordinate system.
/// - `wgs84_lat`: Latitude in `WGS84` coordinate system.
///
/// # Returns
///
/// A tuple `(lon, lat)` representing the coordinates in the `GCJ02` coordinate system:
/// - `lon`: Longitude in the `GCJ02` coordinate system.
/// - `lat`: Latitude in the `GCJ02` coordinate system.
///
/// # Example
/// ```
/// use float_cmp::assert_approx_eq;
/// let p = (121.0917077,30.6107779 );
/// let p = geotool_algorithm::wgs84_to_gcj02(p.0, p.1);
/// println!("{},{}",p.0,p.1);
/// assert_approx_eq!(f64, p.0, 121.09626935575027, epsilon = 1e-17);
/// assert_approx_eq!(f64, p.1, 30.608604331756705, epsilon = 1e-17);
/// ```
pub fn wgs84_to_gcj02(wgs84_lon: f64, wgs84_lat: f64) -> (f64, f64) {
    let (d_lon, d_lat) = delta(wgs84_lon, wgs84_lat);
    (wgs84_lon + d_lon, wgs84_lat + d_lat)
}

/// Converts coordinates from `BD09` to `WGS84` coordinate system.
///
/// # Arguments
///
/// - `lon`: Longitude in `BD09` coordinate system.
/// - `lat`: Latitude in `BD09` coordinate system.
///
/// # Returns
///
/// A tuple `(lon, lat)` representing the coordinates in the `WGS84` coordinate system:
/// - `wgs84_lon`: Longitude in the `WGS84` coordinate system.
/// - `wgs84_lat`: Latitude in the `WGS84` coordinate system.
///
/// # Example
/// ```
/// use float_cmp::assert_approx_eq;
/// let p = (121.0917077,30.6107779);
/// let p = geotool_algorithm::wgs84_to_bd09(p.0, p.1);
/// assert_approx_eq!(f64, p.0, 121.10271732371203, epsilon = 1e-17);
/// assert_approx_eq!(f64, p.1, 30.61484572185035,  epsilon = 1e-17);
/// ```
pub fn wgs84_to_bd09(wgs84_lon: f64, wgs84_lat: f64) -> (f64, f64) {
    let (gcj_lon, gcj_lat) = wgs84_to_gcj02(wgs84_lon, wgs84_lat);
    gcj02_to_bd09(gcj_lon, gcj_lat)
}

/// gcj2wgs_exact convert GCJ-02 coordinate(gcj_lat, gcj_lon) to WGS-84 coordinate.
///
/// # Arguments
///
/// - `gcj_lon`: Longitude in `GCJ02` coordinate system.
/// - `gcj_lat`: Latitude in `GCJ02` coordinate system.
/// - `threshold`: Error threshold. Suggest value `1e-10`.
/// - `max_iter``: Max iterations. Suggest value `100`.
///
/// # Example
///
/// ```
/// use float_cmp::assert_approx_eq;
/// let p = geotool_algorithm::gcj02_to_wgs84_exact(121.09626935575027, 30.608604331756705, 1e-17, 100);
/// assert_approx_eq!(f64, p.0,  121.0917077, epsilon = 1e-17);
/// assert_approx_eq!(f64, p.1, 30.6107779,  epsilon = 1e-17);
/// ```
pub fn gcj02_to_wgs84_exact(
    gcj02_lon: f64,
    gcj02_lat: f64,
    threshold: f64,
    max_iter: usize,
) -> (f64, f64) {
    let (mut wgs_lon, mut wgs_lat) = gcj02_to_wgs84(gcj02_lon, gcj02_lat);

    let mut d_lon = (wgs_lon - gcj02_lon).abs();
    let mut d_lat = (wgs_lat - gcj02_lat).abs();

    let mut m_lon = wgs_lon - d_lon;
    let mut m_lat = wgs_lat - d_lat;
    let mut p_lon = wgs_lon + d_lon;
    let mut p_lat = wgs_lat + d_lat;

    for _i in 0..max_iter {
        (wgs_lon, wgs_lat) = ((m_lon + p_lon) / 2.0, (m_lat + p_lat) / 2.0);
        let (tmp_lon, tmp_lat) = wgs84_to_gcj02(wgs_lon, wgs_lat);
        d_lon = tmp_lon - gcj02_lon;
        d_lat = tmp_lat - gcj02_lat;

        // print message only under debug mode
        #[cfg(debug_assertions)]
        {
            println!("step: {_i}");
            println!("wgs_lon: {wgs_lon}, wgs_lat: {wgs_lat}");
            println!("d_lon: {d_lon:.6e}, d_lat: {d_lat:.6e}");
            println!("p_lon: {p_lon}, p_lat: {p_lat}");
            println!("m_lon: {m_lon}, m_lat: {m_lat}");
        }

        if d_lat.abs() < threshold && d_lon.abs() < threshold {
            return (wgs_lon, wgs_lat);
        }
        if d_lon > 0.0 {
            p_lon = wgs_lon;
        } else {
            m_lon = wgs_lon;
        }
        if d_lat > 0.0 {
            p_lat = wgs_lat;
        } else {
            m_lat = wgs_lat;
        }
    }
    // print message only under debug mode
    #[cfg(debug_assertions)]
    {
        println!("Exeed max iteration number: {max_iter}");
    }
    ((m_lon + p_lon) / 2.0, (m_lat + p_lat) / 2.0)
}
/// gcj2wgs_exact convert GCJ-02 coordinate(gcj_lat, gcj_lon) to WGS-84 coordinate.
///
/// # Arguments
///
/// - `gcj_lon`: Longitude in `GCJ02` coordinate system.
/// - `gcj_lat`: Latitude in `GCJ02` coordinate system.
/// - `threshold`: Error threshold. Suggest value `1e-13`.
/// - `max_iter``: Max iterations. Suggest value `100`.
///
/// # Example
///
/// ```
/// use float_cmp::assert_approx_eq;
/// let p = geotool_algorithm::bd09_to_gcj02_exact(121.10271732371203, 30.61484572185035, 1e-13, 100);
/// assert_approx_eq!(f64, p.0,  121.09626935575027, epsilon = 1e-13);
/// assert_approx_eq!(f64, p.1, 30.608604331756705,  epsilon = 1e-13);
/// ```
pub fn bd09_to_gcj02_exact(
    bd09_lon: f64,
    bd09_lat: f64,
    threshold: f64,
    max_iter: usize,
) -> (f64, f64) {
    let (mut gcj02_lon, mut gcj02_lat) = bd09_to_gcj02(bd09_lon, bd09_lat);

    let mut d_lon = (gcj02_lon - bd09_lon).abs();
    let mut d_lat = (gcj02_lat - bd09_lat).abs();

    let mut m_lon = gcj02_lon - d_lon;
    let mut m_lat = gcj02_lat - d_lat;
    let mut p_lon = gcj02_lon + d_lon;
    let mut p_lat = gcj02_lat + d_lat;

    for _i in 0..max_iter {
        (gcj02_lon, gcj02_lat) = ((m_lon + p_lon) / 2.0, (m_lat + p_lat) / 2.0);
        let (tmp_lon, tmp_lat) = gcj02_to_bd09(gcj02_lon, gcj02_lat);
        d_lon = tmp_lon - bd09_lon;
        d_lat = tmp_lat - bd09_lat;

        // print message only under debug mode
        #[cfg(debug_assertions)]
        {
            println!("step: {_i}");
            println!("gcj02_lon: {gcj02_lon}, gcj02_lat: {gcj02_lat}");
            println!("d_lon: {d_lon:.6e}, d_lat: {d_lat:.6e}");
            println!("p_lon: {p_lon}, p_lat: {p_lat}");
            println!("m_lon: {m_lon}, m_lat: {m_lat}");
        }

        if d_lat.abs() < threshold && d_lon.abs() < threshold {
            return (gcj02_lon, gcj02_lat);
        }
        if d_lon > 0.0 {
            p_lon = gcj02_lon;
        } else {
            m_lon = gcj02_lon;
        }
        if d_lat > 0.0 {
            p_lat = gcj02_lat;
        } else {
            m_lat = gcj02_lat;
        }
    }
    // print message only under debug mode
    #[cfg(debug_assertions)]
    {
        println!("Exeed max iteration number: {max_iter}");
    }
    ((m_lon + p_lon) / 2.0, (m_lat + p_lat) / 2.0)
}
/// Converts coordinates from `BD09` to `WGS84` coordinate system.
///
/// # Arguments
///
/// - `bd09_lon`: Longitude in `BD09` coordinate system.
/// - `bd09_lat`: Latitude in `BD09` coordinate system.
///
/// # Returns
///
/// A tuple `(lon, lat)` representing the coordinates in the `WGS84` coordinate system:
/// - `lon`: Longitude in the `WGS84` coordinate system.
/// - `lat`: Latitude in the `WGS84` coordinate system.
/// - `threshold`: Error threshold. Suggest value `1e-13`.
/// - `max_iter``: Max iterations. Suggest value `100`.
///
/// # Example
/// ```
/// use float_cmp::assert_approx_eq;
/// let p = (121.10271732371203, 30.61484572185035);
/// let p = geotool_algorithm::bd09_to_wgs84_exact(p.0, p.1,1e-13, 100);
/// assert_approx_eq!(f64, p.0, 121.0917077, epsilon = 1e-13);
/// assert_approx_eq!(f64, p.1, 30.6107779, epsilon = 1e-13);
/// ```
pub fn bd09_to_wgs84_exact(
    bd09_lon: f64,
    bd09_lat: f64,
    threshold: f64,
    max_iter: usize,
) -> (f64, f64) {
    let (gcj_lon, gcj_lat) = bd09_to_gcj02_exact(bd09_lon, bd09_lat, threshold, max_iter);
    println!("{gcj_lon},{gcj_lat}");
    gcj02_to_wgs84_exact(gcj_lon, gcj_lat, threshold, max_iter)
}

/// distance calculate the distance between point(lat_a, lon_a) and point(lat_b, lon_b), unit in meter.
pub fn distance_geo(lon_a: f64, lat_a: f64, lon_b: f64, lat_b: f64) -> f64 {
    let arc_lat_a = lat_a * PI / 180.0;
    let arc_lat_b = lat_b * PI / 180.0;
    let x = (arc_lat_a).cos() * (arc_lat_b).cos() * ((lon_a - lon_b) * PI / 180.0).cos();
    let y = (arc_lat_a).sin() * (arc_lat_b).sin();
    let s = (x + y).clamp(-1.0, 1.0);
    let alpha = s.acos();
    alpha * EARTH_R
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_crypto() {
        let (lon, lat) = (121.0917077, 30.6107779);
        let (test_lon, test_lat) = super::wgs84_to_bd09(lon, lat);
        let (test_lon, test_lat) = super::bd09_to_wgs84_exact(test_lon, test_lat, 1e-13, 35);
        println!("{test_lon},{test_lat}");
        let d = super::distance_geo(121.0917077, 30.6107779, test_lon, test_lat);
        println!("distance: {d}");
        float_cmp::assert_approx_eq!(f64, lon, test_lon, epsilon = 1e-13);
        float_cmp::assert_approx_eq!(f64, lat, test_lat, epsilon = 1e-13);
        float_cmp::assert_approx_eq!(f64, d, 0.0, epsilon = 1e-17);
    }
}
