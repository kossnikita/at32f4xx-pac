#[doc = "Register `DMARPD` reader"]
pub type R = crate::R<DMARPD_SPEC>;
#[doc = "Register `DMARPD` writer"]
pub type W = crate::W<DMARPD_SPEC>;
#[doc = "Field `RPD` reader - Receive poll demand"]
pub type RPD_R = crate::FieldReader<u32>;
#[doc = "Field `RPD` writer - Receive poll demand"]
pub type RPD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    pub fn rpd(&self) -> RPD_R {
        RPD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMARPD")
            .field("rpd", &format_args!("{}", self.rpd().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DMARPD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    #[must_use]
    pub fn rpd(&mut self) -> RPD_W<DMARPD_SPEC, 0> {
        RPD_W::new(self)
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
#[doc = "EHERNET DMA receive poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarpd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarpd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARPD_SPEC;
impl crate::RegisterSpec for DMARPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarpd::R`](R) reader structure"]
impl crate::Readable for DMARPD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmarpd::W`](W) writer structure"]
impl crate::Writable for DMARPD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMARPD to value 0"]
impl crate::Resettable for DMARPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
