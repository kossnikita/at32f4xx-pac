#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<CLKCTRL_SPEC>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<CLKCTRL_SPEC>;
#[doc = "Field `SCLL` reader - SCL low level"]
pub type SCLL_R = crate::FieldReader;
#[doc = "Field `SCLL` writer - SCL low level"]
pub type SCLL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLH` reader - SCL high level"]
pub type SCLH_R = crate::FieldReader;
#[doc = "Field `SCLH` writer - SCL high level"]
pub type SCLH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDAD` reader - SDA output delay"]
pub type SDAD_R = crate::FieldReader;
#[doc = "Field `SDAD` writer - SDA output delay"]
pub type SDAD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCLD` reader - SCL output delay"]
pub type SCLD_R = crate::FieldReader;
#[doc = "Field `SCLD` writer - SCL output delay"]
pub type SCLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIVH` reader - High 4 bits of clock divider value"]
pub type DIVH_R = crate::FieldReader;
#[doc = "Field `DIVH` writer - High 4 bits of clock divider value"]
pub type DIVH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIVL` reader - Low 4 bits of clock divider value"]
pub type DIVL_R = crate::FieldReader;
#[doc = "Field `DIVL` writer - Low 4 bits of clock divider value"]
pub type DIVL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
            .field("scll", &self.scll())
            .field("sclh", &self.sclh())
            .field("sdad", &self.sdad())
            .field("scld", &self.scld())
            .field("divh", &self.divh())
            .field("divl", &self.divl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - SCL low level"]
    #[inline(always)]
    #[must_use]
    pub fn scll(&mut self) -> SCLL_W<CLKCTRL_SPEC> {
        SCLL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SCL high level"]
    #[inline(always)]
    #[must_use]
    pub fn sclh(&mut self) -> SCLH_W<CLKCTRL_SPEC> {
        SCLH_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - SDA output delay"]
    #[inline(always)]
    #[must_use]
    pub fn sdad(&mut self) -> SDAD_W<CLKCTRL_SPEC> {
        SDAD_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - SCL output delay"]
    #[inline(always)]
    #[must_use]
    pub fn scld(&mut self) -> SCLD_W<CLKCTRL_SPEC> {
        SCLD_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - High 4 bits of clock divider value"]
    #[inline(always)]
    #[must_use]
    pub fn divh(&mut self) -> DIVH_W<CLKCTRL_SPEC> {
        DIVH_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Low 4 bits of clock divider value"]
    #[inline(always)]
    #[must_use]
    pub fn divl(&mut self) -> DIVL_W<CLKCTRL_SPEC> {
        DIVL_W::new(self, 28)
    }
}
#[doc = "Clock contorl register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKCTRL_SPEC;
impl crate::RegisterSpec for CLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for CLKCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for CLKCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for CLKCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
