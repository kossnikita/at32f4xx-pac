#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Battery powered domain data register (BPR_DTx)"]
    pub dt1: DT1,
    #[doc = "0x04 - Battery powered domain data register (BPR_DTx)"]
    pub dt2: DT2,
    #[doc = "0x08 - Battery powered domain data register (BPR_DTx)"]
    pub dt3: DT3,
    #[doc = "0x0c - Battery powered domain data register (BPR_DTx)"]
    pub dt4: DT4,
    #[doc = "0x10 - Battery powered domain data register (BPR_DTx)"]
    pub dt5: DT5,
    #[doc = "0x14 - Battery powered domain data register (BPR_DTx)"]
    pub dt6: DT6,
    #[doc = "0x18 - Battery powered domain data register (BPR_DTx)"]
    pub dt7: DT7,
    #[doc = "0x1c - Battery powered domain data register (BPR_DTx)"]
    pub dt8: DT8,
    #[doc = "0x20 - Battery powered domain data register (BPR_DTx)"]
    pub dt9: DT9,
    #[doc = "0x24 - Battery powered domain data register (BPR_DTx)"]
    pub dt10: DT10,
    #[doc = "0x28 - RTC clock calibration register (BPR_RTCCAL)"]
    pub rtccal: RTCCAL,
    #[doc = "0x2c - BPR control register (BPR_CTRL)"]
    pub ctrl: CTRL,
    #[doc = "0x30 - BPR control/status register (BPR_CTRLSTS)"]
    pub ctrlsts: CTRLSTS,
    _reserved13: [u8; 0x08],
    #[doc = "0x3c - Battery powered domain data register (BPR_DTx)"]
    pub dt11: DT11,
    #[doc = "0x40 - Battery powered domain data register (BPR_DTx)"]
    pub dt12: DT12,
    #[doc = "0x44 - Battery powered domain data register (BPR_DTx)"]
    pub dt13: DT13,
    #[doc = "0x48 - Battery powered domain data register (BPR_DTx)"]
    pub dt14: DT14,
    #[doc = "0x4c - Battery powered domain data register (BPR_DTx)"]
    pub dt15: DT15,
    #[doc = "0x50 - Battery powered domain data register (BPR_DTx)"]
    pub dt16: DT16,
    #[doc = "0x54 - Battery powered domain data register (BPR_DTx)"]
    pub dt17: DT17,
    #[doc = "0x58 - Battery powered domain data register (BPR_DTx)"]
    pub dt18: DT18,
    #[doc = "0x5c - Battery powered domain data register (BPR_DTx)"]
    pub dt19: DT19,
    #[doc = "0x60 - Battery powered domain data register (BPR_DTx)"]
    pub dt20: DT20,
    #[doc = "0x64 - Battery powered domain data register (BPR_DTx)"]
    pub dt21: DT21,
    #[doc = "0x68 - Battery powered domain data register (BPR_DTx)"]
    pub dt22: DT22,
    #[doc = "0x6c - Battery powered domain data register (BPR_DTx)"]
    pub dt23: DT23,
    #[doc = "0x70 - Battery powered domain data register (BPR_DTx)"]
    pub dt24: DT24,
    #[doc = "0x74 - Battery powered domain data register (BPR_DTx)"]
    pub dt25: DT25,
    #[doc = "0x78 - Battery powered domain data register (BPR_DTx)"]
    pub dt26: DT26,
    #[doc = "0x7c - Battery powered domain data register (BPR_DTx)"]
    pub dt27: DT27,
    #[doc = "0x80 - Battery powered domain data register (BPR_DTx)"]
    pub dt28: DT28,
    #[doc = "0x84 - Battery powered domain data register (BPR_DTx)"]
    pub dt29: DT29,
    #[doc = "0x88 - Battery powered domain data register (BPR_DTx)"]
    pub dt30: DT30,
    #[doc = "0x8c - Battery powered domain data register (BPR_DTx)"]
    pub dt31: DT31,
    #[doc = "0x90 - Battery powered domain data register (BPR_DTx)"]
    pub dt32: DT32,
    #[doc = "0x94 - Battery powered domain data register (BPR_DTx)"]
    pub dt33: DT33,
    #[doc = "0x98 - Battery powered domain data register (BPR_DTx)"]
    pub dt34: DT34,
    #[doc = "0x9c - Battery powered domain data register (BPR_DTx)"]
    pub dt35: DT35,
    #[doc = "0xa0 - Battery powered domain data register (BPR_DTx)"]
    pub dt36: DT36,
    #[doc = "0xa4 - Battery powered domain data register (BPR_DTx)"]
    pub dt37: DT37,
    #[doc = "0xa8 - Battery powered domain data register (BPR_DTx)"]
    pub dt38: DT38,
    #[doc = "0xac - Battery powered domain data register (BPR_DTx)"]
    pub dt39: DT39,
    #[doc = "0xb0 - Battery powered domain data register (BPR_DTx)"]
    pub dt40: DT40,
    #[doc = "0xb4 - Battery powered domain data register (BPR_DTx)"]
    pub dt41: DT41,
    #[doc = "0xb8 - Battery powered domain data register (BPR_DTx)"]
    pub dt42: DT42,
}
#[doc = "DT1 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt1`]
module"]
pub type DT1 = crate::Reg<dt1::DT1_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt1;
#[doc = "DT2 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt2`]
module"]
pub type DT2 = crate::Reg<dt2::DT2_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt2;
#[doc = "DT3 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt3`]
module"]
pub type DT3 = crate::Reg<dt3::DT3_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt3;
#[doc = "DT4 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt4`]
module"]
pub type DT4 = crate::Reg<dt4::DT4_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt4;
#[doc = "DT5 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt5`]
module"]
pub type DT5 = crate::Reg<dt5::DT5_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt5;
#[doc = "DT6 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt6`]
module"]
pub type DT6 = crate::Reg<dt6::DT6_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt6;
#[doc = "DT7 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt7`]
module"]
pub type DT7 = crate::Reg<dt7::DT7_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt7;
#[doc = "DT8 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt8`]
module"]
pub type DT8 = crate::Reg<dt8::DT8_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt8;
#[doc = "DT9 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt9`]
module"]
pub type DT9 = crate::Reg<dt9::DT9_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt9;
#[doc = "DT10 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt10`]
module"]
pub type DT10 = crate::Reg<dt10::DT10_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt10;
#[doc = "DT11 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt11`]
module"]
pub type DT11 = crate::Reg<dt11::DT11_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt11;
#[doc = "DT12 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt12`]
module"]
pub type DT12 = crate::Reg<dt12::DT12_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt12;
#[doc = "DT13 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt13`]
module"]
pub type DT13 = crate::Reg<dt13::DT13_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt13;
#[doc = "DT14 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt14`]
module"]
pub type DT14 = crate::Reg<dt14::DT14_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt14;
#[doc = "DT15 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt15`]
module"]
pub type DT15 = crate::Reg<dt15::DT15_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt15;
#[doc = "DT16 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt16`]
module"]
pub type DT16 = crate::Reg<dt16::DT16_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt16;
#[doc = "DT17 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt17`]
module"]
pub type DT17 = crate::Reg<dt17::DT17_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt17;
#[doc = "DT18 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt18`]
module"]
pub type DT18 = crate::Reg<dt18::DT18_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt18;
#[doc = "DT19 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt19`]
module"]
pub type DT19 = crate::Reg<dt19::DT19_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt19;
#[doc = "DT20 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt20`]
module"]
pub type DT20 = crate::Reg<dt20::DT20_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt20;
#[doc = "DT21 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt21`]
module"]
pub type DT21 = crate::Reg<dt21::DT21_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt21;
#[doc = "DT22 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt22`]
module"]
pub type DT22 = crate::Reg<dt22::DT22_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt22;
#[doc = "DT23 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt23`]
module"]
pub type DT23 = crate::Reg<dt23::DT23_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt23;
#[doc = "DT24 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt24`]
module"]
pub type DT24 = crate::Reg<dt24::DT24_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt24;
#[doc = "DT25 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt25`]
module"]
pub type DT25 = crate::Reg<dt25::DT25_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt25;
#[doc = "DT26 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt26`]
module"]
pub type DT26 = crate::Reg<dt26::DT26_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt26;
#[doc = "DT27 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt27`]
module"]
pub type DT27 = crate::Reg<dt27::DT27_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt27;
#[doc = "DT28 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt28`]
module"]
pub type DT28 = crate::Reg<dt28::DT28_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt28;
#[doc = "DT29 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt29`]
module"]
pub type DT29 = crate::Reg<dt29::DT29_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt29;
#[doc = "DT30 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt30`]
module"]
pub type DT30 = crate::Reg<dt30::DT30_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt30;
#[doc = "DT31 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt31`]
module"]
pub type DT31 = crate::Reg<dt31::DT31_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt31;
#[doc = "DT32 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt32`]
module"]
pub type DT32 = crate::Reg<dt32::DT32_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt32;
#[doc = "DT33 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt33`]
module"]
pub type DT33 = crate::Reg<dt33::DT33_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt33;
#[doc = "DT34 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt34`]
module"]
pub type DT34 = crate::Reg<dt34::DT34_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt34;
#[doc = "DT35 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt35`]
module"]
pub type DT35 = crate::Reg<dt35::DT35_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt35;
#[doc = "DT36 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt36`]
module"]
pub type DT36 = crate::Reg<dt36::DT36_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt36;
#[doc = "DT37 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt37`]
module"]
pub type DT37 = crate::Reg<dt37::DT37_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt37;
#[doc = "DT38 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt38`]
module"]
pub type DT38 = crate::Reg<dt38::DT38_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt38;
#[doc = "DT39 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt39`]
module"]
pub type DT39 = crate::Reg<dt39::DT39_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt39;
#[doc = "DT40 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt40`]
module"]
pub type DT40 = crate::Reg<dt40::DT40_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt40;
#[doc = "DT41 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt41`]
module"]
pub type DT41 = crate::Reg<dt41::DT41_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt41;
#[doc = "DT42 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dt42`]
module"]
pub type DT42 = crate::Reg<dt42::DT42_SPEC>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt42;
#[doc = "RTCCAL (rw) register accessor: RTC clock calibration register (BPR_RTCCAL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rtccal`]
module"]
pub type RTCCAL = crate::Reg<rtccal::RTCCAL_SPEC>;
#[doc = "RTC clock calibration register (BPR_RTCCAL)"]
pub mod rtccal;
#[doc = "CTRL (rw) register accessor: BPR control register (BPR_CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "BPR control register (BPR_CTRL)"]
pub mod ctrl;
#[doc = "CTRLSTS (rw) register accessor: BPR control/status register (BPR_CTRLSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrlsts`]
module"]
pub type CTRLSTS = crate::Reg<ctrlsts::CTRLSTS_SPEC>;
#[doc = "BPR control/status register (BPR_CTRLSTS)"]
pub mod ctrlsts;
