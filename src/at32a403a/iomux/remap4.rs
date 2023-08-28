#[doc = "Register `REMAP4` reader"]
pub type R = crate::R<REMAP4_SPEC>;
#[doc = "Register `REMAP4` writer"]
pub type W = crate::W<REMAP4_SPEC>;
#[doc = "Field `TMR1_GMUX` reader - TMR1 muxing"]
pub type TMR1_GMUX_R = crate::FieldReader;
#[doc = "Field `TMR1_GMUX` writer - TMR1 muxing"]
pub type TMR1_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TMR2_GMUX` reader - TMR2 muxing"]
pub type TMR2_GMUX_R = crate::FieldReader;
#[doc = "Field `TMR2_GMUX` writer - TMR2 muxing"]
pub type TMR2_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TMR2ITR1_GMUX` reader - TMR2 internal trigger 1 muxing"]
pub type TMR2ITR1_GMUX_R = crate::FieldReader;
#[doc = "Field `TMR2ITR1_GMUX` writer - TMR2 internal trigger 1 muxing"]
pub type TMR2ITR1_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TMR3_GMUX` reader - TMR3 muxing"]
pub type TMR3_GMUX_R = crate::FieldReader;
#[doc = "Field `TMR3_GMUX` writer - TMR3 muxing"]
pub type TMR3_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TMR4_GMUX` reader - TMR4 muxing"]
pub type TMR4_GMUX_R = crate::FieldReader;
#[doc = "Field `TMR4_GMUX` writer - TMR4 muxing"]
pub type TMR4_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TMR5CH4_GMUX` reader - TMR5 channel4 internal muxing"]
pub type TMR5CH4_GMUX_R = crate::BitReader;
#[doc = "Field `TMR5CH4_GMUX` writer - TMR5 channel4 internal muxing"]
pub type TMR5CH4_GMUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - TMR1 muxing"]
    #[inline(always)]
    pub fn tmr1_gmux(&self) -> TMR1_GMUX_R {
        TMR1_GMUX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - TMR2 muxing"]
    #[inline(always)]
    pub fn tmr2_gmux(&self) -> TMR2_GMUX_R {
        TMR2_GMUX_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TMR2 internal trigger 1 muxing"]
    #[inline(always)]
    pub fn tmr2itr1_gmux(&self) -> TMR2ITR1_GMUX_R {
        TMR2ITR1_GMUX_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - TMR3 muxing"]
    #[inline(always)]
    pub fn tmr3_gmux(&self) -> TMR3_GMUX_R {
        TMR3_GMUX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - TMR4 muxing"]
    #[inline(always)]
    pub fn tmr4_gmux(&self) -> TMR4_GMUX_R {
        TMR4_GMUX_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - TMR5 channel4 internal muxing"]
    #[inline(always)]
    pub fn tmr5ch4_gmux(&self) -> TMR5CH4_GMUX_R {
        TMR5CH4_GMUX_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TMR1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_gmux(&mut self) -> TMR1_GMUX_W<REMAP4_SPEC, 0> {
        TMR1_GMUX_W::new(self)
    }
    #[doc = "Bits 4:5 - TMR2 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2_gmux(&mut self) -> TMR2_GMUX_W<REMAP4_SPEC, 4> {
        TMR2_GMUX_W::new(self)
    }
    #[doc = "Bits 6:7 - TMR2 internal trigger 1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2itr1_gmux(&mut self) -> TMR2ITR1_GMUX_W<REMAP4_SPEC, 6> {
        TMR2ITR1_GMUX_W::new(self)
    }
    #[doc = "Bits 8:11 - TMR3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3_gmux(&mut self) -> TMR3_GMUX_W<REMAP4_SPEC, 8> {
        TMR3_GMUX_W::new(self)
    }
    #[doc = "Bits 12:15 - TMR4 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr4_gmux(&mut self) -> TMR4_GMUX_W<REMAP4_SPEC, 12> {
        TMR4_GMUX_W::new(self)
    }
    #[doc = "Bit 19 - TMR5 channel4 internal muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5ch4_gmux(&mut self) -> TMR5CH4_GMUX_W<REMAP4_SPEC, 19> {
        TMR5CH4_GMUX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "IO MUX remap register 4 (IOMUX_REMAP4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP4_SPEC;
impl crate::RegisterSpec for REMAP4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap4::R`](R) reader structure"]
impl crate::Readable for REMAP4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap4::W`](W) writer structure"]
impl crate::Writable for REMAP4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REMAP4 to value 0"]
impl crate::Resettable for REMAP4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}