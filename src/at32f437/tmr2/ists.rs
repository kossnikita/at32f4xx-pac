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
    pub fn variant(&self) -> OVFIFR_A {
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
#[doc = "Field `C1IF` reader - Channel 1 interrupt flag"]
pub type C1IF_R = crate::BitReader<C1IFR_A>;
#[doc = "Channel 1 interrupt flag\n\nValue on reset: 0"]
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
impl C1IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1IFR_A {
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
#[doc = "Channel 1 interrupt flag\n\nValue on reset: 0"]
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
#[doc = "Field `C1IF` writer - Channel 1 interrupt flag"]
pub type C1IF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, C1IFW_AW>;
impl<'a, REG, const O: u8> C1IF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt flag clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(C1IFW_AW::Clear)
    }
}
#[doc = "Field `C2IF` reader - Channel 2 interrupt flag"]
pub use C1IF_R as C2IF_R;
#[doc = "Field `C3IF` reader - Channel 3 interrupt flag"]
pub use C1IF_R as C3IF_R;
#[doc = "Field `C4IF` reader - Channel 4 interrupt flag"]
pub use C1IF_R as C4IF_R;
#[doc = "Field `C2IF` writer - Channel 2 interrupt flag"]
pub use C1IF_W as C2IF_W;
#[doc = "Field `C3IF` writer - Channel 3 interrupt flag"]
pub use C1IF_W as C3IF_W;
#[doc = "Field `C4IF` writer - Channel 4 interrupt flag"]
pub use C1IF_W as C4IF_W;
#[doc = "Field `TRGIF` reader - Trigger interrupt flag"]
pub type TRGIF_R = crate::BitReader;
#[doc = "Field `TRGIF` writer - Trigger interrupt flag"]
pub type TRGIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C1RF` reader - Channel 1 recapture flag"]
pub type C1RF_R = crate::BitReader<C1RFR_A>;
#[doc = "Channel 1 recapture flag\n\nValue on reset: 0"]
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
impl C1RF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> C1RFR_A {
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
#[doc = "Channel 1 recapture flag\n\nValue on reset: 0"]
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
#[doc = "Field `C1RF` writer - Channel 1 recapture flag"]
pub type C1RF_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, C1RFW_AW>;
impl<'a, REG, const O: u8> C1RF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Recapture flag clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(C1RFW_AW::Clear)
    }
}
#[doc = "Field `C2RF` reader - Channel 2 recapture flag"]
pub use C1RF_R as C2RF_R;
#[doc = "Field `C3RF` reader - Channel 3 recapture flag"]
pub use C1RF_R as C3RF_R;
#[doc = "Field `C4RF` reader - Channel 4 recapture flag"]
pub use C1RF_R as C4RF_R;
#[doc = "Field `C2RF` writer - Channel 2 recapture flag"]
pub use C1RF_W as C2RF_W;
#[doc = "Field `C3RF` writer - Channel 3 recapture flag"]
pub use C1RF_W as C3RF_W;
#[doc = "Field `C4RF` writer - Channel 4 recapture flag"]
pub use C1RF_W as C4RF_W;
impl R {
    #[doc = "Bit 0 - Overflow interrupt flag"]
    #[inline(always)]
    pub fn ovfif(&self) -> OVFIF_R {
        OVFIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 interrupt flag"]
    #[inline(always)]
    pub fn c1if(&self) -> C1IF_R {
        C1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 interrupt flag"]
    #[inline(always)]
    pub fn c2if(&self) -> C2IF_R {
        C2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 interrupt flag"]
    #[inline(always)]
    pub fn c3if(&self) -> C3IF_R {
        C3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 interrupt flag"]
    #[inline(always)]
    pub fn c4if(&self) -> C4IF_R {
        C4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgif(&self) -> TRGIF_R {
        TRGIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 recapture flag"]
    #[inline(always)]
    pub fn c1rf(&self) -> C1RF_R {
        C1RF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 recapture flag"]
    #[inline(always)]
    pub fn c2rf(&self) -> C2RF_R {
        C2RF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 recapture flag"]
    #[inline(always)]
    pub fn c3rf(&self) -> C3RF_R {
        C3RF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 recapture flag"]
    #[inline(always)]
    pub fn c4rf(&self) -> C4RF_R {
        C4RF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovfif(&mut self) -> OVFIF_W<ISTS_SPEC, 0> {
        OVFIF_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn c1if(&mut self) -> C1IF_W<ISTS_SPEC, 1> {
        C1IF_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn c2if(&mut self) -> C2IF_W<ISTS_SPEC, 2> {
        C2IF_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn c3if(&mut self) -> C3IF_W<ISTS_SPEC, 3> {
        C3IF_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn c4if(&mut self) -> C4IF_W<ISTS_SPEC, 4> {
        C4IF_W::new(self)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgif(&mut self) -> TRGIF_W<ISTS_SPEC, 6> {
        TRGIF_W::new(self)
    }
    #[doc = "Bit 9 - Channel 1 recapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn c1rf(&mut self) -> C1RF_W<ISTS_SPEC, 9> {
        C1RF_W::new(self)
    }
    #[doc = "Bit 10 - Channel 2 recapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn c2rf(&mut self) -> C2RF_W<ISTS_SPEC, 10> {
        C2RF_W::new(self)
    }
    #[doc = "Bit 11 - Channel 3 recapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn c3rf(&mut self) -> C3RF_W<ISTS_SPEC, 11> {
        C3RF_W::new(self)
    }
    #[doc = "Bit 12 - Channel 4 recapture flag"]
    #[inline(always)]
    #[must_use]
    pub fn c4rf(&mut self) -> C4RF_W<ISTS_SPEC, 12> {
        C4RF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x1e1f;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISTS to value 0"]
impl crate::Resettable for ISTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
