#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - status register"]
    pub sts: STS,
    #[doc = "0x04 - control register 1"]
    pub ctrl1: CTRL1,
    #[doc = "0x08 - control register 2"]
    pub ctrl2: CTRL2,
    #[doc = "0x0c - sample time register 1"]
    pub spt1: SPT1,
    #[doc = "0x10 - sample time register 2"]
    pub spt2: SPT2,
    #[doc = "0x14 - Preempted channel 1 data offset register"]
    pub pcdto1: PCDTO1,
    #[doc = "0x18 - Preempted channel 2 data offset register"]
    pub pcdto2: PCDTO2,
    #[doc = "0x1c - Preempted channel 3 data offset register"]
    pub pcdto3: PCDTO3,
    #[doc = "0x20 - Preempted channel 4 data offset register"]
    pub pcdto4: PCDTO4,
    #[doc = "0x24 - Voltage monitoring high boundary register"]
    pub vmhb: VMHB,
    #[doc = "0x28 - Voltage monitoring low boundary register"]
    pub vmlb: VMLB,
    #[doc = "0x2c - Ordinary sequence register 1"]
    pub osq1: OSQ1,
    #[doc = "0x30 - Ordinary sequence register 2"]
    pub osq2: OSQ2,
    #[doc = "0x34 - Ordinary sequence register 3"]
    pub osq3: OSQ3,
    #[doc = "0x38 - Preempted sequence register"]
    pub psq: PSQ,
    #[doc = "0x3c - Preempted data register 1"]
    pub pdt1: PDT1,
    #[doc = "0x40 - Preempted data register 2"]
    pub pdt2: PDT2,
    #[doc = "0x44 - Preempted data register 3"]
    pub pdt3: PDT3,
    #[doc = "0x48 - Preempted data register 4"]
    pub pdt4: PDT4,
    #[doc = "0x4c - Ordinary data register"]
    pub odt: ODT,
    _reserved20: [u8; 0x30],
    #[doc = "0x80 - oversampling register"]
    pub ovsp: OVSP,
    _reserved21: [u8; 0x30],
    #[doc = "0xb4 - Calibration value register"]
    pub calval: CALVAL,
}
#[doc = "STS (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "status register"]
pub mod sts;
#[doc = "CTRL1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl1`]
module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl2`]
module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "control register 2"]
pub mod ctrl2;
#[doc = "SPT1 (rw) register accessor: sample time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spt1`]
module"]
pub type SPT1 = crate::Reg<spt1::SPT1_SPEC>;
#[doc = "sample time register 1"]
pub mod spt1;
#[doc = "SPT2 (rw) register accessor: sample time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spt2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spt2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spt2`]
module"]
pub type SPT2 = crate::Reg<spt2::SPT2_SPEC>;
#[doc = "sample time register 2"]
pub mod spt2;
#[doc = "PCDTO1 (rw) register accessor: Preempted channel 1 data offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcdto1`]
module"]
pub type PCDTO1 = crate::Reg<pcdto1::PCDTO1_SPEC>;
#[doc = "Preempted channel 1 data offset register"]
pub mod pcdto1;
#[doc = "PCDTO2 (rw) register accessor: Preempted channel 2 data offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcdto2`]
module"]
pub type PCDTO2 = crate::Reg<pcdto2::PCDTO2_SPEC>;
#[doc = "Preempted channel 2 data offset register"]
pub mod pcdto2;
#[doc = "PCDTO3 (rw) register accessor: Preempted channel 3 data offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcdto3`]
module"]
pub type PCDTO3 = crate::Reg<pcdto3::PCDTO3_SPEC>;
#[doc = "Preempted channel 3 data offset register"]
pub mod pcdto3;
#[doc = "PCDTO4 (rw) register accessor: Preempted channel 4 data offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcdto4`]
module"]
pub type PCDTO4 = crate::Reg<pcdto4::PCDTO4_SPEC>;
#[doc = "Preempted channel 4 data offset register"]
pub mod pcdto4;
#[doc = "VMHB (rw) register accessor: Voltage monitoring high boundary register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmhb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmhb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`vmhb`]
module"]
pub type VMHB = crate::Reg<vmhb::VMHB_SPEC>;
#[doc = "Voltage monitoring high boundary register"]
pub mod vmhb;
#[doc = "VMLB (rw) register accessor: Voltage monitoring low boundary register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`vmlb`]
module"]
pub type VMLB = crate::Reg<vmlb::VMLB_SPEC>;
#[doc = "Voltage monitoring low boundary register"]
pub mod vmlb;
#[doc = "OSQ1 (rw) register accessor: Ordinary sequence register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osq1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osq1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`osq1`]
module"]
pub type OSQ1 = crate::Reg<osq1::OSQ1_SPEC>;
#[doc = "Ordinary sequence register 1"]
pub mod osq1;
#[doc = "OSQ2 (rw) register accessor: Ordinary sequence register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osq2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osq2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`osq2`]
module"]
pub type OSQ2 = crate::Reg<osq2::OSQ2_SPEC>;
#[doc = "Ordinary sequence register 2"]
pub mod osq2;
#[doc = "OSQ3 (rw) register accessor: Ordinary sequence register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osq3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osq3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`osq3`]
module"]
pub type OSQ3 = crate::Reg<osq3::OSQ3_SPEC>;
#[doc = "Ordinary sequence register 3"]
pub mod osq3;
#[doc = "PSQ (rw) register accessor: Preempted sequence register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`psq`]
module"]
pub type PSQ = crate::Reg<psq::PSQ_SPEC>;
#[doc = "Preempted sequence register"]
pub mod psq;
#[doc = "PDT1 (r) register accessor: Preempted data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdt1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pdt1`]
module"]
pub type PDT1 = crate::Reg<pdt1::PDT1_SPEC>;
#[doc = "Preempted data register 1"]
pub mod pdt1;
#[doc = "PDT2 (r) register accessor: Preempted data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdt2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pdt2`]
module"]
pub type PDT2 = crate::Reg<pdt2::PDT2_SPEC>;
#[doc = "Preempted data register 2"]
pub mod pdt2;
#[doc = "PDT3 (r) register accessor: Preempted data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdt3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pdt3`]
module"]
pub type PDT3 = crate::Reg<pdt3::PDT3_SPEC>;
#[doc = "Preempted data register 3"]
pub mod pdt3;
#[doc = "PDT4 (r) register accessor: Preempted data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdt4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pdt4`]
module"]
pub type PDT4 = crate::Reg<pdt4::PDT4_SPEC>;
#[doc = "Preempted data register 4"]
pub mod pdt4;
#[doc = "ODT (r) register accessor: Ordinary data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`odt`]
module"]
pub type ODT = crate::Reg<odt::ODT_SPEC>;
#[doc = "Ordinary data register"]
pub mod odt;
#[doc = "OVSP (rw) register accessor: oversampling register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ovsp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ovsp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ovsp`]
module"]
pub type OVSP = crate::Reg<ovsp::OVSP_SPEC>;
#[doc = "oversampling register"]
pub mod ovsp;
#[doc = "CALVAL (rw) register accessor: Calibration value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`calval`]
module"]
pub type CALVAL = crate::Reg<calval::CALVAL_SPEC>;
#[doc = "Calibration value register"]
pub mod calval;
