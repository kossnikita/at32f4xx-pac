#[doc = "Register `SWEVT` reader"]
pub type R = crate::R<SWEVT_SPEC>;
#[doc = "Register `SWEVT` writer"]
pub type W = crate::W<SWEVT_SPEC>;
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
#[doc = "Field `OVFSWTR` reader - Overflow event triggered by software"]
pub type OVFSWTR_R = crate::BitReader<OVFSWTRW_A>;
impl OVFSWTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OVFSWTRW_A> {
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
pub type OVFSWTR_W<'a, REG> = crate::BitWriter1S<'a, REG, OVFSWTRW_A>;
impl<'a, REG> OVFSWTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate an overflow event"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(OVFSWTRW_A::Overflow)
    }
}
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
#[doc = "Field `CSWTR(1-4)` reader - Channel %s event triggered by software"]
pub type CSWTR_R = crate::BitReader<C1SWTRW_A>;
impl CSWTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<C1SWTRW_A> {
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
#[doc = "Field `CSWTR(1-4)` writer - Channel %s event triggered by software"]
pub type CSWTR_W<'a, REG> = crate::BitWriter1S<'a, REG, C1SWTRW_A>;
impl<'a, REG> CSWTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate a channel event"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(C1SWTRW_A::Event)
    }
}
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
#[doc = "Field `TRGSWTR` reader - Trigger event triggered by software"]
pub type TRGSWTR_R = crate::BitReader<TRGSWTRW_A>;
impl TRGSWTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRGSWTRW_A> {
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
pub type TRGSWTR_W<'a, REG> = crate::BitWriter1S<'a, REG, TRGSWTRW_A>;
impl<'a, REG> TRGSWTR_W<'a, REG>
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
    #[doc = "Channel (1-4) event triggered by software"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1SWTR` field.</div>"]
    #[inline(always)]
    pub fn cswtr(&self, n: u8) -> CSWTR_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CSWTR_R::new(((self.bits >> (n + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-4) event triggered by software"]
    #[inline(always)]
    pub fn cswtr_iter(&self) -> impl Iterator<Item = CSWTR_R> + '_ {
        (0..4).map(move |n| CSWTR_R::new(((self.bits >> (n + 1)) & 1) != 0))
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
            .field("trgswtr", &self.trgswtr())
            .field("c1swtr", &self.c1swtr())
            .field("c2swtr", &self.c2swtr())
            .field("c3swtr", &self.c3swtr())
            .field("c4swtr", &self.c4swtr())
            .field("ovfswtr", &self.ovfswtr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Overflow event triggered by software"]
    #[inline(always)]
    pub fn ovfswtr(&mut self) -> OVFSWTR_W<'_, SWEVT_SPEC> {
        OVFSWTR_W::new(self, 0)
    }
    #[doc = "Channel (1-4) event triggered by software"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1SWTR` field.</div>"]
    #[inline(always)]
    pub fn cswtr(&mut self, n: u8) -> CSWTR_W<'_, SWEVT_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CSWTR_W::new(self, n + 1)
    }
    #[doc = "Bit 1 - Channel 1 event triggered by software"]
    #[inline(always)]
    pub fn c1swtr(&mut self) -> CSWTR_W<'_, SWEVT_SPEC> {
        CSWTR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 event triggered by software"]
    #[inline(always)]
    pub fn c2swtr(&mut self) -> CSWTR_W<'_, SWEVT_SPEC> {
        CSWTR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 event triggered by software"]
    #[inline(always)]
    pub fn c3swtr(&mut self) -> CSWTR_W<'_, SWEVT_SPEC> {
        CSWTR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 event triggered by software"]
    #[inline(always)]
    pub fn c4swtr(&mut self) -> CSWTR_W<'_, SWEVT_SPEC> {
        CSWTR_W::new(self, 4)
    }
    #[doc = "Bit 6 - Trigger event triggered by software"]
    #[inline(always)]
    pub fn trgswtr(&mut self) -> TRGSWTR_W<'_, SWEVT_SPEC> {
        TRGSWTR_W::new(self, 6)
    }
}
#[doc = "Software event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swevt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swevt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWEVT_SPEC;
impl crate::RegisterSpec for SWEVT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swevt::R`](R) reader structure"]
impl crate::Readable for SWEVT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swevt::W`](W) writer structure"]
impl crate::Writable for SWEVT_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x5f;
}
#[doc = "`reset()` method sets SWEVT to value 0"]
impl crate::Resettable for SWEVT_SPEC {}
