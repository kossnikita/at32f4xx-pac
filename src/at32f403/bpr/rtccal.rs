#[doc = "Register `RTCCAL` reader"]
pub type R = crate::R<RTCCAL_SPEC>;
#[doc = "Register `RTCCAL` writer"]
pub type W = crate::W<RTCCAL_SPEC>;
#[doc = "Field `CALVAL` reader - Calibration value"]
pub type CALVAL_R = crate::FieldReader;
#[doc = "Field `CALVAL` writer - Calibration value"]
pub type CALVAL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Field `CALOUT` reader - Calibration Clock Output"]
pub type CALOUT_R = crate::BitReader<CALOUTR_A>;
#[doc = "Calibration Clock Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALOUTR_A {
    #[doc = "0: No effect"]
    NoOutput = 0,
    #[doc = "1: Output the RTC clock with a frequency divided by 64 on the TAMPER pin"]
    Output = 1,
}
impl From<CALOUTR_A> for bool {
    #[inline(always)]
    fn from(variant: CALOUTR_A) -> Self {
        variant as u8 != 0
    }
}
impl CALOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CALOUTR_A {
        match self.bits {
            false => CALOUTR_A::NoOutput,
            true => CALOUTR_A::Output,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_output(&self) -> bool {
        *self == CALOUTR_A::NoOutput
    }
    #[doc = "Output the RTC clock with a frequency divided by 64 on the TAMPER pin"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == CALOUTR_A::Output
    }
}
#[doc = "Calibration Clock Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALOUTW_AW {
    #[doc = "1: Output the RTC clock with a frequency divided by 64 on the TAMPER pin"]
    Output = 1,
}
impl From<CALOUTW_AW> for bool {
    #[inline(always)]
    fn from(variant: CALOUTW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALOUT` writer - Calibration Clock Output"]
pub type CALOUT_W<'a, REG> = crate::BitWriter1S<'a, REG, CALOUTW_AW>;
impl<'a, REG> CALOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output the RTC clock with a frequency divided by 64 on the TAMPER pin"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(CALOUTW_AW::Output)
    }
}
#[doc = "Field `OUTEN` reader - Output enable"]
pub type OUTEN_R = crate::BitReader<OUTENR_A>;
#[doc = "Output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTENR_A {
    #[doc = "0: Output is disabled"]
    Disabled = 0,
    #[doc = "1: Output is enabled"]
    Enabled = 1,
}
impl From<OUTENR_A> for bool {
    #[inline(always)]
    fn from(variant: OUTENR_A) -> Self {
        variant as u8 != 0
    }
}
impl OUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OUTENR_A {
        match self.bits {
            false => OUTENR_A::Disabled,
            true => OUTENR_A::Enabled,
        }
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OUTENR_A::Disabled
    }
    #[doc = "Output is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OUTENR_A::Enabled
    }
}
#[doc = "Output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTENW_AW {
    #[doc = "1: Output enable"]
    Enable = 1,
}
impl From<OUTENW_AW> for bool {
    #[inline(always)]
    fn from(variant: OUTENW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTEN` writer - Output enable"]
pub type OUTEN_W<'a, REG> = crate::BitWriter1S<'a, REG, OUTENW_AW>;
impl<'a, REG> OUTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(OUTENW_AW::Enable)
    }
}
#[doc = "Field `OUTSEL` reader - Output selection"]
pub type OUTSEL_R = crate::BitReader<OUTSELR_A>;
#[doc = "Output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTSELR_A {
    #[doc = "0: RTC alarm event output"]
    Rtc = 0,
    #[doc = "1: Second event output"]
    Second = 1,
}
impl From<OUTSELR_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSELR_A) -> Self {
        variant as u8 != 0
    }
}
impl OUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OUTSELR_A {
        match self.bits {
            false => OUTSELR_A::Rtc,
            true => OUTSELR_A::Second,
        }
    }
    #[doc = "RTC alarm event output"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == OUTSELR_A::Rtc
    }
    #[doc = "Second event output"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == OUTSELR_A::Second
    }
}
#[doc = "Output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTSELW_AW {
    #[doc = "1: Second event output"]
    Second = 1,
}
impl From<OUTSELW_AW> for bool {
    #[inline(always)]
    fn from(variant: OUTSELW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTSEL` writer - Output selection"]
pub type OUTSEL_W<'a, REG> = crate::BitWriter1S<'a, REG, OUTSELW_AW>;
impl<'a, REG> OUTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Second event output"]
    #[inline(always)]
    pub fn second(self) -> &'a mut crate::W<REG> {
        self.variant(OUTSELW_AW::Second)
    }
}
impl R {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn calval(&self) -> CALVAL_R {
        CALVAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output enable"]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output selection"]
    #[inline(always)]
    pub fn outsel(&self) -> OUTSEL_R {
        OUTSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCCAL")
            .field("calval", &format_args!("{}", self.calval().bits()))
            .field("calout", &format_args!("{}", self.calout().bit()))
            .field("outen", &format_args!("{}", self.outen().bit()))
            .field("outsel", &format_args!("{}", self.outsel().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<RTCCAL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn calval(&mut self) -> CALVAL_W<RTCCAL_SPEC> {
        CALVAL_W::new(self, 0)
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    #[must_use]
    pub fn calout(&mut self) -> CALOUT_W<RTCCAL_SPEC> {
        CALOUT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Output enable"]
    #[inline(always)]
    #[must_use]
    pub fn outen(&mut self) -> OUTEN_W<RTCCAL_SPEC> {
        OUTEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Output selection"]
    #[inline(always)]
    #[must_use]
    pub fn outsel(&mut self) -> OUTSEL_W<RTCCAL_SPEC> {
        OUTSEL_W::new(self, 9)
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
#[doc = "RTC clock calibration register (BPR_RTCCAL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCCAL_SPEC;
impl crate::RegisterSpec for RTCCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccal::R`](R) reader structure"]
impl crate::Readable for RTCCAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtccal::W`](W) writer structure"]
impl crate::Writable for RTCCAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0380;
}
#[doc = "`reset()` method sets RTCCAL to value 0"]
impl crate::Resettable for RTCCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
