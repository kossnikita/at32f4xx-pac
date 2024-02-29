#[repr(C)]
#[derive(Debug)]
#[doc = "Filter Bank %s"]
pub struct FilterBank {
    ffb: [FFB; 2],
}
impl FilterBank {
    #[doc = "0x00..0x08 - CAN filter bank filter bit register %s"]
    #[inline(always)]
    pub const fn ffb(&self, n: usize) -> &FFB {
        &self.ffb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - CAN filter bank filter bit register %s"]
    #[inline(always)]
    pub fn ffb_iter(&self) -> impl Iterator<Item = &FFB> {
        self.ffb.iter()
    }
    #[doc = "0x00 - CAN filter bank filter bit register 1"]
    #[inline(always)]
    pub const fn ffb1(&self) -> &FFB {
        self.ffb(0)
    }
    #[doc = "0x04 - CAN filter bank filter bit register 2"]
    #[inline(always)]
    pub const fn ffb2(&self) -> &FFB {
        self.ffb(1)
    }
}
#[doc = "FFB (rw) register accessor: CAN filter bank filter bit register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ffb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ffb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffb`]
module"]
pub type FFB = crate::Reg<ffb::FFB_SPEC>;
#[doc = "CAN filter bank filter bit register %s"]
pub mod ffb;
