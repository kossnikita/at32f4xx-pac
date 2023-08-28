#[doc = "Register `CMDCTRL` reader"]
pub type R = crate::R<CMDCTRL_SPEC>;
#[doc = "Register `CMDCTRL` writer"]
pub type W = crate::W<CMDCTRL_SPEC>;
#[doc = "Field `CMDIDX` reader - CMDIDX"]
pub type CMDIDX_R = crate::FieldReader;
#[doc = "Field `CMDIDX` writer - CMDIDX"]
pub type CMDIDX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `RSPWT` reader - Wait for response"]
pub type RSPWT_R = crate::FieldReader;
#[doc = "Field `RSPWT` writer - Wait for response"]
pub type RSPWT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `INTWT` reader - CCSM wait for interrupt"]
pub type INTWT_R = crate::BitReader;
#[doc = "Field `INTWT` writer - CCSM wait for interrupt"]
pub type INTWT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PNDWT` reader - CCSM wait for end of transfer"]
pub type PNDWT_R = crate::BitReader;
#[doc = "Field `PNDWT` writer - CCSM wait for end of transfer"]
pub type PNDWT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCSMEN` reader - Command channel state machine"]
pub type CCSMEN_R = crate::BitReader;
#[doc = "Field `CCSMEN` writer - Command channel state machine"]
pub type CCSMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOSUSP` reader - SD I/O suspend command"]
pub type IOSUSP_R = crate::BitReader;
#[doc = "Field `IOSUSP` writer - SD I/O suspend command"]
pub type IOSUSP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - CMDIDX"]
    #[inline(always)]
    pub fn cmdidx(&self) -> CMDIDX_R {
        CMDIDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Wait for response"]
    #[inline(always)]
    pub fn rspwt(&self) -> RSPWT_R {
        RSPWT_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - CCSM wait for interrupt"]
    #[inline(always)]
    pub fn intwt(&self) -> INTWT_R {
        INTWT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CCSM wait for end of transfer"]
    #[inline(always)]
    pub fn pndwt(&self) -> PNDWT_R {
        PNDWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Command channel state machine"]
    #[inline(always)]
    pub fn ccsmen(&self) -> CCSMEN_R {
        CCSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O suspend command"]
    #[inline(always)]
    pub fn iosusp(&self) -> IOSUSP_R {
        IOSUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - CMDIDX"]
    #[inline(always)]
    #[must_use]
    pub fn cmdidx(&mut self) -> CMDIDX_W<CMDCTRL_SPEC, 0> {
        CMDIDX_W::new(self)
    }
    #[doc = "Bits 6:7 - Wait for response"]
    #[inline(always)]
    #[must_use]
    pub fn rspwt(&mut self) -> RSPWT_W<CMDCTRL_SPEC, 6> {
        RSPWT_W::new(self)
    }
    #[doc = "Bit 8 - CCSM wait for interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn intwt(&mut self) -> INTWT_W<CMDCTRL_SPEC, 8> {
        INTWT_W::new(self)
    }
    #[doc = "Bit 9 - CCSM wait for end of transfer"]
    #[inline(always)]
    #[must_use]
    pub fn pndwt(&mut self) -> PNDWT_W<CMDCTRL_SPEC, 9> {
        PNDWT_W::new(self)
    }
    #[doc = "Bit 10 - Command channel state machine"]
    #[inline(always)]
    #[must_use]
    pub fn ccsmen(&mut self) -> CCSMEN_W<CMDCTRL_SPEC, 10> {
        CCSMEN_W::new(self)
    }
    #[doc = "Bit 11 - SD I/O suspend command"]
    #[inline(always)]
    #[must_use]
    pub fn iosusp(&mut self) -> IOSUSP_W<CMDCTRL_SPEC, 11> {
        IOSUSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SDIO command control register (SDIO_CMDCTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDCTRL_SPEC;
impl crate::RegisterSpec for CMDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdctrl::R`](R) reader structure"]
impl crate::Readable for CMDCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmdctrl::W`](W) writer structure"]
impl crate::Writable for CMDCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDCTRL to value 0"]
impl crate::Resettable for CMDCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
