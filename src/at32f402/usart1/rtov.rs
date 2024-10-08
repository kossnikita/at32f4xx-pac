#[doc = "Register `RTOV` reader"]
pub type R = crate::R<RTOV_SPEC>;
#[doc = "Register `RTOV` writer"]
pub type W = crate::W<RTOV_SPEC>;
#[doc = "Field `RTOV` reader - Receiver time out value"]
pub type RTOV_R = crate::FieldReader<u32>;
#[doc = "Field `RTOV` writer - Receiver time out value"]
pub type RTOV_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Receiver time out value"]
    #[inline(always)]
    pub fn rtov(&self) -> RTOV_R {
        RTOV_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTOV").field("rtov", &self.rtov()).finish()
    }
}
impl W {
    #[doc = "Bits 0:23 - Receiver time out value"]
    #[inline(always)]
    #[must_use]
    pub fn rtov(&mut self) -> RTOV_W<RTOV_SPEC> {
        RTOV_W::new(self, 0)
    }
}
#[doc = "Receiver time out value register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtov::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtov::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTOV_SPEC;
impl crate::RegisterSpec for RTOV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtov::R`](R) reader structure"]
impl crate::Readable for RTOV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtov::W`](W) writer structure"]
impl crate::Writable for RTOV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTOV to value 0"]
impl crate::Resettable for RTOV_SPEC {
    const RESET_VALUE: u32 = 0;
}
