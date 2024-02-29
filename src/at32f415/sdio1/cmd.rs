#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `CMDIDX` reader - CMDIDX"]
pub type CMDIDX_R = crate::FieldReader;
#[doc = "Field `CMDIDX` writer - CMDIDX"]
pub type CMDIDX_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RSPWT` reader - WAITRESP"]
pub type RSPWT_R = crate::FieldReader;
#[doc = "Field `RSPWT` writer - WAITRESP"]
pub type RSPWT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTWT` reader - WAITINT"]
pub type INTWT_R = crate::BitReader;
#[doc = "Field `INTWT` writer - WAITINT"]
pub type INTWT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PNDWT` reader - WAITPEND"]
pub type PNDWT_R = crate::BitReader;
#[doc = "Field `PNDWT` writer - WAITPEND"]
pub type PNDWT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDMEN` reader - CPSMEN"]
pub type CMDMEN_R = crate::BitReader;
#[doc = "Field `CMDMEN` writer - CPSMEN"]
pub type CMDMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOSUSP` reader - SDIOSuspend"]
pub type SDIOSUSP_R = crate::BitReader;
#[doc = "Field `SDIOSUSP` writer - SDIOSuspend"]
pub type SDIOSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - CMDIDX"]
    #[inline(always)]
    pub fn cmdidx(&self) -> CMDIDX_R {
        CMDIDX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - WAITRESP"]
    #[inline(always)]
    pub fn rspwt(&self) -> RSPWT_R {
        RSPWT_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - WAITINT"]
    #[inline(always)]
    pub fn intwt(&self) -> INTWT_R {
        INTWT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WAITPEND"]
    #[inline(always)]
    pub fn pndwt(&self) -> PNDWT_R {
        PNDWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPSMEN"]
    #[inline(always)]
    pub fn cmdmen(&self) -> CMDMEN_R {
        CMDMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIOSuspend"]
    #[inline(always)]
    pub fn sdiosusp(&self) -> SDIOSUSP_R {
        SDIOSUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD")
            .field("cmdidx", &format_args!("{}", self.cmdidx().bits()))
            .field("rspwt", &format_args!("{}", self.rspwt().bits()))
            .field("intwt", &format_args!("{}", self.intwt().bit()))
            .field("pndwt", &format_args!("{}", self.pndwt().bit()))
            .field("cmdmen", &format_args!("{}", self.cmdmen().bit()))
            .field("sdiosusp", &format_args!("{}", self.sdiosusp().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - CMDIDX"]
    #[inline(always)]
    #[must_use]
    pub fn cmdidx(&mut self) -> CMDIDX_W<CMD_SPEC> {
        CMDIDX_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - WAITRESP"]
    #[inline(always)]
    #[must_use]
    pub fn rspwt(&mut self) -> RSPWT_W<CMD_SPEC> {
        RSPWT_W::new(self, 6)
    }
    #[doc = "Bit 8 - WAITINT"]
    #[inline(always)]
    #[must_use]
    pub fn intwt(&mut self) -> INTWT_W<CMD_SPEC> {
        INTWT_W::new(self, 8)
    }
    #[doc = "Bit 9 - WAITPEND"]
    #[inline(always)]
    #[must_use]
    pub fn pndwt(&mut self) -> PNDWT_W<CMD_SPEC> {
        PNDWT_W::new(self, 9)
    }
    #[doc = "Bit 10 - CPSMEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmdmen(&mut self) -> CMDMEN_W<CMD_SPEC> {
        CMDMEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - SDIOSuspend"]
    #[inline(always)]
    #[must_use]
    pub fn sdiosusp(&mut self) -> SDIOSUSP_W<CMD_SPEC> {
        SDIOSUSP_W::new(self, 11)
    }
}
#[doc = "SDIO command register (SDIO_CMD)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
