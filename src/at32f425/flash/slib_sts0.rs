#[doc = "Register `SLIB_STS0` reader"]
pub type R = crate::R<SLIB_STS0_SPEC>;
#[doc = "Register `SLIB_STS0` writer"]
pub type W = crate::W<SLIB_STS0_SPEC>;
#[doc = "Field `BTM_AP_ENF` reader - Boot memory store application code enabled flag"]
pub type BTM_AP_ENF_R = crate::BitReader;
#[doc = "Field `BTM_AP_ENF` writer - Boot memory store application code enabled flag"]
pub type BTM_AP_ENF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM_SLIB_ENF` reader - Extension memory sLib enabled flag"]
pub type EM_SLIB_ENF_R = crate::BitReader;
#[doc = "Field `EM_SLIB_ENF` writer - Extension memory sLib enabled flag"]
pub type EM_SLIB_ENF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLIB_ENF` reader - sLib enabled flag"]
pub type SLIB_ENF_R = crate::BitReader;
#[doc = "Field `SLIB_ENF` writer - sLib enabled flag"]
pub type SLIB_ENF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EM_SLIB_INST_SS` reader - Extension memory sLib instruction start sector"]
pub type EM_SLIB_INST_SS_R = crate::FieldReader;
#[doc = "Field `EM_SLIB_INST_SS` writer - Extension memory sLib instruction start sector"]
pub type EM_SLIB_INST_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 0 - Boot memory store application code enabled flag"]
    #[inline(always)]
    pub fn btm_ap_enf(&self) -> BTM_AP_ENF_R {
        BTM_AP_ENF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Extension memory sLib enabled flag"]
    #[inline(always)]
    pub fn em_slib_enf(&self) -> EM_SLIB_ENF_R {
        EM_SLIB_ENF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sLib enabled flag"]
    #[inline(always)]
    pub fn slib_enf(&self) -> SLIB_ENF_R {
        SLIB_ENF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Extension memory sLib instruction start sector"]
    #[inline(always)]
    pub fn em_slib_inst_ss(&self) -> EM_SLIB_INST_SS_R {
        EM_SLIB_INST_SS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Boot memory store application code enabled flag"]
    #[inline(always)]
    #[must_use]
    pub fn btm_ap_enf(&mut self) -> BTM_AP_ENF_W<SLIB_STS0_SPEC, 0> {
        BTM_AP_ENF_W::new(self)
    }
    #[doc = "Bit 2 - Extension memory sLib enabled flag"]
    #[inline(always)]
    #[must_use]
    pub fn em_slib_enf(&mut self) -> EM_SLIB_ENF_W<SLIB_STS0_SPEC, 2> {
        EM_SLIB_ENF_W::new(self)
    }
    #[doc = "Bit 3 - sLib enabled flag"]
    #[inline(always)]
    #[must_use]
    pub fn slib_enf(&mut self) -> SLIB_ENF_W<SLIB_STS0_SPEC, 3> {
        SLIB_ENF_W::new(self)
    }
    #[doc = "Bits 16:23 - Extension memory sLib instruction start sector"]
    #[inline(always)]
    #[must_use]
    pub fn em_slib_inst_ss(&mut self) -> EM_SLIB_INST_SS_W<SLIB_STS0_SPEC, 16> {
        EM_SLIB_INST_SS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "sLib status 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_sts0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLIB_STS0_SPEC;
impl crate::RegisterSpec for SLIB_STS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slib_sts0::R`](R) reader structure"]
impl crate::Readable for SLIB_STS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slib_sts0::W`](W) writer structure"]
impl crate::Writable for SLIB_STS0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLIB_STS0 to value 0"]
impl crate::Resettable for SLIB_STS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
