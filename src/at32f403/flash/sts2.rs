#[doc = "Register `STS2` reader"]
pub type R = crate::R<STS2_SPEC>;
#[doc = "Register `STS2` writer"]
pub type W = crate::W<STS2_SPEC>;
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
        f.debug_struct("STS2")
            .field("obf", &format_args!("{}", self.obf().bit()))
            .field("prgmerr", &format_args!("{}", self.prgmerr().bit()))
            .field("epperr", &format_args!("{}", self.epperr().bit()))
            .field("odf", &format_args!("{}", self.odf().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STS2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 2 - program error"]
    #[inline(always)]
    #[must_use]
    pub fn prgmerr(&mut self) -> PRGMERR_W<STS2_SPEC> {
        PRGMERR_W::new(self, 2)
    }
    #[doc = "Bit 4 - Erase/program protection error"]
    #[inline(always)]
    #[must_use]
    pub fn epperr(&mut self) -> EPPERR_W<STS2_SPEC> {
        EPPERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Operate done flag"]
    #[inline(always)]
    #[must_use]
    pub fn odf(&mut self) -> ODF_W<STS2_SPEC> {
        ODF_W::new(self, 5)
    }
}
#[doc = "Status 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS2_SPEC;
impl crate::RegisterSpec for STS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts2::R`](R) reader structure"]
impl crate::Readable for STS2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts2::W`](W) writer structure"]
impl crate::Writable for STS2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS2 to value 0"]
impl crate::Resettable for STS2_SPEC {
    const RESET_VALUE: u32 = 0;
}
