#[doc = "Register `BAUDR` reader"]
pub type R = crate::R<BAUDR_SPEC>;
#[doc = "Register `BAUDR` writer"]
pub type W = crate::W<BAUDR_SPEC>;
#[doc = "Field `DIV` reader - Division"]
pub type DIV_R = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Division"]
pub type DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Division"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BAUDR").field("div", &self.div()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Division"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<'_, BAUDR_SPEC> {
        DIV_W::new(self, 0)
    }
}
#[doc = "Baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`baudr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baudr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BAUDR_SPEC;
impl crate::RegisterSpec for BAUDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baudr::R`](R) reader structure"]
impl crate::Readable for BAUDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`baudr::W`](W) writer structure"]
impl crate::Writable for BAUDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BAUDR to value 0"]
impl crate::Resettable for BAUDR_SPEC {}
