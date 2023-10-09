#[doc = "Register `DT%s` reader"]
pub type R = crate::R<DT_SPEC>;
#[doc = "Register `DT%s` writer"]
pub type W = crate::W<DT_SPEC>;
#[doc = "Field `DT` reader - BPR data"]
pub type DT_R = crate::FieldReader<u16>;
#[doc = "Field `DT` writer - BPR data"]
pub type DT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT")
            .field("dt", &format_args!("{}", self.dt().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<DT_SPEC, 0> {
        DT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Battery powered domain data register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DT_SPEC;
impl crate::RegisterSpec for DT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt::R`](R) reader structure"]
impl crate::Readable for DT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dt::W`](W) writer structure"]
impl crate::Writable for DT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DT%s to value 0"]
impl crate::Resettable for DT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
