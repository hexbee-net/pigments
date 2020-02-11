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

pub mod chromaticity;
pub mod spectrum;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub enum StandardObserver {
    Two = 2,
    Ten = 10,
}


#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub enum Illuminant {
    A,
    B,
    C,
    D50,
    D55,
    D60,
    D65,
    D75,
    E,
    FL1,
    FL2,
    FL3,
    FL4,
    FL5,
    FL6,
    FL7,
    FL8,
    FL9,
    FL10,
    FL11,
    FL12,
    FL31,
    FL32,
    FL33,
    FL34,
    FL35,
    FL36,
    FL37,
    FL38,
    FL39,
    FL310,
    FL311,
    FL312,
    FL313,
    FL314,
    FL315,
    HP1,
    HP2,
    HP3,
    HP4,
    HP5,
}
