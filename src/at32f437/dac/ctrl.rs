#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `D1EN` reader - DAC1 enable"]
pub type D1EN_R = crate::BitReader;
#[doc = "Field `D1EN` writer - DAC1 enable"]
pub type D1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1OBDIS` reader - DAC1 output buffer disable"]
pub type D1OBDIS_R = crate::BitReader;
#[doc = "Field `D1OBDIS` writer - DAC1 output buffer disable"]
pub type D1OBDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1TRGEN` reader - DAC1 trigger enable"]
pub type D1TRGEN_R = crate::BitReader;
#[doc = "Field `D1TRGEN` writer - DAC1 trigger enable"]
pub type D1TRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1TRGSEL` reader - DAC1 trigger selection"]
pub type D1TRGSEL_R = crate::FieldReader;
#[doc = "Field `D1TRGSEL` writer - DAC1 trigger selection"]
pub type D1TRGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `D1NM` reader - DAC1 noise/triangle wave generation enable"]
pub type D1NM_R = crate::FieldReader;
#[doc = "Field `D1NM` writer - DAC1 noise/triangle wave generation enable"]
pub type D1NM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `D1NBSEL` reader - DAC1 mask/amplitude selector"]
pub type D1NBSEL_R = crate::FieldReader;
#[doc = "Field `D1NBSEL` writer - DAC1 mask/amplitude selector"]
pub type D1NBSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `D1DMAEN` reader - DAC1 DMA enable"]
pub type D1DMAEN_R = crate::BitReader;
#[doc = "Field `D1DMAEN` writer - DAC1 DMA enable"]
pub type D1DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1DMAUDRIEN` reader - DAC1 DMA underrun interrupt enable"]
pub type D1DMAUDRIEN_R = crate::BitReader;
#[doc = "Field `D1DMAUDRIEN` writer - DAC1 DMA underrun interrupt enable"]
pub type D1DMAUDRIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2EN` reader - DAC2 enable"]
pub type D2EN_R = crate::BitReader;
#[doc = "Field `D2EN` writer - DAC2 enable"]
pub type D2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2OBDIS` reader - DAC2 output buffer disable"]
pub type D2OBDIS_R = crate::BitReader;
#[doc = "Field `D2OBDIS` writer - DAC2 output buffer disable"]
pub type D2OBDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2TRGEN` reader - DAC2 trigger enable"]
pub type D2TRGEN_R = crate::BitReader;
#[doc = "Field `D2TRGEN` writer - DAC2 trigger enable"]
pub type D2TRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2TRGSEL` reader - DAC2 trigger selection"]
pub type D2TRGSEL_R = crate::FieldReader;
#[doc = "Field `D2TRGSEL` writer - DAC2 trigger selection"]
pub type D2TRGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `D2NM` reader - DAC2 noise/triangle wave generation enable"]
pub type D2NM_R = crate::FieldReader;
#[doc = "Field `D2NM` writer - DAC2 noise/triangle wave generation enable"]
pub type D2NM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `D2NBSEL` reader - DAC2 mask/amplitude selector"]
pub type D2NBSEL_R = crate::FieldReader;
#[doc = "Field `D2NBSEL` writer - DAC2 mask/amplitude selector"]
pub type D2NBSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `D2DMAEN` reader - DAC2 DMA enable"]
pub type D2DMAEN_R = crate::BitReader;
#[doc = "Field `D2DMAEN` writer - DAC2 DMA enable"]
pub type D2DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2DMAUDRIEN` reader - DAC2 DMA underrun interrupt enable"]
pub type D2DMAUDRIEN_R = crate::BitReader;
#[doc = "Field `D2DMAUDRIEN` writer - DAC2 DMA underrun interrupt enable"]
pub type D2DMAUDRIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DAC1 enable"]
    #[inline(always)]
    pub fn d1en(&self) -> D1EN_R {
        D1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC1 output buffer disable"]
    #[inline(always)]
    pub fn d1obdis(&self) -> D1OBDIS_R {
        D1OBDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC1 trigger enable"]
    #[inline(always)]
    pub fn d1trgen(&self) -> D1TRGEN_R {
        D1TRGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - DAC1 trigger selection"]
    #[inline(always)]
    pub fn d1trgsel(&self) -> D1TRGSEL_R {
        D1TRGSEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - DAC1 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn d1nm(&self) -> D1NM_R {
        D1NM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC1 mask/amplitude selector"]
    #[inline(always)]
    pub fn d1nbsel(&self) -> D1NBSEL_R {
        D1NBSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC1 DMA enable"]
    #[inline(always)]
    pub fn d1dmaen(&self) -> D1DMAEN_R {
        D1DMAEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC1 DMA underrun interrupt enable"]
    #[inline(always)]
    pub fn d1dmaudrien(&self) -> D1DMAUDRIEN_R {
        D1DMAUDRIEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC2 enable"]
    #[inline(always)]
    pub fn d2en(&self) -> D2EN_R {
        D2EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC2 output buffer disable"]
    #[inline(always)]
    pub fn d2obdis(&self) -> D2OBDIS_R {
        D2OBDIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC2 trigger enable"]
    #[inline(always)]
    pub fn d2trgen(&self) -> D2TRGEN_R {
        D2TRGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - DAC2 trigger selection"]
    #[inline(always)]
    pub fn d2trgsel(&self) -> D2TRGSEL_R {
        D2TRGSEL_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - DAC2 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn d2nm(&self) -> D2NM_R {
        D2NM_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DAC2 mask/amplitude selector"]
    #[inline(always)]
    pub fn d2nbsel(&self) -> D2NBSEL_R {
        D2NBSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DAC2 DMA enable"]
    #[inline(always)]
    pub fn d2dmaen(&self) -> D2DMAEN_R {
        D2DMAEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC2 DMA underrun interrupt enable"]
    #[inline(always)]
    pub fn d2dmaudrien(&self) -> D2DMAUDRIEN_R {
        D2DMAUDRIEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("d1en", &format_args!("{}", self.d1en().bit()))
            .field("d1obdis", &format_args!("{}", self.d1obdis().bit()))
            .field("d1trgen", &format_args!("{}", self.d1trgen().bit()))
            .field("d1trgsel", &format_args!("{}", self.d1trgsel().bits()))
            .field("d1nm", &format_args!("{}", self.d1nm().bits()))
            .field("d1nbsel", &format_args!("{}", self.d1nbsel().bits()))
            .field("d1dmaen", &format_args!("{}", self.d1dmaen().bit()))
            .field("d1dmaudrien", &format_args!("{}", self.d1dmaudrien().bit()))
            .field("d2en", &format_args!("{}", self.d2en().bit()))
            .field("d2obdis", &format_args!("{}", self.d2obdis().bit()))
            .field("d2trgen", &format_args!("{}", self.d2trgen().bit()))
            .field("d2trgsel", &format_args!("{}", self.d2trgsel().bits()))
            .field("d2nm", &format_args!("{}", self.d2nm().bits()))
            .field("d2nbsel", &format_args!("{}", self.d2nbsel().bits()))
            .field("d2dmaen", &format_args!("{}", self.d2dmaen().bit()))
            .field("d2dmaudrien", &format_args!("{}", self.d2dmaudrien().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - DAC1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn d1en(&mut self) -> D1EN_W<CTRL_SPEC> {
        D1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC1 output buffer disable"]
    #[inline(always)]
    #[must_use]
    pub fn d1obdis(&mut self) -> D1OBDIS_W<CTRL_SPEC> {
        D1OBDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - DAC1 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn d1trgen(&mut self) -> D1TRGEN_W<CTRL_SPEC> {
        D1TRGEN_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - DAC1 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn d1trgsel(&mut self) -> D1TRGSEL_W<CTRL_SPEC> {
        D1TRGSEL_W::new(self, 3)
    }
    #[doc = "Bits 6:7 - DAC1 noise/triangle wave generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn d1nm(&mut self) -> D1NM_W<CTRL_SPEC> {
        D1NM_W::new(self, 6)
    }
    #[doc = "Bits 8:11 - DAC1 mask/amplitude selector"]
    #[inline(always)]
    #[must_use]
    pub fn d1nbsel(&mut self) -> D1NBSEL_W<CTRL_SPEC> {
        D1NBSEL_W::new(self, 8)
    }
    #[doc = "Bit 12 - DAC1 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn d1dmaen(&mut self) -> D1DMAEN_W<CTRL_SPEC> {
        D1DMAEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - DAC1 DMA underrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn d1dmaudrien(&mut self) -> D1DMAUDRIEN_W<CTRL_SPEC> {
        D1DMAUDRIEN_W::new(self, 13)
    }
    #[doc = "Bit 16 - DAC2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn d2en(&mut self) -> D2EN_W<CTRL_SPEC> {
        D2EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - DAC2 output buffer disable"]
    #[inline(always)]
    #[must_use]
    pub fn d2obdis(&mut self) -> D2OBDIS_W<CTRL_SPEC> {
        D2OBDIS_W::new(self, 17)
    }
    #[doc = "Bit 18 - DAC2 trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn d2trgen(&mut self) -> D2TRGEN_W<CTRL_SPEC> {
        D2TRGEN_W::new(self, 18)
    }
    #[doc = "Bits 19:21 - DAC2 trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn d2trgsel(&mut self) -> D2TRGSEL_W<CTRL_SPEC> {
        D2TRGSEL_W::new(self, 19)
    }
    #[doc = "Bits 22:23 - DAC2 noise/triangle wave generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn d2nm(&mut self) -> D2NM_W<CTRL_SPEC> {
        D2NM_W::new(self, 22)
    }
    #[doc = "Bits 24:27 - DAC2 mask/amplitude selector"]
    #[inline(always)]
    #[must_use]
    pub fn d2nbsel(&mut self) -> D2NBSEL_W<CTRL_SPEC> {
        D2NBSEL_W::new(self, 24)
    }
    #[doc = "Bit 28 - DAC2 DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn d2dmaen(&mut self) -> D2DMAEN_W<CTRL_SPEC> {
        D2DMAEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - DAC2 DMA underrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn d2dmaudrien(&mut self) -> D2DMAUDRIEN_W<CTRL_SPEC> {
        D2DMAUDRIEN_W::new(self, 29)
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
#[doc = "Control register (DAC_CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
