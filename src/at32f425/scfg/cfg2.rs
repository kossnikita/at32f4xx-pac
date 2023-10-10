#[doc = "Register `CFG2` reader"]
pub type R = crate::R<CFG2_SPEC>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<CFG2_SPEC>;
#[doc = "Field `PVM_LK` reader - PVM lock enable"]
pub type PVM_LK_R = crate::BitReader;
#[doc = "Field `PVM_LK` writer - PVM lock enable"]
pub type PVM_LK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2S_FD` reader - I2S full duplex"]
pub type I2S_FD_R = crate::FieldReader;
#[doc = "Field `I2S_FD` writer - I2S full duplex"]
pub type I2S_FD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 2 - PVM lock enable"]
    #[inline(always)]
    pub fn pvm_lk(&self) -> PVM_LK_R {
        PVM_LK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 30:31 - I2S full duplex"]
    #[inline(always)]
    pub fn i2s_fd(&self) -> I2S_FD_R {
        I2S_FD_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG2")
            .field("pvm_lk", &format_args!("{}", self.pvm_lk().bit()))
            .field("i2s_fd", &format_args!("{}", self.i2s_fd().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CFG2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2 - PVM lock enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvm_lk(&mut self) -> PVM_LK_W<CFG2_SPEC, 2> {
        PVM_LK_W::new(self)
    }
    #[doc = "Bits 30:31 - I2S full duplex"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_fd(&mut self) -> I2S_FD_W<CFG2_SPEC, 30> {
        I2S_FD_W::new(self)
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
