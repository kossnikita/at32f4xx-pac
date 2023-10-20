#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Field `OBF` reader - Operate busy flag"]
pub type OBF_R = crate::BitReader;
#[doc = "Field `PRGMERR` reader - program error"]
pub type PRGMERR_R = crate::BitReader;
#[doc = "Field `PRGMERR` writer - program error"]
pub type PRGMERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPPERR` reader - Erase/program protection error"]
pub type EPPERR_R = crate::BitReader;
#[doc = "Field `EPPERR` writer - Erase/program protection error"]
pub type EPPERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODF` reader - Operate done flag"]
pub type ODF_R = crate::BitReader;
#[doc = "Field `ODF` writer - Operate done flag"]
pub type ODF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Operate busy flag"]
    #[inline(always)]
    pub fn obf(&self) -> OBF_R {
        OBF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - program error"]
    #[inline(always)]
    pub fn prgmerr(&self) -> PRGMERR_R {
        PRGMERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Erase/program protection error"]
    #[inline(always)]
    pub fn epperr(&self) -> EPPERR_R {
        EPPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Operate done flag"]
    #[inline(always)]
    pub fn odf(&self) -> ODF_R {
        ODF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("odf", &format_args!("{}", self.odf().bit()))
            .field("epperr", &format_args!("{}", self.epperr().bit()))
            .field("prgmerr", &format_args!("{}", self.prgmerr().bit()))
            .field("obf", &format_args!("{}", self.obf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2 - program error"]
    #[inline(always)]
    #[must_use]
    pub fn prgmerr(&mut self) -> PRGMERR_W<STS_SPEC> {
        PRGMERR_W::new(self, 2)
    }
    #[doc = "Bit 4 - Erase/program protection error"]
    #[inline(always)]
    #[must_use]
    pub fn epperr(&mut self) -> EPPERR_W<STS_SPEC> {
        EPPERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Operate done flag"]
    #[inline(always)]
    #[must_use]
    pub fn odf(&mut self) -> ODF_W<STS_SPEC> {
        ODF_W::new(self, 5)
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
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
