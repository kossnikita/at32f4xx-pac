#[doc = "Register `PSR` reader"]
pub type R = crate::R<PSR_SPEC>;
#[doc = "Register `PSR` writer"]
pub type W = crate::W<PSR_SPEC>;
#[doc = "Field `WTCYC` reader - Wait cycle"]
pub type WTCYC_R = crate::FieldReader;
#[doc = "Field `WTCYC` writer - Wait cycle"]
pub type WTCYC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HFCYC_EN` reader - Half cycle acceleration access enable"]
pub type HFCYC_EN_R = crate::BitReader;
#[doc = "Field `HFCYC_EN` writer - Half cycle acceleration access enable"]
pub type HFCYC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFT_EN` reader - Prefetch enable"]
pub type PFT_EN_R = crate::BitReader;
#[doc = "Field `PFT_EN` writer - Prefetch enable"]
pub type PFT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFT_ENF` reader - Prefetch enabled flag"]
pub type PFT_ENF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Wait cycle"]
    #[inline(always)]
    pub fn wtcyc(&self) -> WTCYC_R {
        WTCYC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Half cycle acceleration access enable"]
    #[inline(always)]
    pub fn hfcyc_en(&self) -> HFCYC_EN_R {
        HFCYC_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Prefetch enable"]
    #[inline(always)]
    pub fn pft_en(&self) -> PFT_EN_R {
        PFT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Prefetch enabled flag"]
    #[inline(always)]
    pub fn pft_enf(&self) -> PFT_ENF_R {
        PFT_ENF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSR")
            .field("wtcyc", &format_args!("{}", self.wtcyc().bits()))
            .field("hfcyc_en", &format_args!("{}", self.hfcyc_en().bit()))
            .field("pft_en", &format_args!("{}", self.pft_en().bit()))
            .field("pft_enf", &format_args!("{}", self.pft_enf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PSR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Wait cycle"]
    #[inline(always)]
    #[must_use]
    pub fn wtcyc(&mut self) -> WTCYC_W<PSR_SPEC> {
        WTCYC_W::new(self, 0)
    }
    #[doc = "Bit 3 - Half cycle acceleration access enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfcyc_en(&mut self) -> HFCYC_EN_W<PSR_SPEC> {
        HFCYC_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn pft_en(&mut self) -> PFT_EN_W<PSR_SPEC> {
        PFT_EN_W::new(self, 4)
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
#[doc = "Performance selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psr::W`](W) writer structure"]
impl crate::Writable for PSR_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSR to value 0x30"]
impl crate::Resettable for PSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}
