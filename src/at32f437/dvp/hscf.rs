#[doc = "Register `HSCF` reader"]
pub type R = crate::R<HSCF_SPEC>;
#[doc = "Register `HSCF` writer"]
pub type W = crate::W<HSCF_SPEC>;
#[doc = "Field `HSRSF` reader - Horizontal scaling resize source factor"]
pub type HSRSF_R = crate::FieldReader<u16>;
#[doc = "Field `HSRSF` writer - Horizontal scaling resize source factor"]
pub type HSRSF_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `HSRTF` reader - Horizontal scaling resize target factor"]
pub type HSRTF_R = crate::FieldReader<u16>;
#[doc = "Field `HSRTF` writer - Horizontal scaling resize target factor"]
pub type HSRTF_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Horizontal scaling resize source factor"]
    #[inline(always)]
    pub fn hsrsf(&self) -> HSRSF_R {
        HSRSF_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Horizontal scaling resize target factor"]
    #[inline(always)]
    pub fn hsrtf(&self) -> HSRTF_R {
        HSRTF_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSCF")
            .field("hsrtf", &format_args!("{}", self.hsrtf().bits()))
            .field("hsrsf", &format_args!("{}", self.hsrsf().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HSCF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:12 - Horizontal scaling resize source factor"]
    #[inline(always)]
    #[must_use]
    pub fn hsrsf(&mut self) -> HSRSF_W<HSCF_SPEC> {
        HSRSF_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Horizontal scaling resize target factor"]
    #[inline(always)]
    #[must_use]
    pub fn hsrtf(&mut self) -> HSRTF_W<HSCF_SPEC> {
        HSRTF_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Horizontal scaling control flow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hscf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hscf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSCF_SPEC;
impl crate::RegisterSpec for HSCF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hscf::R`](R) reader structure"]
impl crate::Readable for HSCF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hscf::W`](W) writer structure"]
impl crate::Writable for HSCF_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSCF to value 0"]
impl crate::Resettable for HSCF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
