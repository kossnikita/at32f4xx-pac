#[doc = "Register `STS3` reader"]
pub type R = crate::R<STS3_SPEC>;
#[doc = "Register `STS3` writer"]
pub type W = crate::W<STS3_SPEC>;
#[doc = "Field `OBF` reader - Operate busy flag"]
pub type OBF_R = crate::BitReader;
#[doc = "Field `PRGMERR` reader - program error"]
pub type PRGMERR_R = crate::BitReader;
#[doc = "Field `PRGMERR` writer - program error"]
pub type PRGMERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPPERR` reader - Erase/program protection error"]
pub type EPPERR_R = crate::BitReader;
#[doc = "Field `EPPERR` writer - Erase/program protection error"]
pub type EPPERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ODF` reader - Operate done flag"]
pub type ODF_R = crate::BitReader;
#[doc = "Field `ODF` writer - Operate done flag"]
pub type ODF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
impl W {
    #[doc = "Bit 2 - program error"]
    #[inline(always)]
    #[must_use]
    pub fn prgmerr(&mut self) -> PRGMERR_W<STS3_SPEC, 2> {
        PRGMERR_W::new(self)
    }
    #[doc = "Bit 4 - Erase/program protection error"]
    #[inline(always)]
    #[must_use]
    pub fn epperr(&mut self) -> EPPERR_W<STS3_SPEC, 4> {
        EPPERR_W::new(self)
    }
    #[doc = "Bit 5 - Operate done flag"]
    #[inline(always)]
    #[must_use]
    pub fn odf(&mut self) -> ODF_W<STS3_SPEC, 5> {
        ODF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Status 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS3_SPEC;
impl crate::RegisterSpec for STS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts3::R`](R) reader structure"]
impl crate::Readable for STS3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts3::W`](W) writer structure"]
impl crate::Writable for STS3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STS3 to value 0"]
impl crate::Resettable for STS3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
