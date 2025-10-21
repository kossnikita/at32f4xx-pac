#[doc = "Register `MACFCTRL` reader"]
pub type R = crate::R<MACFCTRL_SPEC>;
#[doc = "Register `MACFCTRL` writer"]
pub type W = crate::W<MACFCTRL_SPEC>;
#[doc = "Field `FCB_BPA` reader - Flow control busy/back pressure activate"]
pub type FCB_BPA_R = crate::BitReader;
#[doc = "Field `FCB_BPA` writer - Flow control busy/back pressure activate"]
pub type FCB_BPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETF` reader - Enable transmit flow control"]
pub type ETF_R = crate::BitReader;
#[doc = "Field `ETF` writer - Enable transmit flow control"]
pub type ETF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERF` reader - Enable receive flow control"]
pub type ERF_R = crate::BitReader;
#[doc = "Field `ERF` writer - Enable receive flow control"]
pub type ERF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUP` reader - Detect unicast pause frame"]
pub type DUP_R = crate::BitReader;
#[doc = "Field `DUP` writer - Detect unicast pause frame"]
pub type DUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLT` reader - Pause low threshold"]
pub type PLT_R = crate::FieldReader;
#[doc = "Field `PLT` writer - Pause low threshold"]
pub type PLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DZQP` reader - Disable zero-quanta pause"]
pub type DZQP_R = crate::BitReader;
#[doc = "Field `DZQP` writer - Disable zero-quanta pause"]
pub type DZQP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT` reader - Pass time"]
pub type PT_R = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - Pass time"]
pub type PT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACFCTRL")
            .field("fcb_bpa", &self.fcb_bpa())
            .field("etf", &self.etf())
            .field("erf", &self.erf())
            .field("dup", &self.dup())
            .field("plt", &self.plt())
            .field("dzqp", &self.dzqp())
            .field("pt", &self.pt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Flow control busy/back pressure activate"]
    #[inline(always)]
    pub fn fcb_bpa(&mut self) -> FCB_BPA_W<'_, MACFCTRL_SPEC> {
        FCB_BPA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable transmit flow control"]
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W<'_, MACFCTRL_SPEC> {
        ETF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable receive flow control"]
    #[inline(always)]
    pub fn erf(&mut self) -> ERF_W<'_, MACFCTRL_SPEC> {
        ERF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Detect unicast pause frame"]
    #[inline(always)]
    pub fn dup(&mut self) -> DUP_W<'_, MACFCTRL_SPEC> {
        DUP_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Pause low threshold"]
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W<'_, MACFCTRL_SPEC> {
        PLT_W::new(self, 4)
    }
    #[doc = "Bit 7 - Disable zero-quanta pause"]
    #[inline(always)]
    pub fn dzqp(&mut self) -> DZQP_W<'_, MACFCTRL_SPEC> {
        DZQP_W::new(self, 7)
    }
    #[doc = "Bits 16:31 - Pass time"]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W<'_, MACFCTRL_SPEC> {
        PT_W::new(self, 16)
    }
}
#[doc = "Ethernet MAC flow control register\n\nYou can [`read`](crate::Reg::read) this register and get [`macfctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macfctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACFCTRL_SPEC;
impl crate::RegisterSpec for MACFCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macfctrl::R`](R) reader structure"]
impl crate::Readable for MACFCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macfctrl::W`](W) writer structure"]
impl crate::Writable for MACFCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACFCTRL to value 0"]
impl crate::Resettable for MACFCTRL_SPEC {}
