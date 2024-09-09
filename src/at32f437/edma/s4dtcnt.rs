#[doc = "Register `S4DTCNT` reader"]
pub type R = crate::R<S4DTCNT_SPEC>;
#[doc = "Register `S4DTCNT` writer"]
pub type W = crate::W<S4DTCNT_SPEC>;
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
        f.debug_struct("S4DTCNT").field("cnt", &self.cnt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<S4DTCNT_SPEC> {
        CNT_W::new(self, 0)
    }
}
#[doc = "stream 4 number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`s4dtcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s4dtcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S4DTCNT_SPEC;
impl crate::RegisterSpec for S4DTCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s4dtcnt::R`](R) reader structure"]
impl crate::Readable for S4DTCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s4dtcnt::W`](W) writer structure"]
impl crate::Writable for S4DTCNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S4DTCNT to value 0"]
impl crate::Resettable for S4DTCNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
