#[doc = "Register `CTRLSTS` reader"]
pub type R = crate::R<CTRLSTS_SPEC>;
#[doc = "Register `CTRLSTS` writer"]
pub type W = crate::W<CTRLSTS_SPEC>;
#[doc = "Tamper event flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPEFCLRW_A {
    #[doc = "1: Clear the tamper event flag"]
    Clear = 1,
}
impl From<TPEFCLRW_A> for bool {
    #[inline(always)]
    fn from(variant: TPEFCLRW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPEFCLR` writer - Tamper event flag clear"]
pub type TPEFCLR_W<'a, REG> = crate::BitWriter1C<'a, REG, TPEFCLRW_A>;
impl<'a, REG> TPEFCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the tamper event flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TPEFCLRW_A::Clear)
    }
}
#[doc = "Tamper interrupt flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPIFCLRW_A {
    #[doc = "1: Tamper interrupt flag clear"]
    Clear = 1,
}
impl From<TPIFCLRW_A> for bool {
    #[inline(always)]
    fn from(variant: TPIFCLRW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPIFCLR` writer - Tamper interrupt flag clear"]
pub type TPIFCLR_W<'a, REG> = crate::BitWriter1C<'a, REG, TPIFCLRW_A>;
impl<'a, REG> TPIFCLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper interrupt flag clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TPIFCLRW_A::Clear)
    }
}
#[doc = "Tamper pin interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpienr {
    #[doc = "0: Tamper pin interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Tamper pin interrupt is enabled"]
    Enabled = 1,
}
impl From<Tpienr> for bool {
    #[inline(always)]
    fn from(variant: Tpienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPIEN` reader - Tamper pin interrupt enable"]
pub type TPIEN_R = crate::BitReader<Tpienr>;
impl TPIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpienr {
        match self.bits {
            false => Tpienr::Disabled,
            true => Tpienr::Enabled,
        }
    }
    #[doc = "Tamper pin interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tpienr::Disabled
    }
    #[doc = "Tamper pin interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tpienr::Enabled
    }
}
#[doc = "Tamper pin interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TpienwWO {
    #[doc = "0: Tamper pin interrupt disable"]
    Disable = 0,
    #[doc = "1: Tamper pin interrupt enable"]
    Enable = 1,
}
impl From<TpienwWO> for bool {
    #[inline(always)]
    fn from(variant: TpienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPIEN` writer - Tamper pin interrupt enable"]
pub type TPIEN_W<'a, REG> = crate::BitWriter<'a, REG, TpienwWO>;
impl<'a, REG> TPIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper pin interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TpienwWO::Disable)
    }
    #[doc = "Tamper pin interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TpienwWO::Enable)
    }
}
#[doc = "Tamper event flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPEFR_A {
    #[doc = "0: No tamper event"]
    NoEvent = 0,
    #[doc = "1: A tamper event is detected"]
    Event = 1,
}
impl From<TPEFR_A> for bool {
    #[inline(always)]
    fn from(variant: TPEFR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPEF` reader - Tamper event flag"]
pub type TPEF_R = crate::BitReader<TPEFR_A>;
impl TPEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TPEFR_A {
        match self.bits {
            false => TPEFR_A::NoEvent,
            true => TPEFR_A::Event,
        }
    }
    #[doc = "No tamper event"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == TPEFR_A::NoEvent
    }
    #[doc = "A tamper event is detected"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == TPEFR_A::Event
    }
}
#[doc = "Field `TPEF` writer - Tamper event flag"]
pub type TPEF_W<'a, REG> = crate::BitWriter<'a, REG, TPEFR_A>;
impl<'a, REG> TPEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No tamper event"]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(TPEFR_A::NoEvent)
    }
    #[doc = "A tamper event is detected"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(TPEFR_A::Event)
    }
}
#[doc = "Tamper interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPIFR_A {
    #[doc = "0: No tamper event"]
    NoEvent = 0,
    #[doc = "1: A tamper event is detected"]
    Event = 1,
}
impl From<TPIFR_A> for bool {
    #[inline(always)]
    fn from(variant: TPIFR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPIF` reader - Tamper interrupt flag"]
pub type TPIF_R = crate::BitReader<TPIFR_A>;
impl TPIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TPIFR_A {
        match self.bits {
            false => TPIFR_A::NoEvent,
            true => TPIFR_A::Event,
        }
    }
    #[doc = "No tamper event"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == TPIFR_A::NoEvent
    }
    #[doc = "A tamper event is detected"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == TPIFR_A::Event
    }
}
#[doc = "Field `TPIF` writer - Tamper interrupt flag"]
pub type TPIF_W<'a, REG> = crate::BitWriter<'a, REG, TPIFR_A>;
impl<'a, REG> TPIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No tamper event"]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut crate::W<REG> {
        self.variant(TPIFR_A::NoEvent)
    }
    #[doc = "A tamper event is detected"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(TPIFR_A::Event)
    }
}
impl R {
    #[doc = "Bit 2 - Tamper pin interrupt enable"]
    #[inline(always)]
    pub fn tpien(&self) -> TPIEN_R {
        TPIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Tamper event flag"]
    #[inline(always)]
    pub fn tpef(&self) -> TPEF_R {
        TPEF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tamper interrupt flag"]
    #[inline(always)]
    pub fn tpif(&self) -> TPIF_R {
        TPIF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRLSTS")
            .field("tpien", &self.tpien())
            .field("tpef", &self.tpef())
            .field("tpif", &self.tpif())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Tamper event flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tpefclr(&mut self) -> TPEFCLR_W<CTRLSTS_SPEC> {
        TPEFCLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tpifclr(&mut self) -> TPIFCLR_W<CTRLSTS_SPEC> {
        TPIFCLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper pin interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpien(&mut self) -> TPIEN_W<CTRLSTS_SPEC> {
        TPIEN_W::new(self, 2)
    }
    #[doc = "Bit 8 - Tamper event flag"]
    #[inline(always)]
    #[must_use]
    pub fn tpef(&mut self) -> TPEF_W<CTRLSTS_SPEC> {
        TPEF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Tamper interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn tpif(&mut self) -> TPIF_W<CTRLSTS_SPEC> {
        TPIF_W::new(self, 9)
    }
}
#[doc = "BPR control/status register (BPR_CTRLSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLSTS_SPEC;
impl crate::RegisterSpec for CTRLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlsts::R`](R) reader structure"]
impl crate::Readable for CTRLSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlsts::W`](W) writer structure"]
impl crate::Writable for CTRLSTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets CTRLSTS to value 0"]
impl crate::Resettable for CTRLSTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
