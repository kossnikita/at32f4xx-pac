#[doc = "Register `REMAP3` reader"]
pub type R = crate::R<REMAP3_SPEC>;
#[doc = "Register `REMAP3` writer"]
pub type W = crate::W<REMAP3_SPEC>;
#[doc = "Field `TMR9_GMUX` reader - TMR9 muxing"]
pub type TMR9_GMUX_R = crate::FieldReader;
#[doc = "Field `TMR9_GMUX` writer - TMR9 muxing"]
pub type TMR9_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMR10_GMUX` reader - TMR10 muxing"]
pub type TMR10_GMUX_R = crate::FieldReader;
#[doc = "Field `TMR10_GMUX` writer - TMR10 muxing"]
pub type TMR10_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMR11_GMUX` reader - TMR11 muxing"]
pub type TMR11_GMUX_R = crate::FieldReader;
#[doc = "Field `TMR11_GMUX` writer - TMR11 muxing"]
pub type TMR11_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TMR9 muxing"]
    #[inline(always)]
    pub fn tmr9_gmux(&self) -> TMR9_GMUX_R {
        TMR9_GMUX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - TMR10 muxing"]
    #[inline(always)]
    pub fn tmr10_gmux(&self) -> TMR10_GMUX_R {
        TMR10_GMUX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TMR11 muxing"]
    #[inline(always)]
    pub fn tmr11_gmux(&self) -> TMR11_GMUX_R {
        TMR11_GMUX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP3")
            .field("tmr9_gmux", &format_args!("{}", self.tmr9_gmux().bits()))
            .field("tmr10_gmux", &format_args!("{}", self.tmr10_gmux().bits()))
            .field("tmr11_gmux", &format_args!("{}", self.tmr11_gmux().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<REMAP3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - TMR9 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr9_gmux(&mut self) -> TMR9_GMUX_W<REMAP3_SPEC> {
        TMR9_GMUX_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - TMR10 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr10_gmux(&mut self) -> TMR10_GMUX_W<REMAP3_SPEC> {
        TMR10_GMUX_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - TMR11 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr11_gmux(&mut self) -> TMR11_GMUX_W<REMAP3_SPEC> {
        TMR11_GMUX_W::new(self, 8)
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
#[doc = "IO MUX remap register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP3_SPEC;
impl crate::RegisterSpec for REMAP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap3::R`](R) reader structure"]
impl crate::Readable for REMAP3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap3::W`](W) writer structure"]
impl crate::Writable for REMAP3_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REMAP3 to value 0"]
impl crate::Resettable for REMAP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
