#[doc = "Register `FCTRL` reader"]
pub type R = crate::R<FCTRL_SPEC>;
#[doc = "Register `FCTRL` writer"]
pub type W = crate::W<FCTRL_SPEC>;
#[doc = "Field `FCS` reader - Filters configure switch"]
pub type FCS_R = crate::BitReader;
#[doc = "Field `FCS` writer - Filters configure switch"]
pub type FCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Filters configure switch"]
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filters configure switch"]
    #[inline(always)]
    #[must_use]
    pub fn fcs(&mut self) -> FCS_W<FCTRL_SPEC, 0> {
        FCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCTRL_SPEC;
impl crate::RegisterSpec for FCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctrl::R`](R) reader structure"]
impl crate::Readable for FCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fctrl::W`](W) writer structure"]
impl crate::Writable for FCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCTRL to value 0"]
impl crate::Resettable for FCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
