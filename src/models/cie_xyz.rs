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

//! # CIE 1931 XYZ color space

use crate::models::cie_xyy;
use crate::colorimetry::illuminants::{StandardObserver, Illuminant};

#[derive(PartialEq, Clone, Copy, Default, Debug)]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Xyz {
    /// Converts from *CIE XYZ* tristimulus values to *CIE xyY* colorspace and reference *illuminant*.
    ///
    /// # Returns
    ///
    /// *CIE xyY* colorspace array.
    ///
    /// # Examples
    ///
    /// ```
    /// use pigments::models::cie_xyz;
    /// use pigments::models::cie_xyy;
    /// use pigments::colorimetry::illuminants::StandardObserver;
    /// use pigments::colorimetry::illuminants::Illuminant;
    ///
    /// let xyz = cie_xyz::Xyz{x: 0.20654008, y: 0.12197225, z: 0.05136952};
    /// let xyy = cie_xyy::Xyy{x: 0.5436955727155692, y1:0.321079435619259, y2:0.12197225};
    /// assert_eq!(xyy, xyz.to_xyy(StandardObserver::Two, Illuminant::D65));
    ///
    /// ```
    pub fn to_xyy(&self, o: StandardObserver, i: Illuminant) -> cie_xyy::Xyy {
        use crate::colorimetry::illuminants::chromaticity;
        if self.x == 0.0 && self.y == 0.0 && self.z == 0.0 {
            let w = chromaticity::illuminant(o, i);
            cie_xyy::Xyy{
                x: w.x,
                y1: w.y,
                y2: 0.0
            }
        } else {
            cie_xyy::Xyy{
                x: self.x / (self.x + self.y + self.z),
                y1: self.y / (self.x + self.y + self.z),
                y2: self.y
            }
        }
    }

//    'XYZ_to_ATD95',
//    'XYZ_to_CAM16',
//    'XYZ_to_CIECAM02',
//    'XYZ_to_Hunt',
//    'XYZ_to_LLAB',
//    'XYZ_to_Nayatani95',
//    'XYZ_to_RLAB',
//    'XYZ_to_Hunter_Lab',
//    'XYZ_to_Hunter_Rdab',
//    'XYZ_to_IPT',
//    'XYZ_to_JzAzBz',
//    'XYZ_to_K_ab_HunterLab1966',
//    'XYZ_to_Lab',
//    'XYZ_to_Luv',
//    'XYZ_to_OSA_UCS',
//    'XYZ_to_RGB',
//    'XYZ_to_UCS',
//    'XYZ_to_UVW',
//    'XYZ_to_hdr_CIELab',
//    'XYZ_to_hdr_IPT',
//    'XYZ_to_sRGB',
//    'XYZ_to_xy',
//    'XYZ_to_xyY',
//    'XYZ_to_sd'
}
