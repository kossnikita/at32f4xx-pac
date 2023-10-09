#[doc = "Register `VSCF` reader"]
pub type R = crate::R<VSCF_SPEC>;
#[doc = "Register `VSCF` writer"]
pub type W = crate::W<VSCF_SPEC>;
#[doc = "Field `VSRSF` reader - Vertical scaling resize source factor"]
pub type VSRSF_R = crate::FieldReader<u16>;
#[doc = "Field `VSRSF` writer - Vertical scaling resize source factor"]
pub type VSRSF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
#[doc = "Field `VSRTF` reader - Vertical scaling resize target factor"]
pub type VSRTF_R = crate::FieldReader<u16>;
#[doc = "Field `VSRTF` writer - Vertical scaling resize target factor"]
pub type VSRTF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 13, O, u16>;
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
            .field("vsrtf", &format_args!("{}", self.vsrtf().bits()))
            .field("vsrsf", &format_args!("{}", self.vsrsf().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<VSCF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:12 - Vertical scaling resize source factor"]
    #[inline(always)]
    #[must_use]
    pub fn vsrsf(&mut self) -> VSRSF_W<VSCF_SPEC, 0> {
        VSRSF_W::new(self)
    }
    #[doc = "Bits 16:28 - Vertical scaling resize target factor"]
    #[inline(always)]
    #[must_use]
    pub fn vsrtf(&mut self) -> VSRTF_W<VSCF_SPEC, 16> {
        VSRTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Vertical scaling control flow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vscf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vscf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VSCF_SPEC;
impl crate::RegisterSpec for VSCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vscf::R`](R) reader structure"]
impl crate::Readable for VSCF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vscf::W`](W) writer structure"]
impl crate::Writable for VSCF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VSCF to value 0"]
impl crate::Resettable for VSCF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
