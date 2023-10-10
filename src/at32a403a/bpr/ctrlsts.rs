#[doc = "Register `CTRLSTS` reader"]
pub type R = crate::R<CTRLSTS_SPEC>;
#[doc = "Register `CTRLSTS` writer"]
pub type W = crate::W<CTRLSTS_SPEC>;
#[doc = "Tamper event flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPEFCLRW_AW {
    #[doc = "1: Clear the tamper event flag"]
    Clear = 1,
}
impl From<TPEFCLRW_AW> for bool {
    #[inline(always)]
    fn from(variant: TPEFCLRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPEFCLR` writer - Tamper event flag clear"]
pub type TPEFCLR_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O, TPEFCLRW_AW>;
impl<'a, REG, const O: u8> TPEFCLR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the tamper event flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TPEFCLRW_AW::Clear)
    }
}
#[doc = "Tamper interrupt flag clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPIFCLRW_AW {
    #[doc = "1: Tamper interrupt flag clear"]
    Clear = 1,
}
impl From<TPIFCLRW_AW> for bool {
    #[inline(always)]
    fn from(variant: TPIFCLRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPIFCLR` writer - Tamper interrupt flag clear"]
pub type TPIFCLR_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O, TPIFCLRW_AW>;
impl<'a, REG, const O: u8> TPIFCLR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper interrupt flag clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TPIFCLRW_AW::Clear)
    }
}
#[doc = "Field `TPIEN` reader - Tamper pin interrupt enable"]
pub type TPIEN_R = crate::BitReader<TPIENR_A>;
#[doc = "Tamper pin interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPIENR_A {
    #[doc = "0: Tamper pin interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Tamper pin interrupt is enabled"]
    Enabled = 1,
}
impl From<TPIENR_A> for bool {
    #[inline(always)]
    fn from(variant: TPIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl TPIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPIENR_A {
        match self.bits {
            false => TPIENR_A::Disabled,
            true => TPIENR_A::Enabled,
        }
    }
    #[doc = "Tamper pin interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TPIENR_A::Disabled
    }
    #[doc = "Tamper pin interrupt is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TPIENR_A::Enabled
    }
}
#[doc = "Tamper pin interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPIENW_AW {
    #[doc = "0: Tamper pin interrupt disable"]
    Disable = 0,
    #[doc = "1: Tamper pin interrupt enable"]
    Enable = 1,
}
impl From<TPIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: TPIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPIEN` writer - Tamper pin interrupt enable"]
pub type TPIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TPIENW_AW>;
impl<'a, REG, const O: u8> TPIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper pin interrupt disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TPIENW_AW::Disable)
    }
    #[doc = "Tamper pin interrupt enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TPIENW_AW::Enable)
    }
}
#[doc = "Field `TPEF` reader - Tamper event flag"]
pub type TPEF_R = crate::BitReader<TPEFR_A>;
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
impl TPEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPEFR_A {
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
pub type TPEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TPEFR_A>;
impl<'a, REG, const O: u8> TPEF_W<'a, REG, O>
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
#[doc = "Field `TPIF` reader - Tamper interrupt flag"]
pub type TPIF_R = crate::BitReader<TPIFR_A>;
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
impl TPIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPIFR_A {
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
pub type TPIF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TPIFR_A>;
impl<'a, REG, const O: u8> TPIF_W<'a, REG, O>
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
            .field("tpien", &format_args!("{}", self.tpien().bit()))
            .field("tpef", &format_args!("{}", self.tpef().bit()))
            .field("tpif", &format_args!("{}", self.tpif().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRLSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper event flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tpefclr(&mut self) -> TPEFCLR_W<CTRLSTS_SPEC, 0> {
        TPEFCLR_W::new(self)
    }
    #[doc = "Bit 1 - Tamper interrupt flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tpifclr(&mut self) -> TPIFCLR_W<CTRLSTS_SPEC, 1> {
        TPIFCLR_W::new(self)
    }
    #[doc = "Bit 2 - Tamper pin interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpien(&mut self) -> TPIEN_W<CTRLSTS_SPEC, 2> {
        TPIEN_W::new(self)
    }
    #[doc = "Bit 8 - Tamper event flag"]
    #[inline(always)]
    #[must_use]
    pub fn tpef(&mut self) -> TPEF_W<CTRLSTS_SPEC, 8> {
        TPEF_W::new(self)
    }
    #[doc = "Bit 9 - Tamper interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn tpif(&mut self) -> TPIF_W<CTRLSTS_SPEC, 9> {
        TPIF_W::new(self)
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
#[doc = "BPR control/status register (BPR_CTRLSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLSTS_SPEC;
impl crate::RegisterSpec for CTRLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlsts::R`](R) reader structure"]
impl crate::Readable for CTRLSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlsts::W`](W) writer structure"]
impl crate::Writable for CTRLSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03;
}
#[doc = "`reset()` method sets CTRLSTS to value 0"]
impl crate::Resettable for CTRLSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
