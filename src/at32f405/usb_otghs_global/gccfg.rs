#[doc = "Register `GCCFG` reader"]
pub type R = crate::R<GCCFG_SPEC>;
#[doc = "Register `GCCFG` writer"]
pub type W = crate::W<GCCFG_SPEC>;
#[doc = "Field `PWRDOWN` reader - Power down"]
pub type PWRDOWN_R = crate::BitReader;
#[doc = "Field `PWRDOWN` writer - Power down"]
pub type PWRDOWN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFOUTEN` reader - SOF output enable"]
pub type SOFOUTEN_R = crate::BitReader;
#[doc = "Field `SOFOUTEN` writer - SOF output enable"]
pub type SOFOUTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VBUSIG` reader - VBUS Ignored"]
pub type VBUSIG_R = crate::BitReader;
#[doc = "Field `VBUSIG` writer - VBUS Ignored"]
pub type VBUSIG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAIT_CLK_RCV` reader - Wait Clock Soure Recover"]
pub type WAIT_CLK_RCV_R = crate::BitReader;
#[doc = "Field `WAIT_CLK_RCV` writer - Wait Clock Soure Recover"]
pub type WAIT_CLK_RCV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    pub fn pwrdown(&self) -> PWRDOWN_R {
        PWRDOWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    pub fn sofouten(&self) -> SOFOUTEN_R {
        SOFOUTEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - VBUS Ignored"]
    #[inline(always)]
    pub fn vbusig(&self) -> VBUSIG_R {
        VBUSIG_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Wait Clock Soure Recover"]
    #[inline(always)]
    pub fn wait_clk_rcv(&self) -> WAIT_CLK_RCV_R {
        WAIT_CLK_RCV_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GCCFG")
            .field("pwrdown", &format_args!("{}", self.pwrdown().bit()))
            .field("sofouten", &format_args!("{}", self.sofouten().bit()))
            .field("vbusig", &format_args!("{}", self.vbusig().bit()))
            .field(
                "wait_clk_rcv",
                &format_args!("{}", self.wait_clk_rcv().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GCCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdown(&mut self) -> PWRDOWN_W<GCCFG_SPEC, 16> {
        PWRDOWN_W::new(self)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofouten(&mut self) -> SOFOUTEN_W<GCCFG_SPEC, 20> {
        SOFOUTEN_W::new(self)
    }
    #[doc = "Bit 21 - VBUS Ignored"]
    #[inline(always)]
    #[must_use]
    pub fn vbusig(&mut self) -> VBUSIG_W<GCCFG_SPEC, 21> {
        VBUSIG_W::new(self)
    }
    #[doc = "Bit 22 - Wait Clock Soure Recover"]
    #[inline(always)]
    #[must_use]
    pub fn wait_clk_rcv(&mut self) -> WAIT_CLK_RCV_W<GCCFG_SPEC, 22> {
        WAIT_CLK_RCV_W::new(self)
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
#[doc = "OTGHS general core configuration register (OTGHS_GCCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCCFG_SPEC;
impl crate::RegisterSpec for GCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gccfg::R`](R) reader structure"]
impl crate::Readable for GCCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gccfg::W`](W) writer structure"]
impl crate::Writable for GCCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCCFG to value 0"]
impl crate::Resettable for GCCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
