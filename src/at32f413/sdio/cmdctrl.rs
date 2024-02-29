#[doc = "Register `CMDCTRL` reader"]
pub type R = crate::R<CMDCTRL_SPEC>;
#[doc = "Register `CMDCTRL` writer"]
pub type W = crate::W<CMDCTRL_SPEC>;
#[doc = "Field `CMDIDX` reader - CMDIDX"]
pub type CMDIDX_R = crate::FieldReader;
#[doc = "Field `CMDIDX` writer - CMDIDX"]
pub type CMDIDX_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RSPWT` reader - Wait for response"]
pub type RSPWT_R = crate::FieldReader;
#[doc = "Field `RSPWT` writer - Wait for response"]
pub type RSPWT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTWT` reader - CCSM wait for interrupt"]
pub type INTWT_R = crate::BitReader;
#[doc = "Field `INTWT` writer - CCSM wait for interrupt"]
pub type INTWT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PNDWT` reader - CCSM wait for end of transfer"]
pub type PNDWT_R = crate::BitReader;
#[doc = "Field `PNDWT` writer - CCSM wait for end of transfer"]
pub type PNDWT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCSMEN` reader - Command channel state machine"]
pub type CCSMEN_R = crate::BitReader;
#[doc = "Field `CCSMEN` writer - Command channel state machine"]
pub type CCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSUSP` reader - SD I/O suspend command"]
pub type IOSUSP_R = crate::BitReader;
#[doc = "Field `IOSUSP` writer - SD I/O suspend command"]
pub type IOSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDCTRL")
            .field("cmdidx", &format_args!("{}", self.cmdidx().bits()))
            .field("rspwt", &format_args!("{}", self.rspwt().bits()))
            .field("intwt", &format_args!("{}", self.intwt().bit()))
            .field("pndwt", &format_args!("{}", self.pndwt().bit()))
            .field("ccsmen", &format_args!("{}", self.ccsmen().bit()))
            .field("iosusp", &format_args!("{}", self.iosusp().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CMDCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - CMDIDX"]
    #[inline(always)]
    #[must_use]
    pub fn cmdidx(&mut self) -> CMDIDX_W<CMDCTRL_SPEC> {
        CMDIDX_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - Wait for response"]
    #[inline(always)]
    #[must_use]
    pub fn rspwt(&mut self) -> RSPWT_W<CMDCTRL_SPEC> {
        RSPWT_W::new(self, 6)
    }
    #[doc = "Bit 8 - CCSM wait for interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn intwt(&mut self) -> INTWT_W<CMDCTRL_SPEC> {
        INTWT_W::new(self, 8)
    }
    #[doc = "Bit 9 - CCSM wait for end of transfer"]
    #[inline(always)]
    #[must_use]
    pub fn pndwt(&mut self) -> PNDWT_W<CMDCTRL_SPEC> {
        PNDWT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Command channel state machine"]
    #[inline(always)]
    #[must_use]
    pub fn ccsmen(&mut self) -> CCSMEN_W<CMDCTRL_SPEC> {
        CCSMEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - SD I/O suspend command"]
    #[inline(always)]
    #[must_use]
    pub fn iosusp(&mut self) -> IOSUSP_W<CMDCTRL_SPEC> {
        IOSUSP_W::new(self, 11)
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
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMDCTRL to value 0"]
impl crate::Resettable for CMDCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
