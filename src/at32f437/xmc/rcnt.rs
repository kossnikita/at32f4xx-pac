#[doc = "Register `RCNT` reader"]
pub type R = crate::R<RCNT_SPEC>;
#[doc = "Register `RCNT` writer"]
pub type W = crate::W<RCNT_SPEC>;
#[doc = "Field `ERRC` writer - error flag clear"]
pub type ERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RC` reader - Refresh Count"]
pub type RC_R = crate::FieldReader<u16>;
#[doc = "Field `RC` writer - Refresh Count"]
pub type RC_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `ERIEN` reader - error Interrupt Enable"]
pub type ERIEN_R = crate::BitReader;
#[doc = "Field `ERIEN` writer - error Interrupt Enable"]
pub type ERIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:13 - Refresh Count"]
    #[inline(always)]
    pub fn rc(&self) -> RC_R {
        RC_R::new(((self.bits >> 1) & 0x1fff) as u16)
    }
    #[doc = "Bit 14 - error Interrupt Enable"]
    #[inline(always)]
    pub fn erien(&self) -> ERIEN_R {
        ERIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCNT")
            .field("rc", &self.rc())
            .field("erien", &self.erien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - error flag clear"]
    #[inline(always)]
    pub fn errc(&mut self) -> ERRC_W<'_, RCNT_SPEC> {
        ERRC_W::new(self, 0)
    }
    #[doc = "Bits 1:13 - Refresh Count"]
    #[inline(always)]
    pub fn rc(&mut self) -> RC_W<'_, RCNT_SPEC> {
        RC_W::new(self, 1)
    }
    #[doc = "Bit 14 - error Interrupt Enable"]
    #[inline(always)]
    pub fn erien(&mut self) -> ERIEN_W<'_, RCNT_SPEC> {
        ERIEN_W::new(self, 14)
    }
}
#[doc = "SDRAM Refresh Timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCNT_SPEC;
impl crate::RegisterSpec for RCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcnt::R`](R) reader structure"]
impl crate::Readable for RCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcnt::W`](W) writer structure"]
impl crate::Writable for RCNT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCNT to value 0"]
impl crate::Resettable for RCNT_SPEC {}
