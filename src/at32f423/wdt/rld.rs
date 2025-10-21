#[doc = "Register `RLD` reader"]
pub type R = crate::R<RLD_SPEC>;
#[doc = "Register `RLD` writer"]
pub type W = crate::W<RLD_SPEC>;
#[doc = "Field `RLD` reader - Reload value"]
pub type RLD_R = crate::FieldReader<u16>;
#[doc = "Field `RLD` writer - Reload value"]
pub type RLD_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:11 - Reload value"]
    #[inline(always)]
    pub fn rld(&self) -> RLD_R {
        RLD_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RLD").field("rld", &self.rld()).finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Reload value"]
    #[inline(always)]
    pub fn rld(&mut self) -> RLD_W<'_, RLD_SPEC> {
        RLD_W::new(self, 0)
    }
}
#[doc = "Reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`rld::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rld::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RLD_SPEC;
impl crate::RegisterSpec for RLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rld::R`](R) reader structure"]
impl crate::Readable for RLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rld::W`](W) writer structure"]
impl crate::Writable for RLD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RLD to value 0x0fff"]
impl crate::Resettable for RLD_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
