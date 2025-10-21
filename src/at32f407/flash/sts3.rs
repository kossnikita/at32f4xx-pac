#[doc = "Register `STS3` reader"]
pub type R = crate::R<STS3_SPEC>;
#[doc = "Register `STS3` writer"]
pub type W = crate::W<STS3_SPEC>;
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
        f.debug_struct("STS3")
            .field("obf", &self.obf())
            .field("prgmerr", &self.prgmerr())
            .field("epperr", &self.epperr())
            .field("odf", &self.odf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - program error"]
    #[inline(always)]
    pub fn prgmerr(&mut self) -> PRGMERR_W<'_, STS3_SPEC> {
        PRGMERR_W::new(self, 2)
    }
    #[doc = "Bit 4 - Erase/program protection error"]
    #[inline(always)]
    pub fn epperr(&mut self) -> EPPERR_W<'_, STS3_SPEC> {
        EPPERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Operate done flag"]
    #[inline(always)]
    pub fn odf(&mut self) -> ODF_W<'_, STS3_SPEC> {
        ODF_W::new(self, 5)
    }
}
#[doc = "Status 3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS3_SPEC;
impl crate::RegisterSpec for STS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts3::R`](R) reader structure"]
impl crate::Readable for STS3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts3::W`](W) writer structure"]
impl crate::Writable for STS3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STS3 to value 0"]
impl crate::Resettable for STS3_SPEC {}
