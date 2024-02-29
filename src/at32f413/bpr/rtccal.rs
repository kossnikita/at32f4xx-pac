#[doc = "Register `RTCCAL` reader"]
pub type R = crate::R<RTCCAL_SPEC>;
#[doc = "Register `RTCCAL` writer"]
pub type W = crate::W<RTCCAL_SPEC>;
#[doc = "Field `CALVAL` reader - Calibration value"]
pub type CALVAL_R = crate::FieldReader;
#[doc = "Field `CALVAL` writer - Calibration value"]
pub type CALVAL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Calibration Clock Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Caloutr {
    #[doc = "0: No effect"]
    NoOutput = 0,
    #[doc = "1: Output the RTC clock with a frequency divided by 64 on the TAMPER pin"]
    Output = 1,
}
impl From<Caloutr> for bool {
    #[inline(always)]
    fn from(variant: Caloutr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALOUT` reader - Calibration Clock Output"]
pub type CALOUT_R = crate::BitReader<Caloutr>;
impl CALOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Caloutr {
        match self.bits {
            false => Caloutr::NoOutput,
            true => Caloutr::Output,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_output(&self) -> bool {
        *self == Caloutr::NoOutput
    }
    #[doc = "Output the RTC clock with a frequency divided by 64 on the TAMPER pin"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == Caloutr::Output
    }
}
#[doc = "Calibration Clock Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaloutwWO {
    #[doc = "1: Output the RTC clock with a frequency divided by 64 on the TAMPER pin"]
    Output = 1,
}
impl From<CaloutwWO> for bool {
    #[inline(always)]
    fn from(variant: CaloutwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALOUT` writer - Calibration Clock Output"]
pub type CALOUT_W<'a, REG> = crate::BitWriter1S<'a, REG, CaloutwWO>;
impl<'a, REG> CALOUT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output the RTC clock with a frequency divided by 64 on the TAMPER pin"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(CaloutwWO::Output)
    }
}
#[doc = "Output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outenr {
    #[doc = "0: Output is disabled"]
    Disabled = 0,
    #[doc = "1: Output is enabled"]
    Enabled = 1,
}
impl From<Outenr> for bool {
    #[inline(always)]
    fn from(variant: Outenr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTEN` reader - Output enable"]
pub type OUTEN_R = crate::BitReader<Outenr>;
impl OUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outenr {
        match self.bits {
            false => Outenr::Disabled,
            true => Outenr::Enabled,
        }
    }
    #[doc = "Output is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Outenr::Disabled
    }
    #[doc = "Output is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Outenr::Enabled
    }
}
#[doc = "Output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutenwWO {
    #[doc = "1: Output enable"]
    Enable = 1,
}
impl From<OutenwWO> for bool {
    #[inline(always)]
    fn from(variant: OutenwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTEN` writer - Output enable"]
pub type OUTEN_W<'a, REG> = crate::BitWriter1S<'a, REG, OutenwWO>;
impl<'a, REG> OUTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(OutenwWO::Enable)
    }
}
#[doc = "Output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outselr {
    #[doc = "0: RTC alarm event output"]
    Rtc = 0,
    #[doc = "1: Second event output"]
    Second = 1,
}
impl From<Outselr> for bool {
    #[inline(always)]
    fn from(variant: Outselr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTSEL` reader - Output selection"]
pub type OUTSEL_R = crate::BitReader<Outselr>;
impl OUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outselr {
        match self.bits {
            false => Outselr::Rtc,
            true => Outselr::Second,
        }
    }
    #[doc = "RTC alarm event output"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        *self == Outselr::Rtc
    }
    #[doc = "Second event output"]
    #[inline(always)]
    pub fn is_second(&self) -> bool {
        *self == Outselr::Second
    }
}
#[doc = "Output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutselwWO {
    #[doc = "1: Second event output"]
    Second = 1,
}
impl From<OutselwWO> for bool {
    #[inline(always)]
    fn from(variant: OutselwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTSEL` writer - Output selection"]
pub type OUTSEL_W<'a, REG> = crate::BitWriter1S<'a, REG, OutselwWO>;
impl<'a, REG> OUTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Second event output"]
    #[inline(always)]
    pub fn second(self) -> &'a mut crate::W<REG> {
        self.variant(OutselwWO::Second)
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
        core::fmt::Debug::fmt(&self.read(), f)
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
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0380;
}
#[doc = "`reset()` method sets RTCCAL to value 0"]
impl crate::Resettable for RTCCAL_SPEC {
    const RESET_VALUE: u32 = 0;
}
