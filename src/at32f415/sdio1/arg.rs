#[doc = "Register `ARG` reader"]
pub type R = crate::R<ARG_SPEC>;
#[doc = "Register `ARG` writer"]
pub type W = crate::W<ARG_SPEC>;
#[doc = "Field `ARG` reader - Command argument"]
pub type ARG_R = crate::FieldReader<u32>;
#[doc = "Field `ARG` writer - Command argument"]
pub type ARG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    pub fn arg(&self) -> ARG_R {
        ARG_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARG").field("arg", &self.arg()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    #[must_use]
    pub fn arg(&mut self) -> ARG_W<ARG_SPEC> {
        ARG_W::new(self, 0)
    }
}
#[doc = "Bits 31:0 = : Command argument\n\nYou can [`read`](crate::Reg::read) this register and get [`arg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARG_SPEC;
impl crate::RegisterSpec for ARG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arg::R`](R) reader structure"]
impl crate::Readable for ARG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arg::W`](W) writer structure"]
impl crate::Writable for ARG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARG to value 0"]
impl crate::Resettable for ARG_SPEC {
    const RESET_VALUE: u32 = 0;
}
