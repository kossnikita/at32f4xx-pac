#[doc = "Register `FRF` reader"]
pub type R = crate::R<FRF_SPEC>;
#[doc = "Register `FRF` writer"]
pub type W = crate::W<FRF_SPEC>;
#[doc = "Field `EFRCSF` reader - Enhanced frame rate contorl source factor"]
pub type EFRCSF_R = crate::FieldReader;
#[doc = "Field `EFRCSF` writer - Enhanced frame rate contorl source factor"]
pub type EFRCSF_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EFRCTF` reader - Enhanced frame rate control target factor"]
pub type EFRCTF_R = crate::FieldReader;
#[doc = "Field `EFRCTF` writer - Enhanced frame rate control target factor"]
pub type EFRCTF_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Enhanced frame rate contorl source factor"]
    #[inline(always)]
    pub fn efrcsf(&self) -> EFRCSF_R {
        EFRCSF_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Enhanced frame rate control target factor"]
    #[inline(always)]
    pub fn efrctf(&self) -> EFRCTF_R {
        EFRCTF_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRF")
            .field("efrctf", &self.efrctf())
            .field("efrcsf", &self.efrcsf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Enhanced frame rate contorl source factor"]
    #[inline(always)]
    pub fn efrcsf(&mut self) -> EFRCSF_W<'_, FRF_SPEC> {
        EFRCSF_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Enhanced frame rate control target factor"]
    #[inline(always)]
    pub fn efrctf(&mut self) -> EFRCTF_W<'_, FRF_SPEC> {
        EFRCTF_W::new(self, 8)
    }
}
#[doc = "Frame rate flow\n\nYou can [`read`](crate::Reg::read) this register and get [`frf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRF_SPEC;
impl crate::RegisterSpec for FRF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frf::R`](R) reader structure"]
impl crate::Readable for FRF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frf::W`](W) writer structure"]
impl crate::Writable for FRF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRF to value 0"]
impl crate::Resettable for FRF_SPEC {}
