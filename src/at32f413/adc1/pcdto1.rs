#[doc = "Register `PCDTO1` reader"]
pub type R = crate::R<PCDTO1_SPEC>;
#[doc = "Register `PCDTO1` writer"]
pub type W = crate::W<PCDTO1_SPEC>;
#[doc = "Field `PCDTO1` reader - Data offset for Preempted channel 1"]
pub type PCDTO1_R = crate::FieldReader<u16>;
#[doc = "Field `PCDTO1` writer - Data offset for Preempted channel 1"]
pub type PCDTO1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 1"]
    #[inline(always)]
    pub fn pcdto1(&self) -> PCDTO1_R {
        PCDTO1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn pcdto1(&mut self) -> PCDTO1_W<PCDTO1_SPEC, 0> {
        PCDTO1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Preempted channel 1 data offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCDTO1_SPEC;
impl crate::RegisterSpec for PCDTO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcdto1::R`](R) reader structure"]
impl crate::Readable for PCDTO1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcdto1::W`](W) writer structure"]
impl crate::Writable for PCDTO1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCDTO1 to value 0"]
impl crate::Resettable for PCDTO1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
