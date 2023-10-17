#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Field `VMOR` reader - Voltage monitoring out of range flag"]
pub type VMOR_R = crate::BitReader<VMORR_A>;
#[doc = "Voltage monitoring out of range flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VMORR_A {
    #[doc = "0: Voltage is within the value programmed"]
    InRange = 0,
    #[doc = "1: Voltage is outside the value programmed"]
    OutOfRange = 1,
}
impl From<VMORR_A> for bool {
    #[inline(always)]
    fn from(variant: VMORR_A) -> Self {
        variant as u8 != 0
    }
}
impl VMOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VMORR_A {
        match self.bits {
            false => VMORR_A::InRange,
            true => VMORR_A::OutOfRange,
        }
    }
    #[doc = "Voltage is within the value programmed"]
    #[inline(always)]
    pub fn is_in_range(&self) -> bool {
        *self == VMORR_A::InRange
    }
    #[doc = "Voltage is outside the value programmed"]
    #[inline(always)]
    pub fn is_out_of_range(&self) -> bool {
        *self == VMORR_A::OutOfRange
    }
}
#[doc = "Voltage monitoring out of range flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VMORW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<VMORW_AW> for bool {
    #[inline(always)]
    fn from(variant: VMORW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VMOR` writer - Voltage monitoring out of range flag"]
pub type VMOR_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, VMORW_AW>;
impl<'a, REG, const O: u8> VMOR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(VMORW_AW::Clear)
    }
}
#[doc = "Field `CCE` reader - Channels conversion end flag"]
pub type CCE_R = crate::BitReader<CCER_A>;
#[doc = "Channels conversion end flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCER_A {
    #[doc = "0: Conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Conversion is complete"]
    Complete = 1,
}
impl From<CCER_A> for bool {
    #[inline(always)]
    fn from(variant: CCER_A) -> Self {
        variant as u8 != 0
    }
}
impl CCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCER_A {
        match self.bits {
            false => CCER_A::NotComplete,
            true => CCER_A::Complete,
        }
    }
    #[doc = "Conversion is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == CCER_A::NotComplete
    }
    #[doc = "Conversion is complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CCER_A::Complete
    }
}
#[doc = "Channels conversion end flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCEW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<CCEW_AW> for bool {
    #[inline(always)]
    fn from(variant: CCEW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCE` writer - Channels conversion end flag"]
pub type CCE_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, CCEW_AW>;
impl<'a, REG, const O: u8> CCE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CCEW_AW::Clear)
    }
}
#[doc = "Field `PCCE` reader - Preempted channels conversion end flag"]
pub use CCE_R as PCCE_R;
#[doc = "Field `PCCE` writer - Preempted channels conversion end flag"]
pub use CCE_W as PCCE_W;
#[doc = "Field `PCCS` reader - Preempted channel conversion start flag"]
pub type PCCS_R = crate::BitReader<PCCSR_A>;
#[doc = "Preempted channel conversion start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCCSR_A {
    #[doc = "0: No channel conversion started"]
    Idle = 0,
    #[doc = "1: Channel conversion has started"]
    Started = 1,
}
impl From<PCCSR_A> for bool {
    #[inline(always)]
    fn from(variant: PCCSR_A) -> Self {
        variant as u8 != 0
    }
}
impl PCCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCCSR_A {
        match self.bits {
            false => PCCSR_A::Idle,
            true => PCCSR_A::Started,
        }
    }
    #[doc = "No channel conversion started"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == PCCSR_A::Idle
    }
    #[doc = "Channel conversion has started"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == PCCSR_A::Started
    }
}
#[doc = "Preempted channel conversion start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCCSW_AW {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<PCCSW_AW> for bool {
    #[inline(always)]
    fn from(variant: PCCSW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCCS` writer - Preempted channel conversion start flag"]
pub type PCCS_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, PCCSW_AW>;
impl<'a, REG, const O: u8> PCCS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PCCSW_AW::Clear)
    }
}
#[doc = "Field `OCCS` reader - Ordinary channel conversion start flag"]
pub use PCCS_R as OCCS_R;
#[doc = "Field `OCCS` writer - Ordinary channel conversion start flag"]
pub use PCCS_W as OCCS_W;
impl R {
    #[doc = "Bit 0 - Voltage monitoring out of range flag"]
    #[inline(always)]
    pub fn vmor(&self) -> VMOR_R {
        VMOR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channels conversion end flag"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Preempted channels conversion end flag"]
    #[inline(always)]
    pub fn pcce(&self) -> PCCE_R {
        PCCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Preempted channel conversion start flag"]
    #[inline(always)]
    pub fn pccs(&self) -> PCCS_R {
        PCCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ordinary channel conversion start flag"]
    #[inline(always)]
    pub fn occs(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("pccs", &format_args!("{}", self.pccs().bit()))
            .field("occs", &format_args!("{}", self.occs().bit()))
            .field("cce", &format_args!("{}", self.cce().bit()))
            .field("pcce", &format_args!("{}", self.pcce().bit()))
            .field("vmor", &format_args!("{}", self.vmor().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage monitoring out of range flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmor(&mut self) -> VMOR_W<STS_SPEC, 0> {
        VMOR_W::new(self)
    }
    #[doc = "Bit 1 - Channels conversion end flag"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CCE_W<STS_SPEC, 1> {
        CCE_W::new(self)
    }
    #[doc = "Bit 2 - Preempted channels conversion end flag"]
    #[inline(always)]
    #[must_use]
    pub fn pcce(&mut self) -> PCCE_W<STS_SPEC, 2> {
        PCCE_W::new(self)
    }
    #[doc = "Bit 3 - Preempted channel conversion start flag"]
    #[inline(always)]
    #[must_use]
    pub fn pccs(&mut self) -> PCCS_W<STS_SPEC, 3> {
        PCCS_W::new(self)
    }
    #[doc = "Bit 4 - Ordinary channel conversion start flag"]
    #[inline(always)]
    #[must_use]
    pub fn occs(&mut self) -> OCCS_W<STS_SPEC, 4> {
        OCCS_W::new(self)
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
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x1f;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
