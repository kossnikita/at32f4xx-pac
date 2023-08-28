#[doc = "Register `PCDTO2` reader"]
pub type R = crate::R<PCDTO2_SPEC>;
#[doc = "Register `PCDTO2` writer"]
pub type W = crate::W<PCDTO2_SPEC>;
#[doc = "Field `PCDTO2` reader - Data offset for Preempted channel 2"]
pub type PCDTO2_R = crate::FieldReader<u16>;
#[doc = "Field `PCDTO2` writer - Data offset for Preempted channel 2"]
pub type PCDTO2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 2"]
    #[inline(always)]
    pub fn pcdto2(&self) -> PCDTO2_R {
        PCDTO2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for Preempted channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn pcdto2(&mut self) -> PCDTO2_W<PCDTO2_SPEC, 0> {
        PCDTO2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Preempted channel 2 data offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCDTO2_SPEC;
impl crate::RegisterSpec for PCDTO2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcdto2::R`](R) reader structure"]
impl crate::Readable for PCDTO2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcdto2::W`](W) writer structure"]
impl crate::Writable for PCDTO2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCDTO2 to value 0"]
impl crate::Resettable for PCDTO2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
