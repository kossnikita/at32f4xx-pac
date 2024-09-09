#[doc = "Register `S2DTCNT` reader"]
pub type R = crate::R<S2DTCNT_SPEC>;
#[doc = "Register `S2DTCNT` writer"]
pub type W = crate::W<S2DTCNT_SPEC>;
#[doc = "Field `CNT` reader - Number of data items to transfer"]
pub type CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Number of data items to transfer"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S2DTCNT").field("cnt", &self.cnt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<S2DTCNT_SPEC> {
        CNT_W::new(self, 0)
    }
}
#[doc = "stream 2 number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`s2dtcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2dtcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2DTCNT_SPEC;
impl crate::RegisterSpec for S2DTCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s2dtcnt::R`](R) reader structure"]
impl crate::Readable for S2DTCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s2dtcnt::W`](W) writer structure"]
impl crate::Writable for S2DTCNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S2DTCNT to value 0"]
impl crate::Resettable for S2DTCNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
