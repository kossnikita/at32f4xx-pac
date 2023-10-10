#[doc = "Register `S6M0ADDR` reader"]
pub type R = crate::R<S6M0ADDR_SPEC>;
#[doc = "Register `S6M0ADDR` writer"]
pub type W = crate::W<S6M0ADDR_SPEC>;
#[doc = "Field `M0ADDR` reader - Memory 0 address"]
pub type M0ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `M0ADDR` writer - Memory 0 address"]
pub type M0ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    pub fn m0addr(&self) -> M0ADDR_R {
        M0ADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S6M0ADDR")
            .field("m0addr", &format_args!("{}", self.m0addr().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<S6M0ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    #[must_use]
    pub fn m0addr(&mut self) -> M0ADDR_W<S6M0ADDR_SPEC, 0> {
        M0ADDR_W::new(self)
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
#[doc = "stream 6 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6m0addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6m0addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S6M0ADDR_SPEC;
impl crate::RegisterSpec for S6M0ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s6m0addr::R`](R) reader structure"]
impl crate::Readable for S6M0ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s6m0addr::W`](W) writer structure"]
impl crate::Writable for S6M0ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S6M0ADDR to value 0"]
impl crate::Resettable for S6M0ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
