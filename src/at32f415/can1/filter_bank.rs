#[doc = r"Register block"]
#[repr(C)]
pub struct FILTER_BANK {
    #[doc = "0x00..0x08 - CAN filter bank filter bit register"]
    pub ffb: [FFB; 2],
}
impl FILTER_BANK {
    #[doc = "0x00 - CAN filter bank filter bit register"]
    #[inline(always)]
    pub fn ffb1(&self) -> &FFB {
        &self.ffb[0]
    }
    #[doc = "0x04 - CAN filter bank filter bit register"]
    #[inline(always)]
    pub fn ffb2(&self) -> &FFB {
        &self.ffb[1]
    }
}
#[doc = "FFB (rw) register accessor: CAN filter bank filter bit register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ffb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ffb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ffb`]
module"]
pub type FFB = crate::Reg<ffb::FFB_SPEC>;
#[doc = "CAN filter bank filter bit register"]
pub mod ffb;
