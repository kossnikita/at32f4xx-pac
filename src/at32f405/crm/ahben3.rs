#[doc = "Register `AHBEN3` reader"]
pub type R = crate::R<AHBEN3_SPEC>;
#[doc = "Register `AHBEN3` writer"]
pub type W = crate::W<AHBEN3_SPEC>;
#[doc = "Field `QSPI1` reader - QSPI1 clock enable"]
pub type QSPI1_R = crate::BitReader;
#[doc = "Field `QSPI1` writer - QSPI1 clock enable"]
pub type QSPI1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - QSPI1 clock enable"]
    #[inline(always)]
    pub fn qspi1(&self) -> QSPI1_R {
        QSPI1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBEN3")
            .field("qspi1", &self.qspi1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - QSPI1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn qspi1(&mut self) -> QSPI1_W<AHBEN3_SPEC> {
        QSPI1_W::new(self, 1)
    }
}
#[doc = "AHB peripheral clock enable register 3 (CRM_AHBEN3)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahben3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahben3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBEN3_SPEC;
impl crate::RegisterSpec for AHBEN3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben3::R`](R) reader structure"]
impl crate::Readable for AHBEN3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahben3::W`](W) writer structure"]
impl crate::Writable for AHBEN3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBEN3 to value 0"]
impl crate::Resettable for AHBEN3_SPEC {
    const RESET_VALUE: u32 = 0;
}
