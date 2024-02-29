#[doc = "Register `REMAP8` reader"]
pub type R = crate::R<REMAP8_SPEC>;
#[doc = "Register `REMAP8` writer"]
pub type W = crate::W<REMAP8_SPEC>;
#[doc = "Field `TMR1_BK1_CMP_GMUX` reader - TMR1 BK1 CMP muxing"]
pub type TMR1_BK1_CMP_GMUX_R = crate::FieldReader;
#[doc = "Field `TMR1_BK1_CMP_GMUX` writer - TMR1 BK1 CMP muxing"]
pub type TMR1_BK1_CMP_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR1_CH1_CMP_GMUX` reader - TMR1 CH1 CMP muxing"]
pub type TMR1_CH1_CMP_GMUX_R = crate::FieldReader;
#[doc = "Field `TMR1_CH1_CMP_GMUX` writer - TMR1 CH1 CMP muxing"]
pub type TMR1_CH1_CMP_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR2_CH4_CMP_GMUX` reader - TMR2 CH4 CMP muxing"]
pub type TMR2_CH4_CMP_GMUX_R = crate::FieldReader;
#[doc = "Field `TMR2_CH4_CMP_GMUX` writer - TMR2 CH4 CMP muxing"]
pub type TMR2_CH4_CMP_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR3_CH1_CMP_GMUX` reader - TMR3 CH1 CMP muxing"]
pub type TMR3_CH1_CMP_GMUX_R = crate::FieldReader;
#[doc = "Field `TMR3_CH1_CMP_GMUX` writer - TMR3 CH1 CMP muxing"]
pub type TMR3_CH1_CMP_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - TMR1 BK1 CMP muxing"]
    #[inline(always)]
    pub fn tmr1_bk1_cmp_gmux(&self) -> TMR1_BK1_CMP_GMUX_R {
        TMR1_BK1_CMP_GMUX_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - TMR1 CH1 CMP muxing"]
    #[inline(always)]
    pub fn tmr1_ch1_cmp_gmux(&self) -> TMR1_CH1_CMP_GMUX_R {
        TMR1_CH1_CMP_GMUX_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - TMR2 CH4 CMP muxing"]
    #[inline(always)]
    pub fn tmr2_ch4_cmp_gmux(&self) -> TMR2_CH4_CMP_GMUX_R {
        TMR2_CH4_CMP_GMUX_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TMR3 CH1 CMP muxing"]
    #[inline(always)]
    pub fn tmr3_ch1_cmp_gmux(&self) -> TMR3_CH1_CMP_GMUX_R {
        TMR3_CH1_CMP_GMUX_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP8")
            .field(
                "tmr3_ch1_cmp_gmux",
                &format_args!("{}", self.tmr3_ch1_cmp_gmux().bits()),
            )
            .field(
                "tmr2_ch4_cmp_gmux",
                &format_args!("{}", self.tmr2_ch4_cmp_gmux().bits()),
            )
            .field(
                "tmr1_ch1_cmp_gmux",
                &format_args!("{}", self.tmr1_ch1_cmp_gmux().bits()),
            )
            .field(
                "tmr1_bk1_cmp_gmux",
                &format_args!("{}", self.tmr1_bk1_cmp_gmux().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<REMAP8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - TMR1 BK1 CMP muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_bk1_cmp_gmux(&mut self) -> TMR1_BK1_CMP_GMUX_W<REMAP8_SPEC> {
        TMR1_BK1_CMP_GMUX_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - TMR1 CH1 CMP muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_ch1_cmp_gmux(&mut self) -> TMR1_CH1_CMP_GMUX_W<REMAP8_SPEC> {
        TMR1_CH1_CMP_GMUX_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - TMR2 CH4 CMP muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2_ch4_cmp_gmux(&mut self) -> TMR2_CH4_CMP_GMUX_W<REMAP8_SPEC> {
        TMR2_CH4_CMP_GMUX_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - TMR3 CH1 CMP muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3_ch1_cmp_gmux(&mut self) -> TMR3_CH1_CMP_GMUX_W<REMAP8_SPEC> {
        TMR3_CH1_CMP_GMUX_W::new(self, 6)
    }
}
#[doc = "IO MUX remap register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP8_SPEC;
impl crate::RegisterSpec for REMAP8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap8::R`](R) reader structure"]
impl crate::Readable for REMAP8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap8::W`](W) writer structure"]
impl crate::Writable for REMAP8_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP8 to value 0"]
impl crate::Resettable for REMAP8_SPEC {
    const RESET_VALUE: u32 = 0;
}
