// Copyright © 2020 Xavier Basty <xavier@hexbee.net>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod bt2020;
pub mod cie_lab;
pub mod cie_luv;
pub mod cie_ucs;
pub mod cie_uvw;
pub mod cie_xyy;
pub mod cie_xyz;
pub mod cmy;
pub mod cmyk;
pub mod hex;
pub mod hsl;
pub mod hsv;
pub mod ipt;
pub mod jzazbz;
pub mod rgb;
pub mod spectral;
pub mod yiq;
pub mod yuv;

use cgmath::*;
use crate::models::cie_xyz::Xyz;
use crate::models::cie_xyy::Xyy;

pub trait Color {}

pub trait ColorConversion {
    fn as_bt2020(&self);
    fn as_cie_lab(&self);
    fn as_cie_luv(&self);
    fn as_cie_ucs(&self);
    fn as_cie_uvw(&self);
    fn as_cie_xyy(&self);
    fn as_cie_xyz(&self);
    fn as_cmy(&self);
    fn as_cmyk(&self);
    fn as_hex(&self);
    fn as_hsl(&self);
    fn as_hsv(&self);
    fn as_ipt(&self);
    fn as_yiq(&self);
    fn as_yuv(&self);
}

/// Returns the *CIE XYZ* tristimulus values from given *CIE xy* chromaticity coordinates.
///
/// # Arguments
///
/// * `xy` - *CIE xy* chromaticity coordinates.
///
/// # Returns
///
/// *CIE XYZ* tristimulus values.
///
/// # Example
///
/// ```
/// use cgmath::Vector2;
/// use pigments::models::xy_to_xyz;
/// use pigments::models::cie_xyz;
///
/// let xy = Vector2{x: 0.54369557, y: 0.32107944};
/// let xyz = xy_to_xyz(&xy);
/// assert_eq!(xyz, cie_xyz::Xyz{x: 1.6933366085352586, y: 1.0, z: 0.4211574244679136});
/// ```
pub fn xy_to_xyz(xy: &Vector2<f64>) -> Xyz {
    xy_to_xyy(xy, 1.).to_xyz()
}

/// Converts from *CIE xy* chromaticity coordinates to *CIE xyY* colourspace by extending
/// the array last dimension with given `y` *luminance*.
///
/// # Arguments
///
/// * `xy` - *CIE xy* chromaticity coordinates.
/// * `y` - luminance value.
///
/// # Returns
///
/// *CIE xyY* colourspace values.
///
/// # Example
///
/// ```
/// use cgmath::Vector2;
/// use pigments::models::xy_to_xyy;
/// use pigments::models::cie_xyy;
///
/// let xyy = xy_to_xyy(&Vector2{x: 0.54369557, y: 0.32107944}, 1.);
/// assert_eq!(xyy, cie_xyy::Xyy{x: 0.54369557, y1: 0.32107944, y2: 1.});
/// ```
#[inline]
pub fn xy_to_xyy(xy: &Vector2<f64>, y: f64) -> Xyy {
    Xyy{x: xy.x, y1:xy.y, y2: y}
}
