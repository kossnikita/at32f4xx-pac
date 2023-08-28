#[doc = "Register `ALA` reader"]
pub type R = crate::R<ALA_SPEC>;
#[doc = "Register `ALA` writer"]
pub type W = crate::W<ALA_SPEC>;
#[doc = "Field `SU` reader - Second units"]
pub type SU_R = crate::FieldReader;
#[doc = "Field `SU` writer - Second units"]
pub type SU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ST` reader - Second tens"]
pub type ST_R = crate::FieldReader;
#[doc = "Field `ST` writer - Second tens"]
pub type ST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `MASK1` reader - Seconds mask"]
pub type MASK1_R = crate::BitReader;
#[doc = "Field `MASK1` writer - Seconds mask"]
pub type MASK1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MU` reader - Minute units"]
pub type MU_R = crate::FieldReader;
#[doc = "Field `MU` writer - Minute units"]
pub type MU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MT` reader - Minute tens"]
pub type MT_R = crate::FieldReader;
#[doc = "Field `MT` writer - Minute tens"]
pub type MT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `MASK2` reader - Minutes mask"]
pub type MASK2_R = crate::BitReader;
#[doc = "Field `MASK2` writer - Minutes mask"]
pub type MASK2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HU` reader - Hour units"]
pub type HU_R = crate::FieldReader;
#[doc = "Field `HU` writer - Hour units"]
pub type HU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `HT` reader - Hour tens"]
pub type HT_R = crate::FieldReader;
#[doc = "Field `HT` writer - Hour tens"]
pub type HT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `AMPM` reader - AM/PM"]
pub type AMPM_R = crate::BitReader;
#[doc = "Field `AMPM` writer - AM/PM"]
pub type AMPM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MASK3` reader - Hours mask"]
pub type MASK3_R = crate::BitReader;
#[doc = "Field `MASK3` writer - Hours mask"]
pub type MASK3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DU` reader - Date units"]
pub type DU_R = crate::FieldReader;
#[doc = "Field `DU` writer - Date units"]
pub type DU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DT` reader - Date tens"]
pub type DT_R = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens"]
pub type DT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `WKSEL` reader - Date/week mode select"]
pub type WKSEL_R = crate::BitReader;
#[doc = "Field `WKSEL` writer - Date/week mode select"]
pub type WKSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MASK4` reader - Date/week mask"]
pub type MASK4_R = crate::BitReader;
#[doc = "Field `MASK4` writer - Date/week mask"]
pub type MASK4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Second units"]
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Seconds mask"]
    #[inline(always)]
    pub fn mask1(&self) -> MASK1_R {
        MASK1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Minute units"]
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens"]
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Minutes mask"]
    #[inline(always)]
    pub fn mask2(&self) -> MASK2_R {
        MASK2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Hour units"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM"]
    #[inline(always)]
    pub fn ampm(&self) -> AMPM_R {
        AMPM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Hours mask"]
    #[inline(always)]
    pub fn mask3(&self) -> MASK3_R {
        MASK3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Date units"]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Date tens"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Date/week mode select"]
    #[inline(always)]
    pub fn wksel(&self) -> WKSEL_R {
        WKSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Date/week mask"]
    #[inline(always)]
    pub fn mask4(&self) -> MASK4_R {
        MASK4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units"]
    #[inline(always)]
    #[must_use]
    pub fn su(&mut self) -> SU_W<ALA_SPEC, 0> {
        SU_W::new(self)
    }
    #[doc = "Bits 4:6 - Second tens"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<ALA_SPEC, 4> {
        ST_W::new(self)
    }
    #[doc = "Bit 7 - Seconds mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask1(&mut self) -> MASK1_W<ALA_SPEC, 7> {
        MASK1_W::new(self)
    }
    #[doc = "Bits 8:11 - Minute units"]
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MU_W<ALA_SPEC, 8> {
        MU_W::new(self)
    }
    #[doc = "Bits 12:14 - Minute tens"]
    #[inline(always)]
    #[must_use]
    pub fn mt(&mut self) -> MT_W<ALA_SPEC, 12> {
        MT_W::new(self)
    }
    #[doc = "Bit 15 - Minutes mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask2(&mut self) -> MASK2_W<ALA_SPEC, 15> {
        MASK2_W::new(self)
    }
    #[doc = "Bits 16:19 - Hour units"]
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HU_W<ALA_SPEC, 16> {
        HU_W::new(self)
    }
    #[doc = "Bits 20:21 - Hour tens"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<ALA_SPEC, 20> {
        HT_W::new(self)
    }
    #[doc = "Bit 22 - AM/PM"]
    #[inline(always)]
    #[must_use]
    pub fn ampm(&mut self) -> AMPM_W<ALA_SPEC, 22> {
        AMPM_W::new(self)
    }
    #[doc = "Bit 23 - Hours mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask3(&mut self) -> MASK3_W<ALA_SPEC, 23> {
        MASK3_W::new(self)
    }
    #[doc = "Bits 24:27 - Date units"]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DU_W<ALA_SPEC, 24> {
        DU_W::new(self)
    }
    #[doc = "Bits 28:29 - Date tens"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<ALA_SPEC, 28> {
        DT_W::new(self)
    }
    #[doc = "Bit 30 - Date/week mode select"]
    #[inline(always)]
    #[must_use]
    pub fn wksel(&mut self) -> WKSEL_W<ALA_SPEC, 30> {
        WKSEL_W::new(self)
    }
    #[doc = "Bit 31 - Date/week mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask4(&mut self) -> MASK4_W<ALA_SPEC, 31> {
        MASK4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Alarm A register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ala::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ala::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALA_SPEC;
impl crate::RegisterSpec for ALA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ala::R`](R) reader structure"]
impl crate::Readable for ALA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ala::W`](W) writer structure"]
impl crate::Writable for ALA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALA to value 0"]
impl crate::Resettable for ALA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
