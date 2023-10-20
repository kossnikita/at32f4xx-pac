#[doc = "Register `CCTRL` reader"]
pub type R = crate::R<CCTRL_SPEC>;
#[doc = "Register `CCTRL` writer"]
pub type W = crate::W<CCTRL_SPEC>;
#[doc = "Field `MSSEL` reader - Master slave mode select"]
pub type MSSEL_R = crate::FieldReader;
#[doc = "Field `MSSEL` writer - Master slave mode select"]
pub type MSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ASISEL` reader - Adjacent ADC sampling interval select for ordinary shifting mode"]
pub type ASISEL_R = crate::FieldReader;
#[doc = "Field `ASISEL` writer - Adjacent ADC sampling interval select for ordinary shifting mode"]
pub type ASISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MSDRCEN` reader - Ordinary channel DMA request continuation enable for master slave mode"]
pub type MSDRCEN_R = crate::BitReader;
#[doc = "Field `MSDRCEN` writer - Ordinary channel DMA request continuation enable for master slave mode"]
pub type MSDRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSDMASEL_L` reader - Low bit of ordinary channel DMA transfer mode select for master slave mode"]
pub type MSDMASEL_L_R = crate::FieldReader;
#[doc = "Field `MSDMASEL_L` writer - Low bit of ordinary channel DMA transfer mode select for master slave mode"]
pub type MSDMASEL_L_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADCDIV` reader - ADC division"]
pub type ADCDIV_R = crate::FieldReader;
#[doc = "Field `ADCDIV` writer - ADC division"]
pub type ADCDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VBATEN` reader - VBAT enable"]
pub type VBATEN_R = crate::BitReader;
#[doc = "Field `VBATEN` writer - VBAT enable"]
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITSRVEN` reader - Internal temperature sensor and VINTRV enable"]
pub type ITSRVEN_R = crate::BitReader;
#[doc = "Field `ITSRVEN` writer - Internal temperature sensor and VINTRV enable"]
pub type ITSRVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSDMASEL_H` reader - High bit of ordinary channel DMA transfer mode select for master slave mode"]
pub type MSDMASEL_H_R = crate::BitReader;
#[doc = "Field `MSDMASEL_H` writer - High bit of ordinary channel DMA transfer mode select for master slave mode"]
pub type MSDMASEL_H_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Master slave mode select"]
    #[inline(always)]
    pub fn mssel(&self) -> MSSEL_R {
        MSSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Adjacent ADC sampling interval select for ordinary shifting mode"]
    #[inline(always)]
    pub fn asisel(&self) -> ASISEL_R {
        ASISEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Ordinary channel DMA request continuation enable for master slave mode"]
    #[inline(always)]
    pub fn msdrcen(&self) -> MSDRCEN_R {
        MSDRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Low bit of ordinary channel DMA transfer mode select for master slave mode"]
    #[inline(always)]
    pub fn msdmasel_l(&self) -> MSDMASEL_L_R {
        MSDMASEL_L_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - ADC division"]
    #[inline(always)]
    pub fn adcdiv(&self) -> ADCDIV_R {
        ADCDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - VBAT enable"]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Internal temperature sensor and VINTRV enable"]
    #[inline(always)]
    pub fn itsrven(&self) -> ITSRVEN_R {
        ITSRVEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - High bit of ordinary channel DMA transfer mode select for master slave mode"]
    #[inline(always)]
    pub fn msdmasel_h(&self) -> MSDMASEL_H_R {
        MSDMASEL_H_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCTRL")
            .field("msdmasel_h", &format_args!("{}", self.msdmasel_h().bit()))
            .field("itsrven", &format_args!("{}", self.itsrven().bit()))
            .field("vbaten", &format_args!("{}", self.vbaten().bit()))
            .field("adcdiv", &format_args!("{}", self.adcdiv().bits()))
            .field("msdmasel_l", &format_args!("{}", self.msdmasel_l().bits()))
            .field("msdrcen", &format_args!("{}", self.msdrcen().bit()))
            .field("asisel", &format_args!("{}", self.asisel().bits()))
            .field("mssel", &format_args!("{}", self.mssel().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Master slave mode select"]
    #[inline(always)]
    #[must_use]
    pub fn mssel(&mut self) -> MSSEL_W<CCTRL_SPEC> {
        MSSEL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Adjacent ADC sampling interval select for ordinary shifting mode"]
    #[inline(always)]
    #[must_use]
    pub fn asisel(&mut self) -> ASISEL_W<CCTRL_SPEC> {
        ASISEL_W::new(self, 8)
    }
    #[doc = "Bit 13 - Ordinary channel DMA request continuation enable for master slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msdrcen(&mut self) -> MSDRCEN_W<CCTRL_SPEC> {
        MSDRCEN_W::new(self, 13)
    }
    #[doc = "Bits 14:15 - Low bit of ordinary channel DMA transfer mode select for master slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msdmasel_l(&mut self) -> MSDMASEL_L_W<CCTRL_SPEC> {
        MSDMASEL_L_W::new(self, 14)
    }
    #[doc = "Bits 16:19 - ADC division"]
    #[inline(always)]
    #[must_use]
    pub fn adcdiv(&mut self) -> ADCDIV_W<CCTRL_SPEC> {
        ADCDIV_W::new(self, 16)
    }
    #[doc = "Bit 22 - VBAT enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VBATEN_W<CCTRL_SPEC> {
        VBATEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Internal temperature sensor and VINTRV enable"]
    #[inline(always)]
    #[must_use]
    pub fn itsrven(&mut self) -> ITSRVEN_W<CCTRL_SPEC> {
        ITSRVEN_W::new(self, 23)
    }
    #[doc = "Bit 28 - High bit of ordinary channel DMA transfer mode select for master slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msdmasel_h(&mut self) -> MSDMASEL_H_W<CCTRL_SPEC> {
        MSDMASEL_H_W::new(self, 28)
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
#[doc = "Common control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCTRL_SPEC;
impl crate::RegisterSpec for CCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cctrl::R`](R) reader structure"]
impl crate::Readable for CCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cctrl::W`](W) writer structure"]
impl crate::Writable for CCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCTRL to value 0"]
impl crate::Resettable for CCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
