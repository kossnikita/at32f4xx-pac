#[doc = "Register `ACTRL` reader"]
pub type R = crate::R<ACTRL_SPEC>;
#[doc = "Register `ACTRL` writer"]
pub type W = crate::W<ACTRL_SPEC>;
#[doc = "Field `EISRE` reader - Enhanced image scaling resize enable"]
pub type EISRE_R = crate::BitReader;
#[doc = "Field `EISRE` writer - Enhanced image scaling resize enable"]
pub type EISRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EFRCE` reader - Enhanced frame rate control enable"]
pub type EFRCE_R = crate::BitReader;
#[doc = "Field `EFRCE` writer - Enhanced frame rate control enable"]
pub type EFRCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MIBE` reader - Monochrome image binarization enable"]
pub type MIBE_R = crate::BitReader;
#[doc = "Field `MIBE` writer - Monochrome image binarization enable"]
pub type MIBE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCDES` reader - Basic pixel capture/drop extended selection"]
pub type PCDES_R = crate::BitReader;
#[doc = "Field `PCDES` writer - Basic pixel capture/drop extended selection"]
pub type PCDES_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EFDF` reader - Enhanced function data format"]
pub type EFDF_R = crate::FieldReader;
#[doc = "Field `EFDF` writer - Enhanced function data format"]
pub type EFDF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `EFDM` reader - Enhanced function data format management"]
pub type EFDM_R = crate::BitReader;
#[doc = "Field `EFDM` writer - Enhanced function data format management"]
pub type EFDM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IDUN` reader - Input data un-used number"]
pub type IDUN_R = crate::FieldReader;
#[doc = "Field `IDUN` writer - Input data un-used number"]
pub type IDUN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `IDUS` reader - Input data un-used setting"]
pub type IDUS_R = crate::BitReader;
#[doc = "Field `IDUS` writer - Input data un-used setting"]
pub type IDUS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMABT` reader - DMA burst transfer configuration"]
pub type DMABT_R = crate::BitReader;
#[doc = "Field `DMABT` writer - DMA burst transfer configuration"]
pub type DMABT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HSEID` reader - Horizontal synchonization event and interrupt definition"]
pub type HSEID_R = crate::BitReader;
#[doc = "Field `HSEID` writer - Horizontal synchonization event and interrupt definition"]
pub type HSEID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VSEID` reader - Vertical synchonization event and interrupt definition"]
pub type VSEID_R = crate::BitReader;
#[doc = "Field `VSEID` writer - Vertical synchonization event and interrupt definition"]
pub type VSEID_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enhanced image scaling resize enable"]
    #[inline(always)]
    pub fn eisre(&self) -> EISRE_R {
        EISRE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enhanced frame rate control enable"]
    #[inline(always)]
    pub fn efrce(&self) -> EFRCE_R {
        EFRCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Monochrome image binarization enable"]
    #[inline(always)]
    pub fn mibe(&self) -> MIBE_R {
        MIBE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Basic pixel capture/drop extended selection"]
    #[inline(always)]
    pub fn pcdes(&self) -> PCDES_R {
        PCDES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Enhanced function data format"]
    #[inline(always)]
    pub fn efdf(&self) -> EFDF_R {
        EFDF_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Enhanced function data format management"]
    #[inline(always)]
    pub fn efdm(&self) -> EFDM_R {
        EFDM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Input data un-used number"]
    #[inline(always)]
    pub fn idun(&self) -> IDUN_R {
        IDUN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Input data un-used setting"]
    #[inline(always)]
    pub fn idus(&self) -> IDUS_R {
        IDUS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA burst transfer configuration"]
    #[inline(always)]
    pub fn dmabt(&self) -> DMABT_R {
        DMABT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Horizontal synchonization event and interrupt definition"]
    #[inline(always)]
    pub fn hseid(&self) -> HSEID_R {
        HSEID_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Vertical synchonization event and interrupt definition"]
    #[inline(always)]
    pub fn vseid(&self) -> VSEID_R {
        VSEID_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACTRL")
            .field("vseid", &format_args!("{}", self.vseid().bit()))
            .field("hseid", &format_args!("{}", self.hseid().bit()))
            .field("dmabt", &format_args!("{}", self.dmabt().bit()))
            .field("idus", &format_args!("{}", self.idus().bit()))
            .field("idun", &format_args!("{}", self.idun().bits()))
            .field("efdm", &format_args!("{}", self.efdm().bit()))
            .field("efdf", &format_args!("{}", self.efdf().bits()))
            .field("pcdes", &format_args!("{}", self.pcdes().bit()))
            .field("mibe", &format_args!("{}", self.mibe().bit()))
            .field("efrce", &format_args!("{}", self.efrce().bit()))
            .field("eisre", &format_args!("{}", self.eisre().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<ACTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Enhanced image scaling resize enable"]
    #[inline(always)]
    #[must_use]
    pub fn eisre(&mut self) -> EISRE_W<ACTRL_SPEC, 0> {
        EISRE_W::new(self)
    }
    #[doc = "Bit 1 - Enhanced frame rate control enable"]
    #[inline(always)]
    #[must_use]
    pub fn efrce(&mut self) -> EFRCE_W<ACTRL_SPEC, 1> {
        EFRCE_W::new(self)
    }
    #[doc = "Bit 2 - Monochrome image binarization enable"]
    #[inline(always)]
    #[must_use]
    pub fn mibe(&mut self) -> MIBE_W<ACTRL_SPEC, 2> {
        MIBE_W::new(self)
    }
    #[doc = "Bit 3 - Basic pixel capture/drop extended selection"]
    #[inline(always)]
    #[must_use]
    pub fn pcdes(&mut self) -> PCDES_W<ACTRL_SPEC, 3> {
        PCDES_W::new(self)
    }
    #[doc = "Bits 4:5 - Enhanced function data format"]
    #[inline(always)]
    #[must_use]
    pub fn efdf(&mut self) -> EFDF_W<ACTRL_SPEC, 4> {
        EFDF_W::new(self)
    }
    #[doc = "Bit 6 - Enhanced function data format management"]
    #[inline(always)]
    #[must_use]
    pub fn efdm(&mut self) -> EFDM_W<ACTRL_SPEC, 6> {
        EFDM_W::new(self)
    }
    #[doc = "Bits 8:9 - Input data un-used number"]
    #[inline(always)]
    #[must_use]
    pub fn idun(&mut self) -> IDUN_W<ACTRL_SPEC, 8> {
        IDUN_W::new(self)
    }
    #[doc = "Bit 10 - Input data un-used setting"]
    #[inline(always)]
    #[must_use]
    pub fn idus(&mut self) -> IDUS_W<ACTRL_SPEC, 10> {
        IDUS_W::new(self)
    }
    #[doc = "Bit 12 - DMA burst transfer configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dmabt(&mut self) -> DMABT_W<ACTRL_SPEC, 12> {
        DMABT_W::new(self)
    }
    #[doc = "Bit 16 - Horizontal synchonization event and interrupt definition"]
    #[inline(always)]
    #[must_use]
    pub fn hseid(&mut self) -> HSEID_W<ACTRL_SPEC, 16> {
        HSEID_W::new(self)
    }
    #[doc = "Bit 17 - Vertical synchonization event and interrupt definition"]
    #[inline(always)]
    #[must_use]
    pub fn vseid(&mut self) -> VSEID_W<ACTRL_SPEC, 17> {
        VSEID_W::new(self)
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
#[doc = "Advanced Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACTRL_SPEC;
impl crate::RegisterSpec for ACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actrl::R`](R) reader structure"]
impl crate::Readable for ACTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`actrl::W`](W) writer structure"]
impl crate::Writable for ACTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACTRL to value 0"]
impl crate::Resettable for ACTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
