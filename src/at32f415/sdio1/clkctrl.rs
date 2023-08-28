#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<CLKCTRL_SPEC>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<CLKCTRL_SPEC>;
#[doc = "Field `CLKPSC` reader - Clock divide factor"]
pub type CLKPSC_R = crate::FieldReader;
#[doc = "Field `CLKPSC` writer - Clock divide factor"]
pub type CLKPSC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CLKEN` reader - Clock enable bit"]
pub type CLKEN_R = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock enable bit"]
pub type CLKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWRSVG` reader - Power saving configuration bit"]
pub type PWRSVG_R = crate::BitReader;
#[doc = "Field `PWRSVG` writer - Power saving configuration bit"]
pub type PWRSVG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BYPS` reader - Clock divider bypass enable bit"]
pub type BYPS_R = crate::BitReader;
#[doc = "Field `BYPS` writer - Clock divider bypass enable bit"]
pub type BYPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUSWIDTH` reader - Wide bus mode enable bit"]
pub type BUSWIDTH_R = crate::FieldReader;
#[doc = "Field `BUSWIDTH` writer - Wide bus mode enable bit"]
pub type BUSWIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CLKEDG` reader - SDIO_CK dephasing selection bit"]
pub type CLKEDG_R = crate::BitReader;
#[doc = "Field `CLKEDG` writer - SDIO_CK dephasing selection bit"]
pub type CLKEDG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLWCTRLEN` reader - HW Flow Control enable"]
pub type FLWCTRLEN_R = crate::BitReader;
#[doc = "Field `FLWCTRLEN` writer - HW Flow Control enable"]
pub type FLWCTRLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLKPSC98` reader - Clock divide factor bit9 and bit8"]
pub type CLKPSC98_R = crate::FieldReader;
#[doc = "Field `CLKPSC98` writer - Clock divide factor bit9 and bit8"]
pub type CLKPSC98_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:7 - Clock divide factor"]
    #[inline(always)]
    pub fn clkpsc(&self) -> CLKPSC_R {
        CLKPSC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Clock enable bit"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Power saving configuration bit"]
    #[inline(always)]
    pub fn pwrsvg(&self) -> PWRSVG_R {
        PWRSVG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    pub fn byps(&self) -> BYPS_R {
        BYPS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Wide bus mode enable bit"]
    #[inline(always)]
    pub fn buswidth(&self) -> BUSWIDTH_R {
        BUSWIDTH_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - SDIO_CK dephasing selection bit"]
    #[inline(always)]
    pub fn clkedg(&self) -> CLKEDG_R {
        CLKEDG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HW Flow Control enable"]
    #[inline(always)]
    pub fn flwctrlen(&self) -> FLWCTRLEN_R {
        FLWCTRLEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - Clock divide factor bit9 and bit8"]
    #[inline(always)]
    pub fn clkpsc98(&self) -> CLKPSC98_R {
        CLKPSC98_R::new(((self.bits >> 15) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divide factor"]
    #[inline(always)]
    #[must_use]
    pub fn clkpsc(&mut self) -> CLKPSC_W<CLKCTRL_SPEC, 0> {
        CLKPSC_W::new(self)
    }
    #[doc = "Bit 8 - Clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<CLKCTRL_SPEC, 8> {
        CLKEN_W::new(self)
    }
    #[doc = "Bit 9 - Power saving configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsvg(&mut self) -> PWRSVG_W<CLKCTRL_SPEC, 9> {
        PWRSVG_W::new(self)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn byps(&mut self) -> BYPS_W<CLKCTRL_SPEC, 10> {
        BYPS_W::new(self)
    }
    #[doc = "Bits 11:12 - Wide bus mode enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn buswidth(&mut self) -> BUSWIDTH_W<CLKCTRL_SPEC, 11> {
        BUSWIDTH_W::new(self)
    }
    #[doc = "Bit 13 - SDIO_CK dephasing selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn clkedg(&mut self) -> CLKEDG_W<CLKCTRL_SPEC, 13> {
        CLKEDG_W::new(self)
    }
    #[doc = "Bit 14 - HW Flow Control enable"]
    #[inline(always)]
    #[must_use]
    pub fn flwctrlen(&mut self) -> FLWCTRLEN_W<CLKCTRL_SPEC, 14> {
        FLWCTRLEN_W::new(self)
    }
    #[doc = "Bits 15:16 - Clock divide factor bit9 and bit8"]
    #[inline(always)]
    #[must_use]
    pub fn clkpsc98(&mut self) -> CLKPSC98_W<CLKCTRL_SPEC, 15> {
        CLKPSC98_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SDI clock control register (SDIO_CLKCTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
