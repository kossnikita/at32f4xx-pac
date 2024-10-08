#[doc = "Register `CFG1` reader"]
pub type R = crate::R<CFG1_SPEC>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<CFG1_SPEC>;
#[doc = "Field `MEM_MAP_SEL` reader - Memory mapping selection bits"]
pub type MEM_MAP_SEL_R = crate::FieldReader;
#[doc = "Field `MEM_MAP_SEL` writer - Memory mapping selection bits"]
pub type MEM_MAP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA11_12_RMP` reader - PA11/PA12 pins remapping bit"]
pub type PA11_12_RMP_R = crate::BitReader;
#[doc = "Field `PA11_12_RMP` writer - PA11/PA12 pins remapping bit"]
pub type PA11_12_RMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_POL` reader - Infrared output polarity selection"]
pub type IR_POL_R = crate::BitReader;
#[doc = "Field `IR_POL` writer - Infrared output polarity selection"]
pub type IR_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_SRC_SEL` reader - Infrared modulation envelope signal source selection"]
pub type IR_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `IR_SRC_SEL` writer - Infrared modulation envelope signal source selection"]
pub type IR_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PB14_UH` reader - PB14 Ultra high sourcing/sinking strength"]
pub type PB14_UH_R = crate::BitReader;
#[doc = "Field `PB14_UH` writer - PB14 Ultra high sourcing/sinking strength"]
pub type PB14_UH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB13_UH` reader - PB13 Ultra high sourcing/sinking strength"]
pub type PB13_UH_R = crate::BitReader;
#[doc = "Field `PB13_UH` writer - PB13 Ultra high sourcing/sinking strength"]
pub type PB13_UH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB9_UH` reader - PB9 Ultra high sourcing/sinking strength"]
pub type PB9_UH_R = crate::BitReader;
#[doc = "Field `PB9_UH` writer - PB9 Ultra high sourcing/sinking strength"]
pub type PB9_UH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB8_UH` reader - PB8 Ultra high sourcing/sinking strength"]
pub type PB8_UH_R = crate::BitReader;
#[doc = "Field `PB8_UH` writer - PB8 Ultra high sourcing/sinking strength"]
pub type PB8_UH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_map_sel(&self) -> MEM_MAP_SEL_R {
        MEM_MAP_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - PA11/PA12 pins remapping bit"]
    #[inline(always)]
    pub fn pa11_12_rmp(&self) -> PA11_12_RMP_R {
        PA11_12_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Infrared output polarity selection"]
    #[inline(always)]
    pub fn ir_pol(&self) -> IR_POL_R {
        IR_POL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Infrared modulation envelope signal source selection"]
    #[inline(always)]
    pub fn ir_src_sel(&self) -> IR_SRC_SEL_R {
        IR_SRC_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 16 - PB14 Ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pb14_uh(&self) -> PB14_UH_R {
        PB14_UH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PB13 Ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pb13_uh(&self) -> PB13_UH_R {
        PB13_UH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PB9 Ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pb9_uh(&self) -> PB9_UH_R {
        PB9_UH_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PB8 Ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pb8_uh(&self) -> PB8_UH_R {
        PB8_UH_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG1")
            .field("mem_map_sel", &self.mem_map_sel())
            .field("pa11_12_rmp", &self.pa11_12_rmp())
            .field("ir_pol", &self.ir_pol())
            .field("ir_src_sel", &self.ir_src_sel())
            .field("pb14_uh", &self.pb14_uh())
            .field("pb13_uh", &self.pb13_uh())
            .field("pb9_uh", &self.pb9_uh())
            .field("pb8_uh", &self.pb8_uh())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn mem_map_sel(&mut self) -> MEM_MAP_SEL_W<CFG1_SPEC> {
        MEM_MAP_SEL_W::new(self, 0)
    }
    #[doc = "Bit 4 - PA11/PA12 pins remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn pa11_12_rmp(&mut self) -> PA11_12_RMP_W<CFG1_SPEC> {
        PA11_12_RMP_W::new(self, 4)
    }
    #[doc = "Bit 5 - Infrared output polarity selection"]
    #[inline(always)]
    #[must_use]
    pub fn ir_pol(&mut self) -> IR_POL_W<CFG1_SPEC> {
        IR_POL_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Infrared modulation envelope signal source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ir_src_sel(&mut self) -> IR_SRC_SEL_W<CFG1_SPEC> {
        IR_SRC_SEL_W::new(self, 6)
    }
    #[doc = "Bit 16 - PB14 Ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pb14_uh(&mut self) -> PB14_UH_W<CFG1_SPEC> {
        PB14_UH_W::new(self, 16)
    }
    #[doc = "Bit 17 - PB13 Ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pb13_uh(&mut self) -> PB13_UH_W<CFG1_SPEC> {
        PB13_UH_W::new(self, 17)
    }
    #[doc = "Bit 18 - PB9 Ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pb9_uh(&mut self) -> PB9_UH_W<CFG1_SPEC> {
        PB9_UH_W::new(self, 18)
    }
    #[doc = "Bit 19 - PB8 Ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pb8_uh(&mut self) -> PB8_UH_W<CFG1_SPEC> {
        PB8_UH_W::new(self, 19)
    }
}
#[doc = "configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
