#[doc = "Register `ACTR` reader"]
pub type R = crate::R<ACTR_SPEC>;
#[doc = "Register `ACTR` writer"]
pub type W = crate::W<ACTR_SPEC>;
#[doc = "Field `CSDLY` reader - CS delay"]
pub type CSDLY_R = crate::FieldReader;
#[doc = "Field `CSDLY` writer - CS delay"]
pub type CSDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - CS delay"]
    #[inline(always)]
    pub fn csdly(&self) -> CSDLY_R {
        CSDLY_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACTR")
            .field("csdly", &self.csdly())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - CS delay"]
    #[inline(always)]
    pub fn csdly(&mut self) -> CSDLY_W<'_, ACTR_SPEC> {
        CSDLY_W::new(self, 0)
    }
}
#[doc = "AC timing control register\n\nYou can [`read`](crate::Reg::read) this register and get [`actr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACTR_SPEC;
impl crate::RegisterSpec for ACTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actr::R`](R) reader structure"]
impl crate::Readable for ACTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`actr::W`](W) writer structure"]
impl crate::Writable for ACTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACTR to value 0x0f"]
impl crate::Resettable for ACTR_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
