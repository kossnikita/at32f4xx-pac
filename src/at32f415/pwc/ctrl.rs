#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `VRSEL` reader - Voltage regulator state select when deepsleep mode"]
pub type VRSEL_R = crate::BitReader;
#[doc = "Field `VRSEL` writer - Voltage regulator state select when deepsleep mode"]
pub type VRSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LPSEL` reader - Low power mode select when Cortex-M4F sleepdeep"]
pub type LPSEL_R = crate::BitReader;
#[doc = "Field `LPSEL` writer - Low power mode select when Cortex-M4F sleepdeep"]
pub type LPSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLSWEF` reader - Clear SWEF flag"]
pub type CLSWEF_R = crate::BitReader;
#[doc = "Field `CLSWEF` writer - Clear SWEF flag"]
pub type CLSWEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLSEF` reader - Clear SEF flag"]
pub type CLSEF_R = crate::BitReader;
#[doc = "Field `CLSEF` writer - Clear SEF flag"]
pub type CLSEF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PVMEN` reader - Power voltage monitoring enable"]
pub type PVMEN_R = crate::BitReader;
#[doc = "Field `PVMEN` writer - Power voltage monitoring enable"]
pub type PVMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PVMSEL` reader - Power voltage monitoring boundary select"]
pub type PVMSEL_R = crate::FieldReader;
#[doc = "Field `PVMSEL` writer - Power voltage monitoring boundary select"]
pub type PVMSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `BPWEN` reader - Battery powered domain write enable"]
pub type BPWEN_R = crate::BitReader;
#[doc = "Field `BPWEN` writer - Battery powered domain write enable"]
pub type BPWEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Voltage regulator state select when deepsleep mode"]
    #[inline(always)]
    pub fn vrsel(&self) -> VRSEL_R {
        VRSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low power mode select when Cortex-M4F sleepdeep"]
    #[inline(always)]
    pub fn lpsel(&self) -> LPSEL_R {
        LPSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear SWEF flag"]
    #[inline(always)]
    pub fn clswef(&self) -> CLSWEF_R {
        CLSWEF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear SEF flag"]
    #[inline(always)]
    pub fn clsef(&self) -> CLSEF_R {
        CLSEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power voltage monitoring enable"]
    #[inline(always)]
    pub fn pvmen(&self) -> PVMEN_R {
        PVMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Power voltage monitoring boundary select"]
    #[inline(always)]
    pub fn pvmsel(&self) -> PVMSEL_R {
        PVMSEL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Battery powered domain write enable"]
    #[inline(always)]
    pub fn bpwen(&self) -> BPWEN_R {
        BPWEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("vrsel", &format_args!("{}", self.vrsel().bit()))
            .field("lpsel", &format_args!("{}", self.lpsel().bit()))
            .field("clswef", &format_args!("{}", self.clswef().bit()))
            .field("clsef", &format_args!("{}", self.clsef().bit()))
            .field("pvmen", &format_args!("{}", self.pvmen().bit()))
            .field("pvmsel", &format_args!("{}", self.pvmsel().bits()))
            .field("bpwen", &format_args!("{}", self.bpwen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage regulator state select when deepsleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn vrsel(&mut self) -> VRSEL_W<CTRL_SPEC, 0> {
        VRSEL_W::new(self)
    }
    #[doc = "Bit 1 - Low power mode select when Cortex-M4F sleepdeep"]
    #[inline(always)]
    #[must_use]
    pub fn lpsel(&mut self) -> LPSEL_W<CTRL_SPEC, 1> {
        LPSEL_W::new(self)
    }
    #[doc = "Bit 2 - Clear SWEF flag"]
    #[inline(always)]
    #[must_use]
    pub fn clswef(&mut self) -> CLSWEF_W<CTRL_SPEC, 2> {
        CLSWEF_W::new(self)
    }
    #[doc = "Bit 3 - Clear SEF flag"]
    #[inline(always)]
    #[must_use]
    pub fn clsef(&mut self) -> CLSEF_W<CTRL_SPEC, 3> {
        CLSEF_W::new(self)
    }
    #[doc = "Bit 4 - Power voltage monitoring enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvmen(&mut self) -> PVMEN_W<CTRL_SPEC, 4> {
        PVMEN_W::new(self)
    }
    #[doc = "Bits 5:7 - Power voltage monitoring boundary select"]
    #[inline(always)]
    #[must_use]
    pub fn pvmsel(&mut self) -> PVMSEL_W<CTRL_SPEC, 5> {
        PVMSEL_W::new(self)
    }
    #[doc = "Bit 8 - Battery powered domain write enable"]
    #[inline(always)]
    #[must_use]
    pub fn bpwen(&mut self) -> BPWEN_W<CTRL_SPEC, 8> {
        BPWEN_W::new(self)
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
#[doc = "Power control register (PWC_CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
