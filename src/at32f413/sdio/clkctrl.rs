#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<CLKCTRL_SPEC>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<CLKCTRL_SPEC>;
#[doc = "Field `CLKDIV` reader - Clock division"]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock division"]
pub type CLKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CLKOEN` reader - Clock output enable"]
pub type CLKOEN_R = crate::BitReader;
#[doc = "Field `CLKOEN` writer - Clock output enable"]
pub type CLKOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWRSVEN` reader - Power saving mode enable"]
pub type PWRSVEN_R = crate::BitReader;
#[doc = "Field `PWRSVEN` writer - Power saving mode enable"]
pub type PWRSVEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BYPSEN` reader - Clock divider bypass enable bit"]
pub type BYPSEN_R = crate::BitReader;
#[doc = "Field `BYPSEN` writer - Clock divider bypass enable bit"]
pub type BYPSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUSWS` reader - Bus width selection"]
pub type BUSWS_R = crate::FieldReader;
#[doc = "Field `BUSWS` writer - Bus width selection"]
pub type BUSWS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CLKEDS` reader - SDIO_CK edge selection bit"]
pub type CLKEDS_R = crate::BitReader;
#[doc = "Field `CLKEDS` writer - SDIO_CK edge selection bit"]
pub type CLKEDS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HFCEN` reader - Hardware flow control enable"]
pub type HFCEN_R = crate::BitReader;
#[doc = "Field `HFCEN` writer - Hardware flow control enable"]
pub type HFCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLKDIV98` reader - Clock divide factor bit9 and bit8"]
pub type CLKDIV98_R = crate::FieldReader;
#[doc = "Field `CLKDIV98` writer - Clock divide factor bit9 and bit8"]
pub type CLKDIV98_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:7 - Clock division"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Clock output enable"]
    #[inline(always)]
    pub fn clkoen(&self) -> CLKOEN_R {
        CLKOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Power saving mode enable"]
    #[inline(always)]
    pub fn pwrsven(&self) -> PWRSVEN_R {
        PWRSVEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    pub fn bypsen(&self) -> BYPSEN_R {
        BYPSEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Bus width selection"]
    #[inline(always)]
    pub fn busws(&self) -> BUSWS_R {
        BUSWS_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - SDIO_CK edge selection bit"]
    #[inline(always)]
    pub fn clkeds(&self) -> CLKEDS_R {
        CLKEDS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Hardware flow control enable"]
    #[inline(always)]
    pub fn hfcen(&self) -> HFCEN_R {
        HFCEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - Clock divide factor bit9 and bit8"]
    #[inline(always)]
    pub fn clkdiv98(&self) -> CLKDIV98_R {
        CLKDIV98_R::new(((self.bits >> 15) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKCTRL")
            .field("clkdiv", &format_args!("{}", self.clkdiv().bits()))
            .field("clkoen", &format_args!("{}", self.clkoen().bit()))
            .field("pwrsven", &format_args!("{}", self.pwrsven().bit()))
            .field("bypsen", &format_args!("{}", self.bypsen().bit()))
            .field("busws", &format_args!("{}", self.busws().bits()))
            .field("clkeds", &format_args!("{}", self.clkeds().bit()))
            .field("hfcen", &format_args!("{}", self.hfcen().bit()))
            .field("clkdiv98", &format_args!("{}", self.clkdiv98().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CLKCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock division"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CLKCTRL_SPEC, 0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bit 8 - Clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkoen(&mut self) -> CLKOEN_W<CLKCTRL_SPEC, 8> {
        CLKOEN_W::new(self)
    }
    #[doc = "Bit 9 - Power saving mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsven(&mut self) -> PWRSVEN_W<CLKCTRL_SPEC, 9> {
        PWRSVEN_W::new(self)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn bypsen(&mut self) -> BYPSEN_W<CLKCTRL_SPEC, 10> {
        BYPSEN_W::new(self)
    }
    #[doc = "Bits 11:12 - Bus width selection"]
    #[inline(always)]
    #[must_use]
    pub fn busws(&mut self) -> BUSWS_W<CLKCTRL_SPEC, 11> {
        BUSWS_W::new(self)
    }
    #[doc = "Bit 13 - SDIO_CK edge selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn clkeds(&mut self) -> CLKEDS_W<CLKCTRL_SPEC, 13> {
        CLKEDS_W::new(self)
    }
    #[doc = "Bit 14 - Hardware flow control enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfcen(&mut self) -> HFCEN_W<CLKCTRL_SPEC, 14> {
        HFCEN_W::new(self)
    }
    #[doc = "Bits 15:16 - Clock divide factor bit9 and bit8"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv98(&mut self) -> CLKDIV98_W<CLKCTRL_SPEC, 15> {
        CLKDIV98_W::new(self)
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
#[doc = "SD clock control register (SDIO_CLKCTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
