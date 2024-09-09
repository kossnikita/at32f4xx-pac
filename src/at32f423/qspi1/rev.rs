#[doc = "Register `REV` reader"]
pub type R = crate::R<REV_SPEC>;
#[doc = "Register `REV` writer"]
pub type W = crate::W<REV_SPEC>;
#[doc = "Field `REVISION` reader - Revision number"]
pub type REVISION_R = crate::FieldReader<u32>;
#[doc = "Field `REVISION` writer - Revision number"]
pub type REVISION_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - Revision number"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(self.bits & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REV")
            .field("revision", &self.revision())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - Revision number"]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> REVISION_W<REV_SPEC> {
        REVISION_W::new(self, 0)
    }
}
#[doc = "Revision\n\nYou can [`read`](crate::Reg::read) this register and get [`rev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REV_SPEC;
impl crate::RegisterSpec for REV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rev::R`](R) reader structure"]
impl crate::Readable for REV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rev::W`](W) writer structure"]
impl crate::Writable for REV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REV to value 0x0001_0500"]
impl crate::Resettable for REV_SPEC {
    const RESET_VALUE: u32 = 0x0001_0500;
}
