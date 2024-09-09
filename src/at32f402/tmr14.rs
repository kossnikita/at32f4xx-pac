#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x50],
    rmp: RMP,
}
impl RegisterBlock {
    #[doc = "0x50 - TMR14 channel 1 input remap"]
    #[inline(always)]
    pub const fn rmp(&self) -> &RMP {
        &self.rmp
    }
}
#[doc = "RMP (rw) register accessor: TMR14 channel 1 input remap\n\nYou can [`read`](crate::Reg::read) this register and get [`rmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmp`]
module"]
pub type RMP = crate::Reg<rmp::RMP_SPEC>;
#[doc = "TMR14 channel 1 input remap"]
pub mod rmp;
