#[doc = "Register `STCTRL` reader"]
pub type R = crate::R<STCTRL_SPEC>;
#[doc = "Register `STCTRL` writer"]
pub type W = crate::W<STCTRL_SPEC>;
#[doc = "Field `SMSEL` reader - Subordinate TMR mode selection"]
pub type SMSEL_R = crate::FieldReader;
#[doc = "Field `SMSEL` writer - Subordinate TMR mode selection"]
pub type SMSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `STIS` reader - Subordinate TMR input selection"]
pub type STIS_R = crate::FieldReader;
#[doc = "Field `STIS` writer - Subordinate TMR input selection"]
pub type STIS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `STS` reader - Subordinate TMR synchronization"]
pub type STS_R = crate::BitReader;
#[doc = "Field `STS` writer - Subordinate TMR synchronization"]
pub type STS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ESF` reader - External signal filter"]
pub type ESF_R = crate::FieldReader;
#[doc = "Field `ESF` writer - External signal filter"]
pub type ESF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ESDIV` reader - External signal divider"]
pub type ESDIV_R = crate::FieldReader;
#[doc = "Field `ESDIV` writer - External signal divider"]
pub type ESDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ECMBEN` reader - External clock mode B enable"]
pub type ECMBEN_R = crate::BitReader;
#[doc = "Field `ECMBEN` writer - External clock mode B enable"]
pub type ECMBEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ESP` reader - External signal polarity"]
pub type ESP_R = crate::BitReader;
#[doc = "Field `ESP` writer - External signal polarity"]
pub type ESP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - Subordinate TMR mode selection"]
    #[inline(always)]
    pub fn smsel(&self) -> SMSEL_R {
        SMSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Subordinate TMR input selection"]
    #[inline(always)]
    pub fn stis(&self) -> STIS_R {
        STIS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Subordinate TMR synchronization"]
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External signal filter"]
    #[inline(always)]
    pub fn esf(&self) -> ESF_R {
        ESF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External signal divider"]
    #[inline(always)]
    pub fn esdiv(&self) -> ESDIV_R {
        ESDIV_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock mode B enable"]
    #[inline(always)]
    pub fn ecmben(&self) -> ECMBEN_R {
        ECMBEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External signal polarity"]
    #[inline(always)]
    pub fn esp(&self) -> ESP_R {
        ESP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Subordinate TMR mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn smsel(&mut self) -> SMSEL_W<STCTRL_SPEC, 0> {
        SMSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - Subordinate TMR input selection"]
    #[inline(always)]
    #[must_use]
    pub fn stis(&mut self) -> STIS_W<STCTRL_SPEC, 4> {
        STIS_W::new(self)
    }
    #[doc = "Bit 7 - Subordinate TMR synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn sts(&mut self) -> STS_W<STCTRL_SPEC, 7> {
        STS_W::new(self)
    }
    #[doc = "Bits 8:11 - External signal filter"]
    #[inline(always)]
    #[must_use]
    pub fn esf(&mut self) -> ESF_W<STCTRL_SPEC, 8> {
        ESF_W::new(self)
    }
    #[doc = "Bits 12:13 - External signal divider"]
    #[inline(always)]
    #[must_use]
    pub fn esdiv(&mut self) -> ESDIV_W<STCTRL_SPEC, 12> {
        ESDIV_W::new(self)
    }
    #[doc = "Bit 14 - External clock mode B enable"]
    #[inline(always)]
    #[must_use]
    pub fn ecmben(&mut self) -> ECMBEN_W<STCTRL_SPEC, 14> {
        ECMBEN_W::new(self)
    }
    #[doc = "Bit 15 - External signal polarity"]
    #[inline(always)]
    #[must_use]
    pub fn esp(&mut self) -> ESP_W<STCTRL_SPEC, 15> {
        ESP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Subordinate TMR control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STCTRL_SPEC;
impl crate::RegisterSpec for STCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stctrl::R`](R) reader structure"]
impl crate::Readable for STCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stctrl::W`](W) writer structure"]
impl crate::Writable for STCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STCTRL to value 0"]
impl crate::Resettable for STCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
