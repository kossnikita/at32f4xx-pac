#[doc = "Register `RMP` reader"]
pub type R = crate::R<RMP_SPEC>;
#[doc = "Register `RMP` writer"]
pub type W = crate::W<RMP_SPEC>;
#[doc = "Field `TMR14_CH1_IRMP` reader - TMR14 channel 1 input remap"]
pub type TMR14_CH1_IRMP_R = crate::FieldReader;
#[doc = "Field `TMR14_CH1_IRMP` writer - TMR14 channel 1 input remap"]
pub type TMR14_CH1_IRMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 6:7 - TMR14 channel 1 input remap"]
    #[inline(always)]
    pub fn tmr14_ch1_irmp(&self) -> TMR14_CH1_IRMP_R {
        TMR14_CH1_IRMP_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMP")
            .field("tmr14_ch1_irmp", &self.tmr14_ch1_irmp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 6:7 - TMR14 channel 1 input remap"]
    #[inline(always)]
    pub fn tmr14_ch1_irmp(&mut self) -> TMR14_CH1_IRMP_W<'_, RMP_SPEC> {
        TMR14_CH1_IRMP_W::new(self, 6)
    }
}
#[doc = "TMR14 channel 1 input remap\n\nYou can [`read`](crate::Reg::read) this register and get [`rmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMP_SPEC;
impl crate::RegisterSpec for RMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmp::R`](R) reader structure"]
impl crate::Readable for RMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rmp::W`](W) writer structure"]
impl crate::Writable for RMP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RMP to value 0"]
impl crate::Resettable for RMP_SPEC {}
