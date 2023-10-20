#[doc = "Register `S3M1ADDR` reader"]
pub type R = crate::R<S3M1ADDR_SPEC>;
#[doc = "Register `S3M1ADDR` writer"]
pub type W = crate::W<S3M1ADDR_SPEC>;
#[doc = "Field `M1ADDR` reader - Memory 1 address (used in case of Double buffer mode)"]
pub type M1ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `M1ADDR` writer - Memory 1 address (used in case of Double buffer mode)"]
pub type M1ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn m1addr(&self) -> M1ADDR_R {
        M1ADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S3M1ADDR")
            .field("m1addr", &format_args!("{}", self.m1addr().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<S3M1ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    #[must_use]
    pub fn m1addr(&mut self) -> M1ADDR_W<S3M1ADDR_SPEC> {
        M1ADDR_W::new(self, 0)
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
#[doc = "stream 3 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3m1addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3m1addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S3M1ADDR_SPEC;
impl crate::RegisterSpec for S3M1ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s3m1addr::R`](R) reader structure"]
impl crate::Readable for S3M1ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s3m1addr::W`](W) writer structure"]
impl crate::Writable for S3M1ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S3M1ADDR to value 0"]
impl crate::Resettable for S3M1ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
