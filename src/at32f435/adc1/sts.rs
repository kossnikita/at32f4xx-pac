#[doc = "Register `STS` reader"]
pub type R = crate::R<STS_SPEC>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<STS_SPEC>;
#[doc = "Field `VMOR` reader - Voltage monitoring out of range flag"]
pub type VMOR_R = crate::BitReader;
#[doc = "Field `VMOR` writer - Voltage monitoring out of range flag"]
pub type VMOR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCCE` reader - Ordinary channels conversion end flag"]
pub type OCCE_R = crate::BitReader;
#[doc = "Field `OCCE` writer - Ordinary channels conversion end flag"]
pub type OCCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCCE` reader - Preempted channels conversion end flag"]
pub type PCCE_R = crate::BitReader;
#[doc = "Field `PCCE` writer - Preempted channels conversion end flag"]
pub type PCCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCCS` reader - Preempted channel conversion start flag"]
pub type PCCS_R = crate::BitReader;
#[doc = "Field `PCCS` writer - Preempted channel conversion start flag"]
pub type PCCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCCS` reader - Ordinary channel conversion start flag"]
pub type OCCS_R = crate::BitReader;
#[doc = "Field `OCCS` writer - Ordinary channel conversion start flag"]
pub type OCCS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCCO` reader - Ordinary channel conversion overflow flag"]
pub type OCCO_R = crate::BitReader;
#[doc = "Field `OCCO` writer - Ordinary channel conversion overflow flag"]
pub type OCCO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RDY` reader - ADC ready to conversion flag"]
pub type RDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Voltage monitoring out of range flag"]
    #[inline(always)]
    pub fn vmor(&self) -> VMOR_R {
        VMOR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ordinary channels conversion end flag"]
    #[inline(always)]
    pub fn occe(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Preempted channels conversion end flag"]
    #[inline(always)]
    pub fn pcce(&self) -> PCCE_R {
        PCCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Preempted channel conversion start flag"]
    #[inline(always)]
    pub fn pccs(&self) -> PCCS_R {
        PCCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ordinary channel conversion start flag"]
    #[inline(always)]
    pub fn occs(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Ordinary channel conversion overflow flag"]
    #[inline(always)]
    pub fn occo(&self) -> OCCO_R {
        OCCO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC ready to conversion flag"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage monitoring out of range flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmor(&mut self) -> VMOR_W<STS_SPEC, 0> {
        VMOR_W::new(self)
    }
    #[doc = "Bit 1 - Ordinary channels conversion end flag"]
    #[inline(always)]
    #[must_use]
    pub fn occe(&mut self) -> OCCE_W<STS_SPEC, 1> {
        OCCE_W::new(self)
    }
    #[doc = "Bit 2 - Preempted channels conversion end flag"]
    #[inline(always)]
    #[must_use]
    pub fn pcce(&mut self) -> PCCE_W<STS_SPEC, 2> {
        PCCE_W::new(self)
    }
    #[doc = "Bit 3 - Preempted channel conversion start flag"]
    #[inline(always)]
    #[must_use]
    pub fn pccs(&mut self) -> PCCS_W<STS_SPEC, 3> {
        PCCS_W::new(self)
    }
    #[doc = "Bit 4 - Ordinary channel conversion start flag"]
    #[inline(always)]
    #[must_use]
    pub fn occs(&mut self) -> OCCS_W<STS_SPEC, 4> {
        OCCS_W::new(self)
    }
    #[doc = "Bit 5 - Ordinary channel conversion overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn occo(&mut self) -> OCCO_W<STS_SPEC, 5> {
        OCCO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STS_SPEC;
impl crate::RegisterSpec for STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for STS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
