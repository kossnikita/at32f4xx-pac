#[doc = "Register `POLY` reader"]
pub type R = crate::R<POLY_SPEC>;
#[doc = "Register `POLY` writer"]
pub type W = crate::W<POLY_SPEC>;
#[doc = "Field `POLY` reader - polynomial coefficient"]
pub type POLY_R = crate::FieldReader<u32>;
#[doc = "Field `POLY` writer - polynomial coefficient"]
pub type POLY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - polynomial coefficient"]
    #[inline(always)]
    pub fn poly(&self) -> POLY_R {
        POLY_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POLY")
            .field("poly", &format_args!("{}", self.poly().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<POLY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - polynomial coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn poly(&mut self) -> POLY_W<POLY_SPEC> {
        POLY_W::new(self, 0)
    }
}
#[doc = "Polynomial coefficient register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`poly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`poly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POLY_SPEC;
impl crate::RegisterSpec for POLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`poly::R`](R) reader structure"]
impl crate::Readable for POLY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`poly::W`](W) writer structure"]
impl crate::Writable for POLY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POLY to value 0x04c1_1db7"]
impl crate::Resettable for POLY_SPEC {
    const RESET_VALUE: u32 = 0x04c1_1db7;
}
