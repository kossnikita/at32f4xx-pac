#[doc = "Register `AHBEN2` reader"]
pub type R = crate::R<AHBEN2_SPEC>;
#[doc = "Register `AHBEN2` writer"]
pub type W = crate::W<AHBEN2_SPEC>;
#[doc = "Field `DVP` reader - DVP clock enable"]
pub type DVP_R = crate::BitReader;
#[doc = "Field `DVP` writer - DVP clock enable"]
pub type DVP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGFS1` reader - OTGFS1 clock enable"]
pub type OTGFS1_R = crate::BitReader;
#[doc = "Field `OTGFS1` writer - OTGFS1 clock enable"]
pub type OTGFS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO1` reader - SDIO1 clock enable"]
pub type SDIO1_R = crate::BitReader;
#[doc = "Field `SDIO1` writer - SDIO1 clock enable"]
pub type SDIO1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DVP clock enable"]
    #[inline(always)]
    pub fn dvp(&self) -> DVP_R {
        DVP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - OTGFS1 clock enable"]
    #[inline(always)]
    pub fn otgfs1(&self) -> OTGFS1_R {
        OTGFS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO1 clock enable"]
    #[inline(always)]
    pub fn sdio1(&self) -> SDIO1_R {
        SDIO1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBEN2")
            .field("dvp", &format_args!("{}", self.dvp().bit()))
            .field("otgfs1", &format_args!("{}", self.otgfs1().bit()))
            .field("sdio1", &format_args!("{}", self.sdio1().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<AHBEN2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - DVP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dvp(&mut self) -> DVP_W<AHBEN2_SPEC> {
        DVP_W::new(self, 0)
    }
    #[doc = "Bit 7 - OTGFS1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs1(&mut self) -> OTGFS1_W<AHBEN2_SPEC> {
        OTGFS1_W::new(self, 7)
    }
    #[doc = "Bit 15 - SDIO1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1(&mut self) -> SDIO1_W<AHBEN2_SPEC> {
        SDIO1_W::new(self, 15)
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
#[doc = "AHB peripheral clock enable register 2 (CRM_AHBEN2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBEN2_SPEC;
impl crate::RegisterSpec for AHBEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben2::R`](R) reader structure"]
impl crate::Readable for AHBEN2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahben2::W`](W) writer structure"]
impl crate::Writable for AHBEN2_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBEN2 to value 0"]
impl crate::Resettable for AHBEN2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
