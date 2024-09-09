#[doc = "Register `SLIB_STS1` reader"]
pub type R = crate::R<SLIB_STS1_SPEC>;
#[doc = "Register `SLIB_STS1` writer"]
pub type W = crate::W<SLIB_STS1_SPEC>;
#[doc = "Field `SLIB_SS` reader - sLib start sector"]
pub type SLIB_SS_R = crate::FieldReader<u16>;
#[doc = "Field `SLIB_SS` writer - sLib start sector"]
pub type SLIB_SS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SLIB_INST_SS` reader - sLib instruction start sector"]
pub type SLIB_INST_SS_R = crate::FieldReader<u16>;
#[doc = "Field `SLIB_INST_SS` writer - sLib instruction start sector"]
pub type SLIB_INST_SS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SLIB_ES` reader - sLib end sector"]
pub type SLIB_ES_R = crate::FieldReader<u16>;
#[doc = "Field `SLIB_ES` writer - sLib end sector"]
pub type SLIB_ES_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:10 - sLib start sector"]
    #[inline(always)]
    pub fn slib_ss(&self) -> SLIB_SS_R {
        SLIB_SS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - sLib instruction start sector"]
    #[inline(always)]
    pub fn slib_inst_ss(&self) -> SLIB_INST_SS_R {
        SLIB_INST_SS_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 22:31 - sLib end sector"]
    #[inline(always)]
    pub fn slib_es(&self) -> SLIB_ES_R {
        SLIB_ES_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLIB_STS1")
            .field("slib_ss", &self.slib_ss())
            .field("slib_inst_ss", &self.slib_inst_ss())
            .field("slib_es", &self.slib_es())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - sLib start sector"]
    #[inline(always)]
    #[must_use]
    pub fn slib_ss(&mut self) -> SLIB_SS_W<SLIB_STS1_SPEC> {
        SLIB_SS_W::new(self, 0)
    }
    #[doc = "Bits 11:21 - sLib instruction start sector"]
    #[inline(always)]
    #[must_use]
    pub fn slib_inst_ss(&mut self) -> SLIB_INST_SS_W<SLIB_STS1_SPEC> {
        SLIB_INST_SS_W::new(self, 11)
    }
    #[doc = "Bits 22:31 - sLib end sector"]
    #[inline(always)]
    #[must_use]
    pub fn slib_es(&mut self) -> SLIB_ES_W<SLIB_STS1_SPEC> {
        SLIB_ES_W::new(self, 22)
    }
}
#[doc = "sLib status 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`slib_sts1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slib_sts1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLIB_STS1_SPEC;
impl crate::RegisterSpec for SLIB_STS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slib_sts1::R`](R) reader structure"]
impl crate::Readable for SLIB_STS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slib_sts1::W`](W) writer structure"]
impl crate::Writable for SLIB_STS1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLIB_STS1 to value 0"]
impl crate::Resettable for SLIB_STS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
