#[doc = "Register `AHBRST2` reader"]
pub type R = crate::R<AHBRST2_SPEC>;
#[doc = "Register `AHBRST2` writer"]
pub type W = crate::W<AHBRST2_SPEC>;
#[doc = "Field `DVP` reader - DVP reset"]
pub type DVP_R = crate::BitReader;
#[doc = "Field `DVP` writer - DVP reset"]
pub type DVP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGFS1` reader - OTGFS1 reset"]
pub type OTGFS1_R = crate::BitReader;
#[doc = "Field `OTGFS1` writer - OTGFS1 reset"]
pub type OTGFS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO1` reader - SDIO1 reset"]
pub type SDIO1_R = crate::BitReader;
#[doc = "Field `SDIO1` writer - SDIO1 reset"]
pub type SDIO1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DVP reset"]
    #[inline(always)]
    pub fn dvp(&self) -> DVP_R {
        DVP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - OTGFS1 reset"]
    #[inline(always)]
    pub fn otgfs1(&self) -> OTGFS1_R {
        OTGFS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO1 reset"]
    #[inline(always)]
    pub fn sdio1(&self) -> SDIO1_R {
        SDIO1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRST2")
            .field("dvp", &self.dvp())
            .field("otgfs1", &self.otgfs1())
            .field("sdio1", &self.sdio1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DVP reset"]
    #[inline(always)]
    #[must_use]
    pub fn dvp(&mut self) -> DVP_W<AHBRST2_SPEC> {
        DVP_W::new(self, 0)
    }
    #[doc = "Bit 7 - OTGFS1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs1(&mut self) -> OTGFS1_W<AHBRST2_SPEC> {
        OTGFS1_W::new(self, 7)
    }
    #[doc = "Bit 15 - SDIO1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1(&mut self) -> SDIO1_W<AHBRST2_SPEC> {
        SDIO1_W::new(self, 15)
    }
}
#[doc = "AHB peripheral reset register 2 (CRM_AHBRST2)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrst2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrst2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRST2_SPEC;
impl crate::RegisterSpec for AHBRST2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst2::R`](R) reader structure"]
impl crate::Readable for AHBRST2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbrst2::W`](W) writer structure"]
impl crate::Writable for AHBRST2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRST2 to value 0"]
impl crate::Resettable for AHBRST2_SPEC {
    const RESET_VALUE: u32 = 0;
}
