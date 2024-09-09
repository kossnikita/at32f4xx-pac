#[doc = "Register `BUF` reader"]
pub type R = crate::R<BUF_SPEC>;
#[doc = "Register `BUF` writer"]
pub type W = crate::W<BUF_SPEC>;
#[doc = "Field `DT` reader - Buffer data"]
pub type DT_R = crate::FieldReader<u32>;
#[doc = "Field `DT` writer - Buffer data"]
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer data"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUF").field("dt", &self.dt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer data"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<BUF_SPEC> {
        DT_W::new(self, 0)
    }
}
#[doc = "bits 31:0 = Buffer Data: Receive and transmit buffer data\n\nYou can [`read`](crate::Reg::read) this register and get [`buf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUF_SPEC;
impl crate::RegisterSpec for BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf::R`](R) reader structure"]
impl crate::Readable for BUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`buf::W`](W) writer structure"]
impl crate::Writable for BUF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUF to value 0"]
impl crate::Resettable for BUF_SPEC {
    const RESET_VALUE: u32 = 0;
}
