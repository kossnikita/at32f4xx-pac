#[doc = "Register `CTRLSTS1` reader"]
pub type R = crate::R<CTRLSTS1_SPEC>;
#[doc = "Register `CTRLSTS1` writer"]
pub type W = crate::W<CTRLSTS1_SPEC>;
#[doc = "Field `CMP1EN` reader - Comparator1 enable bit"]
pub type CMP1EN_R = crate::BitReader;
#[doc = "Field `CMP1EN` writer - Comparator1 enable bit"]
pub type CMP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1IS` reader - Comparator1 input shift"]
pub type CMP1IS_R = crate::BitReader;
#[doc = "Field `CMP1IS` writer - Comparator1 input shift"]
pub type CMP1IS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1SSEL` reader - Comparator1 speed selection"]
pub type CMP1SSEL_R = crate::FieldReader;
#[doc = "Field `CMP1SSEL` writer - Comparator1 speed selection"]
pub type CMP1SSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMP1INVSEL` reader - Comparator1 inverting selection"]
pub type CMP1INVSEL_R = crate::FieldReader;
#[doc = "Field `CMP1INVSEL` writer - Comparator1 inverting selection"]
pub type CMP1INVSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMP1TAG` reader - Comparator1 output target"]
pub type CMP1TAG_R = crate::FieldReader;
#[doc = "Field `CMP1TAG` writer - Comparator1 output target"]
pub type CMP1TAG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMP1P` reader - Comparator1 polarity"]
pub type CMP1P_R = crate::BitReader;
#[doc = "Field `CMP1P` writer - Comparator1 polarity"]
pub type CMP1P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1HYST` reader - Comparator1 hysteresis"]
pub type CMP1HYST_R = crate::FieldReader;
#[doc = "Field `CMP1HYST` writer - Comparator1 hysteresis"]
pub type CMP1HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMP1VALUE` reader - Comparator1 output value"]
pub type CMP1VALUE_R = crate::BitReader;
#[doc = "Field `CMP1WP` reader - Comparator1 write protect"]
pub type CMP1WP_R = crate::BitReader;
#[doc = "Field `CMP1WP` writer - Comparator1 write protect"]
pub type CMP1WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2EN` reader - Comparator2 enable bit"]
pub type CMP2EN_R = crate::BitReader;
#[doc = "Field `CMP2EN` writer - Comparator2 enable bit"]
pub type CMP2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2SSEL` reader - Comparator2 speed selection"]
pub type CMP2SSEL_R = crate::FieldReader;
#[doc = "Field `CMP2SSEL` writer - Comparator2 speed selection"]
pub type CMP2SSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMP2INVSEL` reader - Comparator2 inverting selection"]
pub type CMP2INVSEL_R = crate::FieldReader;
#[doc = "Field `CMP2INVSEL` writer - Comparator2 inverting selection"]
pub type CMP2INVSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DCMPEN` reader - Double comparator mode enable"]
pub type DCMPEN_R = crate::BitReader;
#[doc = "Field `DCMPEN` writer - Double comparator mode enable"]
pub type DCMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2TAG` reader - Comparator2 output target"]
pub type CMP2TAG_R = crate::FieldReader;
#[doc = "Field `CMP2TAG` writer - Comparator2 output target"]
pub type CMP2TAG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMP2P` reader - Comparator2 polarity"]
pub type CMP2P_R = crate::BitReader;
#[doc = "Field `CMP2P` writer - Comparator2 polarity"]
pub type CMP2P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2HYST` reader - Comparator2 hysteresis"]
pub type CMP2HYST_R = crate::FieldReader;
#[doc = "Field `CMP2HYST` writer - Comparator2 hysteresis"]
pub type CMP2HYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMP2VALUE` reader - Comparator2 output value"]
pub type CMP2VALUE_R = crate::BitReader;
#[doc = "Field `CMP2WP` reader - Comparator2 write protect"]
pub type CMP2WP_R = crate::BitReader;
#[doc = "Field `CMP2WP` writer - Comparator2 write protect"]
pub type CMP2WP_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRLSTS1")
            .field("cmp1en", &format_args!("{}", self.cmp1en().bit()))
            .field("cmp1is", &format_args!("{}", self.cmp1is().bit()))
            .field("cmp1ssel", &format_args!("{}", self.cmp1ssel().bits()))
            .field("cmp1invsel", &format_args!("{}", self.cmp1invsel().bits()))
            .field("cmp1tag", &format_args!("{}", self.cmp1tag().bits()))
            .field("cmp1p", &format_args!("{}", self.cmp1p().bit()))
            .field("cmp1hyst", &format_args!("{}", self.cmp1hyst().bits()))
            .field("cmp1value", &format_args!("{}", self.cmp1value().bit()))
            .field("cmp1wp", &format_args!("{}", self.cmp1wp().bit()))
            .field("cmp2en", &format_args!("{}", self.cmp2en().bit()))
            .field("cmp2ssel", &format_args!("{}", self.cmp2ssel().bits()))
            .field("cmp2invsel", &format_args!("{}", self.cmp2invsel().bits()))
            .field("dcmpen", &format_args!("{}", self.dcmpen().bit()))
            .field("cmp2tag", &format_args!("{}", self.cmp2tag().bits()))
            .field("cmp2p", &format_args!("{}", self.cmp2p().bit()))
            .field("cmp2hyst", &format_args!("{}", self.cmp2hyst().bits()))
            .field("cmp2value", &format_args!("{}", self.cmp2value().bit()))
            .field("cmp2wp", &format_args!("{}", self.cmp2wp().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRLSTS1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator1 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1en(&mut self) -> CMP1EN_W<CTRLSTS1_SPEC> {
        CMP1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator1 input shift"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1is(&mut self) -> CMP1IS_W<CTRLSTS1_SPEC> {
        CMP1IS_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Comparator1 speed selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1ssel(&mut self) -> CMP1SSEL_W<CTRLSTS1_SPEC> {
        CMP1SSEL_W::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator1 inverting selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1invsel(&mut self) -> CMP1INVSEL_W<CTRLSTS1_SPEC> {
        CMP1INVSEL_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Comparator1 output target"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1tag(&mut self) -> CMP1TAG_W<CTRLSTS1_SPEC> {
        CMP1TAG_W::new(self, 8)
    }
    #[doc = "Bit 11 - Comparator1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1p(&mut self) -> CMP1P_W<CTRLSTS1_SPEC> {
        CMP1P_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Comparator1 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1hyst(&mut self) -> CMP1HYST_W<CTRLSTS1_SPEC> {
        CMP1HYST_W::new(self, 12)
    }
    #[doc = "Bit 15 - Comparator1 write protect"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1wp(&mut self) -> CMP1WP_W<CTRLSTS1_SPEC> {
        CMP1WP_W::new(self, 15)
    }
    #[doc = "Bit 16 - Comparator2 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2en(&mut self) -> CMP2EN_W<CTRLSTS1_SPEC> {
        CMP2EN_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Comparator2 speed selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2ssel(&mut self) -> CMP2SSEL_W<CTRLSTS1_SPEC> {
        CMP2SSEL_W::new(self, 18)
    }
    #[doc = "Bits 20:22 - Comparator2 inverting selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2invsel(&mut self) -> CMP2INVSEL_W<CTRLSTS1_SPEC> {
        CMP2INVSEL_W::new(self, 20)
    }
    #[doc = "Bit 23 - Double comparator mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcmpen(&mut self) -> DCMPEN_W<CTRLSTS1_SPEC> {
        DCMPEN_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Comparator2 output target"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2tag(&mut self) -> CMP2TAG_W<CTRLSTS1_SPEC> {
        CMP2TAG_W::new(self, 24)
    }
    #[doc = "Bit 27 - Comparator2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2p(&mut self) -> CMP2P_W<CTRLSTS1_SPEC> {
        CMP2P_W::new(self, 27)
    }
    #[doc = "Bits 28:29 - Comparator2 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2hyst(&mut self) -> CMP2HYST_W<CTRLSTS1_SPEC> {
        CMP2HYST_W::new(self, 28)
    }
    #[doc = "Bit 31 - Comparator2 write protect"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2wp(&mut self) -> CMP2WP_W<CTRLSTS1_SPEC> {
        CMP2WP_W::new(self, 31)
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
