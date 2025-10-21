#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sts: STS,
    ctrl1: CTRL1,
    ctrl2: CTRL2,
    spt1: SPT1,
    spt2: SPT2,
    pcdto: [PCDTO; 4],
    vmb: [VMB; 2],
    osq1: OSQ1,
    osq2: OSQ2,
    osq3: OSQ3,
    psq: PSQ,
    pdt: [PDT; 4],
    odt: ODT,
    _reserved13: [u8; 0x30],
    ovsp: OVSP,
    _reserved14: [u8; 0x30],
    calval: CALVAL,
}
impl RegisterBlock {
    #[doc = "0x00 - status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &STS {
        &self.sts
    }
    #[doc = "0x04 - control register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    #[doc = "0x08 - control register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x0c - sample time register 1"]
    #[inline(always)]
    pub const fn spt1(&self) -> &SPT1 {
        &self.spt1
    }
    #[doc = "0x10 - sample time register 2"]
    #[inline(always)]
    pub const fn spt2(&self) -> &SPT2 {
        &self.spt2
    }
    #[doc = "0x14..0x24 - Data offset for Preempted channel %s"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `PCDTO1` register.</div>"]
    #[inline(always)]
    pub const fn pcdto(&self, n: usize) -> &PCDTO {
        &self.pcdto[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x24 - Data offset for Preempted channel %s"]
    #[inline(always)]
    pub fn pcdto_iter(&self) -> impl Iterator<Item = &PCDTO> {
        self.pcdto.iter()
    }
    #[doc = "0x14 - Data offset for Preempted channel 1"]
    #[inline(always)]
    pub const fn pcdto1(&self) -> &PCDTO {
        self.pcdto(0)
    }
    #[doc = "0x18 - Data offset for Preempted channel 2"]
    #[inline(always)]
    pub const fn pcdto2(&self) -> &PCDTO {
        self.pcdto(1)
    }
    #[doc = "0x1c - Data offset for Preempted channel 3"]
    #[inline(always)]
    pub const fn pcdto3(&self) -> &PCDTO {
        self.pcdto(2)
    }
    #[doc = "0x20 - Data offset for Preempted channel 4"]
    #[inline(always)]
    pub const fn pcdto4(&self) -> &PCDTO {
        self.pcdto(3)
    }
    #[doc = "0x24..0x2c - Voltage monitoring %s boundary register"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `VMHB` register.</div>"]
    #[inline(always)]
    pub const fn vmb(&self, n: usize) -> &VMB {
        &self.vmb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x24..0x2c - Voltage monitoring %s boundary register"]
    #[inline(always)]
    pub fn vmb_iter(&self) -> impl Iterator<Item = &VMB> {
        self.vmb.iter()
    }
    #[doc = "0x24 - Voltage monitoring H boundary register"]
    #[inline(always)]
    pub const fn vmhb(&self) -> &VMB {
        self.vmb(0)
    }
    #[doc = "0x28 - Voltage monitoring L boundary register"]
    #[inline(always)]
    pub const fn vmlb(&self) -> &VMB {
        self.vmb(1)
    }
    #[doc = "0x2c - Ordinary sequence register 1"]
    #[inline(always)]
    pub const fn osq1(&self) -> &OSQ1 {
        &self.osq1
    }
    #[doc = "0x30 - Ordinary sequence register 2"]
    #[inline(always)]
    pub const fn osq2(&self) -> &OSQ2 {
        &self.osq2
    }
    #[doc = "0x34 - Ordinary sequence register 3"]
    #[inline(always)]
    pub const fn osq3(&self) -> &OSQ3 {
        &self.osq3
    }
    #[doc = "0x38 - Preempted sequence register"]
    #[inline(always)]
    pub const fn psq(&self) -> &PSQ {
        &self.psq
    }
    #[doc = "0x3c..0x4c - Preempted data register %s"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `PDT1` register.</div>"]
    #[inline(always)]
    pub const fn pdt(&self, n: usize) -> &PDT {
        &self.pdt[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3c..0x4c - Preempted data register %s"]
    #[inline(always)]
    pub fn pdt_iter(&self) -> impl Iterator<Item = &PDT> {
        self.pdt.iter()
    }
    #[doc = "0x3c - Preempted data register 1"]
    #[inline(always)]
    pub const fn pdt1(&self) -> &PDT {
        self.pdt(0)
    }
    #[doc = "0x40 - Preempted data register 2"]
    #[inline(always)]
    pub const fn pdt2(&self) -> &PDT {
        self.pdt(1)
    }
    #[doc = "0x44 - Preempted data register 3"]
    #[inline(always)]
    pub const fn pdt3(&self) -> &PDT {
        self.pdt(2)
    }
    #[doc = "0x48 - Preempted data register 4"]
    #[inline(always)]
    pub const fn pdt4(&self) -> &PDT {
        self.pdt(3)
    }
    #[doc = "0x4c - Ordinary data register"]
    #[inline(always)]
    pub const fn odt(&self) -> &ODT {
        &self.odt
    }
    #[doc = "0x80 - oversampling register"]
    #[inline(always)]
    pub const fn ovsp(&self) -> &OVSP {
        &self.ovsp
    }
    #[doc = "0xb4 - Calibration value register"]
    #[inline(always)]
    pub const fn calval(&self) -> &CALVAL {
        &self.calval
    }
}
#[doc = "STS (rw) register accessor: status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`] module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "status register"]
pub mod sts;
#[doc = "CTRL1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "control register 2"]
pub mod ctrl2;
#[doc = "SPT1 (rw) register accessor: sample time register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`spt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spt1`] module"]
pub type SPT1 = crate::Reg<spt1::SPT1_SPEC>;
#[doc = "sample time register 1"]
pub mod spt1;
#[doc = "SPT2 (rw) register accessor: sample time register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`spt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spt2`] module"]
pub type SPT2 = crate::Reg<spt2::SPT2_SPEC>;
#[doc = "sample time register 2"]
pub mod spt2;
#[doc = "PCDTO (rw) register accessor: Data offset for Preempted channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`pcdto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcdto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcdto`] module"]
pub type PCDTO = crate::Reg<pcdto::PCDTO_SPEC>;
#[doc = "Data offset for Preempted channel %s"]
pub mod pcdto;
#[doc = "VMB (rw) register accessor: Voltage monitoring %s boundary register\n\nYou can [`read`](crate::Reg::read) this register and get [`vmb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vmb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmb`] module"]
pub type VMB = crate::Reg<vmb::VMB_SPEC>;
#[doc = "Voltage monitoring %s boundary register"]
pub mod vmb;
#[doc = "OSQ1 (rw) register accessor: Ordinary sequence register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`osq1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osq1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osq1`] module"]
pub type OSQ1 = crate::Reg<osq1::OSQ1_SPEC>;
#[doc = "Ordinary sequence register 1"]
pub mod osq1;
#[doc = "OSQ2 (rw) register accessor: Ordinary sequence register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`osq2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osq2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osq2`] module"]
pub type OSQ2 = crate::Reg<osq2::OSQ2_SPEC>;
#[doc = "Ordinary sequence register 2"]
pub mod osq2;
#[doc = "OSQ3 (rw) register accessor: Ordinary sequence register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`osq3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osq3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osq3`] module"]
pub type OSQ3 = crate::Reg<osq3::OSQ3_SPEC>;
#[doc = "Ordinary sequence register 3"]
pub mod osq3;
#[doc = "PSQ (rw) register accessor: Preempted sequence register\n\nYou can [`read`](crate::Reg::read) this register and get [`psq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psq`] module"]
pub type PSQ = crate::Reg<psq::PSQ_SPEC>;
#[doc = "Preempted sequence register"]
pub mod psq;
#[doc = "PDT (r) register accessor: Preempted data register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`pdt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdt`] module"]
pub type PDT = crate::Reg<pdt::PDT_SPEC>;
#[doc = "Preempted data register %s"]
pub mod pdt;
#[doc = "ODT (r) register accessor: Ordinary data register\n\nYou can [`read`](crate::Reg::read) this register and get [`odt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odt`] module"]
pub type ODT = crate::Reg<odt::ODT_SPEC>;
#[doc = "Ordinary data register"]
pub mod odt;
#[doc = "OVSP (rw) register accessor: oversampling register\n\nYou can [`read`](crate::Reg::read) this register and get [`ovsp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ovsp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ovsp`] module"]
pub type OVSP = crate::Reg<ovsp::OVSP_SPEC>;
#[doc = "oversampling register"]
pub mod ovsp;
#[doc = "CALVAL (rw) register accessor: Calibration value register\n\nYou can [`read`](crate::Reg::read) this register and get [`calval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calval`] module"]
pub type CALVAL = crate::Reg<calval::CALVAL_SPEC>;
#[doc = "Calibration value register"]
pub mod calval;
