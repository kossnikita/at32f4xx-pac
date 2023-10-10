#[doc = "Register `GAHBCFG` reader"]
pub type R = crate::R<GAHBCFG_SPEC>;
#[doc = "Register `GAHBCFG` writer"]
pub type W = crate::W<GAHBCFG_SPEC>;
#[doc = "Field `GLBINTMSK` reader - Global interrupt mask"]
pub type GLBINTMSK_R = crate::BitReader;
#[doc = "Field `GLBINTMSK` writer - Global interrupt mask"]
pub type GLBINTMSK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HBSTLEN` reader - Burst Length"]
pub type HBSTLEN_R = crate::FieldReader;
#[doc = "Field `HBSTLEN` writer - Burst Length"]
pub type HBSTLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NPTXFEMPLVL` reader - Non-Periodic TxFIFO empty level"]
pub type NPTXFEMPLVL_R = crate::BitReader;
#[doc = "Field `NPTXFEMPLVL` writer - Non-Periodic TxFIFO empty level"]
pub type NPTXFEMPLVL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTXFEMPLVL` reader - Periodic TxFIFO empty level"]
pub type PTXFEMPLVL_R = crate::BitReader;
#[doc = "Field `PTXFEMPLVL` writer - Periodic TxFIFO empty level"]
pub type PTXFEMPLVL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    pub fn glbintmsk(&self) -> GLBINTMSK_R {
        GLBINTMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Burst Length"]
    #[inline(always)]
    pub fn hbstlen(&self) -> HBSTLEN_R {
        HBSTLEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO empty level"]
    #[inline(always)]
    pub fn nptxfemplvl(&self) -> NPTXFEMPLVL_R {
        NPTXFEMPLVL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    pub fn ptxfemplvl(&self) -> PTXFEMPLVL_R {
        PTXFEMPLVL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAHBCFG")
            .field("glbintmsk", &format_args!("{}", self.glbintmsk().bit()))
            .field("hbstlen", &format_args!("{}", self.hbstlen().bits()))
            .field("dmaen", &format_args!("{}", self.dmaen().bit()))
            .field("nptxfemplvl", &format_args!("{}", self.nptxfemplvl().bit()))
            .field("ptxfemplvl", &format_args!("{}", self.ptxfemplvl().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GAHBCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn glbintmsk(&mut self) -> GLBINTMSK_W<GAHBCFG_SPEC, 0> {
        GLBINTMSK_W::new(self)
    }
    #[doc = "Bits 1:4 - Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn hbstlen(&mut self) -> HBSTLEN_W<GAHBCFG_SPEC, 1> {
        HBSTLEN_W::new(self)
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<GAHBCFG_SPEC, 5> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO empty level"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfemplvl(&mut self) -> NPTXFEMPLVL_W<GAHBCFG_SPEC, 7> {
        NPTXFEMPLVL_W::new(self)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfemplvl(&mut self) -> PTXFEMPLVL_W<GAHBCFG_SPEC, 8> {
        PTXFEMPLVL_W::new(self)
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
#[doc = "OTGHS AHB configuration register (OTGHS_GAHBCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAHBCFG_SPEC;
impl crate::RegisterSpec for GAHBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gahbcfg::R`](R) reader structure"]
impl crate::Readable for GAHBCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gahbcfg::W`](W) writer structure"]
impl crate::Writable for GAHBCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GAHBCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
