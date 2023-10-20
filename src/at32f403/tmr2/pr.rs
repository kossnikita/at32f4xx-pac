#[doc = "Register `PR` reader"]
pub type R = crate::R<PR_SPEC>;
#[doc = "Register `PR` writer"]
pub type W = crate::W<PR_SPEC>;
#[doc = "Field `PR` reader - Period value"]
pub type PR_R = crate::FieldReader<u32>;
#[doc = "Field `PR` writer - Period value"]
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Period value"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR")
            .field("pr", &format_args!("{}", self.pr().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Period value"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<PR_SPEC> {
        PR_W::new(self, 0)
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
#[doc = "Period value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PR_SPEC;
impl crate::RegisterSpec for PR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr::R`](R) reader structure"]
impl crate::Readable for PR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pr::W`](W) writer structure"]
impl crate::Writable for PR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
