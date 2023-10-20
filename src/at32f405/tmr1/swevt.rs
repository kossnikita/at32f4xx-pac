#[doc = "Register `SWEVT` writer"]
pub type W = crate::W<SWEVT_SPEC>;
#[doc = "Overflow event triggered by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFSWTRW_AW {
    #[doc = "1: Generate an overflow event"]
    Overflow = 1,
}
impl From<OVFSWTRW_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFSWTRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFSWTR` writer - Overflow event triggered by software"]
pub type OVFSWTR_W<'a, REG> = crate::BitWriter1S<'a, REG, OVFSWTRW_AW>;
impl<'a, REG> OVFSWTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate an overflow event"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(OVFSWTRW_AW::Overflow)
    }
}
#[doc = "Channel %s event triggered by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1SWTRW_AW {
    #[doc = "1: Generate a channel event"]
    Event = 1,
}
impl From<C1SWTRW_AW> for bool {
    #[inline(always)]
    fn from(variant: C1SWTRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSWTR[1-4]` writer - Channel %s event triggered by software"]
pub type CSWTR_W<'a, REG> = crate::BitWriter1S<'a, REG, C1SWTRW_AW>;
impl<'a, REG> CSWTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a channel event"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(C1SWTRW_AW::Event)
    }
}
#[doc = "HALL event triggered by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALLSWTRW_AW {
    #[doc = "1: Generate a HALL event"]
    Event = 1,
}
impl From<HALLSWTRW_AW> for bool {
    #[inline(always)]
    fn from(variant: HALLSWTRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALLSWTR` writer - HALL event triggered by software"]
pub type HALLSWTR_W<'a, REG> = crate::BitWriter1S<'a, REG, HALLSWTRW_AW>;
impl<'a, REG> HALLSWTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a HALL event"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(HALLSWTRW_AW::Event)
    }
}
#[doc = "Trigger event triggered by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGSWTRW_AW {
    #[doc = "1: Generate a trigger event"]
    Event = 1,
}
impl From<TRGSWTRW_AW> for bool {
    #[inline(always)]
    fn from(variant: TRGSWTRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGSWTR` writer - Trigger event triggered by software"]
pub type TRGSWTR_W<'a, REG> = crate::BitWriter1S<'a, REG, TRGSWTRW_AW>;
impl<'a, REG> TRGSWTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a trigger event"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(TRGSWTRW_AW::Event)
    }
}
#[doc = "Brake event triggered by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRKSWTRW_AW {
    #[doc = "1: Generate a break event"]
    Event = 1,
}
impl From<BRKSWTRW_AW> for bool {
    #[inline(always)]
    fn from(variant: BRKSWTRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRKSWTR` writer - Brake event triggered by software"]
pub type BRKSWTR_W<'a, REG> = crate::BitWriter1S<'a, REG, BRKSWTRW_AW>;
impl<'a, REG> BRKSWTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a break event"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(BRKSWTRW_AW::Event)
    }
}
impl core::fmt::Debug for crate::generic::Reg<SWEVT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Overflow event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn ovfswtr(&mut self) -> OVFSWTR_W<SWEVT_SPEC> {
        OVFSWTR_W::new(self, 0)
    }
    #[doc = "Channel [1-4]
event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn cswtr(&mut self, n: u8) -> CSWTR_W<SWEVT_SPEC> {
        assert!(n < 4);
        CSWTR_W::new(self, n + 1)
    }
    #[doc = "Bit 1 - Channel 1 event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn c1swtr(&mut self) -> CSWTR_W<SWEVT_SPEC> {
        CSWTR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn c2swtr(&mut self) -> CSWTR_W<SWEVT_SPEC> {
        CSWTR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn c3swtr(&mut self) -> CSWTR_W<SWEVT_SPEC> {
        CSWTR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn c4swtr(&mut self) -> CSWTR_W<SWEVT_SPEC> {
        CSWTR_W::new(self, 4)
    }
    #[doc = "Bit 5 - HALL event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn hallswtr(&mut self) -> HALLSWTR_W<SWEVT_SPEC> {
        HALLSWTR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn trgswtr(&mut self) -> TRGSWTR_W<SWEVT_SPEC> {
        TRGSWTR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Brake event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn brkswtr(&mut self) -> BRKSWTR_W<SWEVT_SPEC> {
        BRKSWTR_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Software event register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWEVT_SPEC;
impl crate::RegisterSpec for SWEVT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swevt::W`](W) writer structure"]
impl crate::Writable for SWEVT_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0xe3;
}
#[doc = "`reset()` method sets SWEVT to value 0"]
impl crate::Resettable for SWEVT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
