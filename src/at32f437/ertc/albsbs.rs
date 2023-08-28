#[doc = "Register `ALBSBS` reader"]
pub type R = crate::R<ALBSBS_SPEC>;
#[doc = "Register `ALBSBS` writer"]
pub type W = crate::W<ALBSBS_SPEC>;
#[doc = "Field `SBS` reader - Sub-seconds value"]
pub type SBS_R = crate::FieldReader<u16>;
#[doc = "Field `SBS` writer - Sub-seconds value"]
pub type SBS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `SBSMSK` reader - Sub-second mask"]
pub type SBSMSK_R = crate::FieldReader;
#[doc = "Field `SBSMSK` writer - Sub-second mask"]
pub type SBSMSK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
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
impl W {
    #[doc = "Bits 0:14 - Sub-seconds value"]
    #[inline(always)]
    #[must_use]
    pub fn sbs(&mut self) -> SBS_W<ALBSBS_SPEC, 0> {
        SBS_W::new(self)
    }
    #[doc = "Bits 24:27 - Sub-second mask"]
    #[inline(always)]
    #[must_use]
    pub fn sbsmsk(&mut self) -> SBSMSK_W<ALBSBS_SPEC, 24> {
        SBSMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "alarm B sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`albsbs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`albsbs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALBSBS_SPEC;
impl crate::RegisterSpec for ALBSBS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`albsbs::R`](R) reader structure"]
impl crate::Readable for ALBSBS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`albsbs::W`](W) writer structure"]
impl crate::Writable for ALBSBS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALBSBS to value 0"]
impl crate::Resettable for ALBSBS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
