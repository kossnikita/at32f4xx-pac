#[doc = "Register `LOW_PULSE` reader"]
pub type R = crate::R<LOW_PULSE_SPEC>;
#[doc = "Register `LOW_PULSE` writer"]
pub type W = crate::W<LOW_PULSE_SPEC>;
#[doc = "Field `L_PULSE_CNT` reader - Low pulse Count"]
pub type L_PULSE_CNT_R = crate::FieldReader;
#[doc = "Field `L_PULSE_CNT` writer - Low pulse Count"]
pub type L_PULSE_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Low pulse Count"]
    #[inline(always)]
    pub fn l_pulse_cnt(&self) -> L_PULSE_CNT_R {
        L_PULSE_CNT_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOW_PULSE")
            .field("l_pulse_cnt", &self.l_pulse_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Low pulse Count"]
    #[inline(always)]
    pub fn l_pulse_cnt(&mut self) -> L_PULSE_CNT_W<'_, LOW_PULSE_SPEC> {
        L_PULSE_CNT_W::new(self, 0)
    }
}
#[doc = "LOW_PULSE\n\nYou can [`read`](crate::Reg::read) this register and get [`low_pulse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`low_pulse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOW_PULSE_SPEC;
impl crate::RegisterSpec for LOW_PULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`low_pulse::R`](R) reader structure"]
impl crate::Readable for LOW_PULSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`low_pulse::W`](W) writer structure"]
impl crate::Writable for LOW_PULSE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOW_PULSE to value 0"]
impl crate::Resettable for LOW_PULSE_SPEC {}
