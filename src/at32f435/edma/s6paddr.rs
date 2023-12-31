#[doc = "Register `S6PADDR` reader"]
pub type R = crate::R<S6PADDR_SPEC>;
#[doc = "Register `S6PADDR` writer"]
pub type W = crate::W<S6PADDR_SPEC>;
#[doc = "Field `PADDR` reader - Peripheral address"]
pub type PADDR_R = crate::FieldReader<u32>;
#[doc = "Field `PADDR` writer - Peripheral address"]
pub type PADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn paddr(&self) -> PADDR_R {
        PADDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S6PADDR")
            .field("paddr", &format_args!("{}", self.paddr().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<S6PADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    #[must_use]
    pub fn paddr(&mut self) -> PADDR_W<S6PADDR_SPEC> {
        PADDR_W::new(self, 0)
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
#[doc = "stream 6 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6paddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6paddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S6PADDR_SPEC;
impl crate::RegisterSpec for S6PADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s6paddr::R`](R) reader structure"]
impl crate::Readable for S6PADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s6paddr::W`](W) writer structure"]
impl crate::Writable for S6PADDR_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S6PADDR to value 0"]
impl crate::Resettable for S6PADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
