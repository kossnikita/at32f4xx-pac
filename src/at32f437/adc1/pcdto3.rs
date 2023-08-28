#[doc = "Register `PCDTO3` reader"]
pub type R = crate::R<PCDTO3_SPEC>;
#[doc = "Register `PCDTO3` writer"]
pub type W = crate::W<PCDTO3_SPEC>;
#[doc = "Field `PCDTO3` reader - Data offset for Preempted channel 3"]
pub type PCDTO3_R = crate::FieldReader<u16>;
#[doc = "Field `PCDTO3` writer - Data offset for Preempted channel 3"]
pub type PCDTO3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 3"]
    #[inline(always)]
    pub fn pcdto3(&self) -> PCDTO3_R {
        PCDTO3_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 3"]
    #[inline(always)]
    #[must_use]
    pub fn pcdto3(&mut self) -> PCDTO3_W<PCDTO3_SPEC, 0> {
        PCDTO3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Preempted channel 3 data offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCDTO3_SPEC;
impl crate::RegisterSpec for PCDTO3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcdto3::R`](R) reader structure"]
impl crate::Readable for PCDTO3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcdto3::W`](W) writer structure"]
impl crate::Writable for PCDTO3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCDTO3 to value 0"]
impl crate::Resettable for PCDTO3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
