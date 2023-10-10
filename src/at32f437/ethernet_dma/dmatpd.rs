#[doc = "Register `DMATPD` reader"]
pub type R = crate::R<DMATPD_SPEC>;
#[doc = "Register `DMATPD` writer"]
pub type W = crate::W<DMATPD_SPEC>;
#[doc = "Field `TPD` reader - Transmit poll demand"]
pub type TPD_R = crate::FieldReader<u32>;
#[doc = "Field `TPD` writer - Transmit poll demand"]
pub type TPD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMATPD")
            .field("tpd", &format_args!("{}", self.tpd().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DMATPD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    #[must_use]
    pub fn tpd(&mut self) -> TPD_W<DMATPD_SPEC, 0> {
        TPD_W::new(self)
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
#[doc = "Ethernet DMA transmit poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatpd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatpd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATPD_SPEC;
impl crate::RegisterSpec for DMATPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatpd::R`](R) reader structure"]
impl crate::Readable for DMATPD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmatpd::W`](W) writer structure"]
impl crate::Writable for DMATPD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMATPD to value 0"]
impl crate::Resettable for DMATPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
