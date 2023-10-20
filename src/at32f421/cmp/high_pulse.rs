#[doc = "Register `HIGH_PULSE` reader"]
pub type R = crate::R<HIGH_PULSE_SPEC>;
#[doc = "Register `HIGH_PULSE` writer"]
pub type W = crate::W<HIGH_PULSE_SPEC>;
#[doc = "Field `H_PULSE_CNT` reader - High pulse Count"]
pub type H_PULSE_CNT_R = crate::FieldReader;
#[doc = "Field `H_PULSE_CNT` writer - High pulse Count"]
pub type H_PULSE_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - High pulse Count"]
    #[inline(always)]
    pub fn h_pulse_cnt(&self) -> H_PULSE_CNT_R {
        H_PULSE_CNT_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIGH_PULSE")
            .field(
                "h_pulse_cnt",
                &format_args!("{}", self.h_pulse_cnt().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<HIGH_PULSE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - High pulse Count"]
    #[inline(always)]
    #[must_use]
    pub fn h_pulse_cnt(&mut self) -> H_PULSE_CNT_W<HIGH_PULSE_SPEC> {
        H_PULSE_CNT_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "HIGH_PULSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`high_pulse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`high_pulse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIGH_PULSE_SPEC;
impl crate::RegisterSpec for HIGH_PULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`high_pulse::R`](R) reader structure"]
impl crate::Readable for HIGH_PULSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`high_pulse::W`](W) writer structure"]
impl crate::Writable for HIGH_PULSE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HIGH_PULSE to value 0"]
impl crate::Resettable for HIGH_PULSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
