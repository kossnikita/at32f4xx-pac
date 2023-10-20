#[doc = "Register `BPDC` reader"]
pub type R = crate::R<BPDC_SPEC>;
#[doc = "Register `BPDC` writer"]
pub type W = crate::W<BPDC_SPEC>;
#[doc = "Field `LEXTEN` reader - Low speed external crystal enable"]
pub type LEXTEN_R = crate::BitReader;
#[doc = "Field `LEXTEN` writer - Low speed external crystal enable"]
pub type LEXTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEXTSTBL` reader - Low speed external crystal ready"]
pub type LEXTSTBL_R = crate::BitReader;
#[doc = "Field `LEXTBYPS` reader - Low speed external crystal bypass"]
pub type LEXTBYPS_R = crate::BitReader;
#[doc = "Field `LEXTBYPS` writer - Low speed external crystal bypass"]
pub type LEXTBYPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERTCSEL` reader - ERTC clock source selection"]
pub type ERTCSEL_R = crate::FieldReader;
#[doc = "Field `ERTCSEL` writer - ERTC clock source selection"]
pub type ERTCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ERTCEN` reader - ERTC clock enable"]
pub type ERTCEN_R = crate::BitReader;
#[doc = "Field `ERTCEN` writer - ERTC clock enable"]
pub type ERTCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BPDRST` reader - Battery powered domain software reset"]
pub type BPDRST_R = crate::BitReader;
#[doc = "Field `BPDRST` writer - Battery powered domain software reset"]
pub type BPDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low speed external crystal enable"]
    #[inline(always)]
    pub fn lexten(&self) -> LEXTEN_R {
        LEXTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low speed external crystal ready"]
    #[inline(always)]
    pub fn lextstbl(&self) -> LEXTSTBL_R {
        LEXTSTBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low speed external crystal bypass"]
    #[inline(always)]
    pub fn lextbyps(&self) -> LEXTBYPS_R {
        LEXTBYPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - ERTC clock source selection"]
    #[inline(always)]
    pub fn ertcsel(&self) -> ERTCSEL_R {
        ERTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - ERTC clock enable"]
    #[inline(always)]
    pub fn ertcen(&self) -> ERTCEN_R {
        ERTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Battery powered domain software reset"]
    #[inline(always)]
    pub fn bpdrst(&self) -> BPDRST_R {
        BPDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BPDC")
            .field("lexten", &format_args!("{}", self.lexten().bit()))
            .field("lextstbl", &format_args!("{}", self.lextstbl().bit()))
            .field("lextbyps", &format_args!("{}", self.lextbyps().bit()))
            .field("ertcsel", &format_args!("{}", self.ertcsel().bits()))
            .field("ertcen", &format_args!("{}", self.ertcen().bit()))
            .field("bpdrst", &format_args!("{}", self.bpdrst().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<BPDC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Low speed external crystal enable"]
    #[inline(always)]
    #[must_use]
    pub fn lexten(&mut self) -> LEXTEN_W<BPDC_SPEC> {
        LEXTEN_W::new(self, 0)
    }
    #[doc = "Bit 2 - Low speed external crystal bypass"]
    #[inline(always)]
    #[must_use]
    pub fn lextbyps(&mut self) -> LEXTBYPS_W<BPDC_SPEC> {
        LEXTBYPS_W::new(self, 2)
    }
    #[doc = "Bits 8:9 - ERTC clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ertcsel(&mut self) -> ERTCSEL_W<BPDC_SPEC> {
        ERTCSEL_W::new(self, 8)
    }
    #[doc = "Bit 15 - ERTC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ertcen(&mut self) -> ERTCEN_W<BPDC_SPEC> {
        ERTCEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Battery powered domain software reset"]
    #[inline(always)]
    #[must_use]
    pub fn bpdrst(&mut self) -> BPDRST_W<BPDC_SPEC> {
        BPDRST_W::new(self, 16)
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
#[doc = "Battery powered domain control register (CRM_BPDC)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpdc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpdc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BPDC_SPEC;
impl crate::RegisterSpec for BPDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bpdc::R`](R) reader structure"]
impl crate::Readable for BPDC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bpdc::W`](W) writer structure"]
impl crate::Writable for BPDC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BPDC to value 0"]
impl crate::Resettable for BPDC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
