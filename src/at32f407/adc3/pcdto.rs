#[doc = "Register `PCDTO%s` reader"]
pub type R = crate::R<PCDTO_SPEC>;
#[doc = "Register `PCDTO%s` writer"]
pub type W = crate::W<PCDTO_SPEC>;
#[doc = "Field `PCDTO` reader - Data offset for Preempted channel 1"]
pub type PCDTO_R = crate::FieldReader<u16>;
#[doc = "Field `PCDTO` writer - Data offset for Preempted channel 1"]
pub type PCDTO_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 1"]
    #[inline(always)]
    pub fn pcdto(&self) -> PCDTO_R {
        PCDTO_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn pcdto(&mut self) -> PCDTO_W<PCDTO_SPEC, 0> {
        PCDTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Data offset for Preempted channel %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCDTO_SPEC;
impl crate::RegisterSpec for PCDTO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcdto::R`](R) reader structure"]
impl crate::Readable for PCDTO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcdto::W`](W) writer structure"]
impl crate::Writable for PCDTO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCDTO%s to value 0"]
impl crate::Resettable for PCDTO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
