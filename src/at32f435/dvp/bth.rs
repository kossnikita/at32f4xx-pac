#[doc = "Register `BTH` reader"]
pub type R = crate::R<BTH_SPEC>;
#[doc = "Register `BTH` writer"]
pub type W = crate::W<BTH_SPEC>;
#[doc = "Field `MIBTHD` reader - Monochrome image binarization threshold"]
pub type MIBTHD_R = crate::FieldReader;
#[doc = "Field `MIBTHD` writer - Monochrome image binarization threshold"]
pub type MIBTHD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Monochrome image binarization threshold"]
    #[inline(always)]
    pub fn mibthd(&self) -> MIBTHD_R {
        MIBTHD_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTH")
            .field("mibthd", &self.mibthd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Monochrome image binarization threshold"]
    #[inline(always)]
    #[must_use]
    pub fn mibthd(&mut self) -> MIBTHD_W<BTH_SPEC> {
        MIBTHD_W::new(self, 0)
    }
}
#[doc = "Binarization threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`bth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTH_SPEC;
impl crate::RegisterSpec for BTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bth::R`](R) reader structure"]
impl crate::Readable for BTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bth::W`](W) writer structure"]
impl crate::Writable for BTH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BTH to value 0"]
impl crate::Resettable for BTH_SPEC {
    const RESET_VALUE: u32 = 0;
}
