#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `CAP` reader - Capture function enable"]
pub type CAP_R = crate::BitReader;
#[doc = "Field `CAP` writer - Capture function enable"]
pub type CAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFM` reader - Capture fire mode"]
pub type CFM_R = crate::BitReader;
#[doc = "Field `CFM` writer - Capture fire mode"]
pub type CFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRP` reader - Cropping function enable"]
pub type CRP_R = crate::BitReader;
#[doc = "Field `CRP` writer - Cropping function enable"]
pub type CRP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPEG` reader - JPEG format"]
pub type JPEG_R = crate::BitReader;
#[doc = "Field `JPEG` writer - JPEG format"]
pub type JPEG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SM` reader - synchronization mode"]
pub type SM_R = crate::BitReader;
#[doc = "Field `SM` writer - synchronization mode"]
pub type SM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKP` reader - Pixel clock polarity"]
pub type CKP_R = crate::BitReader;
#[doc = "Field `CKP` writer - Pixel clock polarity"]
pub type CKP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSP` reader - Horizontal synchronization polarity"]
pub type HSP_R = crate::BitReader;
#[doc = "Field `HSP` writer - Horizontal synchronization polarity"]
pub type HSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSP` reader - Vertical synchronization polarity"]
pub type VSP_R = crate::BitReader;
#[doc = "Field `VSP` writer - Vertical synchronization polarity"]
pub type VSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFRC` reader - Basic frame rate control"]
pub type BFRC_R = crate::FieldReader;
#[doc = "Field `BFRC` writer - Basic frame rate control"]
pub type BFRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PDL` reader - Pixel data length"]
pub type PDL_R = crate::FieldReader;
#[doc = "Field `PDL` writer - Pixel data length"]
pub type PDL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENA` reader - DVP enable"]
pub type ENA_R = crate::BitReader;
#[doc = "Field `ENA` writer - DVP enable"]
pub type ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCDC` reader - Basic pixel capture/drop control"]
pub type PCDC_R = crate::FieldReader;
#[doc = "Field `PCDC` writer - Basic pixel capture/drop control"]
pub type PCDC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCDS` reader - Pixel capture/drop selection"]
pub type PCDS_R = crate::BitReader;
#[doc = "Field `PCDS` writer - Pixel capture/drop selection"]
pub type PCDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDC` reader - Line capture/drop control"]
pub type LCDC_R = crate::BitReader;
#[doc = "Field `LCDC` writer - Line capture/drop control"]
pub type LCDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDS` reader - Line capture/drop selection"]
pub type LCDS_R = crate::BitReader;
#[doc = "Field `LCDS` writer - Line capture/drop selection"]
pub type LCDS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture function enable"]
    #[inline(always)]
    pub fn cap(&self) -> CAP_R {
        CAP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture fire mode"]
    #[inline(always)]
    pub fn cfm(&self) -> CFM_R {
        CFM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cropping function enable"]
    #[inline(always)]
    pub fn crp(&self) -> CRP_R {
        CRP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JPEG format"]
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - synchronization mode"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pixel clock polarity"]
    #[inline(always)]
    pub fn ckp(&self) -> CKP_R {
        CKP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Horizontal synchronization polarity"]
    #[inline(always)]
    pub fn hsp(&self) -> HSP_R {
        HSP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Vertical synchronization polarity"]
    #[inline(always)]
    pub fn vsp(&self) -> VSP_R {
        VSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Basic frame rate control"]
    #[inline(always)]
    pub fn bfrc(&self) -> BFRC_R {
        BFRC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pixel data length"]
    #[inline(always)]
    pub fn pdl(&self) -> PDL_R {
        PDL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - DVP enable"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Basic pixel capture/drop control"]
    #[inline(always)]
    pub fn pcdc(&self) -> PCDC_R {
        PCDC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Pixel capture/drop selection"]
    #[inline(always)]
    pub fn pcds(&self) -> PCDS_R {
        PCDS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Line capture/drop control"]
    #[inline(always)]
    pub fn lcdc(&self) -> LCDC_R {
        LCDC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Line capture/drop selection"]
    #[inline(always)]
    pub fn lcds(&self) -> LCDS_R {
        LCDS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("lcds", &self.lcds())
            .field("lcdc", &self.lcdc())
            .field("pcds", &self.pcds())
            .field("pcdc", &self.pcdc())
            .field("ena", &self.ena())
            .field("pdl", &self.pdl())
            .field("bfrc", &self.bfrc())
            .field("vsp", &self.vsp())
            .field("hsp", &self.hsp())
            .field("ckp", &self.ckp())
            .field("sm", &self.sm())
            .field("jpeg", &self.jpeg())
            .field("crp", &self.crp())
            .field("cfm", &self.cfm())
            .field("cap", &self.cap())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Capture function enable"]
    #[inline(always)]
    pub fn cap(&mut self) -> CAP_W<'_, CTRL_SPEC> {
        CAP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Capture fire mode"]
    #[inline(always)]
    pub fn cfm(&mut self) -> CFM_W<'_, CTRL_SPEC> {
        CFM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Cropping function enable"]
    #[inline(always)]
    pub fn crp(&mut self) -> CRP_W<'_, CTRL_SPEC> {
        CRP_W::new(self, 2)
    }
    #[doc = "Bit 3 - JPEG format"]
    #[inline(always)]
    pub fn jpeg(&mut self) -> JPEG_W<'_, CTRL_SPEC> {
        JPEG_W::new(self, 3)
    }
    #[doc = "Bit 4 - synchronization mode"]
    #[inline(always)]
    pub fn sm(&mut self) -> SM_W<'_, CTRL_SPEC> {
        SM_W::new(self, 4)
    }
    #[doc = "Bit 5 - Pixel clock polarity"]
    #[inline(always)]
    pub fn ckp(&mut self) -> CKP_W<'_, CTRL_SPEC> {
        CKP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Horizontal synchronization polarity"]
    #[inline(always)]
    pub fn hsp(&mut self) -> HSP_W<'_, CTRL_SPEC> {
        HSP_W::new(self, 6)
    }
    #[doc = "Bit 7 - Vertical synchronization polarity"]
    #[inline(always)]
    pub fn vsp(&mut self) -> VSP_W<'_, CTRL_SPEC> {
        VSP_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Basic frame rate control"]
    #[inline(always)]
    pub fn bfrc(&mut self) -> BFRC_W<'_, CTRL_SPEC> {
        BFRC_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Pixel data length"]
    #[inline(always)]
    pub fn pdl(&mut self) -> PDL_W<'_, CTRL_SPEC> {
        PDL_W::new(self, 10)
    }
    #[doc = "Bit 14 - DVP enable"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W<'_, CTRL_SPEC> {
        ENA_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Basic pixel capture/drop control"]
    #[inline(always)]
    pub fn pcdc(&mut self) -> PCDC_W<'_, CTRL_SPEC> {
        PCDC_W::new(self, 16)
    }
    #[doc = "Bit 18 - Pixel capture/drop selection"]
    #[inline(always)]
    pub fn pcds(&mut self) -> PCDS_W<'_, CTRL_SPEC> {
        PCDS_W::new(self, 18)
    }
    #[doc = "Bit 19 - Line capture/drop control"]
    #[inline(always)]
    pub fn lcdc(&mut self) -> LCDC_W<'_, CTRL_SPEC> {
        LCDC_W::new(self, 19)
    }
    #[doc = "Bit 20 - Line capture/drop selection"]
    #[inline(always)]
    pub fn lcds(&mut self) -> LCDS_W<'_, CTRL_SPEC> {
        LCDS_W::new(self, 20)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {}
