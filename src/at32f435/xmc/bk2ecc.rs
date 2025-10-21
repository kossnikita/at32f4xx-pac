#[doc = "Register `BK2ECC` reader"]
pub type R = crate::R<BK2ECC_SPEC>;
#[doc = "Register `BK2ECC` writer"]
pub type W = crate::W<BK2ECC_SPEC>;
#[doc = "Field `ECC` reader - ECC result"]
pub type ECC_R = crate::FieldReader<u32>;
#[doc = "Field `ECC` writer - ECC result"]
pub type ECC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC result"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK2ECC").field("ecc", &self.ecc()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - ECC result"]
    #[inline(always)]
    pub fn ecc(&mut self) -> ECC_W<'_, BK2ECC_SPEC> {
        ECC_W::new(self, 0)
    }
}
#[doc = "ECC result register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bk2ecc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk2ecc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK2ECC_SPEC;
impl crate::RegisterSpec for BK2ECC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk2ecc::R`](R) reader structure"]
impl crate::Readable for BK2ECC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk2ecc::W`](W) writer structure"]
impl crate::Writable for BK2ECC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BK2ECC to value 0"]
impl crate::Resettable for BK2ECC_SPEC {}
