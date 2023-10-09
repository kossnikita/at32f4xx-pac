#[doc = "Register `S6_STRIDE` reader"]
pub type R = crate::R<S6_STRIDE_SPEC>;
#[doc = "Register `S6_STRIDE` writer"]
pub type W = crate::W<S6_STRIDE_SPEC>;
#[doc = "Field `SRCSTD` reader - Source stride"]
pub type SRCSTD_R = crate::FieldReader<u16>;
#[doc = "Field `SRCSTD` writer - Source stride"]
pub type SRCSTD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `DSTSTD` reader - Destination stride"]
pub type DSTSTD_R = crate::FieldReader<u16>;
#[doc = "Field `DSTSTD` writer - Destination stride"]
pub type DSTSTD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Source stride"]
    #[inline(always)]
    pub fn srcstd(&self) -> SRCSTD_R {
        SRCSTD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Destination stride"]
    #[inline(always)]
    pub fn dststd(&self) -> DSTSTD_R {
        DSTSTD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S6_STRIDE")
            .field("srcstd", &format_args!("{}", self.srcstd().bits()))
            .field("dststd", &format_args!("{}", self.dststd().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<S6_STRIDE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Source stride"]
    #[inline(always)]
    #[must_use]
    pub fn srcstd(&mut self) -> SRCSTD_W<S6_STRIDE_SPEC, 0> {
        SRCSTD_W::new(self)
    }
    #[doc = "Bits 16:31 - Destination stride"]
    #[inline(always)]
    #[must_use]
    pub fn dststd(&mut self) -> DSTSTD_W<S6_STRIDE_SPEC, 16> {
        DSTSTD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Stream 6 2D Transfer Stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6_stride::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6_stride::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S6_STRIDE_SPEC;
impl crate::RegisterSpec for S6_STRIDE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s6_stride::R`](R) reader structure"]
impl crate::Readable for S6_STRIDE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s6_stride::W`](W) writer structure"]
impl crate::Writable for S6_STRIDE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S6_STRIDE to value 0"]
impl crate::Resettable for S6_STRIDE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
