#[doc = "Register `CWSZ` reader"]
pub type R = crate::R<CWSZ_SPEC>;
#[doc = "Register `CWSZ` writer"]
pub type W = crate::W<CWSZ_SPEC>;
#[doc = "Field `CHNUM` reader - Cropping window horizontal pixel number"]
pub type CHNUM_R = crate::FieldReader<u16>;
#[doc = "Field `CHNUM` writer - Cropping window horizontal pixel number"]
pub type CHNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `CVNUM` reader - Cropping window vertical line number"]
pub type CVNUM_R = crate::FieldReader<u16>;
#[doc = "Field `CVNUM` writer - Cropping window vertical line number"]
pub type CVNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Cropping window horizontal pixel number"]
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Cropping window vertical line number"]
    #[inline(always)]
    pub fn cvnum(&self) -> CVNUM_R {
        CVNUM_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CWSZ")
            .field("cvnum", &self.cvnum())
            .field("chnum", &self.chnum())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Cropping window horizontal pixel number"]
    #[inline(always)]
    #[must_use]
    pub fn chnum(&mut self) -> CHNUM_W<CWSZ_SPEC> {
        CHNUM_W::new(self, 0)
    }
    #[doc = "Bits 16:29 - Cropping window vertical line number"]
    #[inline(always)]
    #[must_use]
    pub fn cvnum(&mut self) -> CVNUM_W<CWSZ_SPEC> {
        CVNUM_W::new(self, 16)
    }
}
#[doc = "Crop window size\n\nYou can [`read`](crate::Reg::read) this register and get [`cwsz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwsz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CWSZ_SPEC;
impl crate::RegisterSpec for CWSZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwsz::R`](R) reader structure"]
impl crate::Readable for CWSZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cwsz::W`](W) writer structure"]
impl crate::Writable for CWSZ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CWSZ to value 0"]
impl crate::Resettable for CWSZ_SPEC {
    const RESET_VALUE: u32 = 0;
}
