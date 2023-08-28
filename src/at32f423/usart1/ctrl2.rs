#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `IDL` reader - bit 3-0 for usart identification"]
pub type IDL_R = crate::FieldReader;
#[doc = "Field `IDL` writer - bit 3-0 for usart identification"]
pub type IDL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `IDBN` reader - Identification bit num"]
pub type IDBN_R = crate::BitReader;
#[doc = "Field `IDBN` writer - Identification bit num"]
pub type IDBN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BFBN` reader - Break frame bit num"]
pub type BFBN_R = crate::BitReader;
#[doc = "Field `BFBN` writer - Break frame bit num"]
pub type BFBN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BFIEN` reader - Break frame interrupt enable"]
pub type BFIEN_R = crate::BitReader;
#[doc = "Field `BFIEN` writer - Break frame interrupt enable"]
pub type BFIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LBCP` reader - Last bit clock pulse"]
pub type LBCP_R = crate::BitReader;
#[doc = "Field `LBCP` writer - Last bit clock pulse"]
pub type LBCP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLKPHA` reader - Clock phase"]
pub type CLKPHA_R = crate::BitReader;
#[doc = "Field `CLKPHA` writer - Clock phase"]
pub type CLKPHA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLKPOL` reader - Clock polarity"]
pub type CLKPOL_R = crate::BitReader;
#[doc = "Field `CLKPOL` writer - Clock polarity"]
pub type CLKPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type CLKEN_R = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type CLKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STOPBN` reader - STOP bit num"]
pub type STOPBN_R = crate::FieldReader;
#[doc = "Field `STOPBN` writer - STOP bit num"]
pub type STOPBN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LINEN` reader - LIN mode enable"]
pub type LINEN_R = crate::BitReader;
#[doc = "Field `LINEN` writer - LIN mode enable"]
pub type LINEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRPSWAP` reader - Transmit receive pin swap"]
pub type TRPSWAP_R = crate::BitReader;
#[doc = "Field `TRPSWAP` writer - Transmit receive pin swap"]
pub type TRPSWAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXREV` reader - RX polarity reverse"]
pub type RXREV_R = crate::BitReader;
#[doc = "Field `RXREV` writer - RX polarity reverse"]
pub type RXREV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXREV` reader - TX polarity reverse"]
pub type TXREV_R = crate::BitReader;
#[doc = "Field `TXREV` writer - TX polarity reverse"]
pub type TXREV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTREV` reader - DT register polarity reverse"]
pub type DTREV_R = crate::BitReader;
#[doc = "Field `DTREV` writer - DT register polarity reverse"]
pub type DTREV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTF` reader - MSB transmit first"]
pub type MTF_R = crate::BitReader;
#[doc = "Field `MTF` writer - MSB transmit first"]
pub type MTF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IDH` reader - bit 7-4 for usart identification"]
pub type IDH_R = crate::FieldReader;
#[doc = "Field `IDH` writer - bit 7-4 for usart identification"]
pub type IDH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - bit 3-0 for usart identification"]
    #[inline(always)]
    pub fn idl(&self) -> IDL_R {
        IDL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Identification bit num"]
    #[inline(always)]
    pub fn idbn(&self) -> IDBN_R {
        IDBN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Break frame bit num"]
    #[inline(always)]
    pub fn bfbn(&self) -> BFBN_R {
        BFBN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Break frame interrupt enable"]
    #[inline(always)]
    pub fn bfien(&self) -> BFIEN_R {
        BFIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcp(&self) -> LBCP_R {
        LBCP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn clkpha(&self) -> CLKPHA_R {
        CLKPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bit num"]
    #[inline(always)]
    pub fn stopbn(&self) -> STOPBN_R {
        STOPBN_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmit receive pin swap"]
    #[inline(always)]
    pub fn trpswap(&self) -> TRPSWAP_R {
        TRPSWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX polarity reverse"]
    #[inline(always)]
    pub fn rxrev(&self) -> RXREV_R {
        RXREV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX polarity reverse"]
    #[inline(always)]
    pub fn txrev(&self) -> TXREV_R {
        TXREV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DT register polarity reverse"]
    #[inline(always)]
    pub fn dtrev(&self) -> DTREV_R {
        DTREV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MSB transmit first"]
    #[inline(always)]
    pub fn mtf(&self) -> MTF_R {
        MTF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 28:31 - bit 7-4 for usart identification"]
    #[inline(always)]
    pub fn idh(&self) -> IDH_R {
        IDH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - bit 3-0 for usart identification"]
    #[inline(always)]
    #[must_use]
    pub fn idl(&mut self) -> IDL_W<CTRL2_SPEC, 0> {
        IDL_W::new(self)
    }
    #[doc = "Bit 4 - Identification bit num"]
    #[inline(always)]
    #[must_use]
    pub fn idbn(&mut self) -> IDBN_W<CTRL2_SPEC, 4> {
        IDBN_W::new(self)
    }
    #[doc = "Bit 5 - Break frame bit num"]
    #[inline(always)]
    #[must_use]
    pub fn bfbn(&mut self) -> BFBN_W<CTRL2_SPEC, 5> {
        BFBN_W::new(self)
    }
    #[doc = "Bit 6 - Break frame interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bfien(&mut self) -> BFIEN_W<CTRL2_SPEC, 6> {
        BFIEN_W::new(self)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    #[must_use]
    pub fn lbcp(&mut self) -> LBCP_W<CTRL2_SPEC, 8> {
        LBCP_W::new(self)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn clkpha(&mut self) -> CLKPHA_W<CTRL2_SPEC, 9> {
        CLKPHA_W::new(self)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn clkpol(&mut self) -> CLKPOL_W<CTRL2_SPEC, 10> {
        CLKPOL_W::new(self)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<CTRL2_SPEC, 11> {
        CLKEN_W::new(self)
    }
    #[doc = "Bits 12:13 - STOP bit num"]
    #[inline(always)]
    #[must_use]
    pub fn stopbn(&mut self) -> STOPBN_W<CTRL2_SPEC, 12> {
        STOPBN_W::new(self)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn linen(&mut self) -> LINEN_W<CTRL2_SPEC, 14> {
        LINEN_W::new(self)
    }
    #[doc = "Bit 15 - Transmit receive pin swap"]
    #[inline(always)]
    #[must_use]
    pub fn trpswap(&mut self) -> TRPSWAP_W<CTRL2_SPEC, 15> {
        TRPSWAP_W::new(self)
    }
    #[doc = "Bit 16 - RX polarity reverse"]
    #[inline(always)]
    #[must_use]
    pub fn rxrev(&mut self) -> RXREV_W<CTRL2_SPEC, 16> {
        RXREV_W::new(self)
    }
    #[doc = "Bit 17 - TX polarity reverse"]
    #[inline(always)]
    #[must_use]
    pub fn txrev(&mut self) -> TXREV_W<CTRL2_SPEC, 17> {
        TXREV_W::new(self)
    }
    #[doc = "Bit 18 - DT register polarity reverse"]
    #[inline(always)]
    #[must_use]
    pub fn dtrev(&mut self) -> DTREV_W<CTRL2_SPEC, 18> {
        DTREV_W::new(self)
    }
    #[doc = "Bit 19 - MSB transmit first"]
    #[inline(always)]
    #[must_use]
    pub fn mtf(&mut self) -> MTF_W<CTRL2_SPEC, 19> {
        MTF_W::new(self)
    }
    #[doc = "Bits 28:31 - bit 7-4 for usart identification"]
    #[inline(always)]
    #[must_use]
    pub fn idh(&mut self) -> IDH_W<CTRL2_SPEC, 28> {
        IDH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
