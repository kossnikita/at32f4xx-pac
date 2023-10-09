#[doc = "Register `CTRLSTS` reader"]
pub type R = crate::R<CTRLSTS_SPEC>;
#[doc = "Register `CTRLSTS` writer"]
pub type W = crate::W<CTRLSTS_SPEC>;
#[doc = "Field `LICKEN` reader - Low speed internal clock enable"]
pub type LICKEN_R = crate::BitReader;
#[doc = "Field `LICKEN` writer - Low speed internal clock enable"]
pub type LICKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LICKSTBL` reader - Low speed internal clock ready"]
pub type LICKSTBL_R = crate::BitReader;
#[doc = "Field `RSTFC` reader - Reset reset flag"]
pub type RSTFC_R = crate::BitReader;
#[doc = "Field `RSTFC` writer - Reset reset flag"]
pub type RSTFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NRSTF` reader - PIN reset flag"]
pub type NRSTF_R = crate::BitReader;
#[doc = "Field `NRSTF` writer - PIN reset flag"]
pub type NRSTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PORRSTF` reader - POR/LVR reset flag"]
pub type PORRSTF_R = crate::BitReader;
#[doc = "Field `PORRSTF` writer - POR/LVR reset flag"]
pub type PORRSTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SWRSTF` reader - Software reset flag"]
pub type SWRSTF_R = crate::BitReader;
#[doc = "Field `SWRSTF` writer - Software reset flag"]
pub type SWRSTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WDTRSTF` reader - Watchdog timer reset flag"]
pub type WDTRSTF_R = crate::BitReader;
#[doc = "Field `WDTRSTF` writer - Watchdog timer reset flag"]
pub type WDTRSTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WWDTRSTF` reader - Window watchdog timer reset flag"]
pub type WWDTRSTF_R = crate::BitReader;
#[doc = "Field `WWDTRSTF` writer - Window watchdog timer reset flag"]
pub type WWDTRSTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPRSTF` reader - Low-power reset flag"]
pub type LPRSTF_R = crate::BitReader;
#[doc = "Field `LPRSTF` writer - Low-power reset flag"]
pub type LPRSTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Low speed internal clock enable"]
    #[inline(always)]
    pub fn licken(&self) -> LICKEN_R {
        LICKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low speed internal clock ready"]
    #[inline(always)]
    pub fn lickstbl(&self) -> LICKSTBL_R {
        LICKSTBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset reset flag"]
    #[inline(always)]
    pub fn rstfc(&self) -> RSTFC_R {
        RSTFC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn nrstf(&self) -> NRSTF_R {
        NRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - POR/LVR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn swrstf(&self) -> SWRSTF_R {
        SWRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Watchdog timer reset flag"]
    #[inline(always)]
    pub fn wdtrstf(&self) -> WDTRSTF_R {
        WDTRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog timer reset flag"]
    #[inline(always)]
    pub fn wwdtrstf(&self) -> WWDTRSTF_R {
        WWDTRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lprstf(&self) -> LPRSTF_R {
        LPRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRLSTS")
            .field("licken", &format_args!("{}", self.licken().bit()))
            .field("lickstbl", &format_args!("{}", self.lickstbl().bit()))
            .field("rstfc", &format_args!("{}", self.rstfc().bit()))
            .field("nrstf", &format_args!("{}", self.nrstf().bit()))
            .field("porrstf", &format_args!("{}", self.porrstf().bit()))
            .field("swrstf", &format_args!("{}", self.swrstf().bit()))
            .field("wdtrstf", &format_args!("{}", self.wdtrstf().bit()))
            .field("wwdtrstf", &format_args!("{}", self.wwdtrstf().bit()))
            .field("lprstf", &format_args!("{}", self.lprstf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRLSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Low speed internal clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn licken(&mut self) -> LICKEN_W<CTRLSTS_SPEC, 0> {
        LICKEN_W::new(self)
    }
    #[doc = "Bit 24 - Reset reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn rstfc(&mut self) -> RSTFC_W<CTRLSTS_SPEC, 24> {
        RSTFC_W::new(self)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn nrstf(&mut self) -> NRSTF_W<CTRLSTS_SPEC, 26> {
        NRSTF_W::new(self)
    }
    #[doc = "Bit 27 - POR/LVR reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn porrstf(&mut self) -> PORRSTF_W<CTRLSTS_SPEC, 27> {
        PORRSTF_W::new(self)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn swrstf(&mut self) -> SWRSTF_W<CTRLSTS_SPEC, 28> {
        SWRSTF_W::new(self)
    }
    #[doc = "Bit 29 - Watchdog timer reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrstf(&mut self) -> WDTRSTF_W<CTRLSTS_SPEC, 29> {
        WDTRSTF_W::new(self)
    }
    #[doc = "Bit 30 - Window watchdog timer reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn wwdtrstf(&mut self) -> WWDTRSTF_W<CTRLSTS_SPEC, 30> {
        WWDTRSTF_W::new(self)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn lprstf(&mut self) -> LPRSTF_W<CTRLSTS_SPEC, 31> {
        LPRSTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control/status register (CRM_CTRLSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLSTS_SPEC;
impl crate::RegisterSpec for CTRLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlsts::R`](R) reader structure"]
impl crate::Readable for CTRLSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlsts::W`](W) writer structure"]
impl crate::Writable for CTRLSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLSTS to value 0x0c00_0000"]
impl crate::Resettable for CTRLSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c00_0000;
}
