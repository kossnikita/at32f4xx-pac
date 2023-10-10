#[doc = "Register `DDTH12L` reader"]
pub type R = crate::R<DDTH12L_SPEC>;
#[doc = "Register `DDTH12L` writer"]
pub type W = crate::W<DDTH12L_SPEC>;
#[doc = "Field `DD1DT12L` reader - DAC1 12-bit left-aligned data"]
pub type DD1DT12L_R = crate::FieldReader<u16>;
#[doc = "Field `DD1DT12L` writer - DAC1 12-bit left-aligned data"]
pub type DD1DT12L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `DD2DT12L` reader - DAC2 12-bit right-aligned data"]
pub type DD2DT12L_R = crate::FieldReader<u16>;
#[doc = "Field `DD2DT12L` writer - DAC2 12-bit right-aligned data"]
pub type DD2DT12L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 4:15 - DAC1 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dd1dt12l(&self) -> DD1DT12L_R {
        DD1DT12L_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - DAC2 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dd2dt12l(&self) -> DD2DT12L_R {
        DD2DT12L_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDTH12L")
            .field("dd1dt12l", &format_args!("{}", self.dd1dt12l().bits()))
            .field("dd2dt12l", &format_args!("{}", self.dd2dt12l().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DDTH12L_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC1 12-bit left-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dd1dt12l(&mut self) -> DD1DT12L_W<DDTH12L_SPEC, 4> {
        DD1DT12L_W::new(self)
    }
    #[doc = "Bits 20:31 - DAC2 12-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dd2dt12l(&mut self) -> DD2DT12L_W<DDTH12L_SPEC, 20> {
        DD2DT12L_W::new(self)
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
#[doc = "DUAL DAC 12-bit left aligned data holding register (DAC_DDTH12L), Bits 19:16 Reserved, Bits 3:0 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddth12l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddth12l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDTH12L_SPEC;
impl crate::RegisterSpec for DDTH12L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddth12l::R`](R) reader structure"]
impl crate::Readable for DDTH12L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddth12l::W`](W) writer structure"]
impl crate::Writable for DDTH12L_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDTH12L to value 0"]
impl crate::Resettable for DDTH12L_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
