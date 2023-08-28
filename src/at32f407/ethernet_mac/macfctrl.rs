#[doc = "Register `MACFCTRL` reader"]
pub type R = crate::R<MACFCTRL_SPEC>;
#[doc = "Register `MACFCTRL` writer"]
pub type W = crate::W<MACFCTRL_SPEC>;
#[doc = "Field `FCB_BPA` reader - Flow control busy/back pressure activate"]
pub type FCB_BPA_R = crate::BitReader;
#[doc = "Field `FCB_BPA` writer - Flow control busy/back pressure activate"]
pub type FCB_BPA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETF` reader - Enable transmit flow control"]
pub type ETF_R = crate::BitReader;
#[doc = "Field `ETF` writer - Enable transmit flow control"]
pub type ETF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERF` reader - Enable receive flow control"]
pub type ERF_R = crate::BitReader;
#[doc = "Field `ERF` writer - Enable receive flow control"]
pub type ERF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DUP` reader - Detect unicast pause frame"]
pub type DUP_R = crate::BitReader;
#[doc = "Field `DUP` writer - Detect unicast pause frame"]
pub type DUP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PLT` reader - Pause low threshold"]
pub type PLT_R = crate::FieldReader;
#[doc = "Field `PLT` writer - Pause low threshold"]
pub type PLT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DZQP` reader - Disable zero-quanta pause"]
pub type DZQP_R = crate::BitReader;
#[doc = "Field `DZQP` writer - Disable zero-quanta pause"]
pub type DZQP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PT` reader - Pass time"]
pub type PT_R = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - Pass time"]
pub type PT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - Flow control busy/back pressure activate"]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FCB_BPA_R {
        FCB_BPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable transmit flow control"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable receive flow control"]
    #[inline(always)]
    pub fn erf(&self) -> ERF_R {
        ERF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Detect unicast pause frame"]
    #[inline(always)]
    pub fn dup(&self) -> DUP_R {
        DUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Disable zero-quanta pause"]
    #[inline(always)]
    pub fn dzqp(&self) -> DZQP_R {
        DZQP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pass time"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow control busy/back pressure activate"]
    #[inline(always)]
    #[must_use]
    pub fn fcb_bpa(&mut self) -> FCB_BPA_W<MACFCTRL_SPEC, 0> {
        FCB_BPA_W::new(self)
    }
    #[doc = "Bit 1 - Enable transmit flow control"]
    #[inline(always)]
    #[must_use]
    pub fn etf(&mut self) -> ETF_W<MACFCTRL_SPEC, 1> {
        ETF_W::new(self)
    }
    #[doc = "Bit 2 - Enable receive flow control"]
    #[inline(always)]
    #[must_use]
    pub fn erf(&mut self) -> ERF_W<MACFCTRL_SPEC, 2> {
        ERF_W::new(self)
    }
    #[doc = "Bit 3 - Detect unicast pause frame"]
    #[inline(always)]
    #[must_use]
    pub fn dup(&mut self) -> DUP_W<MACFCTRL_SPEC, 3> {
        DUP_W::new(self)
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PLT_W<MACFCTRL_SPEC, 4> {
        PLT_W::new(self)
    }
    #[doc = "Bit 7 - Disable zero-quanta pause"]
    #[inline(always)]
    #[must_use]
    pub fn dzqp(&mut self) -> DZQP_W<MACFCTRL_SPEC, 7> {
        DZQP_W::new(self)
    }
    #[doc = "Bits 16:31 - Pass time"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<MACFCTRL_SPEC, 16> {
        PT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet MAC flow control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macfctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macfctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACFCTRL_SPEC;
impl crate::RegisterSpec for MACFCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macfctrl::R`](R) reader structure"]
impl crate::Readable for MACFCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macfctrl::W`](W) writer structure"]
impl crate::Writable for MACFCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACFCTRL to value 0"]
impl crate::Resettable for MACFCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
