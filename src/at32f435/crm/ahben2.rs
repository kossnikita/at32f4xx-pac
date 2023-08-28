#[doc = "Register `AHBEN2` reader"]
pub type R = crate::R<AHBEN2_SPEC>;
#[doc = "Register `AHBEN2` writer"]
pub type W = crate::W<AHBEN2_SPEC>;
#[doc = "Field `DVPEN` reader - DVP clock enable"]
pub type DVPEN_R = crate::BitReader;
#[doc = "Field `DVPEN` writer - DVP clock enable"]
pub type DVPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OTGFS1EN` reader - OTGFS1 clock enable"]
pub type OTGFS1EN_R = crate::BitReader;
#[doc = "Field `OTGFS1EN` writer - OTGFS1 clock enable"]
pub type OTGFS1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIO1EN` reader - SDIO1 clock enable"]
pub type SDIO1EN_R = crate::BitReader;
#[doc = "Field `SDIO1EN` writer - SDIO1 clock enable"]
pub type SDIO1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - DVP clock enable"]
    #[inline(always)]
    pub fn dvpen(&self) -> DVPEN_R {
        DVPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - OTGFS1 clock enable"]
    #[inline(always)]
    pub fn otgfs1en(&self) -> OTGFS1EN_R {
        OTGFS1EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO1 clock enable"]
    #[inline(always)]
    pub fn sdio1en(&self) -> SDIO1EN_R {
        SDIO1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DVP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dvpen(&mut self) -> DVPEN_W<AHBEN2_SPEC, 0> {
        DVPEN_W::new(self)
    }
    #[doc = "Bit 7 - OTGFS1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs1en(&mut self) -> OTGFS1EN_W<AHBEN2_SPEC, 7> {
        OTGFS1EN_W::new(self)
    }
    #[doc = "Bit 15 - SDIO1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1en(&mut self) -> SDIO1EN_W<AHBEN2_SPEC, 15> {
        SDIO1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBEN2 to value 0"]
impl crate::Resettable for AHBEN2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
