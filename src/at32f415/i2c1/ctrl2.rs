#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `CLKFREQ` reader - Input clock frequency"]
pub type CLKFREQ_R = crate::FieldReader;
#[doc = "Field `CLKFREQ` writer - Input clock frequency"]
pub type CLKFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errienr {
    #[doc = "0: Error interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Error interrupt is enabled"]
    Enabled = 1,
}
impl From<Errienr> for bool {
    #[inline(always)]
    fn from(variant: Errienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIEN` reader - Error interrupt enable"]
pub type ERRIEN_R = crate::BitReader<Errienr>;
impl ERRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errienr {
        match self.bits {
            false => Errienr::Disabled,
            true => Errienr::Enabled,
        }
    }
    #[doc = "Error interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Errienr::Disabled
    }
    #[doc = "Error interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Errienr::Enabled
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrienwWO {
    #[doc = "0: Disable Error interrupt"]
    Disable = 0,
    #[doc = "1: Enable Error interrupt"]
    Enable = 1,
}
impl From<ErrienwWO> for bool {
    #[inline(always)]
    fn from(variant: ErrienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIEN` writer - Error interrupt enable"]
pub type ERRIEN_W<'a, REG> = crate::BitWriter<'a, REG, ErrienwWO>;
impl<'a, REG> ERRIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Error interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ErrienwWO::Disable)
    }
    #[doc = "Enable Error interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ErrienwWO::Enable)
    }
}
#[doc = "Event interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Evtienr {
    #[doc = "0: Event interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Event interrupt is enabled"]
    Enabled = 1,
}
impl From<Evtienr> for bool {
    #[inline(always)]
    fn from(variant: Evtienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVTIEN` reader - Event interrupt enable"]
pub type EVTIEN_R = crate::BitReader<Evtienr>;
impl EVTIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Evtienr {
        match self.bits {
            false => Evtienr::Disabled,
            true => Evtienr::Enabled,
        }
    }
    #[doc = "Event interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Evtienr::Disabled
    }
    #[doc = "Event interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Evtienr::Enabled
    }
}
#[doc = "Event interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvtienwWO {
    #[doc = "0: Disable Event interrupt"]
    Disable = 0,
    #[doc = "1: Enable Event interrupt"]
    Enable = 1,
}
impl From<EvtienwWO> for bool {
    #[inline(always)]
    fn from(variant: EvtienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVTIEN` writer - Event interrupt enable"]
pub type EVTIEN_W<'a, REG> = crate::BitWriter<'a, REG, EvtienwWO>;
impl<'a, REG> EVTIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Event interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EvtienwWO::Disable)
    }
    #[doc = "Enable Event interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EvtienwWO::Enable)
    }
}
#[doc = "Data transmission interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dataienr {
    #[doc = "0: Data transfer interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Data transfer interrupt is enabled"]
    Enabled = 1,
}
impl From<Dataienr> for bool {
    #[inline(always)]
    fn from(variant: Dataienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAIEN` reader - Data transmission interrupt enable"]
pub type DATAIEN_R = crate::BitReader<Dataienr>;
impl DATAIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dataienr {
        match self.bits {
            false => Dataienr::Disabled,
            true => Dataienr::Enabled,
        }
    }
    #[doc = "Data transfer interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dataienr::Disabled
    }
    #[doc = "Data transfer interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dataienr::Enabled
    }
}
#[doc = "Data transmission interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataienwWO {
    #[doc = "0: Disable Data transfer interrupt"]
    Disable = 0,
    #[doc = "1: Enable Data transfer interrupt"]
    Enable = 1,
}
impl From<DataienwWO> for bool {
    #[inline(always)]
    fn from(variant: DataienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAIEN` writer - Data transmission interrupt enable"]
pub type DATAIEN_W<'a, REG> = crate::BitWriter<'a, REG, DataienwWO>;
impl<'a, REG> DATAIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Data transfer interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DataienwWO::Disable)
    }
    #[doc = "Enable Data transfer interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DataienwWO::Enable)
    }
}
#[doc = "DMA transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaenr {
    #[doc = "0: DMA transfer is disabled"]
    Disabled = 0,
    #[doc = "1: DMA transfer is enabled"]
    Enabled = 1,
}
impl From<Dmaenr> for bool {
    #[inline(always)]
    fn from(variant: Dmaenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA transfer enable"]
pub type DMAEN_R = crate::BitReader<Dmaenr>;
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaenr {
        match self.bits {
            false => Dmaenr::Disabled,
            true => Dmaenr::Enabled,
        }
    }
    #[doc = "DMA transfer is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmaenr::Disabled
    }
    #[doc = "DMA transfer is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmaenr::Enabled
    }
}
#[doc = "DMA transfer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaenwWO {
    #[doc = "0: Disable DMA transfer"]
    Disable = 0,
    #[doc = "1: Enable DMA transfer"]
    Enable = 1,
}
impl From<DmaenwWO> for bool {
    #[inline(always)]
    fn from(variant: DmaenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` writer - DMA transfer enable"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DmaenwWO>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DMA transfer"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmaenwWO::Disable)
    }
    #[doc = "Enable DMA transfer"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmaenwWO::Enable)
    }
}
#[doc = "DMA transfer end indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEND_A {
    #[doc = "0: Next DMA EOT is not the last transfer"]
    NotLast = 0,
    #[doc = "1: Next DMA EOT is the last transfer"]
    Last = 1,
}
impl From<DMAEND_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEND` reader - DMA transfer end indication"]
pub type DMAEND_R = crate::BitReader<DMAEND_A>;
impl DMAEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAEND_A {
        match self.bits {
            false => DMAEND_A::NotLast,
            true => DMAEND_A::Last,
        }
    }
    #[doc = "Next DMA EOT is not the last transfer"]
    #[inline(always)]
    pub fn is_not_last(&self) -> bool {
        *self == DMAEND_A::NotLast
    }
    #[doc = "Next DMA EOT is the last transfer"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == DMAEND_A::Last
    }
}
#[doc = "Field `DMAEND` writer - DMA transfer end indication"]
pub type DMAEND_W<'a, REG> = crate::BitWriter<'a, REG, DMAEND_A>;
impl<'a, REG> DMAEND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Next DMA EOT is not the last transfer"]
    #[inline(always)]
    pub fn not_last(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEND_A::NotLast)
    }
    #[doc = "Next DMA EOT is the last transfer"]
    #[inline(always)]
    pub fn last(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEND_A::Last)
    }
}
impl R {
    #[doc = "Bits 0:7 - Input clock frequency"]
    #[inline(always)]
    pub fn clkfreq(&self) -> CLKFREQ_R {
        CLKFREQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    pub fn errien(&self) -> ERRIEN_R {
        ERRIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    pub fn evtien(&self) -> EVTIEN_R {
        EVTIEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data transmission interrupt enable"]
    #[inline(always)]
    pub fn dataien(&self) -> DATAIEN_R {
        DATAIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA transfer enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA transfer end indication"]
    #[inline(always)]
    pub fn dmaend(&self) -> DMAEND_R {
        DMAEND_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("dmaend", &self.dmaend())
            .field("dmaen", &self.dmaen())
            .field("dataien", &self.dataien())
            .field("evtien", &self.evtien())
            .field("errien", &self.errien())
            .field("clkfreq", &self.clkfreq())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Input clock frequency"]
    #[inline(always)]
    #[must_use]
    pub fn clkfreq(&mut self) -> CLKFREQ_W<CTRL2_SPEC> {
        CLKFREQ_W::new(self, 0)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errien(&mut self) -> ERRIEN_W<CTRL2_SPEC> {
        ERRIEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn evtien(&mut self) -> EVTIEN_W<CTRL2_SPEC> {
        EVTIEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data transmission interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dataien(&mut self) -> DATAIEN_W<CTRL2_SPEC> {
        DATAIEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - DMA transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CTRL2_SPEC> {
        DMAEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - DMA transfer end indication"]
    #[inline(always)]
    #[must_use]
    pub fn dmaend(&mut self) -> DMAEND_W<CTRL2_SPEC> {
        DMAEND_W::new(self, 12)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: u32 = 0;
}
