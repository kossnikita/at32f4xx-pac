#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
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
    #[doc = "0x14 - Data offset for Preempted channel %s"]
    pub pcdto1: PCDTO,
    _reserved6: [u8; 0x02],
    #[doc = "0x18 - Data offset for Preempted channel %s"]
    pub pcdto2: PCDTO,
    _reserved7: [u8; 0x02],
    #[doc = "0x1c - Data offset for Preempted channel %s"]
    pub pcdto3: PCDTO,
    _reserved8: [u8; 0x02],
    #[doc = "0x20 - Data offset for Preempted channel %s"]
    pub pcdto4: PCDTO,
    _reserved9: [u8; 0x02],
    #[doc = "0x24 - Voltage monitoring %s boundary register"]
    pub vmhb: VMB,
    _reserved10: [u8; 0x02],
    #[doc = "0x28 - Voltage monitoring %s boundary register"]
    pub vmlb: VMB,
    _reserved11: [u8; 0x02],
    #[doc = "0x2c - Ordinary sequence register 1"]
    pub osq1: OSQ1,
    #[doc = "0x30 - Ordinary sequence register 2"]
    pub osq2: OSQ2,
    #[doc = "0x34 - Ordinary sequence register 3"]
    pub osq3: OSQ3,
    #[doc = "0x38 - Preempted sequence register"]
    pub psq: PSQ,
    #[doc = "0x3c - Preempted data register %s"]
    pub pdt1: PDT,
    _reserved16: [u8; 0x02],
    #[doc = "0x40 - Preempted data register %s"]
    pub pdt2: PDT,
    _reserved17: [u8; 0x02],
    #[doc = "0x44 - Preempted data register %s"]
    pub pdt3: PDT,
    _reserved18: [u8; 0x02],
    #[doc = "0x48 - Preempted data register %s"]
    pub pdt4: PDT,
    _reserved19: [u8; 0x02],
    #[doc = "0x4c - Ordinary data register"]
    pub odt: ODT,
    _reserved20: [u8; 0x32],
    #[doc = "0x80 - oversampling register"]
    pub ovsp: OVSP,
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
#[doc = "PCDTO (rw) register accessor: Data offset for Preempted channel %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pcdto`]
module"]
pub type PCDTO = crate::Reg<pcdto::PCDTO_SPEC>;
#[doc = "Data offset for Preempted channel %s"]
pub mod pcdto;
#[doc = "VMB (rw) register accessor: Voltage monitoring %s boundary register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`vmb`]
module"]
pub type VMB = crate::Reg<vmb::VMB_SPEC>;
#[doc = "Voltage monitoring %s boundary register"]
pub mod vmb;
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
#[doc = "PDT (r) register accessor: Preempted data register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pdt`]
module"]
pub type PDT = crate::Reg<pdt::PDT_SPEC>;
#[doc = "Preempted data register %s"]
pub mod pdt;
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
