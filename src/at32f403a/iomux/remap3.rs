#[doc = "Register `REMAP3` reader"]
pub type R = crate::R<REMAP3_SPEC>;
#[doc = "Register `REMAP3` writer"]
pub type W = crate::W<REMAP3_SPEC>;
#[doc = "Field `TMR9_GMUX` reader - TMR9 muxing"]
pub type TMR9_GMUX_R = crate::FieldReader;
#[doc = "Field `TMR9_GMUX` writer - TMR9 muxing"]
pub type TMR9_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TMR9 muxing"]
    #[inline(always)]
    pub fn tmr9_gmux(&self) -> TMR9_GMUX_R {
        TMR9_GMUX_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP3")
            .field("tmr9_gmux", &self.tmr9_gmux())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - TMR9 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9_gmux(&mut self) -> TMR9_GMUX_W<REMAP3_SPEC> {
        TMR9_GMUX_W::new(self, 0)
    }
}
#[doc = "IO MUX remap register 3 (IOMUX_REMAP3)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP3_SPEC;
impl crate::RegisterSpec for REMAP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap3::R`](R) reader structure"]
impl crate::Readable for REMAP3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap3::W`](W) writer structure"]
impl crate::Writable for REMAP3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP3 to value 0"]
impl crate::Resettable for REMAP3_SPEC {
    const RESET_VALUE: u32 = 0;
}
