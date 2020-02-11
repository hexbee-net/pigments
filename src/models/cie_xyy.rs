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

//! # CIE xyY color space

use crate::models::cie_xyz;

#[derive(PartialEq, Clone, Copy, Default, Debug)]
pub struct Xyy {
    pub x: f64,
    pub y1: f64,
    pub y2: f64,
}

impl Xyy {
    /// Converts from *CIE xyY* colorspace to *CIE XYZ* tristimulus values.
    ///
    /// # Returns
    ///
    /// *CIE XYZ* tristimulus values.
    ///
    /// # Examples
    ///
    /// ```
    /// use pigments::models::cie_xyy;
    /// use pigments::models::cie_xyz;
    ///
    /// let xyy = cie_xyy::Xyy{x: 0.54369557, y1:0.32107944, y2:0.12197225};
    /// let xyz = cie_xyz::Xyz{x: 0.2065400761504147, y: 0.12197225, z: 0.05136951866655647};
    /// assert_eq!(xyz, xyy.to_xyz());
    /// ```
    pub fn to_xyz(&self) -> cie_xyz::Xyz {
        if self.y1 == 0.0 {
            cie_xyz::Xyz{x: 0.0, y: 0.0, z: 0.0}
        } else {
            cie_xyz::Xyz{
                x: self.x * self.y2 / self.y1,
                y: self.y2,
                z: (1.0 - self.x - self.y1) * self.y2 / self.y1
            }
        }
    }
}
