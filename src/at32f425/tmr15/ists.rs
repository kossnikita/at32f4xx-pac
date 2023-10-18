#[doc = "Register `ISTS` reader"]
pub type R = crate::R<ISTS_SPEC>;
#[doc = "Register `ISTS` writer"]
pub type W = crate::W<ISTS_SPEC>;
#[doc = "Field `OVFIF` reader - Overflow interrupt flag"]
pub type OVFIF_R = crate::BitReader<OVFIFR_A>;
#[doc = "Overflow interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFIFR_A {
    #[doc = "0: No overflow event occurs"]
    NoOverflow = 0,
    #[doc = "1: An overflow event is generated"]
    Overflow = 1,
}
impl From<OVFIFR_A> for bool {
    #[inline(always)]
    fn from(variant: OVFIFR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVFIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVFIFR_A {
        match self.bits {
            false => OVFIFR_A::NoOverflow,
            true => OVFIFR_A::Overflow,
        }
    }
    #[doc = "No overflow event occurs"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == OVFIFR_A::NoOverflow
    }
    #[doc = "An overflow event is generated"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == OVFIFR_A::Overflow
    }
}
#[doc = "Overflow interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFIFW_AW {
    #[doc = "0: Overflow interrupt flag clear"]
    Clear = 0,
}
impl From<OVFIFW_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFIFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVFIF` writer - Overflow interrupt flag"]
pub type OVFIF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, OVFIFW_AW>;
impl<'a, REG, const O: u8> OVFIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overflow interrupt flag clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OVFIFW_AW::Clear)
    }
}
#[doc = "Field `CIF[1-2]` reader - Channel %s interrupt flag"]
pub type CIF_R = crate::BitReader<C1IFR_A>;
#[doc = "Channel %s interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1IFR_A {
    #[doc = "0: No compare event occurs"]
    NoEvent = 0,
    #[doc = "1: Capture/Compare event is generated"]
    CaptureCompare = 1,
}
impl From<C1IFR_A> for bool {
    #[inline(always)]
    fn from(variant: C1IFR_A) -> Self {
        variant as u8 != 0
    }
}
impl CIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1IFR_A {
        match self.bits {
            false => C1IFR_A::NoEvent,
            true => C1IFR_A::CaptureCompare,
        }
    }
    #[doc = "No compare event occurs"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == C1IFR_A::NoEvent
    }
    #[doc = "Capture/Compare event is generated"]
    #[inline(always)]
    pub fn is_capture_compare(&self) -> bool {
        *self == C1IFR_A::CaptureCompare
    }
}
#[doc = "Channel %s interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1IFW_AW {
    #[doc = "0: Interrupt flag clear"]
    Clear = 0,
}
impl From<C1IFW_AW> for bool {
    #[inline(always)]
    fn from(variant: C1IFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIF[1-2]` writer - Channel %s interrupt flag"]
pub type CIF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, C1IFW_AW>;
impl<'a, REG, const O: u8> CIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt flag clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(C1IFW_AW::Clear)
    }
}
#[doc = "Field `HALLIF` reader - HALL interrupt flag"]
pub type HALLIF_R = crate::BitReader;
#[doc = "Field `HALLIF` writer - HALL interrupt flag"]
pub type HALLIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRGIF` reader - Trigger interrupt flag"]
pub type TRGIF_R = crate::BitReader<TRGIFR_A>;
#[doc = "Trigger interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGIFR_A {
    #[doc = "0: No trigger event occurs"]
    NoEvent = 0,
    #[doc = "1: Trigger event is generated"]
    TriggerEvent = 1,
}
impl From<TRGIFR_A> for bool {
    #[inline(always)]
    fn from(variant: TRGIFR_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TRGIFR_A {
        match self.bits {
            false => TRGIFR_A::NoEvent,
            true => TRGIFR_A::TriggerEvent,
        }
    }
    #[doc = "No trigger event occurs"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == TRGIFR_A::NoEvent
    }
    #[doc = "Trigger event is generated"]
    #[inline(always)]
    pub fn is_trigger_event(&self) -> bool {
        *self == TRGIFR_A::TriggerEvent
    }
}
#[doc = "Trigger interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGIFW_AW {
    #[doc = "0: Trigger interrupt flag clear"]
    Clear = 0,
}
impl From<TRGIFW_AW> for bool {
    #[inline(always)]
    fn from(variant: TRGIFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGIF` writer - Trigger interrupt flag"]
