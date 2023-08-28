#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `CSRST` reader - Core soft reset"]
pub type CSRST_R = crate::BitReader;
#[doc = "Field `CSRST` writer - Core soft reset"]
pub type CSRST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DISUSB` reader - Disable usb phy"]
pub type DISUSB_R = crate::BitReader;
#[doc = "Field `DISUSB` writer - Disable usb phy"]
pub type DISUSB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPM` reader - Low power mode"]
pub type LPM_R = crate::BitReader;
#[doc = "Field `LPM` writer - Low power mode"]
pub type LPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSP` reader - Soft suspend config"]
pub type SSP_R = crate::BitReader;
#[doc = "Field `SSP` writer - Soft suspend config"]
pub type SSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GRESUME` reader - Generate resume request"]
pub type GRESUME_R = crate::BitReader;
#[doc = "Field `GRESUME` writer - Generate resume request"]
pub type GRESUME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LSOFIEN` reader - Lost start of frame interrupt enable"]
pub type LSOFIEN_R = crate::BitReader;
#[doc = "Field `LSOFIEN` writer - Lost start of frame interrupt enable"]
pub type LSOFIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFIEN` reader - Start of frame interrupt enable"]
pub type SOFIEN_R = crate::BitReader;
#[doc = "Field `SOFIEN` writer - Start of frame interrupt enable"]
pub type SOFIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTIEN` reader - Bus reset interrupt enable"]
pub type RSTIEN_R = crate::BitReader;
#[doc = "Field `RSTIEN` writer - Bus reset interrupt enable"]
pub type RSTIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPIEN` reader - Bus suspend mode interrupt enable"]
pub type SPIEN_R = crate::BitReader;
#[doc = "Field `SPIEN` writer - Bus suspend mode interrupt enable"]
pub type SPIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WKIEN` reader - Wakeup/Remote wakeup interrupt enable"]
pub type WKIEN_R = crate::BitReader;
#[doc = "Field `WKIEN` writer - Wakeup/Remote wakeup interrupt enable"]
pub type WKIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BEIEN` reader - Bus error interrupt enable"]
pub type BEIEN_R = crate::BitReader;
#[doc = "Field `BEIEN` writer - Bus error interrupt enable"]
pub type BEIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UCFORIEN` reader - USB Core fifo overrun interrupt enable"]
pub type UCFORIEN_R = crate::BitReader;
#[doc = "Field `UCFORIEN` writer - USB Core fifo overrun interrupt enable"]
pub type UCFORIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCIEN` reader - transmission completed interrupt enable"]
pub type TCIEN_R = crate::BitReader;
#[doc = "Field `TCIEN` writer - transmission completed interrupt enable"]
pub type TCIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    pub fn csrst(&self) -> CSRST_R {
        CSRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable usb phy"]
    #[inline(always)]
    pub fn disusb(&self) -> DISUSB_R {
        DISUSB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low power mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Soft suspend config"]
    #[inline(always)]
    pub fn ssp(&self) -> SSP_R {
        SSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Generate resume request"]
    #[inline(always)]
    pub fn gresume(&self) -> GRESUME_R {
        GRESUME_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Lost start of frame interrupt enable"]
    #[inline(always)]
    pub fn lsofien(&self) -> LSOFIEN_R {
        LSOFIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of frame interrupt enable"]
    #[inline(always)]
    pub fn sofien(&self) -> SOFIEN_R {
        SOFIEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus reset interrupt enable"]
    #[inline(always)]
    pub fn rstien(&self) -> RSTIEN_R {
        RSTIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus suspend mode interrupt enable"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup/Remote wakeup interrupt enable"]
    #[inline(always)]
    pub fn wkien(&self) -> WKIEN_R {
        WKIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bus error interrupt enable"]
    #[inline(always)]
    pub fn beien(&self) -> BEIEN_R {
        BEIEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USB Core fifo overrun interrupt enable"]
    #[inline(always)]
    pub fn ucforien(&self) -> UCFORIEN_R {
        UCFORIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - transmission completed interrupt enable"]
    #[inline(always)]
    pub fn tcien(&self) -> TCIEN_R {
        TCIEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    #[must_use]
    pub fn csrst(&mut self) -> CSRST_W<CTRL_SPEC, 0> {
        CSRST_W::new(self)
    }
    #[doc = "Bit 1 - Disable usb phy"]
    #[inline(always)]
    #[must_use]
    pub fn disusb(&mut self) -> DISUSB_W<CTRL_SPEC, 1> {
        DISUSB_W::new(self)
    }
    #[doc = "Bit 2 - Low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpm(&mut self) -> LPM_W<CTRL_SPEC, 2> {
        LPM_W::new(self)
    }
    #[doc = "Bit 3 - Soft suspend config"]
    #[inline(always)]
    #[must_use]
    pub fn ssp(&mut self) -> SSP_W<CTRL_SPEC, 3> {
        SSP_W::new(self)
    }
    #[doc = "Bit 4 - Generate resume request"]
    #[inline(always)]
    #[must_use]
    pub fn gresume(&mut self) -> GRESUME_W<CTRL_SPEC, 4> {
        GRESUME_W::new(self)
    }
    #[doc = "Bit 8 - Lost start of frame interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsofien(&mut self) -> LSOFIEN_W<CTRL_SPEC, 8> {
        LSOFIEN_W::new(self)
    }
    #[doc = "Bit 9 - Start of frame interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofien(&mut self) -> SOFIEN_W<CTRL_SPEC, 9> {
        SOFIEN_W::new(self)
    }
    #[doc = "Bit 10 - Bus reset interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstien(&mut self) -> RSTIEN_W<CTRL_SPEC, 10> {
        RSTIEN_W::new(self)
    }
    #[doc = "Bit 11 - Bus suspend mode interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<CTRL_SPEC, 11> {
        SPIEN_W::new(self)
    }
    #[doc = "Bit 12 - Wakeup/Remote wakeup interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wkien(&mut self) -> WKIEN_W<CTRL_SPEC, 12> {
        WKIEN_W::new(self)
    }
    #[doc = "Bit 13 - Bus error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn beien(&mut self) -> BEIEN_W<CTRL_SPEC, 13> {
        BEIEN_W::new(self)
    }
    #[doc = "Bit 14 - USB Core fifo overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucforien(&mut self) -> UCFORIEN_W<CTRL_SPEC, 14> {
        UCFORIEN_W::new(self)
    }
    #[doc = "Bit 15 - transmission completed interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcien(&mut self) -> TCIEN_W<CTRL_SPEC, 15> {
        TCIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0x03"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
