#[doc = "Register `AHBEN2` reader"]
pub type R = crate::R<AHBEN2_SPEC>;
#[doc = "Register `AHBEN2` writer"]
pub type W = crate::W<AHBEN2_SPEC>;
#[doc = "Field `OTGFS1` reader - OTGFS1 clock enable"]
pub type OTGFS1_R = crate::BitReader;
#[doc = "Field `OTGFS1` writer - OTGFS1 clock enable"]
pub type OTGFS1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - OTGFS1 clock enable"]
    #[inline(always)]
    pub fn otgfs1(&self) -> OTGFS1_R {
        OTGFS1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBEN2")
            .field("otgfs1", &self.otgfs1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 7 - OTGFS1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs1(&mut self) -> OTGFS1_W<AHBEN2_SPEC> {
        OTGFS1_W::new(self, 7)
    }
}
#[doc = "AHB peripheral clock enable register 2 (CRM_AHBEN2)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahben2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahben2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBEN2_SPEC;
impl crate::RegisterSpec for AHBEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben2::R`](R) reader structure"]
impl crate::Readable for AHBEN2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahben2::W`](W) writer structure"]
impl crate::Writable for AHBEN2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBEN2 to value 0"]
impl crate::Resettable for AHBEN2_SPEC {
    const RESET_VALUE: u32 = 0;
}
