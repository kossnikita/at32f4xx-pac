#[doc = "Register `CFG2` reader"]
pub type R = crate::R<CFG2_SPEC>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<CFG2_SPEC>;
#[doc = "Field `LOCKUP_LK` reader - CM4F LOCKUP bit enable"]
pub type LOCKUP_LK_R = crate::BitReader;
#[doc = "Field `LOCKUP_LK` writer - CM4F LOCKUP bit enable"]
pub type LOCKUP_LK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRAM_OPERR_LK` reader - SRAM odd parity error lock enable"]
pub type SRAM_OPERR_LK_R = crate::BitReader;
#[doc = "Field `SRAM_OPERR_LK` writer - SRAM odd parity error lock enable"]
pub type SRAM_OPERR_LK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PVM_LK` reader - PVM lock enable"]
pub type PVM_LK_R = crate::BitReader;
#[doc = "Field `PVM_LK` writer - PVM lock enable"]
pub type PVM_LK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SRAM_OPERR_STS` reader - SRAM odd parity error status"]
pub type SRAM_OPERR_STS_R = crate::BitReader;
#[doc = "Field `SRAM_OPERR_STS` writer - SRAM odd parity error status"]
pub type SRAM_OPERR_STS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2S_FD` reader - I2S full duplex config"]
pub type I2S_FD_R = crate::FieldReader;
#[doc = "Field `I2S_FD` writer - I2S full duplex config"]
pub type I2S_FD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0 - CM4F LOCKUP bit enable"]
    #[inline(always)]
    pub fn lockup_lk(&self) -> LOCKUP_LK_R {
        LOCKUP_LK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM odd parity error lock enable"]
    #[inline(always)]
    pub fn sram_operr_lk(&self) -> SRAM_OPERR_LK_R {
        SRAM_OPERR_LK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVM lock enable"]
    #[inline(always)]
    pub fn pvm_lk(&self) -> PVM_LK_R {
        PVM_LK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM odd parity error status"]
    #[inline(always)]
    pub fn sram_operr_sts(&self) -> SRAM_OPERR_STS_R {
        SRAM_OPERR_STS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 30:31 - I2S full duplex config"]
    #[inline(always)]
    pub fn i2s_fd(&self) -> I2S_FD_R {
        I2S_FD_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CM4F LOCKUP bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn lockup_lk(&mut self) -> LOCKUP_LK_W<CFG2_SPEC, 0> {
        LOCKUP_LK_W::new(self)
    }
    #[doc = "Bit 1 - SRAM odd parity error lock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sram_operr_lk(&mut self) -> SRAM_OPERR_LK_W<CFG2_SPEC, 1> {
        SRAM_OPERR_LK_W::new(self)
    }
    #[doc = "Bit 2 - PVM lock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvm_lk(&mut self) -> PVM_LK_W<CFG2_SPEC, 2> {
        PVM_LK_W::new(self)
    }
    #[doc = "Bit 8 - SRAM odd parity error status"]
    #[inline(always)]
    #[must_use]
    pub fn sram_operr_sts(&mut self) -> SRAM_OPERR_STS_W<CFG2_SPEC, 8> {
        SRAM_OPERR_STS_W::new(self)
    }
    #[doc = "Bits 30:31 - I2S full duplex config"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_fd(&mut self) -> I2S_FD_W<CFG2_SPEC, 30> {
        I2S_FD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for CFG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}