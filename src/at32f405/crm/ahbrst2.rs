#[doc = "Register `AHBRST2` reader"]
pub type R = crate::R<AHBRST2_SPEC>;
#[doc = "Register `AHBRST2` writer"]
pub type W = crate::W<AHBRST2_SPEC>;
#[doc = "Field `OTGFS1RST` reader - OTGFS1 reset"]
pub type OTGFS1RST_R = crate::BitReader;
#[doc = "Field `OTGFS1RST` writer - OTGFS1 reset"]
pub type OTGFS1RST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 7 - OTGFS1 reset"]
    #[inline(always)]
    pub fn otgfs1rst(&self) -> OTGFS1RST_R {
        OTGFS1RST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - OTGFS1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs1rst(&mut self) -> OTGFS1RST_W<AHBRST2_SPEC, 7> {
        OTGFS1RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AHB peripheral reset register 2 (CRM_AHBRST2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRST2_SPEC;
impl crate::RegisterSpec for AHBRST2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst2::R`](R) reader structure"]
impl crate::Readable for AHBRST2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbrst2::W`](W) writer structure"]
impl crate::Writable for AHBRST2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBRST2 to value 0"]
impl crate::Resettable for AHBRST2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
