#[doc = "Register `PCDTO4` reader"]
pub type R = crate::R<PCDTO4_SPEC>;
#[doc = "Register `PCDTO4` writer"]
pub type W = crate::W<PCDTO4_SPEC>;
#[doc = "Field `PCDTO4` reader - Data offset for Preempted channel 4"]
pub type PCDTO4_R = crate::FieldReader<u16>;
#[doc = "Field `PCDTO4` writer - Data offset for Preempted channel 4"]
pub type PCDTO4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 4"]
    #[inline(always)]
    pub fn pcdto4(&self) -> PCDTO4_R {
        PCDTO4_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 4"]
    #[inline(always)]
    #[must_use]
    pub fn pcdto4(&mut self) -> PCDTO4_W<PCDTO4_SPEC, 0> {
        PCDTO4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Preempted channel 4 data offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCDTO4_SPEC;
impl crate::RegisterSpec for PCDTO4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcdto4::R`](R) reader structure"]
impl crate::Readable for PCDTO4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcdto4::W`](W) writer structure"]
impl crate::Writable for PCDTO4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCDTO4 to value 0"]
impl crate::Resettable for PCDTO4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}