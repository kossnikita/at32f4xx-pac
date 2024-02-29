#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Voltage monitoring out of range flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vmorr {
    #[doc = "0: Voltage is within the value programmed"]
    InRange = 0,
    #[doc = "1: Voltage is outside the value programmed"]
    OutOfRange = 1,
}
impl From<Vmorr> for bool {
    #[inline(always)]
    fn from(variant: Vmorr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VMOR` reader - Voltage monitoring out of range flag"]
pub type VMOR_R = crate::BitReader<Vmorr>;
impl VMOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vmorr {
        match self.bits {
            false => Vmorr::InRange,
            true => Vmorr::OutOfRange,
        }
    }
    #[doc = "Voltage is within the value programmed"]
    #[inline(always)]
    pub fn is_in_range(&self) -> bool {
        *self == Vmorr::InRange
    }
    #[doc = "Voltage is outside the value programmed"]
    #[inline(always)]
    pub fn is_out_of_range(&self) -> bool {
        *self == Vmorr::OutOfRange
    }
}
#[doc = "Voltage monitoring out of range flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VmorwWO {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<VmorwWO> for bool {
    #[inline(always)]
    fn from(variant: VmorwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VMOR` writer - Voltage monitoring out of range flag"]
pub type VMOR_W<'a, REG> = crate::BitWriter0C<'a, REG, VmorwWO>;
impl<'a, REG> VMOR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(VmorwWO::Clear)
    }
}
#[doc = "Ordinary channels conversion end flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Occer {
    #[doc = "0: Conversion is not complete"]
    NotComplete = 0,
    #[doc = "1: Conversion is complete"]
    Complete = 1,
}
impl From<Occer> for bool {
    #[inline(always)]
    fn from(variant: Occer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCCE` reader - Ordinary channels conversion end flag"]
pub type OCCE_R = crate::BitReader<Occer>;
impl OCCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Occer {
        match self.bits {
            false => Occer::NotComplete,
            true => Occer::Complete,
        }
    }
    #[doc = "Conversion is not complete"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == Occer::NotComplete
    }
    #[doc = "Conversion is complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == Occer::Complete
    }
}
#[doc = "Ordinary channels conversion end flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OccewWO {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<OccewWO> for bool {
    #[inline(always)]
    fn from(variant: OccewWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OCCE` writer - Ordinary channels conversion end flag"]
pub type OCCE_W<'a, REG> = crate::BitWriter0C<'a, REG, OccewWO>;
impl<'a, REG> OCCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OccewWO::Clear)
    }
}
#[doc = "Field `PCCE` reader - Preempted channels conversion end flag"]
pub use OCCE_R as PCCE_R;
#[doc = "Field `PCCE` writer - Preempted channels conversion end flag"]
pub use OCCE_W as PCCE_W;
#[doc = "Preempted channel conversion start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pccsr {
    #[doc = "0: No channel conversion started"]
    Idle = 0,
    #[doc = "1: Channel conversion has started"]
    Started = 1,
}
impl From<Pccsr> for bool {
    #[inline(always)]
    fn from(variant: Pccsr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCCS` reader - Preempted channel conversion start flag"]
pub type PCCS_R = crate::BitReader<Pccsr>;
impl PCCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pccsr {
        match self.bits {
            false => Pccsr::Idle,
            true => Pccsr::Started,
        }
    }
    #[doc = "No channel conversion started"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Pccsr::Idle
    }
    #[doc = "Channel conversion has started"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == Pccsr::Started
    }
}
#[doc = "Preempted channel conversion start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PccswWO {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<PccswWO> for bool {
    #[inline(always)]
    fn from(variant: PccswWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCCS` writer - Preempted channel conversion start flag"]
pub type PCCS_W<'a, REG> = crate::BitWriter0C<'a, REG, PccswWO>;
impl<'a, REG> PCCS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PccswWO::Clear)
    }
}
#[doc = "Field `OCCS` reader - Ordinary channel conversion start flag"]
pub use PCCS_R as OCCS_R;
#[doc = "Field `OCCS` writer - Ordinary channel conversion start flag"]
pub use PCCS_W as OCCS_W;
#[doc = "Field `OCCO` reader - Ordinary channel conversion overflow flag"]
pub type OCCO_R = crate::BitReader;
#[doc = "Field `OCCO` writer - Ordinary channel conversion overflow flag"]
pub type OCCO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDY` reader - ADC ready to conversion flag"]
pub type RDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Voltage monitoring out of range flag"]
    #[inline(always)]
    pub fn vmor(&self) -> VMOR_R {
        VMOR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ordinary channels conversion end flag"]
    #[inline(always)]
    pub fn occe(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 1) & 1) != 0)
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
    #[doc = "Bit 5 - Ordinary channel conversion overflow flag"]
    #[inline(always)]
    pub fn occo(&self) -> OCCO_R {
        OCCO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC ready to conversion flag"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("rdy", &format_args!("{}", self.rdy().bit()))
            .field("occo", &format_args!("{}", self.occo().bit()))
            .field("pccs", &format_args!("{}", self.pccs().bit()))
            .field("occs", &format_args!("{}", self.occs().bit()))
            .field("occe", &format_args!("{}", self.occe().bit()))
            .field("pcce", &format_args!("{}", self.pcce().bit()))
            .field("vmor", &format_args!("{}", self.vmor().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage monitoring out of range flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmor(&mut self) -> VMOR_W<STS_SPEC> {
        VMOR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Ordinary channels conversion end flag"]
    #[inline(always)]
    #[must_use]
    pub fn occe(&mut self) -> OCCE_W<STS_SPEC> {
        OCCE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Preempted channels conversion end flag"]
    #[inline(always)]
    #[must_use]
    pub fn pcce(&mut self) -> PCCE_W<STS_SPEC> {
        PCCE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Preempted channel conversion start flag"]
    #[inline(always)]
    #[must_use]
    pub fn pccs(&mut self) -> PCCS_W<STS_SPEC> {
        PCCS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Ordinary channel conversion start flag"]
    #[inline(always)]
    #[must_use]
    pub fn occs(&mut self) -> OCCS_W<STS_SPEC> {
        OCCS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Ordinary channel conversion overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn occo(&mut self) -> OCCO_W<STS_SPEC> {
        OCCO_W::new(self, 5)
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
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1f;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: u32 = 0;
}
