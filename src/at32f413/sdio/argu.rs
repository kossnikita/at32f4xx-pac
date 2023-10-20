#[doc = "Register `ARGU` reader"]
pub type R = crate::R<ARGU_SPEC>;
#[doc = "Register `ARGU` writer"]
pub type W = crate::W<ARGU_SPEC>;
#[doc = "Field `ARGU` reader - Command argument"]
pub type ARGU_R = crate::FieldReader<u32>;
#[doc = "Field `ARGU` writer - Command argument"]
pub type ARGU_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    pub fn argu(&self) -> ARGU_R {
        ARGU_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARGU")
            .field("argu", &format_args!("{}", self.argu().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<ARGU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    #[must_use]
    pub fn argu(&mut self) -> ARGU_W<ARGU_SPEC> {
        ARGU_W::new(self, 0)
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
#[doc = "Bits 31:0 = : Command argument\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`argu::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`argu::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARGU_SPEC;
impl crate::RegisterSpec for ARGU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`argu::R`](R) reader structure"]
impl crate::Readable for ARGU_SPEC {}
#[doc = "`write(|w| ..)` method takes [`argu::W`](W) writer structure"]
impl crate::Writable for ARGU_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARGU to value 0"]
impl crate::Resettable for ARGU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
