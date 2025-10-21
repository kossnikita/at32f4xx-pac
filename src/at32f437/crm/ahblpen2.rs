#[doc = "Register `AHBLPEN2` reader"]
pub type R = crate::R<AHBLPEN2_SPEC>;
#[doc = "Register `AHBLPEN2` writer"]
pub type W = crate::W<AHBLPEN2_SPEC>;
#[doc = "Field `DVPLP` reader - DVP clock enable during sleep mode"]
pub type DVPLP_R = crate::BitReader;
#[doc = "Field `DVPLP` writer - DVP clock enable during sleep mode"]
pub type DVPLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGFS1LP` reader - OTGFS1 clock enable during sleep mode"]
pub type OTGFS1LP_R = crate::BitReader;
#[doc = "Field `OTGFS1LP` writer - OTGFS1 clock enable during sleep mode"]
pub type OTGFS1LP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO1LP` reader - SDIO1 clock enable during sleep mode"]
pub type SDIO1LP_R = crate::BitReader;
#[doc = "Field `SDIO1LP` writer - SDIO1 clock enable during sleep mode"]
pub type SDIO1LP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DVP clock enable during sleep mode"]
    #[inline(always)]
    pub fn dvplp(&self) -> DVPLP_R {
        DVPLP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - OTGFS1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn otgfs1lp(&self) -> OTGFS1LP_R {
        OTGFS1LP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sdio1lp(&self) -> SDIO1LP_R {
        SDIO1LP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBLPEN2")
            .field("dvplp", &self.dvplp())
            .field("otgfs1lp", &self.otgfs1lp())
            .field("sdio1lp", &self.sdio1lp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DVP clock enable during sleep mode"]
    #[inline(always)]
    pub fn dvplp(&mut self) -> DVPLP_W<'_, AHBLPEN2_SPEC> {
        DVPLP_W::new(self, 0)
    }
    #[doc = "Bit 7 - OTGFS1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn otgfs1lp(&mut self) -> OTGFS1LP_W<'_, AHBLPEN2_SPEC> {
        OTGFS1LP_W::new(self, 7)
    }
    #[doc = "Bit 15 - SDIO1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sdio1lp(&mut self) -> SDIO1LP_W<'_, AHBLPEN2_SPEC> {
        SDIO1LP_W::new(self, 15)
    }
}
#[doc = "AHB peripheral Low-power clock enable register 2 (CRM_AHBLPEN2)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahblpen2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahblpen2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBLPEN2_SPEC;
impl crate::RegisterSpec for AHBLPEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahblpen2::R`](R) reader structure"]
impl crate::Readable for AHBLPEN2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahblpen2::W`](W) writer structure"]
impl crate::Writable for AHBLPEN2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBLPEN2 to value 0x8081"]
impl crate::Resettable for AHBLPEN2_SPEC {
    const RESET_VALUE: u32 = 0x8081;
}
