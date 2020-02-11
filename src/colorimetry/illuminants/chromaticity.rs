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

use super::{StandardObserver, Illuminant};
use cgmath::Vector2;


/// Defines *CIE* illuminants chromaticity coordinates for the
/// *CIE 1931 2 Degree Standard Observer* and
/// *CIE 1964 10 Degree Standard Observer*.
///
/// The following *CIE* illuminants are available:
///
///  - CIE Standard Illuminant A
///  - CIE Illuminant B
///  - CIE Illuminant C
///  - CIE Illuminant D Series (D50, D55, D60, D65, D75)
///  - CIE Illuminant E
///  - Illuminants F Series (FL1, FL2, FL3, FL4, FL5, FL6, FL7, FL8, FL9, FL10,
///    FL11, FL12, FL3.1, FL3.10, FL3.11, FL3.12, FL3.13, FL3.14, FL3.15, FL3.2,
///    FL3.3, FL3.4, FL3.5, FL3.6, FL3.7, FL3.8, FL3.9)
///  - High Pressure Discharge Lamps (HP1, HP2, HP3, HP4, HP5)
pub fn illuminant(o: StandardObserver, i: Illuminant) -> Vector2<f64> {
    match o {
        StandardObserver::Two => {
            match i {
                Illuminant::A =>        { Vector2{x: 0.44758, y: 0.40745} },
                Illuminant::B =>        { Vector2{x: 0.34842, y: 0.35161} },
                Illuminant::C =>        { Vector2{x: 0.31006, y: 0.31616} },
                Illuminant::D50 =>      { Vector2{x: 0.34570, y: 0.35850} },
                Illuminant::D55 =>      { Vector2{x: 0.33243, y: 0.34744} },
                Illuminant::D60 =>      { Vector2{x: 0.321626242047397, y: 0.337736995955436} },
                Illuminant::D65 =>      { Vector2{x: 0.31270, y: 0.32900} },
                Illuminant::D75 =>      { Vector2{x: 0.29903, y: 0.31488} },
                Illuminant::E =>        { Vector2{x: 1.0 / 3.0, y: 1.0 / 3.0} },
                Illuminant::FL1 =>       { Vector2{x: 0.31310, y: 0.33710} },
                Illuminant::FL2 =>       { Vector2{x: 0.37210, y: 0.37510} },
                Illuminant::FL3 =>       { Vector2{x: 0.40910, y: 0.39410} },
                Illuminant::FL4 =>       { Vector2{x: 0.44020, y: 0.40310} },
                Illuminant::FL5 =>       { Vector2{x: 0.31380, y: 0.34520} },
                Illuminant::FL6 =>       { Vector2{x: 0.37790, y: 0.38820} },
                Illuminant::FL7 =>       { Vector2{x: 0.31290, y: 0.32920} },
                Illuminant::FL8 =>       { Vector2{x: 0.34580, y: 0.35860} },
                Illuminant::FL9 =>       { Vector2{x: 0.37410, y: 0.37270} },
                Illuminant::FL10 =>      { Vector2{x: 0.34580, y: 0.35880} },
                Illuminant::FL11 =>      { Vector2{x: 0.38050, y: 0.37690} },
                Illuminant::FL12 =>      { Vector2{x: 0.43700, y: 0.40420} },
                Illuminant::FL31 =>     { Vector2{x: 0.44070, y: 0.40330} },
                Illuminant::FL32 =>     { Vector2{x: 0.38080, y: 0.37340} },
                Illuminant::FL33 =>     { Vector2{x: 0.31530, y: 0.34390} },
                Illuminant::FL34 =>     { Vector2{x: 0.44290, y: 0.40430} },
                Illuminant::FL35 =>     { Vector2{x: 0.37490, y: 0.36720} },
                Illuminant::FL36 =>     { Vector2{x: 0.34880, y: 0.36000} },
                Illuminant::FL37 =>     { Vector2{x: 0.43840, y: 0.40450} },
                Illuminant::FL38 =>     { Vector2{x: 0.38200, y: 0.38320} },
                Illuminant::FL39 =>     { Vector2{x: 0.34990, y: 0.35910} },
                Illuminant::FL310 =>    { Vector2{x: 0.34550, y: 0.35600} },
                Illuminant::FL311 =>    { Vector2{x: 0.32450, y: 0.34340} },
                Illuminant::FL312 =>    { Vector2{x: 0.43770, y: 0.40370} },
                Illuminant::FL313 =>    { Vector2{x: 0.38300, y: 0.37240} },
                Illuminant::FL314 =>    { Vector2{x: 0.34470, y: 0.36090} },
                Illuminant::FL315 =>    { Vector2{x: 0.31270, y: 0.32880} },
                Illuminant::HP1 =>      { Vector2{x: 0.53300, y: 0.4150} },
                Illuminant::HP2 =>      { Vector2{x: 0.47780, y: 0.41580} },
                Illuminant::HP3 =>      { Vector2{x: 0.43020, y: 0.40750} },
                Illuminant::HP4 =>      { Vector2{x: 0.38120, y: 0.37970} },
                Illuminant::HP5 =>      { Vector2{x: 0.37760, y: 0.37130} },
            }
        },
        StandardObserver::Ten => {
            match i {
                Illuminant::A =>        { Vector2{x: 0.45117, y: 0.40594} },
                Illuminant::B =>        { Vector2{x: 0.34980, y: 0.35270} },
                Illuminant::C =>        { Vector2{x: 0.31039, y: 0.31905} },
                Illuminant::D50 =>      { Vector2{x: 0.34773, y: 0.35952} },
                Illuminant::D55 =>      { Vector2{x: 0.33412, y: 0.34877} },
                Illuminant::D60 =>      { Vector2{x: 0.322986926715820, y: 0.339275732345997} },
                Illuminant::D65 =>      { Vector2{x: 0.31382, y: 0.33100} },
                Illuminant::D75 =>      { Vector2{x: 0.29968, y: 0.31740} },
                Illuminant::E =>        { Vector2{x: 1.0 / 3.0, y: 1.0 / 3.0} },
                Illuminant::FL1 =>       { Vector2{x: 0.31811, y: 0.33559} },
                Illuminant::FL2 =>       { Vector2{x: 0.37925, y: 0.36733} },
                Illuminant::FL3 =>       { Vector2{x: 0.41761, y: 0.38324} },
                Illuminant::FL4 =>       { Vector2{x: 0.44920, y: 0.39074} },
                Illuminant::FL5 =>       { Vector2{x: 0.31975, y: 0.34246} },
                Illuminant::FL6 =>       { Vector2{x: 0.38660, y: 0.37847} },
                Illuminant::FL7 =>       { Vector2{x: 0.31569, y: 0.32960} },
                Illuminant::FL8 =>       { Vector2{x: 0.34902, y: 0.35939} },
                Illuminant::FL9 =>       { Vector2{x: 0.37829, y: 0.37045} },
                Illuminant::FL10 =>      { Vector2{x: 0.35090, y: 0.35444} },
                Illuminant::FL11 =>      { Vector2{x: 0.38541, y: 0.37123} },
                Illuminant::FL12 =>      { Vector2{x: 0.44256, y: 0.39717} },
                Illuminant::FL31 =>     { Vector2{x: 0.449830684010003, y: 0.390231404321266} },
                Illuminant::FL32 =>     { Vector2{x: 0.386924116672933, y: 0.365756034732821} },
                Illuminant::FL33 =>     { Vector2{x: 0.321176986855865, y: 0.340501092654981} },
                Illuminant::FL34 =>     { Vector2{x: 0.448121275113995, y: 0.397077112142482} },
                Illuminant::FL35 =>     { Vector2{x: 0.377814166608895, y: 0.366625766963060} },
                Illuminant::FL36 =>     { Vector2{x: 0.351976478983504, y: 0.361094432889677} },
                Illuminant::FL37 =>     { Vector2{x: 0.444309208810922, y: 0.396791387314871} },
                Illuminant::FL38 =>     { Vector2{x: 0.387588931999771, y: 0.376305569410173} },
                Illuminant::FL39 =>     { Vector2{x: 0.354688990710449, y: 0.353445033593383} },
                Illuminant::FL310 =>    { Vector2{x: 0.349344792334400, y: 0.354984421140869} },
                Illuminant::FL311 =>    { Vector2{x: 0.329267975695120, y: 0.338865386643537} },
                Illuminant::FL312 =>    { Vector2{x: 0.442252080438001, y: 0.401220551071252} },
                Illuminant::FL313 =>    { Vector2{x: 0.386275268780817, y: 0.374283190950586} },
                Illuminant::FL314 =>    { Vector2{x: 0.347255078638291, y: 0.366808242504180} },
                Illuminant::FL315 =>    { Vector2{x: 0.314613997909246, y: 0.333377149377113} },
                Illuminant::HP1 =>      { Vector2{x: 0.543334600247307, y: 0.405289298480431} },
                Illuminant::HP2 =>      { Vector2{x: 0.482647330648721, y: 0.410815644179685} },
                Illuminant::HP3 =>      { Vector2{x: 0.435560034503954, y: 0.398801084399711} },
                Illuminant::HP4 =>      { Vector2{x: 0.385193641123543, y: 0.368275479241015} },
                Illuminant::HP5 =>      { Vector2{x: 0.380316415606638, y: 0.366617114797851} },
            }
        },
    }
}
