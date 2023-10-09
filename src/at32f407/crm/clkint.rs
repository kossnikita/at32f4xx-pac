#[doc = "Register `CLKINT` reader"]
pub type R = crate::R<CLKINT_SPEC>;
#[doc = "Register `CLKINT` writer"]
pub type W = crate::W<CLKINT_SPEC>;
#[doc = "Field `LICKSTBLF` reader - LICK ready interrupt flag"]
pub type LICKSTBLF_R = crate::BitReader;
#[doc = "Field `LEXTSTBLF` reader - LEXT ready interrupt flag"]
pub type LEXTSTBLF_R = crate::BitReader;
#[doc = "Field `HICKSTBLF` reader - HICK ready interrupt flag"]
pub type HICKSTBLF_R = crate::BitReader;
#[doc = "Field `HEXTSTBLF` reader - HEXT ready interrupt flag"]
pub type HEXTSTBLF_R = crate::BitReader;
#[doc = "Field `PLLSTBLF` reader - PLL ready interrupt flag"]
pub type PLLSTBLF_R = crate::BitReader;
#[doc = "Field `CFDF` reader - Clock failure detection interrupt flag"]
pub type CFDF_R = crate::BitReader;
#[doc = "Field `LICKSTBLIEN` reader - LICK ready interrupt enable"]
pub type LICKSTBLIEN_R = crate::BitReader;
#[doc = "Field `LICKSTBLIEN` writer - LICK ready interrupt enable"]
pub type LICKSTBLIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LEXTSTBLIEN` reader - LEXT ready interrupt enable"]
pub type LEXTSTBLIEN_R = crate::BitReader;
#[doc = "Field `LEXTSTBLIEN` writer - LEXT ready interrupt enable"]
pub type LEXTSTBLIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HICKSTBLIEN` reader - HICK ready interrupt enable"]
pub type HICKSTBLIEN_R = crate::BitReader;
#[doc = "Field `HICKSTBLIEN` writer - HICK ready interrupt enable"]
pub type HICKSTBLIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HEXTSTBLIEN` reader - HEXT ready interrupt enable"]
pub type HEXTSTBLIEN_R = crate::BitReader;
#[doc = "Field `HEXTSTBLIEN` writer - HEXT ready interrupt enable"]
pub type HEXTSTBLIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLSTBLIEN` reader - PLL ready interrupt enable"]
pub type PLLSTBLIEN_R = crate::BitReader;
#[doc = "Field `PLLSTBLIEN` writer - PLL ready interrupt enable"]
pub type PLLSTBLIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LICKSTBLFC` writer - LICK ready interrupt clear"]
pub type LICKSTBLFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LEXTSTBLFC` writer - LEXT ready interrupt clear"]
pub type LEXTSTBLFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HICKSTBLFC` writer - HICK ready interrupt clear"]
pub type HICKSTBLFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HEXTSTBLFC` writer - HEXT ready interrupt clear"]
pub type HEXTSTBLFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLLSTBLFC` writer - PLL ready interrupt clear"]
pub type PLLSTBLFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CFDFC` writer - Clock failure detection interrupt clear"]
pub type CFDFC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - LICK ready interrupt flag"]
    #[inline(always)]
    pub fn lickstblf(&self) -> LICKSTBLF_R {
        LICKSTBLF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LEXT ready interrupt flag"]
    #[inline(always)]
    pub fn lextstblf(&self) -> LEXTSTBLF_R {
        LEXTSTBLF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HICK ready interrupt flag"]
    #[inline(always)]
    pub fn hickstblf(&self) -> HICKSTBLF_R {
        HICKSTBLF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HEXT ready interrupt flag"]
    #[inline(always)]
    pub fn hextstblf(&self) -> HEXTSTBLF_R {
        HEXTSTBLF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllstblf(&self) -> PLLSTBLF_R {
        PLLSTBLF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock failure detection interrupt flag"]
    #[inline(always)]
    pub fn cfdf(&self) -> CFDF_R {
        CFDF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LICK ready interrupt enable"]
    #[inline(always)]
    pub fn lickstblien(&self) -> LICKSTBLIEN_R {
        LICKSTBLIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LEXT ready interrupt enable"]
    #[inline(always)]
    pub fn lextstblien(&self) -> LEXTSTBLIEN_R {
        LEXTSTBLIEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HICK ready interrupt enable"]
    #[inline(always)]
    pub fn hickstblien(&self) -> HICKSTBLIEN_R {
        HICKSTBLIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HEXT ready interrupt enable"]
    #[inline(always)]
    pub fn hextstblien(&self) -> HEXTSTBLIEN_R {
        HEXTSTBLIEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllstblien(&self) -> PLLSTBLIEN_R {
        PLLSTBLIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKINT")
            .field("lickstblf", &format_args!("{}", self.lickstblf().bit()))
            .field("lextstblf", &format_args!("{}", self.lextstblf().bit()))
            .field("hickstblf", &format_args!("{}", self.hickstblf().bit()))
            .field("hextstblf", &format_args!("{}", self.hextstblf().bit()))
            .field("pllstblf", &format_args!("{}", self.pllstblf().bit()))
            .field("cfdf", &format_args!("{}", self.cfdf().bit()))
            .field("lickstblien", &format_args!("{}", self.lickstblien().bit()))
            .field("lextstblien", &format_args!("{}", self.lextstblien().bit()))
            .field("hickstblien", &format_args!("{}", self.hickstblien().bit()))
            .field("hextstblien", &format_args!("{}", self.hextstblien().bit()))
            .field("pllstblien", &format_args!("{}", self.pllstblien().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CLKINT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 8 - LICK ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lickstblien(&mut self) -> LICKSTBLIEN_W<CLKINT_SPEC, 8> {
        LICKSTBLIEN_W::new(self)
    }
    #[doc = "Bit 9 - LEXT ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lextstblien(&mut self) -> LEXTSTBLIEN_W<CLKINT_SPEC, 9> {
        LEXTSTBLIEN_W::new(self)
    }
    #[doc = "Bit 10 - HICK ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hickstblien(&mut self) -> HICKSTBLIEN_W<CLKINT_SPEC, 10> {
        HICKSTBLIEN_W::new(self)
    }
    #[doc = "Bit 11 - HEXT ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hextstblien(&mut self) -> HEXTSTBLIEN_W<CLKINT_SPEC, 11> {
        HEXTSTBLIEN_W::new(self)
    }
    #[doc = "Bit 12 - PLL ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllstblien(&mut self) -> PLLSTBLIEN_W<CLKINT_SPEC, 12> {
        PLLSTBLIEN_W::new(self)
    }
    #[doc = "Bit 16 - LICK ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lickstblfc(&mut self) -> LICKSTBLFC_W<CLKINT_SPEC, 16> {
        LICKSTBLFC_W::new(self)
    }
    #[doc = "Bit 17 - LEXT ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lextstblfc(&mut self) -> LEXTSTBLFC_W<CLKINT_SPEC, 17> {
        LEXTSTBLFC_W::new(self)
    }
    #[doc = "Bit 18 - HICK ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hickstblfc(&mut self) -> HICKSTBLFC_W<CLKINT_SPEC, 18> {
        HICKSTBLFC_W::new(self)
    }
    #[doc = "Bit 19 - HEXT ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hextstblfc(&mut self) -> HEXTSTBLFC_W<CLKINT_SPEC, 19> {
        HEXTSTBLFC_W::new(self)
    }
    #[doc = "Bit 20 - PLL ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllstblfc(&mut self) -> PLLSTBLFC_W<CLKINT_SPEC, 20> {
        PLLSTBLFC_W::new(self)
    }
    #[doc = "Bit 23 - Clock failure detection interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cfdfc(&mut self) -> CFDFC_W<CLKINT_SPEC, 23> {
        CFDFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock interrupt register (CRM_CLKINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKINT_SPEC;
impl crate::RegisterSpec for CLKINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkint::R`](R) reader structure"]
impl crate::Readable for CLKINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkint::W`](W) writer structure"]
impl crate::Writable for CLKINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKINT to value 0"]
impl crate::Resettable for CLKINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
