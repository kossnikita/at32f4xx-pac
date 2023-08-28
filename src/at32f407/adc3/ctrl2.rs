#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `ADCEN` reader - A/D converter enable"]
pub type ADCEN_R = crate::BitReader;
#[doc = "Field `ADCEN` writer - A/D converter enable"]
pub type ADCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RPEN` reader - Repeat mode enable"]
pub type RPEN_R = crate::BitReader;
#[doc = "Field `RPEN` writer - Repeat mode enable"]
pub type RPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADCAL` reader - A/D Calibration"]
pub type ADCAL_R = crate::BitReader;
#[doc = "Field `ADCAL` writer - A/D Calibration"]
pub type ADCAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADCALINIT` reader - initialize A/D calibration"]
pub type ADCALINIT_R = crate::BitReader;
#[doc = "Field `ADCALINIT` writer - initialize A/D calibration"]
pub type ADCALINIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCDMAEN` reader - DMA transfer enable of ordinary channels"]
pub type OCDMAEN_R = crate::BitReader;
#[doc = "Field `OCDMAEN` writer - DMA transfer enable of ordinary channels"]
pub type OCDMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTALIGN` reader - Data alignment"]
pub type DTALIGN_R = crate::BitReader;
#[doc = "Field `DTALIGN` writer - Data alignment"]
pub type DTALIGN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCTESEL_L` reader - Low bit of trigger event select for preempted channels conversion"]
pub type PCTESEL_L_R = crate::FieldReader;
#[doc = "Field `PCTESEL_L` writer - Low bit of trigger event select for preempted channels conversion"]
pub type PCTESEL_L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `PCTEN` reader - Trigger mode enable for preempted channels conversion"]
pub type PCTEN_R = crate::BitReader;
#[doc = "Field `PCTEN` writer - Trigger mode enable for preempted channels conversion"]
pub type PCTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCTESEL_L` reader - Low bit of trigger event select for ordinary channels conversion"]
pub type OCTESEL_L_R = crate::FieldReader;
#[doc = "Field `OCTESEL_L` writer - Low bit of trigger event select for ordinary channels conversion"]
pub type OCTESEL_L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `OCTEN` reader - Trigger mode enable for ordinary channels conversion"]
pub type OCTEN_R = crate::BitReader;
#[doc = "Field `OCTEN` writer - Trigger mode enable for ordinary channels conversion"]
pub type OCTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCSWTRG` reader - Conversion trigger by software of preempted channels"]
pub type PCSWTRG_R = crate::BitReader;
#[doc = "Field `PCSWTRG` writer - Conversion trigger by software of preempted channels"]
pub type PCSWTRG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCSWTRG` reader - Conversion trigger by software of ordinary channels"]
pub type OCSWTRG_R = crate::BitReader;
#[doc = "Field `OCSWTRG` writer - Conversion trigger by software of ordinary channels"]
pub type OCSWTRG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCTESEL_H` reader - High bit of trigger event select for preempted channels conversion"]
pub type PCTESEL_H_R = crate::BitReader;
#[doc = "Field `PCTESEL_H` writer - High bit of trigger event select for preempted channels conversion"]
pub type PCTESEL_H_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCTESEL_H` reader - High bit of trigger event select for ordinary channels conversion"]
pub type OCTESEL_H_R = crate::BitReader;
#[doc = "Field `OCTESEL_H` writer - High bit of trigger event select for ordinary channels conversion"]
pub type OCTESEL_H_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - A/D converter enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repeat mode enable"]
    #[inline(always)]
    pub fn rpen(&self) -> RPEN_R {
        RPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A/D Calibration"]
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - initialize A/D calibration"]
    #[inline(always)]
    pub fn adcalinit(&self) -> ADCALINIT_R {
        ADCALINIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA transfer enable of ordinary channels"]
    #[inline(always)]
    pub fn ocdmaen(&self) -> OCDMAEN_R {
        OCDMAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dtalign(&self) -> DTALIGN_R {
        DTALIGN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Low bit of trigger event select for preempted channels conversion"]
    #[inline(always)]
    pub fn pctesel_l(&self) -> PCTESEL_L_R {
        PCTESEL_L_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Trigger mode enable for preempted channels conversion"]
    #[inline(always)]
    pub fn pcten(&self) -> PCTEN_R {
        PCTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Low bit of trigger event select for ordinary channels conversion"]
    #[inline(always)]
    pub fn octesel_l(&self) -> OCTESEL_L_R {
        OCTESEL_L_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Trigger mode enable for ordinary channels conversion"]
    #[inline(always)]
    pub fn octen(&self) -> OCTEN_R {
        OCTEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Conversion trigger by software of preempted channels"]
    #[inline(always)]
    pub fn pcswtrg(&self) -> PCSWTRG_R {
        PCSWTRG_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Conversion trigger by software of ordinary channels"]
    #[inline(always)]
    pub fn ocswtrg(&self) -> OCSWTRG_R {
        OCSWTRG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - High bit of trigger event select for preempted channels conversion"]
    #[inline(always)]
    pub fn pctesel_h(&self) -> PCTESEL_H_R {
        PCTESEL_H_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - High bit of trigger event select for ordinary channels conversion"]
    #[inline(always)]
    pub fn octesel_h(&self) -> OCTESEL_H_R {
        OCTESEL_H_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A/D converter enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<CTRL2_SPEC, 0> {
        ADCEN_W::new(self)
    }
    #[doc = "Bit 1 - Repeat mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn rpen(&mut self) -> RPEN_W<CTRL2_SPEC, 1> {
        RPEN_W::new(self)
    }
    #[doc = "Bit 2 - A/D Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcal(&mut self) -> ADCAL_W<CTRL2_SPEC, 2> {
        ADCAL_W::new(self)
    }
    #[doc = "Bit 3 - initialize A/D calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcalinit(&mut self) -> ADCALINIT_W<CTRL2_SPEC, 3> {
        ADCALINIT_W::new(self)
    }
    #[doc = "Bit 8 - DMA transfer enable of ordinary channels"]
    #[inline(always)]
    #[must_use]
    pub fn ocdmaen(&mut self) -> OCDMAEN_W<CTRL2_SPEC, 8> {
        OCDMAEN_W::new(self)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    #[must_use]
    pub fn dtalign(&mut self) -> DTALIGN_W<CTRL2_SPEC, 11> {
        DTALIGN_W::new(self)
    }
    #[doc = "Bits 12:14 - Low bit of trigger event select for preempted channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn pctesel_l(&mut self) -> PCTESEL_L_W<CTRL2_SPEC, 12> {
        PCTESEL_L_W::new(self)
    }
    #[doc = "Bit 15 - Trigger mode enable for preempted channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn pcten(&mut self) -> PCTEN_W<CTRL2_SPEC, 15> {
        PCTEN_W::new(self)
    }
    #[doc = "Bits 17:19 - Low bit of trigger event select for ordinary channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn octesel_l(&mut self) -> OCTESEL_L_W<CTRL2_SPEC, 17> {
        OCTESEL_L_W::new(self)
    }
    #[doc = "Bit 20 - Trigger mode enable for ordinary channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn octen(&mut self) -> OCTEN_W<CTRL2_SPEC, 20> {
        OCTEN_W::new(self)
    }
    #[doc = "Bit 21 - Conversion trigger by software of preempted channels"]
    #[inline(always)]
    #[must_use]
    pub fn pcswtrg(&mut self) -> PCSWTRG_W<CTRL2_SPEC, 21> {
        PCSWTRG_W::new(self)
    }
    #[doc = "Bit 22 - Conversion trigger by software of ordinary channels"]
    #[inline(always)]
    #[must_use]
    pub fn ocswtrg(&mut self) -> OCSWTRG_W<CTRL2_SPEC, 22> {
        OCSWTRG_W::new(self)
    }
    #[doc = "Bit 24 - High bit of trigger event select for preempted channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn pctesel_h(&mut self) -> PCTESEL_H_W<CTRL2_SPEC, 24> {
        PCTESEL_H_W::new(self)
    }
    #[doc = "Bit 25 - High bit of trigger event select for ordinary channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn octesel_h(&mut self) -> OCTESEL_H_W<CTRL2_SPEC, 25> {
        OCTESEL_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
