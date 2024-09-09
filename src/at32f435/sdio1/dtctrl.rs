#[doc = "Register `DTCTRL` reader"]
pub type R = crate::R<DTCTRL_SPEC>;
#[doc = "Register `DTCTRL` writer"]
pub type W = crate::W<DTCTRL_SPEC>;
#[doc = "Field `TFREN` reader - DTEN"]
pub type TFREN_R = crate::BitReader;
#[doc = "Field `TFREN` writer - DTEN"]
pub type TFREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFRDIR` reader - DTDIR"]
pub type TFRDIR_R = crate::BitReader;
#[doc = "Field `TFRDIR` writer - DTDIR"]
pub type TFRDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFRMODE` reader - DTMODE"]
pub type TFRMODE_R = crate::BitReader;
#[doc = "Field `TFRMODE` writer - DTMODE"]
pub type TFRMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKSIZE` reader - DBLOCKSIZE"]
pub type BLKSIZE_R = crate::FieldReader;
#[doc = "Field `BLKSIZE` writer - DBLOCKSIZE"]
pub type BLKSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RDWTSTART` reader - PWSTART"]
pub type RDWTSTART_R = crate::BitReader;
#[doc = "Field `RDWTSTART` writer - PWSTART"]
pub type RDWTSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDWTSTOP` reader - PWSTOP"]
pub type RDWTSTOP_R = crate::BitReader;
#[doc = "Field `RDWTSTOP` writer - PWSTOP"]
pub type RDWTSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDWTMODE` reader - RWMOD"]
pub type RDWTMODE_R = crate::BitReader;
#[doc = "Field `RDWTMODE` writer - RWMOD"]
pub type RDWTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOEN` reader - SD I/O function enable"]
pub type IOEN_R = crate::BitReader;
#[doc = "Field `IOEN` writer - SD I/O function enable"]
pub type IOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    pub fn tfren(&self) -> TFREN_R {
        TFREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    pub fn tfrdir(&self) -> TFRDIR_R {
        TFRDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DTMODE"]
    #[inline(always)]
    pub fn tfrmode(&self) -> TFRMODE_R {
        TFRMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    pub fn blksize(&self) -> BLKSIZE_R {
        BLKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - PWSTART"]
    #[inline(always)]
    pub fn rdwtstart(&self) -> RDWTSTART_R {
        RDWTSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PWSTOP"]
    #[inline(always)]
    pub fn rdwtstop(&self) -> RDWTSTOP_R {
        RDWTSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    pub fn rdwtmode(&self) -> RDWTMODE_R {
        RDWTMODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O function enable"]
    #[inline(always)]
    pub fn ioen(&self) -> IOEN_R {
        IOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTCTRL")
            .field("tfren", &self.tfren())
            .field("tfrdir", &self.tfrdir())
            .field("tfrmode", &self.tfrmode())
            .field("dmaen", &self.dmaen())
            .field("blksize", &self.blksize())
            .field("rdwtstart", &self.rdwtstart())
            .field("rdwtstop", &self.rdwtstop())
            .field("rdwtmode", &self.rdwtmode())
            .field("ioen", &self.ioen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    #[must_use]
    pub fn tfren(&mut self) -> TFREN_W<DTCTRL_SPEC> {
        TFREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    #[must_use]
    pub fn tfrdir(&mut self) -> TFRDIR_W<DTCTRL_SPEC> {
        TFRDIR_W::new(self, 1)
    }
    #[doc = "Bit 2 - DTMODE"]
    #[inline(always)]
    #[must_use]
    pub fn tfrmode(&mut self) -> TFRMODE_W<DTCTRL_SPEC> {
        TFRMODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - DMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<DTCTRL_SPEC> {
        DMAEN_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn blksize(&mut self) -> BLKSIZE_W<DTCTRL_SPEC> {
        BLKSIZE_W::new(self, 4)
    }
    #[doc = "Bit 8 - PWSTART"]
    #[inline(always)]
    #[must_use]
    pub fn rdwtstart(&mut self) -> RDWTSTART_W<DTCTRL_SPEC> {
        RDWTSTART_W::new(self, 8)
    }
    #[doc = "Bit 9 - PWSTOP"]
    #[inline(always)]
    #[must_use]
    pub fn rdwtstop(&mut self) -> RDWTSTOP_W<DTCTRL_SPEC> {
        RDWTSTOP_W::new(self, 9)
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    #[must_use]
    pub fn rdwtmode(&mut self) -> RDWTMODE_W<DTCTRL_SPEC> {
        RDWTMODE_W::new(self, 10)
    }
    #[doc = "Bit 11 - SD I/O function enable"]
    #[inline(always)]
    #[must_use]
    pub fn ioen(&mut self) -> IOEN_W<DTCTRL_SPEC> {
        IOEN_W::new(self, 11)
    }
}
#[doc = "SDIO data control register (SDIO_DCTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`dtctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTCTRL_SPEC;
impl crate::RegisterSpec for DTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtctrl::R`](R) reader structure"]
impl crate::Readable for DTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtctrl::W`](W) writer structure"]
impl crate::Writable for DTCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTCTRL to value 0"]
impl crate::Resettable for DTCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
