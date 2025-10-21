#[doc = "Register `PSR` reader"]
pub type R = crate::R<PSR_SPEC>;
#[doc = "Register `PSR` writer"]
pub type W = crate::W<PSR_SPEC>;
#[doc = "Field `WTCYC` reader - Wait cycle"]
pub type WTCYC_R = crate::FieldReader;
#[doc = "Field `WTCYC` writer - Wait cycle"]
pub type WTCYC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PFT_EN` reader - Prefetch enable"]
pub type PFT_EN_R = crate::BitReader;
#[doc = "Field `PFT_EN` writer - Prefetch enable"]
pub type PFT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFT_ENF` reader - Prefetch enabled flag"]
pub type PFT_ENF_R = crate::BitReader;
#[doc = "Field `PFT_EN2` reader - Prefetch enable 2"]
pub type PFT_EN2_R = crate::BitReader;
#[doc = "Field `PFT_EN2` writer - Prefetch enable 2"]
pub type PFT_EN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFT_ENF2` reader - Prefetch enabled flag 2"]
pub type PFT_ENF2_R = crate::BitReader;
#[doc = "Field `PFT_LAT_DIS` reader - Prefetch latency disable"]
pub type PFT_LAT_DIS_R = crate::BitReader;
#[doc = "Field `PFT_LAT_DIS` writer - Prefetch latency disable"]
pub type PFT_LAT_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Wait cycle"]
    #[inline(always)]
    pub fn wtcyc(&self) -> WTCYC_R {
        WTCYC_R::new((self.bits & 7) as u8)
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
    #[doc = "Bit 6 - Prefetch enable 2"]
    #[inline(always)]
    pub fn pft_en2(&self) -> PFT_EN2_R {
        PFT_EN2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Prefetch enabled flag 2"]
    #[inline(always)]
    pub fn pft_enf2(&self) -> PFT_ENF2_R {
        PFT_ENF2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Prefetch latency disable"]
    #[inline(always)]
    pub fn pft_lat_dis(&self) -> PFT_LAT_DIS_R {
        PFT_LAT_DIS_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSR")
            .field("wtcyc", &self.wtcyc())
            .field("pft_en", &self.pft_en())
            .field("pft_enf", &self.pft_enf())
            .field("pft_en2", &self.pft_en2())
            .field("pft_enf2", &self.pft_enf2())
            .field("pft_lat_dis", &self.pft_lat_dis())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Wait cycle"]
    #[inline(always)]
    pub fn wtcyc(&mut self) -> WTCYC_W<'_, PSR_SPEC> {
        WTCYC_W::new(self, 0)
    }
    #[doc = "Bit 4 - Prefetch enable"]
    #[inline(always)]
    pub fn pft_en(&mut self) -> PFT_EN_W<'_, PSR_SPEC> {
        PFT_EN_W::new(self, 4)
    }
    #[doc = "Bit 6 - Prefetch enable 2"]
    #[inline(always)]
    pub fn pft_en2(&mut self) -> PFT_EN2_W<'_, PSR_SPEC> {
        PFT_EN2_W::new(self, 6)
    }
    #[doc = "Bit 8 - Prefetch latency disable"]
    #[inline(always)]
    pub fn pft_lat_dis(&mut self) -> PFT_LAT_DIS_W<'_, PSR_SPEC> {
        PFT_LAT_DIS_W::new(self, 8)
    }
}
#[doc = "Performance selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psr::W`](W) writer structure"]
impl crate::Writable for PSR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSR to value 0x30"]
impl crate::Resettable for PSR_SPEC {
    const RESET_VALUE: u32 = 0x30;
}
