#[doc = "Register `DMABM` reader"]
pub type R = crate::R<DMABM_SPEC>;
#[doc = "Register `DMABM` writer"]
pub type W = crate::W<DMABM_SPEC>;
#[doc = "Field `SWR` reader - Software reset"]
pub type SWR_R = crate::BitReader;
#[doc = "Field `SWR` writer - Software reset"]
pub type SWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA` reader - DMA Arbitration"]
pub type DA_R = crate::BitReader;
#[doc = "Field `DA` writer - DMA Arbitration"]
pub type DA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSL` reader - Descriptor skip length"]
pub type DSL_R = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor skip length"]
pub type DSL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PBL` reader - Programmable burst length"]
pub type PBL_R = crate::FieldReader;
#[doc = "Field `PBL` writer - Programmable burst length"]
pub type PBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PR` reader - Priority ratio"]
pub type PR_R = crate::FieldReader;
#[doc = "Field `PR` writer - Priority ratio"]
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FB` reader - Fixed burst"]
pub type FB_R = crate::BitReader;
#[doc = "Field `FB` writer - Fixed burst"]
pub type FB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDP` reader - Rx DMA PBL"]
pub type RDP_R = crate::FieldReader;
#[doc = "Field `RDP` writer - Rx DMA PBL"]
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `USP` reader - Use separate PBL"]
pub type USP_R = crate::BitReader;
#[doc = "Field `USP` writer - Use separate PBL"]
pub type USP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBLx8` reader - PNLx8 mode"]
pub type PBLX8_R = crate::BitReader;
#[doc = "Field `PBLx8` writer - PNLx8 mode"]
pub type PBLX8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAB` reader - Address-aligned beats"]
pub type AAB_R = crate::BitReader;
#[doc = "Field `AAB` writer - Address-aligned beats"]
pub type AAB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Priority ratio"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use separate PBL"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PNLx8 mode"]
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Address-aligned beats"]
    #[inline(always)]
    pub fn aab(&self) -> AAB_R {
        AAB_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMABM")
            .field("swr", &format_args!("{}", self.swr().bit()))
            .field("da", &format_args!("{}", self.da().bit()))
            .field("dsl", &format_args!("{}", self.dsl().bits()))
            .field("pbl", &format_args!("{}", self.pbl().bits()))
            .field("pr", &format_args!("{}", self.pr().bits()))
            .field("fb", &format_args!("{}", self.fb().bit()))
            .field("rdp", &format_args!("{}", self.rdp().bits()))
            .field("usp", &format_args!("{}", self.usp().bit()))
            .field("pblx8", &format_args!("{}", self.pblx8().bit()))
            .field("aab", &format_args!("{}", self.aab().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DMABM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SWR_W<DMABM_SPEC> {
        SWR_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<DMABM_SPEC> {
        DA_W::new(self, 1)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DSL_W<DMABM_SPEC> {
        DSL_W::new(self, 2)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    #[must_use]
    pub fn pbl(&mut self) -> PBL_W<DMABM_SPEC> {
        PBL_W::new(self, 8)
    }
    #[doc = "Bits 14:15 - Priority ratio"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<DMABM_SPEC> {
        PR_W::new(self, 14)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<DMABM_SPEC> {
        FB_W::new(self, 16)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<DMABM_SPEC> {
        RDP_W::new(self, 17)
    }
    #[doc = "Bit 23 - Use separate PBL"]
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> USP_W<DMABM_SPEC> {
        USP_W::new(self, 23)
    }
    #[doc = "Bit 24 - PNLx8 mode"]
    #[inline(always)]
    #[must_use]
    pub fn pblx8(&mut self) -> PBLX8_W<DMABM_SPEC> {
        PBLX8_W::new(self, 24)
    }
    #[doc = "Bit 25 - Address-aligned beats"]
    #[inline(always)]
    #[must_use]
    pub fn aab(&mut self) -> AAB_W<DMABM_SPEC> {
        AAB_W::new(self, 25)
    }
}
#[doc = "Ethernet DMA bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmabm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmabm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMABM_SPEC;
impl crate::RegisterSpec for DMABM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmabm::R`](R) reader structure"]
impl crate::Readable for DMABM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmabm::W`](W) writer structure"]
impl crate::Writable for DMABM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMABM to value 0x0002_0101"]
impl crate::Resettable for DMABM_SPEC {
    const RESET_VALUE: u32 = 0x0002_0101;
}
