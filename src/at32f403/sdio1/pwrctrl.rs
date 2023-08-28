#[doc = "Register `PWRCTRL` reader"]
pub type R = crate::R<PWRCTRL_SPEC>;
#[doc = "Register `PWRCTRL` writer"]
pub type W = crate::W<PWRCTRL_SPEC>;
#[doc = "Field `PS` reader - Power switch"]
pub type PS_R = crate::FieldReader;
#[doc = "Field `PS` writer - Power switch"]
pub type PS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Power switch"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power switch"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<PWRCTRL_SPEC, 0> {
        PS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Bits 1:0 = PWRCTRL: Power supply control bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWRCTRL_SPEC;
impl crate::RegisterSpec for PWRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrctrl::R`](R) reader structure"]
impl crate::Readable for PWRCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwrctrl::W`](W) writer structure"]
impl crate::Writable for PWRCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCTRL to value 0"]
impl crate::Resettable for PWRCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}