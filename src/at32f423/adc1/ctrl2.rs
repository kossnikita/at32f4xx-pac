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
#[doc = "Field `ADCALINIT` reader - Initialize A/D calibration"]
pub type ADCALINIT_R = crate::BitReader;
#[doc = "Field `ADCALINIT` writer - Initialize A/D calibration"]
pub type ADCALINIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADABRT` reader - ADC conversion abort"]
pub type ADABRT_R = crate::BitReader;
#[doc = "Field `ADABRT` writer - ADC conversion abort"]
pub type ADABRT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCDMAEN` reader - Ordinary channel DMA transfer enable for independent mode"]
pub type OCDMAEN_R = crate::BitReader;
#[doc = "Field `OCDMAEN` writer - Ordinary channel DMA transfer enable for independent mode"]
pub type OCDMAEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCDRCEN` reader - Ordinary channel DMA request continuation enable for independent mode"]
pub type OCDRCEN_R = crate::BitReader;
#[doc = "Field `OCDRCEN` writer - Ordinary channel DMA request continuation enable for independent mode"]
pub type OCDRCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOCSFEN` reader - Each ordinary channel conversion set OCCE flag enable"]
pub type EOCSFEN_R = crate::BitReader;
#[doc = "Field `EOCSFEN` writer - Each ordinary channel conversion set OCCE flag enable"]
pub type EOCSFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DTALIGN` reader - Data alignment"]
pub type DTALIGN_R = crate::BitReader;
#[doc = "Field `DTALIGN` writer - Data alignment"]
pub type DTALIGN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCTESEL` reader - trigger event select for preempted channels conversion"]
pub type PCTESEL_R = crate::FieldReader;
#[doc = "Field `PCTESEL` writer - trigger event select for preempted channels conversion"]
pub type PCTESEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `PCETE` reader - Preempted channel external trigger edge select"]
pub type PCETE_R = crate::FieldReader;
#[doc = "Field `PCETE` writer - Preempted channel external trigger edge select"]
pub type PCETE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PCSWTRG` reader - Preempted channel software conversion trigger"]
pub type PCSWTRG_R = crate::BitReader;
#[doc = "Field `PCSWTRG` writer - Preempted channel software conversion trigger"]
pub type PCSWTRG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCTESEL` reader - trigger event select for ordinary channels conversion"]
pub type OCTESEL_R = crate::FieldReader;
#[doc = "Field `OCTESEL` writer - trigger event select for ordinary channels conversion"]
pub type OCTESEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `OCETE` reader - Ordinary channel external trigger edge select"]
pub type OCETE_R = crate::FieldReader;
#[doc = "Field `OCETE` writer - Ordinary channel external trigger edge select"]
pub type OCETE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `OCSWTRG` reader - Ordinary channel software conversion trigger"]
pub type OCSWTRG_R = crate::BitReader;
#[doc = "Field `OCSWTRG` writer - Ordinary channel software conversion trigger"]
pub type OCSWTRG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 3 - Initialize A/D calibration"]
    #[inline(always)]
    pub fn adcalinit(&self) -> ADCALINIT_R {
        ADCALINIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC conversion abort"]
    #[inline(always)]
    pub fn adabrt(&self) -> ADABRT_R {
        ADABRT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Ordinary channel DMA transfer enable for independent mode"]
    #[inline(always)]
    pub fn ocdmaen(&self) -> OCDMAEN_R {
        OCDMAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Ordinary channel DMA request continuation enable for independent mode"]
    #[inline(always)]
    pub fn ocdrcen(&self) -> OCDRCEN_R {
        OCDRCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Each ordinary channel conversion set OCCE flag enable"]
    #[inline(always)]
    pub fn eocsfen(&self) -> EOCSFEN_R {
        EOCSFEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dtalign(&self) -> DTALIGN_R {
        DTALIGN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:19 - trigger event select for preempted channels conversion"]
    #[inline(always)]
    pub fn pctesel(&self) -> PCTESEL_R {
        PCTESEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Preempted channel external trigger edge select"]
    #[inline(always)]
    pub fn pcete(&self) -> PCETE_R {
        PCETE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Preempted channel software conversion trigger"]
    #[inline(always)]
    pub fn pcswtrg(&self) -> PCSWTRG_R {
        PCSWTRG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - trigger event select for ordinary channels conversion"]
    #[inline(always)]
    pub fn octesel(&self) -> OCTESEL_R {
        OCTESEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Ordinary channel external trigger edge select"]
    #[inline(always)]
    pub fn ocete(&self) -> OCETE_R {
        OCETE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Ordinary channel software conversion trigger"]
    #[inline(always)]
    pub fn ocswtrg(&self) -> OCSWTRG_R {
        OCSWTRG_R::new(((self.bits >> 30) & 1) != 0)
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
    #[doc = "Bit 3 - Initialize A/D calibration"]
    #[inline(always)]
    #[must_use]
    pub fn adcalinit(&mut self) -> ADCALINIT_W<CTRL2_SPEC, 3> {
        ADCALINIT_W::new(self)
    }
    #[doc = "Bit 4 - ADC conversion abort"]
    #[inline(always)]
    #[must_use]
    pub fn adabrt(&mut self) -> ADABRT_W<CTRL2_SPEC, 4> {
        ADABRT_W::new(self)
    }
    #[doc = "Bit 8 - Ordinary channel DMA transfer enable for independent mode"]
    #[inline(always)]
    #[must_use]
    pub fn ocdmaen(&mut self) -> OCDMAEN_W<CTRL2_SPEC, 8> {
        OCDMAEN_W::new(self)
    }
    #[doc = "Bit 9 - Ordinary channel DMA request continuation enable for independent mode"]
    #[inline(always)]
    #[must_use]
    pub fn ocdrcen(&mut self) -> OCDRCEN_W<CTRL2_SPEC, 9> {
        OCDRCEN_W::new(self)
    }
    #[doc = "Bit 10 - Each ordinary channel conversion set OCCE flag enable"]
    #[inline(always)]
    #[must_use]
    pub fn eocsfen(&mut self) -> EOCSFEN_W<CTRL2_SPEC, 10> {
        EOCSFEN_W::new(self)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    #[must_use]
    pub fn dtalign(&mut self) -> DTALIGN_W<CTRL2_SPEC, 11> {
        DTALIGN_W::new(self)
    }
    #[doc = "Bits 16:19 - trigger event select for preempted channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn pctesel(&mut self) -> PCTESEL_W<CTRL2_SPEC, 16> {
        PCTESEL_W::new(self)
    }
    #[doc = "Bits 20:21 - Preempted channel external trigger edge select"]
    #[inline(always)]
    #[must_use]
    pub fn pcete(&mut self) -> PCETE_W<CTRL2_SPEC, 20> {
        PCETE_W::new(self)
    }
    #[doc = "Bit 22 - Preempted channel software conversion trigger"]
    #[inline(always)]
    #[must_use]
    pub fn pcswtrg(&mut self) -> PCSWTRG_W<CTRL2_SPEC, 22> {
        PCSWTRG_W::new(self)
    }
    #[doc = "Bits 24:27 - trigger event select for ordinary channels conversion"]
    #[inline(always)]
    #[must_use]
    pub fn octesel(&mut self) -> OCTESEL_W<CTRL2_SPEC, 24> {
        OCTESEL_W::new(self)
    }
    #[doc = "Bits 28:29 - Ordinary channel external trigger edge select"]
    #[inline(always)]
    #[must_use]
    pub fn ocete(&mut self) -> OCETE_W<CTRL2_SPEC, 28> {
        OCETE_W::new(self)
    }
    #[doc = "Bit 30 - Ordinary channel software conversion trigger"]
    #[inline(always)]
    #[must_use]
    pub fn ocswtrg(&mut self) -> OCSWTRG_W<CTRL2_SPEC, 30> {
        OCSWTRG_W::new(self)
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
