#[doc = "Register `DMATDLADDR` reader"]
pub type R = crate::R<DMATDLADDR_SPEC>;
#[doc = "Register `DMATDLADDR` writer"]
pub type W = crate::W<DMATDLADDR_SPEC>;
#[doc = "Field `STL` reader - Start of transmit list"]
pub type STL_R = crate::FieldReader<u32>;
#[doc = "Field `STL` writer - Start of transmit list"]
pub type STL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
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
            .field("stl", &self.stl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of transmit list"]
    #[inline(always)]
    #[must_use]
    pub fn stl(&mut self) -> STL_W<DMATDLADDR_SPEC> {
        STL_W::new(self, 0)
    }
}
#[doc = "Ethernet DMA transmit descriptor list address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatdladdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatdladdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATDLADDR_SPEC;
impl crate::RegisterSpec for DMATDLADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatdladdr::R`](R) reader structure"]
impl crate::Readable for DMATDLADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmatdladdr::W`](W) writer structure"]
impl crate::Writable for DMATDLADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMATDLADDR to value 0"]
impl crate::Resettable for DMATDLADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
