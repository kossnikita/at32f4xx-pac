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
            .field("cmdidx", &self.cmdidx())
            .field("rspwt", &self.rspwt())
            .field("intwt", &self.intwt())
            .field("pndwt", &self.pndwt())
            .field("cmdmen", &self.cmdmen())
            .field("sdiosusp", &self.sdiosusp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - CMDIDX"]
    #[inline(always)]
    pub fn cmdidx(&mut self) -> CMDIDX_W<'_, CMD_SPEC> {
        CMDIDX_W::new(self, 0)
    }
    #[doc = "Bits 6:7 - WAITRESP"]
    #[inline(always)]
    pub fn rspwt(&mut self) -> RSPWT_W<'_, CMD_SPEC> {
        RSPWT_W::new(self, 6)
    }
    #[doc = "Bit 8 - WAITINT"]
    #[inline(always)]
    pub fn intwt(&mut self) -> INTWT_W<'_, CMD_SPEC> {
        INTWT_W::new(self, 8)
    }
    #[doc = "Bit 9 - WAITPEND"]
    #[inline(always)]
    pub fn pndwt(&mut self) -> PNDWT_W<'_, CMD_SPEC> {
        PNDWT_W::new(self, 9)
    }
    #[doc = "Bit 10 - CPSMEN"]
    #[inline(always)]
    pub fn cmdmen(&mut self) -> CMDMEN_W<'_, CMD_SPEC> {
        CMDMEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - SDIOSuspend"]
    #[inline(always)]
    pub fn sdiosusp(&mut self) -> SDIOSUSP_W<'_, CMD_SPEC> {
        SDIOSUSP_W::new(self, 11)
    }
}
#[doc = "SDIO command register (SDIO_CMD)\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {}
