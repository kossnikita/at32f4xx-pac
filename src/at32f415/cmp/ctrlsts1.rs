#[doc = "Register `CTRLSTS1` reader"]
pub type R = crate::R<CTRLSTS1_SPEC>;
#[doc = "Register `CTRLSTS1` writer"]
pub type W = crate::W<CTRLSTS1_SPEC>;
#[doc = "Field `CMP1EN` reader - Comparator1 enable bit"]
pub type CMP1EN_R = crate::BitReader;
#[doc = "Field `CMP1EN` writer - Comparator1 enable bit"]
pub type CMP1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP1IS` reader - Comparator1 input shift"]
pub type CMP1IS_R = crate::BitReader;
#[doc = "Field `CMP1IS` writer - Comparator1 input shift"]
pub type CMP1IS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP1SSEL` reader - Comparator1 speed selection"]
pub type CMP1SSEL_R = crate::FieldReader;
#[doc = "Field `CMP1SSEL` writer - Comparator1 speed selection"]
pub type CMP1SSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CMP1INVSEL` reader - Comparator1 inverting selection"]
pub type CMP1INVSEL_R = crate::FieldReader;
#[doc = "Field `CMP1INVSEL` writer - Comparator1 inverting selection"]
pub type CMP1INVSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CMP1TAG` reader - Comparator1 output target"]
pub type CMP1TAG_R = crate::FieldReader;
#[doc = "Field `CMP1TAG` writer - Comparator1 output target"]
pub type CMP1TAG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CMP1P` reader - Comparator1 polarity"]
pub type CMP1P_R = crate::BitReader;
#[doc = "Field `CMP1P` writer - Comparator1 polarity"]
pub type CMP1P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP1HYST` reader - Comparator1 hysteresis"]
pub type CMP1HYST_R = crate::FieldReader;
#[doc = "Field `CMP1HYST` writer - Comparator1 hysteresis"]
pub type CMP1HYST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CMP1VALUE` reader - Comparator1 output value"]
pub type CMP1VALUE_R = crate::BitReader;
#[doc = "Field `CMP1WP` reader - Comparator1 write protect"]
pub type CMP1WP_R = crate::BitReader;
#[doc = "Field `CMP1WP` writer - Comparator1 write protect"]
pub type CMP1WP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP2EN` reader - Comparator2 enable bit"]
pub type CMP2EN_R = crate::BitReader;
#[doc = "Field `CMP2EN` writer - Comparator2 enable bit"]
pub type CMP2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP2SSEL` reader - Comparator2 speed selection"]
pub type CMP2SSEL_R = crate::FieldReader;
#[doc = "Field `CMP2SSEL` writer - Comparator2 speed selection"]
pub type CMP2SSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CMP2INVSEL` reader - Comparator2 inverting selection"]
pub type CMP2INVSEL_R = crate::FieldReader;
#[doc = "Field `CMP2INVSEL` writer - Comparator2 inverting selection"]
pub type CMP2INVSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DCMPEN` reader - Double comparator mode enable"]
pub type DCMPEN_R = crate::BitReader;
#[doc = "Field `DCMPEN` writer - Double comparator mode enable"]
pub type DCMPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP2TAG` reader - Comparator2 output target"]
pub type CMP2TAG_R = crate::FieldReader;
#[doc = "Field `CMP2TAG` writer - Comparator2 output target"]
pub type CMP2TAG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CMP2P` reader - Comparator2 polarity"]
pub type CMP2P_R = crate::BitReader;
#[doc = "Field `CMP2P` writer - Comparator2 polarity"]
pub type CMP2P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMP2HYST` reader - Comparator2 hysteresis"]
pub type CMP2HYST_R = crate::FieldReader;
#[doc = "Field `CMP2HYST` writer - Comparator2 hysteresis"]
pub type CMP2HYST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CMP2VALUE` reader - Comparator2 output value"]
pub type CMP2VALUE_R = crate::BitReader;
#[doc = "Field `CMP2WP` reader - Comparator2 write protect"]
pub type CMP2WP_R = crate::BitReader;
#[doc = "Field `CMP2WP` writer - Comparator2 write protect"]
pub type CMP2WP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Comparator1 enable bit"]
    #[inline(always)]
    pub fn cmp1en(&self) -> CMP1EN_R {
        CMP1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator1 input shift"]
    #[inline(always)]
    pub fn cmp1is(&self) -> CMP1IS_R {
        CMP1IS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator1 speed selection"]
    #[inline(always)]
    pub fn cmp1ssel(&self) -> CMP1SSEL_R {
        CMP1SSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator1 inverting selection"]
    #[inline(always)]
    pub fn cmp1invsel(&self) -> CMP1INVSEL_R {
        CMP1INVSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator1 output target"]
    #[inline(always)]
    pub fn cmp1tag(&self) -> CMP1TAG_R {
        CMP1TAG_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Comparator1 polarity"]
    #[inline(always)]
    pub fn cmp1p(&self) -> CMP1P_R {
        CMP1P_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Comparator1 hysteresis"]
    #[inline(always)]
    pub fn cmp1hyst(&self) -> CMP1HYST_R {
        CMP1HYST_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Comparator1 output value"]
    #[inline(always)]
    pub fn cmp1value(&self) -> CMP1VALUE_R {
        CMP1VALUE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator1 write protect"]
    #[inline(always)]
    pub fn cmp1wp(&self) -> CMP1WP_R {
        CMP1WP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Comparator2 enable bit"]
    #[inline(always)]
    pub fn cmp2en(&self) -> CMP2EN_R {
        CMP2EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Comparator2 speed selection"]
    #[inline(always)]
    pub fn cmp2ssel(&self) -> CMP2SSEL_R {
        CMP2SSEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Comparator2 inverting selection"]
    #[inline(always)]
    pub fn cmp2invsel(&self) -> CMP2INVSEL_R {
        CMP2INVSEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Double comparator mode enable"]
    #[inline(always)]
    pub fn dcmpen(&self) -> DCMPEN_R {
        DCMPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Comparator2 output target"]
    #[inline(always)]
    pub fn cmp2tag(&self) -> CMP2TAG_R {
        CMP2TAG_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Comparator2 polarity"]
    #[inline(always)]
    pub fn cmp2p(&self) -> CMP2P_R {
        CMP2P_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Comparator2 hysteresis"]
    #[inline(always)]
    pub fn cmp2hyst(&self) -> CMP2HYST_R {
        CMP2HYST_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Comparator2 output value"]
    #[inline(always)]
    pub fn cmp2value(&self) -> CMP2VALUE_R {
        CMP2VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator2 write protect"]
    #[inline(always)]
    pub fn cmp2wp(&self) -> CMP2WP_R {
        CMP2WP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator1 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1en(&mut self) -> CMP1EN_W<CTRLSTS1_SPEC, 0> {
        CMP1EN_W::new(self)
    }
    #[doc = "Bit 1 - Comparator1 input shift"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1is(&mut self) -> CMP1IS_W<CTRLSTS1_SPEC, 1> {
        CMP1IS_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparator1 speed selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1ssel(&mut self) -> CMP1SSEL_W<CTRLSTS1_SPEC, 2> {
        CMP1SSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator1 inverting selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1invsel(&mut self) -> CMP1INVSEL_W<CTRLSTS1_SPEC, 4> {
        CMP1INVSEL_W::new(self)
    }
    #[doc = "Bits 8:10 - Comparator1 output target"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1tag(&mut self) -> CMP1TAG_W<CTRLSTS1_SPEC, 8> {
        CMP1TAG_W::new(self)
    }
    #[doc = "Bit 11 - Comparator1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1p(&mut self) -> CMP1P_W<CTRLSTS1_SPEC, 11> {
        CMP1P_W::new(self)
    }
    #[doc = "Bits 12:13 - Comparator1 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1hyst(&mut self) -> CMP1HYST_W<CTRLSTS1_SPEC, 12> {
        CMP1HYST_W::new(self)
    }
    #[doc = "Bit 15 - Comparator1 write protect"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1wp(&mut self) -> CMP1WP_W<CTRLSTS1_SPEC, 15> {
        CMP1WP_W::new(self)
    }
    #[doc = "Bit 16 - Comparator2 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2en(&mut self) -> CMP2EN_W<CTRLSTS1_SPEC, 16> {
        CMP2EN_W::new(self)
    }
    #[doc = "Bits 18:19 - Comparator2 speed selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2ssel(&mut self) -> CMP2SSEL_W<CTRLSTS1_SPEC, 18> {
        CMP2SSEL_W::new(self)
    }
    #[doc = "Bits 20:22 - Comparator2 inverting selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2invsel(&mut self) -> CMP2INVSEL_W<CTRLSTS1_SPEC, 20> {
        CMP2INVSEL_W::new(self)
    }
    #[doc = "Bit 23 - Double comparator mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcmpen(&mut self) -> DCMPEN_W<CTRLSTS1_SPEC, 23> {
        DCMPEN_W::new(self)
    }
    #[doc = "Bits 24:26 - Comparator2 output target"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2tag(&mut self) -> CMP2TAG_W<CTRLSTS1_SPEC, 24> {
        CMP2TAG_W::new(self)
    }
    #[doc = "Bit 27 - Comparator2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2p(&mut self) -> CMP2P_W<CTRLSTS1_SPEC, 27> {
        CMP2P_W::new(self)
    }
    #[doc = "Bits 28:29 - Comparator2 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2hyst(&mut self) -> CMP2HYST_W<CTRLSTS1_SPEC, 28> {
        CMP2HYST_W::new(self)
    }
    #[doc = "Bit 31 - Comparator2 write protect"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2wp(&mut self) -> CMP2WP_W<CTRLSTS1_SPEC, 31> {
        CMP2WP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CMP control/status register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLSTS1_SPEC;
impl crate::RegisterSpec for CTRLSTS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlsts1::R`](R) reader structure"]
impl crate::Readable for CTRLSTS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlsts1::W`](W) writer structure"]
impl crate::Writable for CTRLSTS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLSTS1 to value 0"]
impl crate::Resettable for CTRLSTS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}