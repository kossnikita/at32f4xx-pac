#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Internal high-speed clock calibration ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calonr {
    #[doc = "0: Calibration disabled"]
    Disabled = 0,
    #[doc = "1: Calibration enabled, and starts searching for a pulse on the USB_SOF"]
    Enabled = 1,
}
impl From<Calonr> for bool {
    #[inline(always)]
    fn from(variant: Calonr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALON` reader - Internal high-speed clock calibration ready"]
pub type CALON_R = crate::BitReader<Calonr>;
impl CALON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Calonr {
        match self.bits {
            false => Calonr::Disabled,
            true => Calonr::Enabled,
        }
    }
    #[doc = "Calibration disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Calonr::Disabled
    }
    #[doc = "Calibration enabled, and starts searching for a pulse on the USB_SOF"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Calonr::Enabled
    }
}
#[doc = "Internal high-speed clock calibration ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalonwWO {
    #[doc = "0: Calibration disabled"]
    Disable = 0,
    #[doc = "1: Calibration enabled, and starts searching for a pulse on the USB_SOF"]
    Enable = 1,
}
impl From<CalonwWO> for bool {
    #[inline(always)]
    fn from(variant: CalonwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALON` writer - Internal high-speed clock calibration ready"]
pub type CALON_W<'a, REG> = crate::BitWriter<'a, REG, CalonwWO>;
impl<'a, REG> CALON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CalonwWO::Disable)
    }
    #[doc = "Calibration enabled, and starts searching for a pulse on the USB_SOF"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CalonwWO::Enable)
    }
}
#[doc = "Enable trim\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENTRIM_A {
    #[doc = "0: HICKCAL is calibrated"]
    Hickcal = 0,
    #[doc = "1: HICKTRIM is calibrated"]
    Hicktrim = 1,
}
impl From<ENTRIM_A> for bool {
    #[inline(always)]
    fn from(variant: ENTRIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENTRIM` reader - Enable trim"]
pub type ENTRIM_R = crate::BitReader<ENTRIM_A>;
impl ENTRIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENTRIM_A {
        match self.bits {
            false => ENTRIM_A::Hickcal,
            true => ENTRIM_A::Hicktrim,
        }
    }
    #[doc = "HICKCAL is calibrated"]
    #[inline(always)]
    pub fn is_hickcal(&self) -> bool {
        *self == ENTRIM_A::Hickcal
    }
    #[doc = "HICKTRIM is calibrated"]
    #[inline(always)]
    pub fn is_hicktrim(&self) -> bool {
        *self == ENTRIM_A::Hicktrim
    }
}
#[doc = "Field `ENTRIM` writer - Enable trim"]
pub type ENTRIM_W<'a, REG> = crate::BitWriter<'a, REG, ENTRIM_A>;
impl<'a, REG> ENTRIM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HICKCAL is calibrated"]
    #[inline(always)]
    pub fn hickcal(self) -> &'a mut crate::W<REG> {
        self.variant(ENTRIM_A::Hickcal)
    }
    #[doc = "HICKTRIM is calibrated"]
    #[inline(always)]
    pub fn hicktrim(self) -> &'a mut crate::W<REG> {
        self.variant(ENTRIM_A::Hicktrim)
    }
}
#[doc = "RSLOST error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eienr {
    #[doc = "0: Interrupt generation disabled"]
    Disabled = 0,
    #[doc = "1: ACC interrupt is generated when RSLOST=1 in the ACC_STS register"]
    Enabled = 1,
}
impl From<Eienr> for bool {
    #[inline(always)]
    fn from(variant: Eienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EIEN` reader - RSLOST error interrupt enable"]
pub type EIEN_R = crate::BitReader<Eienr>;
impl EIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eienr {
        match self.bits {
            false => Eienr::Disabled,
            true => Eienr::Enabled,
        }
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Eienr::Disabled
    }
    #[doc = "ACC interrupt is generated when RSLOST=1 in the ACC_STS register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Eienr::Enabled
    }
}
#[doc = "RSLOST error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EienwWO {
    #[doc = "0: Interrupt generation disabled"]
    Disable = 0,
    #[doc = "1: ACC interrupt is generated when RSLOST=1 in the ACC_STS register"]
    Enable = 1,
}
impl From<EienwWO> for bool {
    #[inline(always)]
    fn from(variant: EienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EIEN` writer - RSLOST error interrupt enable"]
pub type EIEN_W<'a, REG> = crate::BitWriter<'a, REG, EienwWO>;
impl<'a, REG> EIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EienwWO::Disable)
    }
    #[doc = "ACC interrupt is generated when RSLOST=1 in the ACC_STS register"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EienwWO::Enable)
    }
}
#[doc = "CALRDY interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calrdyienr {
    #[doc = "0: Interrupt generation disabled"]
    Disabled = 0,
    #[doc = "1: ACC interrupt is generated when CALRDY=1 in the ACC_STS register"]
    Enabled = 1,
}
impl From<Calrdyienr> for bool {
    #[inline(always)]
    fn from(variant: Calrdyienr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALRDYIEN` reader - CALRDY interrupt enable"]
pub type CALRDYIEN_R = crate::BitReader<Calrdyienr>;
impl CALRDYIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Calrdyienr {
        match self.bits {
            false => Calrdyienr::Disabled,
            true => Calrdyienr::Enabled,
        }
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Calrdyienr::Disabled
    }
    #[doc = "ACC interrupt is generated when CALRDY=1 in the ACC_STS register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Calrdyienr::Enabled
    }
}
#[doc = "CALRDY interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalrdyienwWO {
    #[doc = "0: Interrupt generation disabled"]
    Disable = 0,
    #[doc = "1: ACC interrupt is generated when CALRDY=1 in the ACC_STS register"]
    Enable = 1,
}
impl From<CalrdyienwWO> for bool {
    #[inline(always)]
    fn from(variant: CalrdyienwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALRDYIEN` writer - CALRDY interrupt enable"]
pub type CALRDYIEN_W<'a, REG> = crate::BitWriter<'a, REG, CalrdyienwWO>;
impl<'a, REG> CALRDYIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CalrdyienwWO::Disable)
    }
    #[doc = "ACC interrupt is generated when CALRDY=1 in the ACC_STS register"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CalrdyienwWO::Enable)
    }
}
#[doc = "Field `STEP` reader - Calibrated step"]
pub type STEP_R = crate::FieldReader;
#[doc = "Field `STEP` writer - Calibrated step"]
pub type STEP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Internal high-speed clock calibration ready"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable trim"]
    #[inline(always)]
    pub fn entrim(&self) -> ENTRIM_R {
        ENTRIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - RSLOST error interrupt enable"]
    #[inline(always)]
    pub fn eien(&self) -> EIEN_R {
        EIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CALRDY interrupt enable"]
    #[inline(always)]
    pub fn calrdyien(&self) -> CALRDYIEN_R {
        CALRDYIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Calibrated step"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("calon", &format_args!("{}", self.calon().bit()))
            .field("entrim", &format_args!("{}", self.entrim().bit()))
            .field("eien", &format_args!("{}", self.eien().bit()))
            .field("calrdyien", &format_args!("{}", self.calrdyien().bit()))
            .field("step", &format_args!("{}", self.step().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Internal high-speed clock calibration ready"]
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<CTRL1_SPEC> {
        CALON_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable trim"]
    #[inline(always)]
    #[must_use]
    pub fn entrim(&mut self) -> ENTRIM_W<CTRL1_SPEC> {
        ENTRIM_W::new(self, 1)
    }
    #[doc = "Bit 4 - RSLOST error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eien(&mut self) -> EIEN_W<CTRL1_SPEC> {
        EIEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - CALRDY interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn calrdyien(&mut self) -> CALRDYIEN_W<CTRL1_SPEC> {
        CALRDYIEN_W::new(self, 5)
    }
    #[doc = "Bits 8:11 - Calibrated step"]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> STEP_W<CTRL1_SPEC> {
        STEP_W::new(self, 8)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0x0100"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
