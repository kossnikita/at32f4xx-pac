#[doc = "Register `S2LLP` reader"]
pub type R = crate::R<S2LLP_SPEC>;
#[doc = "Register `S2LLP` writer"]
pub type W = crate::W<S2LLP_SPEC>;
#[doc = "Field `LLP` reader - Link list pointer"]
pub type LLP_R = crate::FieldReader<u32>;
#[doc = "Field `LLP` writer - Link list pointer"]
pub type LLP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Link list pointer"]
    #[inline(always)]
    pub fn llp(&self) -> LLP_R {
        LLP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S2LLP")
            .field("llp", &format_args!("{}", self.llp().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<S2LLP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Link list pointer"]
    #[inline(always)]
    #[must_use]
    pub fn llp(&mut self) -> LLP_W<S2LLP_SPEC> {
        LLP_W::new(self, 0)
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
#[doc = "Stream 2 Link List Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2llp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2llp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2LLP_SPEC;
impl crate::RegisterSpec for S2LLP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s2llp::R`](R) reader structure"]
impl crate::Readable for S2LLP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s2llp::W`](W) writer structure"]
impl crate::Writable for S2LLP_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S2LLP to value 0"]
impl crate::Resettable for S2LLP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
