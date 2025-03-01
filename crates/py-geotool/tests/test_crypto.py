import geotool
import numpy as np
import pytest
import copy
bd09 = (121.10271732371203, 30.61484572185035)
gcj02 = (121.09626935575027, 30.608604331756705)
wgs84 = (121.0917077, 30.6107779)


@pytest.mark.parametrize(
    ("src", "dst", "input", "expected"),
    [
        ("WGS84", "BD09", wgs84, bd09),
        ("WGS84", "GCJ02", wgs84, gcj02),
        ("GCJ02", "BD09", gcj02, bd09),
        ("GCJ02", "WGS84", gcj02, wgs84),
        ("BD09", "GCJ02", bd09, gcj02),
        ("BD09", "WGS84", bd09, wgs84),
        pytest.param("WGS84", "WGS84", bd09, wgs84, marks=pytest.mark.xfail(strict=True)),
        pytest.param("cgcs", "WGS84", bd09, wgs84, marks=pytest.mark.xfail(strict=True)),
    ],
)
@pytest.mark.parametrize("exact", [True, False])
def test_convert(src, dst, exact, input, expected):
    lon, lat = geotool.crypto(input[0], input[1], src, dst, exact)

    assert lon == pytest.approx(expected[0])
    assert lat == pytest.approx(expected[1])


bd09_array = (np.array([121.10271732371203, 121.10271732371203]), np.array([30.61484572185035, 30.61484572185035]))
gcj02_array = (np.array([121.09626935575027, 121.09626935575027]), np.array([30.608604331756705, 30.608604331756705]))
wgs84_array = (
    np.array([121.0917077, 121.0917077], dtype=np.float64),
    np.array([30.6107779, 30.6107779], dtype=np.float64),
)


@pytest.mark.parametrize(
    ("src", "dst", "input", "expected"),
    [
        ("WGS84", "BD09", wgs84_array, bd09),
        ("WGS84", "GCJ02", wgs84_array, gcj02),
        ("GCJ02", "BD09", gcj02_array, bd09),
        ("GCJ02", "WGS84", gcj02_array, wgs84),
        ("BD09", "GCJ02", bd09_array, gcj02),
        ("BD09", "WGS84", bd09_array, wgs84),
    ],
)
@pytest.mark.parametrize("exact", [True, False])
def test_convert_array(src, dst, input, expected, exact):
    lon, lat = geotool.crypto(input[0], input[1], src, dst, exact)
    assert lon[0] == pytest.approx(expected[0])
    assert lat[0] == pytest.approx(expected[1])
