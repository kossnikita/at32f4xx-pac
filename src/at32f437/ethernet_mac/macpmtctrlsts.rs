#[doc = "Register `MACPMTCTRLSTS` reader"]
pub type R = crate::R<MACPMTCTRLSTS_SPEC>;
#[doc = "Register `MACPMTCTRLSTS` writer"]
pub type W = crate::W<MACPMTCTRLSTS_SPEC>;
#[doc = "Field `PD` reader - Power down"]
pub type PD_R = crate::BitReader;
#[doc = "Field `PD` writer - Power down"]
pub type PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EMP` reader - Enable magic packet"]
pub type EMP_R = crate::BitReader;
#[doc = "Field `EMP` writer - Enable magic packet"]
pub type EMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERWF` reader - Enable remote wakeup frame"]
pub type ERWF_R = crate::BitReader;
#[doc = "Field `ERWF` writer - Enable remote wakeup frame"]
pub type ERWF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RMP` reader - Received magic packet"]
pub type RMP_R = crate::BitReader;
#[doc = "Field `RMP` writer - Received magic packet"]
pub type RMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RRWF` reader - Recevied remote wakeup frame"]
pub type RRWF_R = crate::BitReader;
#[doc = "Field `RRWF` writer - Recevied remote wakeup frame"]
pub type RRWF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GUC` reader - Global unicast"]
pub type GUC_R = crate::BitReader;
#[doc = "Field `GUC` writer - Global unicast"]
pub type GUC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RWFFPR` reader - Remote wakeup frame filter register pointer reset"]
pub type RWFFPR_R = crate::BitReader;
#[doc = "Field `RWFFPR` writer - Remote wakeup frame filter register pointer reset"]
pub type RWFFPR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable magic packet"]
    #[inline(always)]
    pub fn emp(&self) -> EMP_R {
        EMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable remote wakeup frame"]
    #[inline(always)]
    pub fn erwf(&self) -> ERWF_R {
        ERWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Received magic packet"]
    #[inline(always)]
    pub fn rmp(&self) -> RMP_R {
        RMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Recevied remote wakeup frame"]
    #[inline(always)]
    pub fn rrwf(&self) -> RRWF_R {
        RRWF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn guc(&self) -> GUC_R {
        GUC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Remote wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn rwffpr(&self) -> RWFFPR_R {
        RWFFPR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<MACPMTCTRLSTS_SPEC, 0> {
        PD_W::new(self)
    }
    #[doc = "Bit 1 - Enable magic packet"]
    #[inline(always)]
    #[must_use]
    pub fn emp(&mut self) -> EMP_W<MACPMTCTRLSTS_SPEC, 1> {
        EMP_W::new(self)
    }
    #[doc = "Bit 2 - Enable remote wakeup frame"]
    #[inline(always)]
    #[must_use]
    pub fn erwf(&mut self) -> ERWF_W<MACPMTCTRLSTS_SPEC, 2> {
        ERWF_W::new(self)
    }
    #[doc = "Bit 5 - Received magic packet"]
    #[inline(always)]
    #[must_use]
    pub fn rmp(&mut self) -> RMP_W<MACPMTCTRLSTS_SPEC, 5> {
        RMP_W::new(self)
    }
    #[doc = "Bit 6 - Recevied remote wakeup frame"]
    #[inline(always)]
    #[must_use]
    pub fn rrwf(&mut self) -> RRWF_W<MACPMTCTRLSTS_SPEC, 6> {
        RRWF_W::new(self)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    #[must_use]
    pub fn guc(&mut self) -> GUC_W<MACPMTCTRLSTS_SPEC, 9> {
        GUC_W::new(self)
    }
    #[doc = "Bit 31 - Remote wakeup frame filter register pointer reset"]
    #[inline(always)]
    #[must_use]
    pub fn rwffpr(&mut self) -> RWFFPR_W<MACPMTCTRLSTS_SPEC, 31> {
        RWFFPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet MAC PMT control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macpmtctrlsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macpmtctrlsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPMTCTRLSTS_SPEC;
impl crate::RegisterSpec for MACPMTCTRLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macpmtctrlsts::R`](R) reader structure"]
impl crate::Readable for MACPMTCTRLSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macpmtctrlsts::W`](W) writer structure"]
impl crate::Writable for MACPMTCTRLSTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACPMTCTRLSTS to value 0"]
impl crate::Resettable for MACPMTCTRLSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
