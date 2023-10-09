#[doc = "Register `DTCTRL` reader"]
pub type R = crate::R<DTCTRL_SPEC>;
#[doc = "Register `DTCTRL` writer"]
pub type W = crate::W<DTCTRL_SPEC>;
#[doc = "Field `TFREN` reader - DTEN"]
pub type TFREN_R = crate::BitReader;
#[doc = "Field `TFREN` writer - DTEN"]
pub type TFREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFRDIR` reader - DTDIR"]
pub type TFRDIR_R = crate::BitReader;
#[doc = "Field `TFRDIR` writer - DTDIR"]
pub type TFRDIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TFRMODE` reader - DTMODE"]
pub type TFRMODE_R = crate::BitReader;
#[doc = "Field `TFRMODE` writer - DTMODE"]
pub type TFRMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BLKSIZE` reader - DBLOCKSIZE"]
pub type BLKSIZE_R = crate::FieldReader;
#[doc = "Field `BLKSIZE` writer - DBLOCKSIZE"]
pub type BLKSIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `RDWTSTART` reader - PWSTART"]
pub type RDWTSTART_R = crate::BitReader;
#[doc = "Field `RDWTSTART` writer - PWSTART"]
pub type RDWTSTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDWTSTOP` reader - PWSTOP"]
pub type RDWTSTOP_R = crate::BitReader;
#[doc = "Field `RDWTSTOP` writer - PWSTOP"]
pub type RDWTSTOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDWTMODE` reader - RWMOD"]
pub type RDWTMODE_R = crate::BitReader;
#[doc = "Field `RDWTMODE` writer - RWMOD"]
pub type RDWTMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SDIOEN` reader - SDIOEN"]
pub type SDIOEN_R = crate::BitReader;
#[doc = "Field `SDIOEN` writer - SDIOEN"]
pub type SDIOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 11 - SDIOEN"]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTCTRL")
            .field("tfren", &format_args!("{}", self.tfren().bit()))
            .field("tfrdir", &format_args!("{}", self.tfrdir().bit()))
            .field("tfrmode", &format_args!("{}", self.tfrmode().bit()))
            .field("dmaen", &format_args!("{}", self.dmaen().bit()))
            .field("blksize", &format_args!("{}", self.blksize().bits()))
            .field("rdwtstart", &format_args!("{}", self.rdwtstart().bit()))
            .field("rdwtstop", &format_args!("{}", self.rdwtstop().bit()))
            .field("rdwtmode", &format_args!("{}", self.rdwtmode().bit()))
            .field("sdioen", &format_args!("{}", self.sdioen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DTCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    #[must_use]
    pub fn tfren(&mut self) -> TFREN_W<DTCTRL_SPEC, 0> {
        TFREN_W::new(self)
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    #[must_use]
    pub fn tfrdir(&mut self) -> TFRDIR_W<DTCTRL_SPEC, 1> {
        TFRDIR_W::new(self)
    }
    #[doc = "Bit 2 - DTMODE"]
    #[inline(always)]
    #[must_use]
    pub fn tfrmode(&mut self) -> TFRMODE_W<DTCTRL_SPEC, 2> {
        TFRMODE_W::new(self)
    }
    #[doc = "Bit 3 - DMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<DTCTRL_SPEC, 3> {
        DMAEN_W::new(self)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn blksize(&mut self) -> BLKSIZE_W<DTCTRL_SPEC, 4> {
        BLKSIZE_W::new(self)
    }
    #[doc = "Bit 8 - PWSTART"]
    #[inline(always)]
    #[must_use]
    pub fn rdwtstart(&mut self) -> RDWTSTART_W<DTCTRL_SPEC, 8> {
        RDWTSTART_W::new(self)
    }
    #[doc = "Bit 9 - PWSTOP"]
    #[inline(always)]
    #[must_use]
    pub fn rdwtstop(&mut self) -> RDWTSTOP_W<DTCTRL_SPEC, 9> {
        RDWTSTOP_W::new(self)
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    #[must_use]
    pub fn rdwtmode(&mut self) -> RDWTMODE_W<DTCTRL_SPEC, 10> {
        RDWTMODE_W::new(self)
    }
    #[doc = "Bit 11 - SDIOEN"]
    #[inline(always)]
    #[must_use]
    pub fn sdioen(&mut self) -> SDIOEN_W<DTCTRL_SPEC, 11> {
        SDIOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SDIO data control register (SDIO_DCTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTCTRL_SPEC;
impl crate::RegisterSpec for DTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtctrl::R`](R) reader structure"]
impl crate::Readable for DTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtctrl::W`](W) writer structure"]
impl crate::Writable for DTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTCTRL to value 0"]
impl crate::Resettable for DTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
