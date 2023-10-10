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
pub type OVFSWTR_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, OVFSWTRW_AW>;
impl<'a, REG, const O: u8> OVFSWTR_W<'a, REG, O>
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
pub type CSWTR_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, C1SWTRW_AW>;
impl<'a, REG, const O: u8> CSWTR_W<'a, REG, O>
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
pub type HALLSWTR_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, HALLSWTRW_AW>;
impl<'a, REG, const O: u8> HALLSWTR_W<'a, REG, O>
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
pub type TRGSWTR_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, TRGSWTRW_AW>;
impl<'a, REG, const O: u8> TRGSWTR_W<'a, REG, O>
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
pub type BRKSWTR_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, BRKSWTRW_AW>;
impl<'a, REG, const O: u8> BRKSWTR_W<'a, REG, O>
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
    pub fn ovfswtr(&mut self) -> OVFSWTR_W<SWEVT_SPEC, 0> {
        OVFSWTR_W::new(self)
    }
    #[doc = "Channel [1-4]
event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn cswtr<const O: u8>(&mut self) -> CSWTR_W<SWEVT_SPEC, O> {
        CSWTR_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn c1swtr(&mut self) -> CSWTR_W<SWEVT_SPEC, 1> {
        CSWTR_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn c2swtr(&mut self) -> CSWTR_W<SWEVT_SPEC, 2> {
        CSWTR_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn c3swtr(&mut self) -> CSWTR_W<SWEVT_SPEC, 3> {
        CSWTR_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn c4swtr(&mut self) -> CSWTR_W<SWEVT_SPEC, 4> {
        CSWTR_W::new(self)
    }
    #[doc = "Bit 5 - HALL event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn hallswtr(&mut self) -> HALLSWTR_W<SWEVT_SPEC, 5> {
        HALLSWTR_W::new(self)
    }
    #[doc = "Bit 6 - Trigger event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn trgswtr(&mut self) -> TRGSWTR_W<SWEVT_SPEC, 6> {
        TRGSWTR_W::new(self)
    }
    #[doc = "Bit 7 - Brake event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn brkswtr(&mut self) -> BRKSWTR_W<SWEVT_SPEC, 7> {
        BRKSWTR_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xe3;
}
#[doc = "`reset()` method sets SWEVT to value 0"]
impl crate::Resettable for SWEVT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
