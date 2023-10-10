#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `CBCTRL` reader - Channel buffer control"]
pub type CBCTRL_R = crate::BitReader<CBCTRLR_A>;
#[doc = "Channel buffer control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBCTRLR_A {
    #[doc = "0: CxEN, CxCEN and CxOCTRL bits are not buffered"]
    Disabled = 0,
    #[doc = "1: CxEN, CxCEN and CxOCTRL bits are buffered"]
    Enabled = 1,
}
impl From<CBCTRLR_A> for bool {
    #[inline(always)]
    fn from(variant: CBCTRLR_A) -> Self {
        variant as u8 != 0
    }
}
impl CBCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBCTRLR_A {
        match self.bits {
            false => CBCTRLR_A::Disabled,
            true => CBCTRLR_A::Enabled,
        }
    }
    #[doc = "CxEN, CxCEN and CxOCTRL bits are not buffered"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CBCTRLR_A::Disabled
    }
    #[doc = "CxEN, CxCEN and CxOCTRL bits are buffered"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CBCTRLR_A::Enabled
    }
}
#[doc = "Channel buffer control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CBCTRLW_AW {
    #[doc = "0: CxEN, CxCEN and CxOCTRL bits buffer disable"]
    Disable = 0,
    #[doc = "1: CxEN, CxCEN and CxOCTRL bits buffer enable"]
    Enable = 1,
}
impl From<CBCTRLW_AW> for bool {
    #[inline(always)]
    fn from(variant: CBCTRLW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CBCTRL` writer - Channel buffer control"]
pub type CBCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CBCTRLW_AW>;
impl<'a, REG, const O: u8> CBCTRL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CxEN, CxCEN and CxOCTRL bits buffer disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CBCTRLW_AW::Disable)
    }
    #[doc = "CxEN, CxCEN and CxOCTRL bits buffer enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CBCTRLW_AW::Enable)
    }
}
#[doc = "Field `CCFS` reader - Channel control bit fresh select"]
pub type CCFS_R = crate::BitReader<CCFS_A>;
#[doc = "Channel control bit fresh select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCFS_A {
    #[doc = "0: Control bits are updated by setting the HALL bit"]
    Hall = 0,
    #[doc = "1: Control bits are updated by setting the HALL bit or a rising edge on TRGIN"]
    Trgin = 1,
}
impl From<CCFS_A> for bool {
    #[inline(always)]
    fn from(variant: CCFS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCFS_A {
        match self.bits {
            false => CCFS_A::Hall,
            true => CCFS_A::Trgin,
        }
    }
    #[doc = "Control bits are updated by setting the HALL bit"]
    #[inline(always)]
    pub fn is_hall(&self) -> bool {
        *self == CCFS_A::Hall
    }
    #[doc = "Control bits are updated by setting the HALL bit or a rising edge on TRGIN"]
    #[inline(always)]
    pub fn is_trgin(&self) -> bool {
        *self == CCFS_A::Trgin
    }
}
#[doc = "Field `CCFS` writer - Channel control bit fresh select"]
pub type CCFS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CCFS_A>;
impl<'a, REG, const O: u8> CCFS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Control bits are updated by setting the HALL bit"]
    #[inline(always)]
    pub fn hall(self) -> &'a mut crate::W<REG> {
        self.variant(CCFS_A::Hall)
    }
    #[doc = "Control bits are updated by setting the HALL bit or a rising edge on TRGIN"]
    #[inline(always)]
    pub fn trgin(self) -> &'a mut crate::W<REG> {
        self.variant(CCFS_A::Trgin)
    }
}
#[doc = "Field `DRS` reader - DMA request source"]
pub type DRS_R = crate::BitReader<DRS_A>;
#[doc = "DMA request source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRS_A {
    #[doc = "0: Capture/compare event"]
    CaptureCompare = 0,
    #[doc = "1: Overflow event"]
    Overflow = 1,
}
impl From<DRS_A> for bool {
    #[inline(always)]
    fn from(variant: DRS_A) -> Self {
        variant as u8 != 0
    }
}
impl DRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRS_A {
        match self.bits {
            false => DRS_A::CaptureCompare,
            true => DRS_A::Overflow,
        }
    }
    #[doc = "Capture/compare event"]
    #[inline(always)]
    pub fn is_capture_compare(&self) -> bool {
        *self == DRS_A::CaptureCompare
    }
    #[doc = "Overflow event"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == DRS_A::Overflow
    }
}
#[doc = "Field `DRS` writer - DMA request source"]
pub type DRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DRS_A>;
impl<'a, REG, const O: u8> DRS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Capture/compare event"]
    #[inline(always)]
    pub fn capture_compare(self) -> &'a mut crate::W<REG> {
        self.variant(DRS_A::CaptureCompare)
    }
    #[doc = "Overflow event"]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(DRS_A::Overflow)
    }
}
#[doc = "Field `C1IOS` reader - Channel 1 idle output state"]
pub type C1IOS_R = crate::BitReader;
#[doc = "Field `C1IOS` writer - Channel 1 idle output state"]
pub type C1IOS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C1CIOS` reader - Channel 1 complementary idle output state"]
pub type C1CIOS_R = crate::BitReader;
#[doc = "Field `C1CIOS` writer - Channel 1 complementary idle output state"]
pub type C1CIOS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Channel buffer control"]
    #[inline(always)]
    pub fn cbctrl(&self) -> CBCTRL_R {
        CBCTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Channel control bit fresh select"]
    #[inline(always)]
    pub fn ccfs(&self) -> CCFS_R {
        CCFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA request source"]
    #[inline(always)]
    pub fn drs(&self) -> DRS_R {
        DRS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 1 idle output state"]
    #[inline(always)]
    pub fn c1ios(&self) -> C1IOS_R {
        C1IOS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 complementary idle output state"]
    #[inline(always)]
    pub fn c1cios(&self) -> C1CIOS_R {
        C1CIOS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("c1cios", &format_args!("{}", self.c1cios().bit()))
            .field("c1ios", &format_args!("{}", self.c1ios().bit()))
            .field("drs", &format_args!("{}", self.drs().bit()))
            .field("ccfs", &format_args!("{}", self.ccfs().bit()))
            .field("cbctrl", &format_args!("{}", self.cbctrl().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Channel buffer control"]
    #[inline(always)]
    #[must_use]
    pub fn cbctrl(&mut self) -> CBCTRL_W<CTRL2_SPEC, 0> {
        CBCTRL_W::new(self)
    }
    #[doc = "Bit 2 - Channel control bit fresh select"]
    #[inline(always)]
    #[must_use]
    pub fn ccfs(&mut self) -> CCFS_W<CTRL2_SPEC, 2> {
        CCFS_W::new(self)
    }
    #[doc = "Bit 3 - DMA request source"]
    #[inline(always)]
    #[must_use]
    pub fn drs(&mut self) -> DRS_W<CTRL2_SPEC, 3> {
        DRS_W::new(self)
    }
    #[doc = "Bit 8 - Channel 1 idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c1ios(&mut self) -> C1IOS_W<CTRL2_SPEC, 8> {
        C1IOS_W::new(self)
    }
    #[doc = "Bit 9 - Channel 1 complementary idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c1cios(&mut self) -> C1CIOS_W<CTRL2_SPEC, 9> {
        C1CIOS_W::new(self)
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
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
