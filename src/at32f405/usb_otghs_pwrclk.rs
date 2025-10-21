#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pcgcctl: PCGCCTL,
}
impl RegisterBlock {
    #[doc = "0x00 - OTGHS power and clock gating control register (OTGHS_PCGCCTL)"]
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &PCGCCTL {
        &self.pcgcctl
    }
}
#[doc = "PCGCCTL (rw) register accessor: OTGHS power and clock gating control register (OTGHS_PCGCCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`pcgcctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcgcctl`] module"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = "OTGHS power and clock gating control register (OTGHS_PCGCCTL)"]
pub mod pcgcctl;
