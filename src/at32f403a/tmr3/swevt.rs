#[doc = "Register `SWEVT` reader"]
pub type R = crate::R<SWEVT_SPEC>;
#[doc = "Register `SWEVT` writer"]
pub type W = crate::W<SWEVT_SPEC>;
#[doc = "Field `OVFSWTR` reader - Overflow event triggered by software"]
pub type OVFSWTR_R = crate::BitReader<OVFSWTRW_A>;
#[doc = "Overflow event triggered by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFSWTRW_A {
    #[doc = "1: Generate an overflow event"]
    Overflow = 1,
}
impl From<OVFSWTRW_A> for bool {
    #[inline(always)]
    fn from(variant: OVFSWTRW_A) -> Self {
        variant as u8 != 0
    }
}
impl OVFSWTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OVFSWTRW_A> {
        match self.bits {
            true => Some(OVFSWTRW_A::Overflow),
            _ => None,
        }
    }
    #[doc = "Generate an overflow event"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == OVFSWTRW_A::Overflow
    }
}
#[doc = "Field `OVFSWTR` writer - Overflow event triggered by software"]
pub type OVFSWTR_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, OVFSWTRW_A>;
impl<'a, REG, const O: u8> OVFSWTR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate an overflow event"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(OVFSWTRW_A::Overflow)
    }
}
#[doc = "Field `CSWTR[1-4]` reader - Channel %s event triggered by software"]
pub type CSWTR_R = crate::BitReader<C1SWTRW_A>;
#[doc = "Channel %s event triggered by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1SWTRW_A {
    #[doc = "1: Generate a channel event"]
    Event = 1,
}
impl From<C1SWTRW_A> for bool {
    #[inline(always)]
    fn from(variant: C1SWTRW_A) -> Self {
        variant as u8 != 0
    }
}
impl CSWTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<C1SWTRW_A> {
        match self.bits {
            true => Some(C1SWTRW_A::Event),
            _ => None,
        }
    }
    #[doc = "Generate a channel event"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == C1SWTRW_A::Event
    }
}
#[doc = "Field `CSWTR[1-4]` writer - Channel %s event triggered by software"]
pub type CSWTR_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, C1SWTRW_A>;
impl<'a, REG, const O: u8> CSWTR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a channel event"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(C1SWTRW_A::Event)
    }
}
#[doc = "Field `TRGSWTR` reader - Trigger event triggered by software"]
pub type TRGSWTR_R = crate::BitReader<TRGSWTRW_A>;
#[doc = "Trigger event triggered by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGSWTRW_A {
    #[doc = "1: Generate a trigger event"]
    Event = 1,
}
impl From<TRGSWTRW_A> for bool {
    #[inline(always)]
    fn from(variant: TRGSWTRW_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGSWTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRGSWTRW_A> {
        match self.bits {
            true => Some(TRGSWTRW_A::Event),
            _ => None,
        }
    }
    #[doc = "Generate a trigger event"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == TRGSWTRW_A::Event
    }
}
#[doc = "Field `TRGSWTR` writer - Trigger event triggered by software"]
pub type TRGSWTR_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, TRGSWTRW_A>;
impl<'a, REG, const O: u8> TRGSWTR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a trigger event"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(TRGSWTRW_A::Event)
    }
}
impl R {
    #[doc = "Bit 0 - Overflow event triggered by software"]
    #[inline(always)]
    pub fn ovfswtr(&self) -> OVFSWTR_R {
        OVFSWTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Channel [1-4]
event triggered by software"]
    #[inline(always)]
    pub unsafe fn cswtr(&self, n: u8) -> CSWTR_R {
        CSWTR_R::new(((self.bits >> (n - 1 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 event triggered by software"]
    #[inline(always)]
    pub fn c1swtr(&self) -> CSWTR_R {
        CSWTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 event triggered by software"]
    #[inline(always)]
    pub fn c2swtr(&self) -> CSWTR_R {
        CSWTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 event triggered by software"]
    #[inline(always)]
    pub fn c3swtr(&self) -> CSWTR_R {
        CSWTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 event triggered by software"]
    #[inline(always)]
    pub fn c4swtr(&self) -> CSWTR_R {
        CSWTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger event triggered by software"]
    #[inline(always)]
    pub fn trgswtr(&self) -> TRGSWTR_R {
        TRGSWTR_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWEVT")
            .field("trgswtr", &format_args!("{}", self.trgswtr().bit()))
            .field("c1swtr", &format_args!("{}", self.c1swtr().bit()))
            .field("c2swtr", &format_args!("{}", self.c2swtr().bit()))
            .field("c3swtr", &format_args!("{}", self.c3swtr().bit()))
            .field("c4swtr", &format_args!("{}", self.c4swtr().bit()))
            .field("ovfswtr", &format_args!("{}", self.ovfswtr().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SWEVT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
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
    #[doc = "Bit 6 - Trigger event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn trgswtr(&mut self) -> TRGSWTR_W<SWEVT_SPEC, 6> {
        TRGSWTR_W::new(self)
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
#[doc = "Software event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swevt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWEVT_SPEC;
impl crate::RegisterSpec for SWEVT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swevt::R`](R) reader structure"]
impl crate::Readable for SWEVT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swevt::W`](W) writer structure"]
impl crate::Writable for SWEVT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x43;
}
#[doc = "`reset()` method sets SWEVT to value 0"]
impl crate::Resettable for SWEVT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
