#[doc = "Register `TMR2_RMP` reader"]
pub type R = crate::R<TMR2_RMP_SPEC>;
#[doc = "Register `TMR2_RMP` writer"]
pub type W = crate::W<TMR2_RMP_SPEC>;
#[doc = "Field `TMR2_CH1_IRMP` reader - TMR2 channel 1 input remap"]
pub type TMR2_CH1_IRMP_R = crate::FieldReader;
#[doc = "Field `TMR2_CH1_IRMP` writer - TMR2 channel 1 input remap"]
pub type TMR2_CH1_IRMP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 10:11 - TMR2 channel 1 input remap"]
    #[inline(always)]
    pub fn tmr2_ch1_irmp(&self) -> TMR2_CH1_IRMP_R {
        TMR2_CH1_IRMP_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMR2_RMP")
            .field(
                "tmr2_ch1_irmp",
                &format_args!("{}", self.tmr2_ch1_irmp().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TMR2_RMP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 10:11 - TMR2 channel 1 input remap"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2_ch1_irmp(&mut self) -> TMR2_CH1_IRMP_W<TMR2_RMP_SPEC, 10> {
        TMR2_CH1_IRMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TMR2 channel input remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmr2_rmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmr2_rmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMR2_RMP_SPEC;
impl crate::RegisterSpec for TMR2_RMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr2_rmp::R`](R) reader structure"]
impl crate::Readable for TMR2_RMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmr2_rmp::W`](W) writer structure"]
impl crate::Writable for TMR2_RMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMR2_RMP to value 0"]
impl crate::Resettable for TMR2_RMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
