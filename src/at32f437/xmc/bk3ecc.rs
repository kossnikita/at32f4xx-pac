#[doc = "Register `BK3ECC` reader"]
pub type R = crate::R<BK3ECC_SPEC>;
#[doc = "Register `BK3ECC` writer"]
pub type W = crate::W<BK3ECC_SPEC>;
#[doc = "Field `ECC` reader - ECC result"]
pub type ECC_R = crate::FieldReader<u32>;
#[doc = "Field `ECC` writer - ECC result"]
pub type ECC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC result"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK3ECC")
            .field("ecc", &format_args!("{}", self.ecc().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<BK3ECC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - ECC result"]
    #[inline(always)]
    #[must_use]
    pub fn ecc(&mut self) -> ECC_W<BK3ECC_SPEC, 0> {
        ECC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "ECC result register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk3ecc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk3ecc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BK3ECC_SPEC;
impl crate::RegisterSpec for BK3ECC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk3ecc::R`](R) reader structure"]
impl crate::Readable for BK3ECC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bk3ecc::W`](W) writer structure"]
impl crate::Writable for BK3ECC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BK3ECC to value 0"]
impl crate::Resettable for BK3ECC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
