#[doc = "Register `ISTS` reader"]
pub type R = crate::R<ISTS_SPEC>;
#[doc = "Register `ISTS` writer"]
pub type W = crate::W<ISTS_SPEC>;
#[doc = "Overflow interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovfifr {
    #[doc = "0: No overflow event occurs"]
    NoOverflow = 0,
    #[doc = "1: An overflow event is generated"]
    Overflow = 1,
}
impl From<Ovfifr> for bool {
    #[inline(always)]
    fn from(variant: Ovfifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFIF` reader - Overflow interrupt flag"]
pub type OVFIF_R = crate::BitReader<Ovfifr>;
impl OVFIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovfifr {
        match self.bits {
            false => Ovfifr::NoOverflow,
            true => Ovfifr::Overflow,
        }
    }
    #[doc = "No overflow event occurs"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == Ovfifr::NoOverflow
    }
    #[doc = "An overflow event is generated"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Ovfifr::Overflow
    }
}
#[doc = "Overflow interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OvfifwWO {
    #[doc = "0: Overflow interrupt flag clear"]
    Clear = 0,
}
impl From<OvfifwWO> for bool {
    #[inline(always)]
    fn from(variant: OvfifwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFIF` writer - Overflow interrupt flag"]
pub type OVFIF_W<'a, REG> = crate::BitWriter0C<'a, REG, OvfifwWO>;
impl<'a, REG> OVFIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overflow interrupt flag clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OvfifwWO::Clear)
    }
}
#[doc = "Channel %s interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1ifr {
    #[doc = "0: No compare event occurs"]
    NoEvent = 0,
    #[doc = "1: Capture/Compare event is generated"]
    CaptureCompare = 1,
}
impl From<C1ifr> for bool {
    #[inline(always)]
    fn from(variant: C1ifr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIF(1-1)` reader - Channel %s interrupt flag"]
pub type CIF_R = crate::BitReader<C1ifr>;
impl CIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1ifr {
        match self.bits {
            false => C1ifr::NoEvent,
            true => C1ifr::CaptureCompare,
        }
    }
    #[doc = "No compare event occurs"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == C1ifr::NoEvent
    }
    #[doc = "Capture/Compare event is generated"]
    #[inline(always)]
    pub fn is_capture_compare(&self) -> bool {
        *self == C1ifr::CaptureCompare
    }
}
#[doc = "Channel %s interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1ifwWO {
    #[doc = "0: Interrupt flag clear"]
    Clear = 0,
}
impl From<C1ifwWO> for bool {
    #[inline(always)]
    fn from(variant: C1ifwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIF(1-1)` writer - Channel %s interrupt flag"]
pub type CIF_W<'a, REG> = crate::BitWriter0C<'a, REG, C1ifwWO>;
impl<'a, REG> CIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt flag clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(C1ifwWO::Clear)
    }
}
#[doc = "Channel %s recapture flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1rfr {
    #[doc = "0: No capture is detected"]
    NoEvent = 0,
    #[doc = "1: Capture is detected"]
    Capture = 1,
}
impl From<C1rfr> for bool {
    #[inline(always)]
    fn from(variant: C1rfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRF(1-1)` reader - Channel %s recapture flag"]
pub type CRF_R = crate::BitReader<C1rfr>;
impl CRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1rfr {
        match self.bits {
            false => C1rfr::NoEvent,
            true => C1rfr::Capture,
        }
    }
    #[doc = "No capture is detected"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == C1rfr::NoEvent
    }
    #[doc = "Capture is detected"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == C1rfr::Capture
    }
}
#[doc = "Channel %s recapture flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1rfwWO {
    #[doc = "0: Recapture flag clear"]
    Clear = 0,
}
impl From<C1rfwWO> for bool {
    #[inline(always)]
    fn from(variant: C1rfwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRF(1-1)` writer - Channel %s recapture flag"]
pub type CRF_W<'a, REG> = crate::BitWriter0C<'a, REG, C1rfwWO>;
impl<'a, REG> CRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Recapture flag clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(C1rfwWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Overflow interrupt flag"]
    #[inline(always)]
    pub fn ovfif(&self) -> OVFIF_R {
        OVFIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Channel (1-1) interrupt flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1IF` field.</div>"]
    #[inline(always)]
    pub fn cif(&self, n: u8) -> CIF_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CIF_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-1) interrupt flag"]
    #[inline(always)]
    pub fn cif_iter(&self) -> impl Iterator<Item = CIF_R> + '_ {
        (0..1).map(move |n| CIF_R::new(((self.bits >> (n * 0 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Channel 1 interrupt flag"]
    #[inline(always)]
    pub fn c1if(&self) -> CIF_R {
        CIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Channel (1-1) recapture flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1RF` field.</div>"]
    #[inline(always)]
    pub fn crf(&self, n: u8) -> CRF_R {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CRF_R::new(((self.bits >> (n * 0 + 9)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Channel (1-1) recapture flag"]
    #[inline(always)]
    pub fn crf_iter(&self) -> impl Iterator<Item = CRF_R> + '_ {
        (0..1).map(move |n| CRF_R::new(((self.bits >> (n * 0 + 9)) & 1) != 0))
    }
    #[doc = "Bit 9 - Channel 1 recapture flag"]
    #[inline(always)]
    pub fn c1rf(&self) -> CRF_R {
        CRF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISTS")
            .field("c1rf", &self.c1rf())
            .field("c1if", &self.c1if())
            .field("ovfif", &self.ovfif())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovfif(&mut self) -> OVFIF_W<ISTS_SPEC> {
        OVFIF_W::new(self, 0)
    }
    #[doc = "Channel (1-1) interrupt flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1IF` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn cif(&mut self, n: u8) -> CIF_W<ISTS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CIF_W::new(self, n * 0 + 1)
    }
    #[doc = "Bit 1 - Channel 1 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn c1if(&mut self) -> CIF_W<ISTS_SPEC> {
        CIF_W::new(self, 1)
    }
    #[doc = "Channel (1-1) recapture flag"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `C1RF` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn crf(&mut self, n: u8) -> CRF_W<ISTS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 1][n as usize];
        CRF_W::new(self, n * 0 + 9)
    }
    #[doc = "Bit 9 - Channel 1 recapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn c1rf(&mut self) -> CRF_W<ISTS_SPEC> {
        CRF_W::new(self, 9)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ists::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ists::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISTS_SPEC;
impl crate::RegisterSpec for ISTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ists::R`](R) reader structure"]
impl crate::Readable for ISTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ists::W`](W) writer structure"]
impl crate::Writable for ISTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0203;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISTS to value 0"]
impl crate::Resettable for ISTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
