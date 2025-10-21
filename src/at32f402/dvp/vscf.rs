#[doc = "Register `VSCF` reader"]
pub type R = crate::R<VSCF_SPEC>;
#[doc = "Register `VSCF` writer"]
pub type W = crate::W<VSCF_SPEC>;
#[doc = "Field `VSRSF` reader - Vertical scaling resize source factor"]
pub type VSRSF_R = crate::FieldReader<u16>;
#[doc = "Field `VSRSF` writer - Vertical scaling resize source factor"]
pub type VSRSF_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `VSRTF` reader - Vertical scaling resize target factor"]
pub type VSRTF_R = crate::FieldReader<u16>;
#[doc = "Field `VSRTF` writer - Vertical scaling resize target factor"]
pub type VSRTF_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Vertical scaling resize source factor"]
    #[inline(always)]
    pub fn vsrsf(&self) -> VSRSF_R {
        VSRSF_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Vertical scaling resize target factor"]
    #[inline(always)]
    pub fn vsrtf(&self) -> VSRTF_R {
        VSRTF_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VSCF")
            .field("vsrtf", &self.vsrtf())
            .field("vsrsf", &self.vsrsf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:12 - Vertical scaling resize source factor"]
    #[inline(always)]
    pub fn vsrsf(&mut self) -> VSRSF_W<'_, VSCF_SPEC> {
        VSRSF_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Vertical scaling resize target factor"]
    #[inline(always)]
    pub fn vsrtf(&mut self) -> VSRTF_W<'_, VSCF_SPEC> {
        VSRTF_W::new(self, 16)
    }
}
#[doc = "Vertical scaling control flow\n\nYou can [`read`](crate::Reg::read) this register and get [`vscf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vscf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VSCF_SPEC;
impl crate::RegisterSpec for VSCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vscf::R`](R) reader structure"]
impl crate::Readable for VSCF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vscf::W`](W) writer structure"]
impl crate::Writable for VSCF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VSCF to value 0"]
impl crate::Resettable for VSCF_SPEC {}
