#[doc = "Register `ALASBS` reader"]
pub type R = crate::R<ALASBS_SPEC>;
#[doc = "Register `ALASBS` writer"]
pub type W = crate::W<ALASBS_SPEC>;
#[doc = "Field `SBS` reader - Sub-seconds value"]
pub type SBS_R = crate::FieldReader<u16>;
#[doc = "Field `SBS` writer - Sub-seconds value"]
pub type SBS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `SBSMSK` reader - Sub-second mask"]
pub type SBSMSK_R = crate::FieldReader;
#[doc = "Field `SBSMSK` writer - Sub-second mask"]
pub type SBSMSK_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:14 - Sub-seconds value"]
    #[inline(always)]
    pub fn sbs(&self) -> SBS_R {
        SBS_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - Sub-second mask"]
    #[inline(always)]
    pub fn sbsmsk(&self) -> SBSMSK_R {
        SBSMSK_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALASBS")
            .field("sbsmsk", &self.sbsmsk())
            .field("sbs", &self.sbs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - Sub-seconds value"]
    #[inline(always)]
    #[must_use]
    pub fn sbs(&mut self) -> SBS_W<ALASBS_SPEC> {
        SBS_W::new(self, 0)
    }
    #[doc = "Bits 24:27 - Sub-second mask"]
    #[inline(always)]
    #[must_use]
    pub fn sbsmsk(&mut self) -> SBSMSK_W<ALASBS_SPEC> {
        SBSMSK_W::new(self, 24)
    }
}
#[doc = "alarm A sub second register\n\nYou can [`read`](crate::Reg::read) this register and get [`alasbs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alasbs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALASBS_SPEC;
impl crate::RegisterSpec for ALASBS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alasbs::R`](R) reader structure"]
impl crate::Readable for ALASBS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alasbs::W`](W) writer structure"]
impl crate::Writable for ALASBS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALASBS to value 0"]
impl crate::Resettable for ALASBS_SPEC {
    const RESET_VALUE: u32 = 0;
}
