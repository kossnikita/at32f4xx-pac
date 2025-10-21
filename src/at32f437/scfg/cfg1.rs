#[doc = "Register `CFG1` reader"]
pub type R = crate::R<CFG1_SPEC>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<CFG1_SPEC>;
#[doc = "Field `MEM_MAP_SEL` reader - Memory address mapping selection bits"]
pub type MEM_MAP_SEL_R = crate::FieldReader;
#[doc = "Field `MEM_MAP_SEL` writer - Memory address mapping selection bits"]
pub type MEM_MAP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IR_POL` reader - IR output polarity selection"]
pub type IR_POL_R = crate::BitReader;
#[doc = "Field `IR_POL` writer - IR output polarity selection"]
pub type IR_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_SRC_SEL` reader - IR signal source selection"]
pub type IR_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `IR_SRC_SEL` writer - IR signal source selection"]
pub type IR_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWAP_XMC` reader - XMC address mapping swap"]
pub type SWAP_XMC_R = crate::FieldReader;
#[doc = "Field `SWAP_XMC` writer - XMC address mapping swap"]
pub type SWAP_XMC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Memory address mapping selection bits"]
    #[inline(always)]
    pub fn mem_map_sel(&self) -> MEM_MAP_SEL_R {
        MEM_MAP_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - IR output polarity selection"]
    #[inline(always)]
    pub fn ir_pol(&self) -> IR_POL_R {
        IR_POL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - IR signal source selection"]
    #[inline(always)]
    pub fn ir_src_sel(&self) -> IR_SRC_SEL_R {
        IR_SRC_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:11 - XMC address mapping swap"]
    #[inline(always)]
    pub fn swap_xmc(&self) -> SWAP_XMC_R {
        SWAP_XMC_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG1")
            .field("mem_map_sel", &self.mem_map_sel())
            .field("ir_pol", &self.ir_pol())
            .field("ir_src_sel", &self.ir_src_sel())
            .field("swap_xmc", &self.swap_xmc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Memory address mapping selection bits"]
    #[inline(always)]
    pub fn mem_map_sel(&mut self) -> MEM_MAP_SEL_W<'_, CFG1_SPEC> {
        MEM_MAP_SEL_W::new(self, 0)
    }
    #[doc = "Bit 5 - IR output polarity selection"]
    #[inline(always)]
    pub fn ir_pol(&mut self) -> IR_POL_W<'_, CFG1_SPEC> {
        IR_POL_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - IR signal source selection"]
    #[inline(always)]
    pub fn ir_src_sel(&mut self) -> IR_SRC_SEL_W<'_, CFG1_SPEC> {
        IR_SRC_SEL_W::new(self, 6)
    }
    #[doc = "Bits 10:11 - XMC address mapping swap"]
    #[inline(always)]
    pub fn swap_xmc(&mut self) -> SWAP_XMC_W<'_, CFG1_SPEC> {
        SWAP_XMC_W::new(self, 10)
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
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {}
