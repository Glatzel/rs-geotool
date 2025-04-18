use numpy::{PyArrayDyn, PyArrayMethods};
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::{PyObject, Python, pyfunction};
use rayon::prelude::*;

#[pyfunction]
pub fn py_lbh2xyz(
    py: Python,
    lon_py: PyObject,
    lat_py: PyObject,
    height_py: PyObject,
    semi_major_axis: f64,
    inverse_flattening: f64,
) -> Result<pyo3::Bound<'_, PyTuple>, PyErr> {
    let ellipsoid = pyxis::Ellipsoid::from_semi_major_and_invf(semi_major_axis, inverse_flattening);
    if let (Ok(lon_ref), Ok(lat_ref), Ok(height_ref)) = (
        lon_py.downcast_bound::<PyArrayDyn<f64>>(py),
        lat_py.downcast_bound::<PyArrayDyn<f64>>(py),
        height_py.downcast_bound::<PyArrayDyn<f64>>(py),
    ) {
        let lon_array = unsafe { lon_ref.as_slice_mut().unwrap() };
        let lat_array = unsafe { lat_ref.as_slice_mut().unwrap() };
        let height_array = unsafe { height_ref.as_slice_mut().unwrap() };

        lon_array
            .par_iter_mut()
            .zip(lat_array.par_iter_mut())
            .zip(height_array.par_iter_mut())
            .for_each(|((lon, lat), height)| {
                (*lon, *lat, *height) = pyxis::lbh2xyz(*lon, *lat, *height, &ellipsoid);
            });
        (lon_ref, lat_ref, height_ref).into_pyobject(py)
    } else if let (Ok(lon), Ok(lat), Ok(height)) = (
        lon_py.extract::<f64>(py),
        lat_py.extract::<f64>(py),
        height_py.extract::<f64>(py),
    ) {
        pyxis::lbh2xyz(lon, lat, height, &ellipsoid).into_pyobject(py)
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Input must be a float or a 1D numpy.ndarray of floats.",
        ))
    }
}
#[pyfunction]
pub fn py_xyz2lbh(
    py: Python,
    x_py: PyObject,
    y_py: PyObject,
    z_py: PyObject,
    semi_major_axis: f64,
    inverse_flattening: f64,
    threshold: f64,
    max_iter: usize,
) -> Result<pyo3::Bound<'_, PyTuple>, PyErr> {
    let ellipsoid = pyxis::Ellipsoid::from_semi_major_and_invf(semi_major_axis, inverse_flattening);
    if let (Ok(x_ref), Ok(y_ref), Ok(z_ref)) = (
        x_py.downcast_bound::<PyArrayDyn<f64>>(py),
        y_py.downcast_bound::<PyArrayDyn<f64>>(py),
        z_py.downcast_bound::<PyArrayDyn<f64>>(py),
    ) {
        let x_array = unsafe { x_ref.as_slice_mut().unwrap() };
        let y_array = unsafe { y_ref.as_slice_mut().unwrap() };
        let z_array = unsafe { z_ref.as_slice_mut().unwrap() };

        x_array
            .par_iter_mut()
            .zip(y_array.par_iter_mut())
            .zip(z_array.par_iter_mut())
            .for_each(|((l, b), h)| {
                (*l, *b, *h) = pyxis::xyz2lbh(*l, *b, *h, &ellipsoid, threshold, max_iter);
            });
        (x_ref, y_ref, z_ref).into_pyobject(py)
    } else if let (Ok(x), Ok(y), Ok(z)) = (
        x_py.extract::<f64>(py),
        y_py.extract::<f64>(py),
        z_py.extract::<f64>(py),
    ) {
        pyxis::xyz2lbh(x, y, z, &ellipsoid, threshold, max_iter).into_pyobject(py)
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Input must be a float or a 1D numpy.ndarray of floats.",
        ))
    }
}
