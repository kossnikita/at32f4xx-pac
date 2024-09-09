#[doc = "Register `AHBRST3` reader"]
pub type R = crate::R<AHBRST3_SPEC>;
#[doc = "Register `AHBRST3` writer"]
pub type W = crate::W<AHBRST3_SPEC>;
#[doc = "Field `XMC` reader - XMC reset"]
pub type XMC_R = crate::BitReader;
#[doc = "Field `XMC` writer - XMC reset"]
pub type XMC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XMC reset"]
    #[inline(always)]
    pub fn xmc(&self) -> XMC_R {
        XMC_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRST3").field("xmc", &self.xmc()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - XMC reset"]
    #[inline(always)]
    #[must_use]
    pub fn xmc(&mut self) -> XMC_W<AHBRST3_SPEC> {
        XMC_W::new(self, 0)
    }
}
#[doc = "AHB peripheral reset register 3 (CRM_AHBRST3)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrst3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrst3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRST3_SPEC;
impl crate::RegisterSpec for AHBRST3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst3::R`](R) reader structure"]
impl crate::Readable for AHBRST3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbrst3::W`](W) writer structure"]
impl crate::Writable for AHBRST3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRST3 to value 0"]
impl crate::Resettable for AHBRST3_SPEC {
    const RESET_VALUE: u32 = 0;
}
