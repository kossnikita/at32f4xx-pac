#[doc = r"Register block"]
#[repr(C)]
#[derive(Debug)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x50],
    #[doc = "0x50 - TMR14 channel 1 input remap"]
    pub rmp: RMP,
}
#[doc = "RMP (rw) register accessor: TMR14 channel 1 input remap\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rmp`]
module"]
pub type RMP = crate::Reg<rmp::RMP_SPEC>;
#[doc = "TMR14 channel 1 input remap"]
pub mod rmp;
