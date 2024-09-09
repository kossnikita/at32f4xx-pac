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
            .field("tmr2_is1_irmp", &self.tmr2_is1_irmp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 10:11 - TMR2 internal selection 3 remap"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2_is1_irmp(&mut self) -> TMR2_IS1_IRMP_W<RMP_SPEC> {
        TMR2_IS1_IRMP_W::new(self, 10)
    }
}
#[doc = "TMR2 input remap register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMP_SPEC;
impl crate::RegisterSpec for RMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmp::R`](R) reader structure"]
impl crate::Readable for RMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rmp::W`](W) writer structure"]
impl crate::Writable for RMP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RMP to value 0"]
impl crate::Resettable for RMP_SPEC {
    const RESET_VALUE: u32 = 0;
}
