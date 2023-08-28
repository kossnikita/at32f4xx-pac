#[doc = "Register `C6DTCNT` reader"]
pub type R = crate::R<C6DTCNT_SPEC>;
#[doc = "Register `C6DTCNT` writer"]
pub type W = crate::W<C6DTCNT_SPEC>;
#[doc = "Field `CNT` reader - Number of data to transfer"]
pub type CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Number of data to transfer"]
pub type CNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<C6DTCNT_SPEC, 0> {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA channel 6 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c6dtcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c6dtcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C6DTCNT_SPEC;
impl crate::RegisterSpec for C6DTCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c6dtcnt::R`](R) reader structure"]
impl crate::Readable for C6DTCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c6dtcnt::W`](W) writer structure"]
impl crate::Writable for C6DTCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C6DTCNT to value 0"]
impl crate::Resettable for C6DTCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}