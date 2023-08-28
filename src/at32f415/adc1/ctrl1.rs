#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `VMCSEL` reader - Voltage monitoring channel select"]
pub type VMCSEL_R = crate::FieldReader;
#[doc = "Field `VMCSEL` writer - Voltage monitoring channel select"]
pub type VMCSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `CCEIEN` reader - Channel conversion end interrupt enable"]
pub type CCEIEN_R = crate::BitReader;
#[doc = "Field `CCEIEN` writer - Channel conversion end interrupt enable"]
pub type CCEIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMORIEN` reader - Voltage monitoring out of range interrupt enable"]
pub type VMORIEN_R = crate::BitReader;
#[doc = "Field `VMORIEN` writer - Voltage monitoring out of range interrupt enable"]
pub type VMORIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCCEIEN` reader - Conversion end interrupt enable for preempted channels"]
pub type PCCEIEN_R = crate::BitReader;
#[doc = "Field `PCCEIEN` writer - Conversion end interrupt enable for preempted channels"]
pub type PCCEIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SQEN` reader - Sequence mode enable"]
pub type SQEN_R = crate::BitReader;
#[doc = "Field `SQEN` writer - Sequence mode enable"]
pub type SQEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VMSGEN` reader - Voltage monitoring enable on a single channel"]
pub type VMSGEN_R = crate::BitReader;
#[doc = "Field `VMSGEN` writer - Voltage monitoring enable on a single channel"]
pub type VMSGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCAUTOEN` reader - Preempted group automatic conversion enable after ordinary group"]
pub type PCAUTOEN_R = crate::BitReader;
#[doc = "Field `PCAUTOEN` writer - Preempted group automatic conversion enable after ordinary group"]
pub type PCAUTOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCPEN` reader - Partitioned mode enable on ordinary channels"]
pub type OCPEN_R = crate::BitReader;
#[doc = "Field `OCPEN` writer - Partitioned mode enable on ordinary channels"]
pub type OCPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PCPEN` reader - Partitioned mode enable on preempted channels"]
pub type PCPEN_R = crate::BitReader;
#[doc = "Field `PCPEN` writer - Partitioned mode enable on preempted channels"]
pub type PCPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCPCNT` reader - Partitioned mode conversion count of ordinary channels"]
pub type OCPCNT_R = crate::FieldReader;
#[doc = "Field `OCPCNT` writer - Partitioned mode conversion count of ordinary channels"]
pub type OCPCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `PCVMEN` reader - Voltage monitoring enable on preempted channels"]
pub type PCVMEN_R = crate::BitReader;
#[doc = "Field `PCVMEN` writer - Voltage monitoring enable on preempted channels"]
pub type PCVMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OCVMEN` reader - Voltage monitoring enable on ordinary channels"]
pub type OCVMEN_R = crate::BitReader;
#[doc = "Field `OCVMEN` writer - Voltage monitoring enable on ordinary channels"]
pub type OCVMEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:4 - Voltage monitoring channel select"]
    #[inline(always)]
    pub fn vmcsel(&self) -> VMCSEL_R {
        VMCSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Channel conversion end interrupt enable"]
    #[inline(always)]
    pub fn cceien(&self) -> CCEIEN_R {
        CCEIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Voltage monitoring out of range interrupt enable"]
    #[inline(always)]
    pub fn vmorien(&self) -> VMORIEN_R {
        VMORIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Conversion end interrupt enable for preempted channels"]
    #[inline(always)]
    pub fn pcceien(&self) -> PCCEIEN_R {
        PCCEIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sequence mode enable"]
    #[inline(always)]
    pub fn sqen(&self) -> SQEN_R {
        SQEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Voltage monitoring enable on a single channel"]
    #[inline(always)]
    pub fn vmsgen(&self) -> VMSGEN_R {
        VMSGEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Preempted group automatic conversion enable after ordinary group"]
    #[inline(always)]
    pub fn pcautoen(&self) -> PCAUTOEN_R {
        PCAUTOEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Partitioned mode enable on ordinary channels"]
    #[inline(always)]
    pub fn ocpen(&self) -> OCPEN_R {
        OCPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Partitioned mode enable on preempted channels"]
    #[inline(always)]
    pub fn pcpen(&self) -> PCPEN_R {
        PCPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Partitioned mode conversion count of ordinary channels"]
    #[inline(always)]
    pub fn ocpcnt(&self) -> OCPCNT_R {
        OCPCNT_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 22 - Voltage monitoring enable on preempted channels"]
    #[inline(always)]
    pub fn pcvmen(&self) -> PCVMEN_R {
        PCVMEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Voltage monitoring enable on ordinary channels"]
    #[inline(always)]
    pub fn ocvmen(&self) -> OCVMEN_R {
        OCVMEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Voltage monitoring channel select"]
    #[inline(always)]
    #[must_use]
    pub fn vmcsel(&mut self) -> VMCSEL_W<CTRL1_SPEC, 0> {
        VMCSEL_W::new(self)
    }
    #[doc = "Bit 5 - Channel conversion end interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cceien(&mut self) -> CCEIEN_W<CTRL1_SPEC, 5> {
        CCEIEN_W::new(self)
    }
    #[doc = "Bit 6 - Voltage monitoring out of range interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmorien(&mut self) -> VMORIEN_W<CTRL1_SPEC, 6> {
        VMORIEN_W::new(self)
    }
    #[doc = "Bit 7 - Conversion end interrupt enable for preempted channels"]
    #[inline(always)]
    #[must_use]
    pub fn pcceien(&mut self) -> PCCEIEN_W<CTRL1_SPEC, 7> {
        PCCEIEN_W::new(self)
    }
    #[doc = "Bit 8 - Sequence mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn sqen(&mut self) -> SQEN_W<CTRL1_SPEC, 8> {
        SQEN_W::new(self)
    }
    #[doc = "Bit 9 - Voltage monitoring enable on a single channel"]
    #[inline(always)]
    #[must_use]
    pub fn vmsgen(&mut self) -> VMSGEN_W<CTRL1_SPEC, 9> {
        VMSGEN_W::new(self)
    }
    #[doc = "Bit 10 - Preempted group automatic conversion enable after ordinary group"]
    #[inline(always)]
    #[must_use]
    pub fn pcautoen(&mut self) -> PCAUTOEN_W<CTRL1_SPEC, 10> {
        PCAUTOEN_W::new(self)
    }
    #[doc = "Bit 11 - Partitioned mode enable on ordinary channels"]
    #[inline(always)]
    #[must_use]
    pub fn ocpen(&mut self) -> OCPEN_W<CTRL1_SPEC, 11> {
        OCPEN_W::new(self)
    }
    #[doc = "Bit 12 - Partitioned mode enable on preempted channels"]
    #[inline(always)]
    #[must_use]
    pub fn pcpen(&mut self) -> PCPEN_W<CTRL1_SPEC, 12> {
        PCPEN_W::new(self)
    }
    #[doc = "Bits 13:15 - Partitioned mode conversion count of ordinary channels"]
    #[inline(always)]
    #[must_use]
    pub fn ocpcnt(&mut self) -> OCPCNT_W<CTRL1_SPEC, 13> {
        OCPCNT_W::new(self)
    }
    #[doc = "Bit 22 - Voltage monitoring enable on preempted channels"]
    #[inline(always)]
    #[must_use]
    pub fn pcvmen(&mut self) -> PCVMEN_W<CTRL1_SPEC, 22> {
        PCVMEN_W::new(self)
    }
    #[doc = "Bit 23 - Voltage monitoring enable on ordinary channels"]
    #[inline(always)]
    #[must_use]
    pub fn ocvmen(&mut self) -> OCVMEN_W<CTRL1_SPEC, 23> {
        OCVMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
