#[doc = "Register `RMP` reader"]
pub type R = crate::R<RMP_SPEC>;
#[doc = "Register `RMP` writer"]
pub type W = crate::W<RMP_SPEC>;
#[doc = "Field `TMR2_IS1_IRMP` reader - TMR2 internal selection 3 remap"]
pub type TMR2_IS1_IRMP_R = crate::FieldReader;
#[doc = "Field `TMR2_IS1_IRMP` writer - TMR2 internal selection 3 remap"]
pub type TMR2_IS1_IRMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 10:11 - TMR2 internal selection 3 remap"]
    #[inline(always)]
    pub fn tmr2_is1_irmp(&self) -> TMR2_IS1_IRMP_R {
        TMR2_IS1_IRMP_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMP")
            .field(
                "tmr2_is1_irmp",
                &format_args!("{}", self.tmr2_is1_irmp().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<RMP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 10:11 - TMR2 internal selection 3 remap"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2_is1_irmp(&mut self) -> TMR2_IS1_IRMP_W<RMP_SPEC> {
        TMR2_IS1_IRMP_W::new(self, 10)
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
#[doc = "TMR2 input remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMP_SPEC;
impl crate::RegisterSpec for RMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmp::R`](R) reader structure"]
impl crate::Readable for RMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rmp::W`](W) writer structure"]
impl crate::Writable for RMP_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RMP to value 0"]
impl crate::Resettable for RMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
