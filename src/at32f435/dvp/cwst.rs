#[doc = "Register `CWST` reader"]
pub type R = crate::R<CWST_SPEC>;
#[doc = "Register `CWST` writer"]
pub type W = crate::W<CWST_SPEC>;
#[doc = "Field `CHSTR` reader - Cropping window horizontal start pixel"]
pub type CHSTR_R = crate::FieldReader<u16>;
#[doc = "Field `CHSTR` writer - Cropping window horizontal start pixel"]
pub type CHSTR_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `CVSTR` reader - Cropping window vertical start line"]
pub type CVSTR_R = crate::FieldReader<u16>;
#[doc = "Field `CVSTR` writer - Cropping window vertical start line"]
pub type CVSTR_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:13 - Cropping window horizontal start pixel"]
    #[inline(always)]
    pub fn chstr(&self) -> CHSTR_R {
        CHSTR_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:28 - Cropping window vertical start line"]
    #[inline(always)]
    pub fn cvstr(&self) -> CVSTR_R {
        CVSTR_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CWST")
            .field("cvstr", &self.cvstr())
            .field("chstr", &self.chstr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Cropping window horizontal start pixel"]
    #[inline(always)]
    #[must_use]
    pub fn chstr(&mut self) -> CHSTR_W<CWST_SPEC> {
        CHSTR_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Cropping window vertical start line"]
    #[inline(always)]
    #[must_use]
    pub fn cvstr(&mut self) -> CVSTR_W<CWST_SPEC> {
        CVSTR_W::new(self, 16)
    }
}
#[doc = "Crop window start\n\nYou can [`read`](crate::Reg::read) this register and get [`cwst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CWST_SPEC;
impl crate::RegisterSpec for CWST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwst::R`](R) reader structure"]
impl crate::Readable for CWST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cwst::W`](W) writer structure"]
impl crate::Writable for CWST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CWST to value 0"]
impl crate::Resettable for CWST_SPEC {
    const RESET_VALUE: u32 = 0;
}