pub type TRGIF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, TRGIFW_AW>;
impl<'a, REG, const O: u8> TRGIF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger interrupt flag clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TRGIFW_AW::Clear)
    }
}
#[doc = "Field `BRKIF` reader - Brake interrupt flag"]
pub type BRKIF_R = crate::BitReader;
#[doc = "Field `BRKIF` writer - Brake interrupt flag"]
pub type BRKIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CRF[1-2]` reader - Channel %s recapture flag"]
pub type CRF_R = crate::BitReader<C1RFR_A>;
#[doc = "Channel %s recapture flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1RFR_A {
    #[doc = "0: No capture is detected"]
    NoEvent = 0,
    #[doc = "1: Capture is detected"]
    Capture = 1,
}
impl From<C1RFR_A> for bool {
    #[inline(always)]
    fn from(variant: C1RFR_A) -> Self {
        variant as u8 != 0
    }
}
impl CRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1RFR_A {
        match self.bits {
            false => C1RFR_A::NoEvent,
            true => C1RFR_A::Capture,
        }
    }
    #[doc = "No capture is detected"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == C1RFR_A::NoEvent
    }
    #[doc = "Capture is detected"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == C1RFR_A::Capture
    }
}
#[doc = "Channel %s recapture flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1RFW_AW {
    #[doc = "0: Recapture flag clear"]
    Clear = 0,
}
impl From<C1RFW_AW> for bool {
    #[inline(always)]
    fn from(variant: C1RFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRF[1-2]` writer - Channel %s recapture flag"]
pub type CRF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, C1RFW_AW>;
impl<'a, REG, const O: u8> CRF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Recapture flag clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(C1RFW_AW::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Overflow interrupt flag"]
    #[inline(always)]
    pub fn ovfif(&self) -> OVFIF_R {
        OVFIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Channel [1-2]
interrupt flag"]
    #[inline(always)]
    pub unsafe fn cif(&self, n: u8) -> CIF_R {
        CIF_R::new(((self.bits >> (n - 1 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 interrupt flag"]
    #[inline(always)]
    pub fn c1if(&self) -> CIF_R {
        CIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 interrupt flag"]
    #[inline(always)]
    pub fn c2if(&self) -> CIF_R {
        CIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - HALL interrupt flag"]
    #[inline(always)]
    pub fn hallif(&self) -> HALLIF_R {
        HALLIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgif(&self) -> TRGIF_R {
        TRGIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Brake interrupt flag"]
    #[inline(always)]
    pub fn brkif(&self) -> BRKIF_R {
        BRKIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Channel [1-2]
recapture flag"]
    #[inline(always)]
    pub unsafe fn crf(&self, n: u8) -> CRF_R {
        CRF_R::new(((self.bits >> (n - 1 + 9)) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 recapture flag"]
    #[inline(always)]
    pub fn c1rf(&self) -> CRF_R {
        CRF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 recapture flag"]
    #[inline(always)]
    pub fn c2rf(&self) -> CRF_R {
        CRF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISTS")
            .field("c1rf", &format_args!("{}", self.c1rf().bit()))
            .field("c2rf", &format_args!("{}", self.c2rf().bit()))
            .field("brkif", &format_args!("{}", self.brkif().bit()))
            .field("trgif", &format_args!("{}", self.trgif().bit()))
            .field("hallif", &format_args!("{}", self.hallif().bit()))
            .field("c1if", &format_args!("{}", self.c1if().bit()))
            .field("c2if", &format_args!("{}", self.c2if().bit()))
            .field("ovfif", &format_args!("{}", self.ovfif().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<ISTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovfif(&mut self) -> OVFIF_W<ISTS_SPEC, 0> {
        OVFIF_W::new(self)
    }
    #[doc = "Channel [1-2]
interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn cif<const O: u8>(&mut self) -> CIF_W<ISTS_SPEC, O> {
        CIF_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn c1if(&mut self) -> CIF_W<ISTS_SPEC, 1> {
        CIF_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn c2if(&mut self) -> CIF_W<ISTS_SPEC, 2> {
        CIF_W::new(self)
    }
    #[doc = "Bit 5 - HALL interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn hallif(&mut self) -> HALLIF_W<ISTS_SPEC, 5> {
        HALLIF_W::new(self)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgif(&mut self) -> TRGIF_W<ISTS_SPEC, 6> {
        TRGIF_W::new(self)
    }
    #[doc = "Bit 7 - Brake interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn brkif(&mut self) -> BRKIF_W<ISTS_SPEC, 7> {
        BRKIF_W::new(self)
    }
    #[doc = "Channel [1-2]
recapture flag"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn crf<const O: u8>(&mut self) -> CRF_W<ISTS_SPEC, O> {
        CRF_W::new(self)
    }
    #[doc = "Bit 9 - Channel 1 recapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn c1rf(&mut self) -> CRF_W<ISTS_SPEC, 9> {
        CRF_W::new(self)
    }
    #[doc = "Bit 10 - Channel 2 recapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn c2rf(&mut self) -> CRF_W<ISTS_SPEC, 10> {
        CRF_W::new(self)
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
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ists::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ists::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISTS_SPEC;
impl crate::RegisterSpec for ISTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ists::R`](R) reader structure"]
impl crate::Readable for ISTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ists::W`](W) writer structure"]
impl crate::Writable for ISTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0243;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISTS to value 0"]
impl crate::Resettable for ISTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
