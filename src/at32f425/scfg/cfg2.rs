#[doc = "Register `CFG2` reader"]
pub type R = crate::R<CFG2_SPEC>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<CFG2_SPEC>;
#[doc = "Field `PVM_LK` reader - PVM lock enable"]
pub type PVM_LK_R = crate::BitReader;
#[doc = "Field `PVM_LK` writer - PVM lock enable"]
pub type PVM_LK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S_FD` reader - I2S full duplex"]
pub type I2S_FD_R = crate::FieldReader;
#[doc = "Field `I2S_FD` writer - I2S full duplex"]
pub type I2S_FD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
            .field("pvm_lk", &self.pvm_lk())
            .field("i2s_fd", &self.i2s_fd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - PVM lock enable"]
    #[inline(always)]
    pub fn pvm_lk(&mut self) -> PVM_LK_W<'_, CFG2_SPEC> {
        PVM_LK_W::new(self, 2)
    }
    #[doc = "Bits 30:31 - I2S full duplex"]
    #[inline(always)]
    pub fn i2s_fd(&mut self) -> I2S_FD_W<'_, CFG2_SPEC> {
        I2S_FD_W::new(self, 30)
    }
}
#[doc = "configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for CFG2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for CFG2_SPEC {}
