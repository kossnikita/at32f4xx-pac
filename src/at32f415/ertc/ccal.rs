#[doc = "Register `CCAL` reader"]
pub type R = crate::R<CCAL_SPEC>;
#[doc = "Register `CCAL` writer"]
pub type W = crate::W<CCAL_SPEC>;
#[doc = "Field `CALVAL` reader - Calibration value"]
pub type CALVAL_R = crate::FieldReader;
#[doc = "Field `CALVAL` writer - Calibration value"]
pub type CALVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CALDIR` reader - Calibration direction"]
pub type CALDIR_R = crate::BitReader;
#[doc = "Field `CALDIR` writer - Calibration direction"]
pub type CALDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Calibration value"]
    #[inline(always)]
    pub fn calval(&self) -> CALVAL_R {
        CALVAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Calibration direction"]
    #[inline(always)]
    pub fn caldir(&self) -> CALDIR_R {
        CALDIR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCAL")
            .field("caldir", &self.caldir())
            .field("calval", &self.calval())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn calval(&mut self) -> CALVAL_W<CCAL_SPEC> {
        CALVAL_W::new(self, 0)
    }
    #[doc = "Bit 7 - Calibration direction"]
    #[inline(always)]
    #[must_use]
    pub fn caldir(&mut self) -> CALDIR_W<CCAL_SPEC> {
        CALDIR_W::new(self, 7)
    }
}
#[doc = "Calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCAL_SPEC;
impl crate::RegisterSpec for CCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccal::R`](R) reader structure"]
impl crate::Readable for CCAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccal::W`](W) writer structure"]
impl crate::Writable for CCAL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCAL to value 0"]
impl crate::Resettable for CCAL_SPEC {
    const RESET_VALUE: u32 = 0;
}
