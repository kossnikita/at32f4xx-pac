#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sts: STS,
    dt: DT,
    _reserved2: [u8; 0x02],
    baudr: BAUDR,
    _reserved3: [u8; 0x02],
    ctrl1: CTRL1,
    ctrl2: CTRL2,
    ctrl3: CTRL3,
    gdiv: GDIV,
    rtov: RTOV,
    ifc: IFC,
}
impl RegisterBlock {
    #[doc = "0x00 - Status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &STS {
        &self.sts
    }
    #[doc = "0x04 - Data register"]
    #[inline(always)]
    pub const fn dt(&self) -> &DT {
        &self.dt
    }
    #[doc = "0x08 - Baud rate register"]
    #[inline(always)]
    pub const fn baudr(&self) -> &BAUDR {
        &self.baudr
    }
    #[doc = "0x0c - Control register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &CTRL1 {
        &self.ctrl1
    }
    #[doc = "0x10 - Control register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &CTRL2 {
        &self.ctrl2
    }
    #[doc = "0x14 - Control register 3"]
    #[inline(always)]
    pub const fn ctrl3(&self) -> &CTRL3 {
        &self.ctrl3
    }
    #[doc = "0x18 - Guard time and division register"]
    #[inline(always)]
    pub const fn gdiv(&self) -> &GDIV {
        &self.gdiv
    }
    #[doc = "0x1c - Receiver time out value register"]
    #[inline(always)]
    pub const fn rtov(&self) -> &RTOV {
        &self.rtov
    }
    #[doc = "0x20 - Interruption flag clear register"]
    #[inline(always)]
    pub const fn ifc(&self) -> &IFC {
        &self.ifc
    }
}
#[doc = "STS (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
pub type STS = crate::Reg<sts::STS_SPEC>;
#[doc = "Status register"]
pub mod sts;
#[doc = "DT (rw) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`]
module"]
pub type DT = crate::Reg<dt::DT_SPEC>;
#[doc = "Data register"]
pub mod dt;
#[doc = "BAUDR (rw) register accessor: Baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`baudr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baudr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baudr`]
module"]
pub type BAUDR = crate::Reg<baudr::BAUDR_SPEC>;
#[doc = "Baud rate register"]
pub mod baudr;
#[doc = "CTRL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`]
module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Control register 2"]
pub mod ctrl2;
#[doc = "CTRL3 (rw) register accessor: Control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl3`]
module"]
pub type CTRL3 = crate::Reg<ctrl3::CTRL3_SPEC>;
#[doc = "Control register 3"]
pub mod ctrl3;
#[doc = "GDIV (rw) register accessor: Guard time and division register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdiv`]
module"]
pub type GDIV = crate::Reg<gdiv::GDIV_SPEC>;
#[doc = "Guard time and division register"]
pub mod gdiv;
#[doc = "RTOV (rw) register accessor: Receiver time out value register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtov::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtov::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtov`]
module"]
pub type RTOV = crate::Reg<rtov::RTOV_SPEC>;
#[doc = "Receiver time out value register"]
pub mod rtov;
#[doc = "IFC (rw) register accessor: Interruption flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifc`]
module"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interruption flag clear register"]
pub mod ifc;
