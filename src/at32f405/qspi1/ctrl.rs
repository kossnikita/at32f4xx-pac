#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `CLKDIV` reader - SPI clock divider"]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - SPI clock divider"]
pub type CLKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SCKMODE` reader - Sckout mode"]
pub type SCKMODE_R = crate::BitReader;
#[doc = "Field `SCKMODE` writer - Sckout mode"]
pub type SCKMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XIPIDLE` reader - XIP port idle status"]
pub type XIPIDLE_R = crate::BitReader;
#[doc = "Field `XIPIDLE` writer - XIP port idle status"]
pub type XIPIDLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ABORT` reader - Abort instruction"]
pub type ABORT_R = crate::BitReader;
#[doc = "Field `ABORT` writer - Abort instruction"]
pub type ABORT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BUSY` reader - Busy bit of spi status"]
pub type BUSY_R = crate::FieldReader;
#[doc = "Field `BUSY` writer - Busy bit of spi status"]
pub type BUSY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `XIPRCMDF` reader - XIP read command flush"]
pub type XIPRCMDF_R = crate::BitReader;
#[doc = "Field `XIPRCMDF` writer - XIP read command flush"]
pub type XIPRCMDF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XIPSEL` reader - XIP port selection"]
pub type XIPSEL_R = crate::BitReader;
#[doc = "Field `XIPSEL` writer - XIP port selection"]
pub type XIPSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `KEYEN` reader - encryption key enable"]
pub type KEYEN_R = crate::BitReader;
#[doc = "Field `KEYEN` writer - encryption key enable"]
pub type KEYEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - SPI clock divider"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Sckout mode"]
    #[inline(always)]
    pub fn sckmode(&self) -> SCKMODE_R {
        SCKMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - XIP port idle status"]
    #[inline(always)]
    pub fn xipidle(&self) -> XIPIDLE_R {
        XIPIDLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Abort instruction"]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Busy bit of spi status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - XIP read command flush"]
    #[inline(always)]
    pub fn xiprcmdf(&self) -> XIPRCMDF_R {
        XIPRCMDF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - XIP port selection"]
    #[inline(always)]
    pub fn xipsel(&self) -> XIPSEL_R {
        XIPSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - encryption key enable"]
    #[inline(always)]
    pub fn keyen(&self) -> KEYEN_R {
        KEYEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("clkdiv", &format_args!("{}", self.clkdiv().bits()))
            .field("sckmode", &format_args!("{}", self.sckmode().bit()))
            .field("xipidle", &format_args!("{}", self.xipidle().bit()))
            .field("abort", &format_args!("{}", self.abort().bit()))
            .field("busy", &format_args!("{}", self.busy().bits()))
            .field("xiprcmdf", &format_args!("{}", self.xiprcmdf().bit()))
            .field("xipsel", &format_args!("{}", self.xipsel().bit()))
            .field("keyen", &format_args!("{}", self.keyen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<CTRL_SPEC, 0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bit 4 - Sckout mode"]
    #[inline(always)]
    #[must_use]
    pub fn sckmode(&mut self) -> SCKMODE_W<CTRL_SPEC, 4> {
        SCKMODE_W::new(self)
    }
    #[doc = "Bit 7 - XIP port idle status"]
    #[inline(always)]
    #[must_use]
    pub fn xipidle(&mut self) -> XIPIDLE_W<CTRL_SPEC, 7> {
        XIPIDLE_W::new(self)
    }
    #[doc = "Bit 8 - Abort instruction"]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<CTRL_SPEC, 8> {
        ABORT_W::new(self)
    }
    #[doc = "Bits 16:18 - Busy bit of spi status"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<CTRL_SPEC, 16> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 19 - XIP read command flush"]
    #[inline(always)]
    #[must_use]
    pub fn xiprcmdf(&mut self) -> XIPRCMDF_W<CTRL_SPEC, 19> {
        XIPRCMDF_W::new(self)
    }
    #[doc = "Bit 20 - XIP port selection"]
    #[inline(always)]
    #[must_use]
    pub fn xipsel(&mut self) -> XIPSEL_W<CTRL_SPEC, 20> {
        XIPSEL_W::new(self)
    }
    #[doc = "Bit 21 - encryption key enable"]
    #[inline(always)]
    #[must_use]
    pub fn keyen(&mut self) -> KEYEN_W<CTRL_SPEC, 21> {
        KEYEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
