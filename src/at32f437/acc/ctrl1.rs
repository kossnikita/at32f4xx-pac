#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `CALON` reader - Calibration on"]
pub type CALON_R = crate::BitReader<CALONR_A>;
#[doc = "Calibration on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALONR_A {
    #[doc = "0: Calibration disabled"]
    Disabled = 0,
    #[doc = "1: Calibration enabled, and starts searching for a pulse on the USB_SOF"]
    Enabled = 1,
}
impl From<CALONR_A> for bool {
    #[inline(always)]
    fn from(variant: CALONR_A) -> Self {
        variant as u8 != 0
    }
}
impl CALON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALONR_A {
        match self.bits {
            false => CALONR_A::Disabled,
            true => CALONR_A::Enabled,
        }
    }
    #[doc = "Calibration disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CALONR_A::Disabled
    }
    #[doc = "Calibration enabled, and starts searching for a pulse on the USB_SOF"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CALONR_A::Enabled
    }
}
#[doc = "Calibration on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALONW_AW {
    #[doc = "0: Calibration disabled"]
    Disable = 0,
    #[doc = "1: Calibration enabled, and starts searching for a pulse on the USB_SOF"]
    Enable = 1,
}
impl From<CALONW_AW> for bool {
    #[inline(always)]
    fn from(variant: CALONW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALON` writer - Calibration on"]
pub type CALON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CALONW_AW>;
impl<'a, REG, const O: u8> CALON_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CALONW_AW::Disable)
    }
    #[doc = "Calibration enabled, and starts searching for a pulse on the USB_SOF"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CALONW_AW::Enable)
    }
}
#[doc = "Field `ENTRIM` reader - Enable trim"]
pub type ENTRIM_R = crate::BitReader<ENTRIM_A>;
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
impl ENTRIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENTRIM_A {
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
pub type ENTRIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ENTRIM_A>;
impl<'a, REG, const O: u8> ENTRIM_W<'a, REG, O>
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
#[doc = "Field `SOFSEL` reader - SOF Select"]
pub type SOFSEL_R = crate::BitReader;
#[doc = "Field `SOFSEL` writer - SOF Select"]
pub type SOFSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EIEN` reader - RSLOST error interrupt enable"]
pub type EIEN_R = crate::BitReader<EIENR_A>;
#[doc = "RSLOST error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIENR_A {
    #[doc = "0: Interrupt generation disabled"]
    Disabled = 0,
    #[doc = "1: ACC interrupt is generated when RSLOST=1 in the ACC_STS register"]
    Enabled = 1,
}
impl From<EIENR_A> for bool {
    #[inline(always)]
    fn from(variant: EIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl EIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIENR_A {
        match self.bits {
            false => EIENR_A::Disabled,
            true => EIENR_A::Enabled,
        }
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIENR_A::Disabled
    }
    #[doc = "ACC interrupt is generated when RSLOST=1 in the ACC_STS register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIENR_A::Enabled
    }
}
#[doc = "RSLOST error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIENW_AW {
    #[doc = "0: Interrupt generation disabled"]
    Disable = 0,
    #[doc = "1: ACC interrupt is generated when RSLOST=1 in the ACC_STS register"]
    Enable = 1,
}
impl From<EIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: EIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EIEN` writer - RSLOST error interrupt enable"]
pub type EIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, EIENW_AW>;
impl<'a, REG, const O: u8> EIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EIENW_AW::Disable)
    }
    #[doc = "ACC interrupt is generated when RSLOST=1 in the ACC_STS register"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EIENW_AW::Enable)
    }
}
#[doc = "Field `CALRDYIEN` reader - CALRDY interrupt enable"]
pub type CALRDYIEN_R = crate::BitReader<CALRDYIENR_A>;
#[doc = "CALRDY interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALRDYIENR_A {
    #[doc = "0: Interrupt generation disabled"]
    Disabled = 0,
    #[doc = "1: ACC interrupt is generated when CALRDY=1 in the ACC_STS register"]
    Enabled = 1,
}
impl From<CALRDYIENR_A> for bool {
    #[inline(always)]
    fn from(variant: CALRDYIENR_A) -> Self {
        variant as u8 != 0
    }
}
impl CALRDYIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALRDYIENR_A {
        match self.bits {
            false => CALRDYIENR_A::Disabled,
            true => CALRDYIENR_A::Enabled,
        }
    }
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CALRDYIENR_A::Disabled
    }
    #[doc = "ACC interrupt is generated when CALRDY=1 in the ACC_STS register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CALRDYIENR_A::Enabled
    }
}
#[doc = "CALRDY interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALRDYIENW_AW {
    #[doc = "0: Interrupt generation disabled"]
    Disable = 0,
    #[doc = "1: ACC interrupt is generated when CALRDY=1 in the ACC_STS register"]
    Enable = 1,
}
impl From<CALRDYIENW_AW> for bool {
    #[inline(always)]
    fn from(variant: CALRDYIENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALRDYIEN` writer - CALRDY interrupt enable"]
pub type CALRDYIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CALRDYIENW_AW>;
impl<'a, REG, const O: u8> CALRDYIEN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt generation disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CALRDYIENW_AW::Disable)
    }
    #[doc = "ACC interrupt is generated when CALRDY=1 in the ACC_STS register"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CALRDYIENW_AW::Enable)
    }
}
#[doc = "Field `STEP` reader - Calibrated step"]
pub type STEP_R = crate::FieldReader;
#[doc = "Field `STEP` writer - Calibrated step"]
pub type STEP_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 0 - Calibration on"]
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable trim"]
    #[inline(always)]
    pub fn entrim(&self) -> ENTRIM_R {
        ENTRIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SOF Select"]
    #[inline(always)]
    pub fn sofsel(&self) -> SOFSEL_R {
        SOFSEL_R::new(((self.bits >> 2) & 1) != 0)
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
impl W {
    #[doc = "Bit 0 - Calibration on"]
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<CTRL1_SPEC, 0> {
        CALON_W::new(self)
    }
    #[doc = "Bit 1 - Enable trim"]
    #[inline(always)]
    #[must_use]
    pub fn entrim(&mut self) -> ENTRIM_W<CTRL1_SPEC, 1> {
        ENTRIM_W::new(self)
    }
    #[doc = "Bit 2 - SOF Select"]
    #[inline(always)]
    #[must_use]
    pub fn sofsel(&mut self) -> SOFSEL_W<CTRL1_SPEC, 2> {
        SOFSEL_W::new(self)
    }
    #[doc = "Bit 4 - RSLOST error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eien(&mut self) -> EIEN_W<CTRL1_SPEC, 4> {
        EIEN_W::new(self)
    }
    #[doc = "Bit 5 - CALRDY interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn calrdyien(&mut self) -> CALRDYIEN_W<CTRL1_SPEC, 5> {
        CALRDYIEN_W::new(self)
    }
    #[doc = "Bits 8:11 - Calibrated step"]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> STEP_W<CTRL1_SPEC, 8> {
        STEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0x0100"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
