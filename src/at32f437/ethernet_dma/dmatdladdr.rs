#[doc = "Register `DMATDLADDR` reader"]
pub type R = crate::R<DMATDLADDR_SPEC>;
#[doc = "Register `DMATDLADDR` writer"]
pub type W = crate::W<DMATDLADDR_SPEC>;
#[doc = "Field `STL` reader - Start of transmit list"]
pub type STL_R = crate::FieldReader<u32>;
#[doc = "Field `STL` writer - Start of transmit list"]
pub type STL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of transmit list"]
    #[inline(always)]
    pub fn stl(&self) -> STL_R {
        STL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMATDLADDR")
            .field("stl", &format_args!("{}", self.stl().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DMATDLADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of transmit list"]
    #[inline(always)]
    #[must_use]
    pub fn stl(&mut self) -> STL_W<DMATDLADDR_SPEC, 0> {
        STL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet DMA transmit descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatdladdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatdladdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATDLADDR_SPEC;
impl crate::RegisterSpec for DMATDLADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatdladdr::R`](R) reader structure"]
impl crate::Readable for DMATDLADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmatdladdr::W`](W) writer structure"]
impl crate::Writable for DMATDLADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMATDLADDR to value 0"]
impl crate::Resettable for DMATDLADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
