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
pub type SBSMSK_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
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
            .field("sbsmsk", &format_args!("{}", self.sbsmsk().bits()))
            .field("sbs", &format_args!("{}", self.sbs().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<ALASBS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
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
#[doc = "alarm A sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alasbs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alasbs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALASBS_SPEC;
impl crate::RegisterSpec for ALASBS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alasbs::R`](R) reader structure"]
impl crate::Readable for ALASBS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alasbs::W`](W) writer structure"]
impl crate::Writable for ALASBS_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALASBS to value 0"]
impl crate::Resettable for ALASBS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
