#[doc = "Register `CTRLL` reader"]
pub type R = crate::R<CTRLL_SPEC>;
#[doc = "Register `CTRLL` writer"]
pub type W = crate::W<CTRLL_SPEC>;
#[doc = "Field `TSF` reader - Time second flag"]
pub type TSF_R = crate::BitReader;
#[doc = "Field `TSF` writer - Time second flag"]
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAF` reader - Time alarm flag"]
pub type TAF_R = crate::BitReader;
#[doc = "Field `TAF` writer - Time alarm flag"]
pub type TAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFF` reader - Overflow Flag"]
pub type OVFF_R = crate::BitReader;
#[doc = "Field `OVFF` writer - Overflow Flag"]
pub type OVFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDF` reader - RTC update finish"]
pub type UPDF_R = crate::BitReader;
#[doc = "Field `UPDF` writer - RTC update finish"]
pub type UPDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGEN` reader - RTC configuration enable"]
pub type CFGEN_R = crate::BitReader;
#[doc = "Field `CFGEN` writer - RTC configuration enable"]
pub type CFGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGF` reader - RTC configuration finish"]
pub type CFGF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Time second flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time alarm flag"]
    #[inline(always)]
    pub fn taf(&self) -> TAF_R {
        TAF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    pub fn ovff(&self) -> OVFF_R {
        OVFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC update finish"]
    #[inline(always)]
    pub fn updf(&self) -> UPDF_R {
        UPDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC configuration enable"]
    #[inline(always)]
    pub fn cfgen(&self) -> CFGEN_R {
        CFGEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC configuration finish"]
    #[inline(always)]
    pub fn cfgf(&self) -> CFGF_R {
        CFGF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRLL")
            .field("tsf", &self.tsf())
            .field("taf", &self.taf())
            .field("ovff", &self.ovff())
            .field("updf", &self.updf())
            .field("cfgen", &self.cfgen())
            .field("cfgf", &self.cfgf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Time second flag"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W<'_, CTRLL_SPEC> {
        TSF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Time alarm flag"]
    #[inline(always)]
    pub fn taf(&mut self) -> TAF_W<'_, CTRLL_SPEC> {
        TAF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    pub fn ovff(&mut self) -> OVFF_W<'_, CTRLL_SPEC> {
        OVFF_W::new(self, 2)
    }
    #[doc = "Bit 3 - RTC update finish"]
    #[inline(always)]
    pub fn updf(&mut self) -> UPDF_W<'_, CTRLL_SPEC> {
        UPDF_W::new(self, 3)
    }
    #[doc = "Bit 4 - RTC configuration enable"]
    #[inline(always)]
    pub fn cfgen(&mut self) -> CFGEN_W<'_, CTRLL_SPEC> {
        CFGEN_W::new(self, 4)
    }
}
#[doc = "RTC Control Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLL_SPEC;
impl crate::RegisterSpec for CTRLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrll::R`](R) reader structure"]
impl crate::Readable for CTRLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrll::W`](W) writer structure"]
impl crate::Writable for CTRLL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLL to value 0x20"]
impl crate::Resettable for CTRLL_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
