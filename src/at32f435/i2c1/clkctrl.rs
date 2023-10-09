#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<CLKCTRL_SPEC>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<CLKCTRL_SPEC>;
#[doc = "Field `SCLL` reader - SCL low level"]
pub type SCLL_R = crate::FieldReader;
#[doc = "Field `SCLL` writer - SCL low level"]
pub type SCLL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SCLH` reader - SCL high level"]
pub type SCLH_R = crate::FieldReader;
#[doc = "Field `SCLH` writer - SCL high level"]
pub type SCLH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SDAD` reader - SDA output delay"]
pub type SDAD_R = crate::FieldReader;
#[doc = "Field `SDAD` writer - SDA output delay"]
pub type SDAD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SCLD` reader - SCL output delay"]
pub type SCLD_R = crate::FieldReader;
#[doc = "Field `SCLD` writer - SCL output delay"]
pub type SCLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DIVH` reader - High 4 bits of clock divider value"]
pub type DIVH_R = crate::FieldReader;
#[doc = "Field `DIVH` writer - High 4 bits of clock divider value"]
pub type DIVH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DIVL` reader - Low 4 bits of clock divider value"]
pub type DIVL_R = crate::FieldReader;
#[doc = "Field `DIVL` writer - Low 4 bits of clock divider value"]
pub type DIVL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:7 - SCL low level"]
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCL high level"]
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - SDA output delay"]
    #[inline(always)]
    pub fn sdad(&self) -> SDAD_R {
        SDAD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SCL output delay"]
    #[inline(always)]
    pub fn scld(&self) -> SCLD_R {
        SCLD_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - High 4 bits of clock divider value"]
    #[inline(always)]
    pub fn divh(&self) -> DIVH_R {
        DIVH_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Low 4 bits of clock divider value"]
    #[inline(always)]
    pub fn divl(&self) -> DIVL_R {
        DIVL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKCTRL")
            .field("scll", &format_args!("{}", self.scll().bits()))
            .field("sclh", &format_args!("{}", self.sclh().bits()))
            .field("sdad", &format_args!("{}", self.sdad().bits()))
            .field("scld", &format_args!("{}", self.scld().bits()))
            .field("divh", &format_args!("{}", self.divh().bits()))
            .field("divl", &format_args!("{}", self.divl().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CLKCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCL low level"]
    #[inline(always)]
    #[must_use]
    pub fn scll(&mut self) -> SCLL_W<CLKCTRL_SPEC, 0> {
        SCLL_W::new(self)
    }
    #[doc = "Bits 8:15 - SCL high level"]
    #[inline(always)]
    #[must_use]
    pub fn sclh(&mut self) -> SCLH_W<CLKCTRL_SPEC, 8> {
        SCLH_W::new(self)
    }
    #[doc = "Bits 16:19 - SDA output delay"]
    #[inline(always)]
    #[must_use]
    pub fn sdad(&mut self) -> SDAD_W<CLKCTRL_SPEC, 16> {
        SDAD_W::new(self)
    }
    #[doc = "Bits 20:23 - SCL output delay"]
    #[inline(always)]
    #[must_use]
    pub fn scld(&mut self) -> SCLD_W<CLKCTRL_SPEC, 20> {
        SCLD_W::new(self)
    }
    #[doc = "Bits 24:27 - High 4 bits of clock divider value"]
    #[inline(always)]
    #[must_use]
    pub fn divh(&mut self) -> DIVH_W<CLKCTRL_SPEC, 24> {
        DIVH_W::new(self)
    }
    #[doc = "Bits 28:31 - Low 4 bits of clock divider value"]
    #[inline(always)]
    #[must_use]
    pub fn divl(&mut self) -> DIVL_W<CLKCTRL_SPEC, 28> {
        DIVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock contorl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCTRL_SPEC;
impl crate::RegisterSpec for CLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for CLKCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for CLKCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for CLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
